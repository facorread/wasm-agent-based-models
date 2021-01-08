/* This file is part of wasm-agent-based-models:
   Reliable and efficient agent-based models in Rust and WebAssembly

    Copyright 2020 Fabio A. Correa Duran facorread@gmail.com

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen(module = "/src/app.js")]
extern "C" {
    // This project used to have a pub type JsAbmParams. Unfortunately, limitations on wasm-bindgen make this impractical. See
    // https://github.com/rustwasm/wasm-bindgen/issues/1187
    // https://docs.rs/wasm-bindgen/0.2.69/wasm_bindgen/closure/struct.Closure.html
    fn js_n0() -> u32;
    fn js_world_length() -> i32;
    fn js_dark_figures() -> bool;
    fn js_scenario(rs_step_closure: &::js_sys::Function);
    fn js_message(msg: &str);
    fn js_error(msg: &str);
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Use the tags begin-similar-code and end-similar-code to mark a block of code that is similar between rust-agent-based-models and wasm-agent-based-models.
// begin-similar-code 0

///! This software uses the Entity-Component-System (ECS) architecture and other principles discussed at https://kyren.github.io/2018/09/14/rustconf-talk.html
#[cfg(feature = "graphics")]
use plotters::prelude::*;
#[cfg(feature = "net")]
use rand::distributions::weighted::{WeightedError, WeightedIndex};
use rand::distributions::{Bernoulli, Distribution};
#[cfg(feature = "landscape")]
use rand_distr::Normal;
use slotmap::{SecondaryMap, SlotMap};
#[cfg(feature = "net-graphics")]
use std::collections::{BTreeMap, BTreeSet};
// use std::fmt::Write as FmtWrite; // See https://doc.rust-lang.org/std/macro.writeln.html
#[cfg(feature = "csv-output")]
use std::io::Write as IoWrite; // See https://doc.rust-lang.org/std/macro.writeln.html
#[cfg(feature = "landscape")]
use wrapping_coords2d::WrappingCoords2d;

// Model properties
#[derive(Clone, Copy, PartialEq)]
enum Health {
    S,
    I,
}

// Housekeeping
slotmap::new_key_type! {
    struct AgentKey;
    struct LinkKey;
}

/// Simulation results for a time step
#[derive(Clone, Default)]
struct TimeStepResults {
    /// Time step
    time_step: u32,
    /// Number of agents
    #[cfg(feature = "agent-metrics")]
    n: u32,
    /// Susceptibles
    #[cfg(feature = "agent-metrics")]
    s: u32,
    /// Infected
    #[cfg(feature = "agent-metrics")]
    i: u32,
    /// Maximum network degree of susceptibles
    #[cfg(feature = "net-metrics")]
    d_s: i32,
    /// Maximum network degree of infectious
    #[cfg(feature = "net-metrics")]
    d_i: i32,
    /// Infected cells
    #[cfg(feature = "landscape-metrics")]
    c_i: u32,
    /// Histogram of network degrees
    #[cfg(feature = "net-graphics")]
    degree_histogram: BTreeMap<i32, u32>,
    /// Health status
    #[cfg(feature = "landscape-graphics")]
    cell_health: Vec<Health>,
}

/// Simulation scenario, including parameters and results
#[derive(Clone, Default)]
struct Scenario {
    /// Sequential scenario number
    id: u32,
    /// Model parameter: Infection probability
    infection_probability: f64,
    /// Simulation results: Set of network degrees that ever ocurred in this scenario
    #[cfg(feature = "net-graphics")]
    histogram_degrees_set: BTreeSet<i32>,
    /// Simulation results: Maximum network degree that ever ocurred in this scenario
    #[cfg(feature = "net-graphics")]
    histogram_max_degree: i32,
    /// Simulation results: Height of the network degree histogram for this scenario
    #[cfg(feature = "net-graphics")]
    histogram_height: u32,
    /// Simulation results: Height of the time series figure for agents for this scenario
    #[cfg(feature = "graphics")]
    agent_time_series_height: u32,
    /// Simulation results: Height of the time series figure for agents for this scenario
    #[cfg(feature = "landscape-graphics")]
    cell_time_series_height: u32,
    /// Simulation results for all time steps
    time_series: std::vec::Vec<TimeStepResults>,
}

// end-similar-code 0

#[wasm_bindgen]
pub fn rs_deploy_scenario() {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    // let window = web_sys::window().expect("Trying to communicate with the web browser window");
    // let document = window
    //     .document()
    //     .expect("Trying to communicate with the HTML document");

    // js_message(&format!(
    //     "Welcome to wasm-agent-based-models! n0 = {}",
    //     js_n0()
    // ));

    let world_length = js_world_length();
    // begin-similar-code 1
    // Model parameter: Initial number of agents
    let n0: usize = js_n0() as usize;
    // Model parameter: Scale-free network parameter: new links per agent
    #[cfg(feature = "net")]
    let net_k: usize = 7;
    // Model parameter: Dimensions of the virtual landscape, in number of cells
    #[cfg(feature = "landscape")]
    let coord = WrappingCoords2d::new(world_length, world_length).unwrap();
    let birth_distro = Bernoulli::new(0.01).unwrap();
    let initial_infection_distro = Bernoulli::new(0.3).unwrap();
    // Normal distribution to choose cells in the landscape
    #[cfg(feature = "landscape")]
    let visit_distro = Normal::new(50.0f32, 10f32).unwrap();
    #[cfg(feature = "net")]
    let link_distro = Bernoulli::new(0.01).unwrap();
    let recovery_distro = Bernoulli::new(0.8).unwrap();
    let survival_distro = Bernoulli::new(0.8).unwrap();
    // end-similar-code 1
    #[cfg(feature = "net-graphics")]
    let compress_histogram = true;
    let mut scenario = Scenario::default();

    // begin-similar-code 2
    // Use Pcg64 for reproducible random numbers; change to thread_rng for production
    // let mut rng = rand::thread_rng();
    #[allow(clippy::unreadable_literal)]
    let mut rng = rand_pcg::Pcg64::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7ac28fa16a64abf96);
    // Model state: Agent health
    let mut health = SlotMap::with_capacity_and_key(2 * n0);
    // Model state: Bidirectional links between agents
    #[cfg(feature = "net")]
    let mut links = slotmap::SlotMap::with_capacity_and_key(n0 * n0);
    // Model state: Health status of each cell in the landscape
    #[cfg(feature = "landscape")]
    let mut cell_health = vec![Health::S; coord.size()];
    // Model state: Cell health storage for the next time step. This implements parallel updating of cells.
    #[cfg(feature = "landscape")]
    let mut next_cell_health = cell_health.clone();
    // Model initialization: Agents
    while health.len() < n0 {
        let _k: AgentKey = health.insert(Health::S);
    }
    let infection_distro = Bernoulli::new(scenario.infection_probability).unwrap();
    // end-similar-code 2
    let mut time_step = 0;
    let rs_step_closure = Closure::wrap(Box::new(move || {
        let mut time_step_results: TimeStepResults = Default::default();

        // begin-similar-code 3
        // Initialization of this time step: Network seed
        #[cfg(feature = "net")]
        {
            if links.is_empty() && health.len() > 1 {
                let mut h_it = health.iter();
                let (key0, _value) = h_it.next().unwrap();
                let (key1, _value) = h_it.next().unwrap();
                let _link_id: LinkKey = links.insert((key0, key1));
            }
            // Initialization of this time step: Network
            let keys_vec: Vec<AgentKey> = health.keys().collect();
            let mut idx_map = SecondaryMap::with_capacity(health.capacity());
            let mut weights_vec: Vec<i32> = {
                let mut weights_map = SecondaryMap::with_capacity(health.capacity());
                keys_vec.iter().enumerate().for_each(|(idx, &k)| {
                    weights_map.insert(k, 0);
                    idx_map.insert(k, idx);
                });
                links.values().for_each(|&(key0, key1)| {
                    weights_map[key0] += 1;
                    weights_map[key1] += 1;
                });
                keys_vec.iter().map(|&k| weights_map[k]).collect()
            };
            keys_vec
                .iter()
                .enumerate()
                .for_each(|(agent_idx, &agent_key)| {
                    let new_links = if weights_vec[agent_idx] == 0 {
                        net_k
                    } else if link_distro.sample(&mut rng) {
                        1
                    } else {
                        0
                    };
                    if new_links > 0 {
                        let mut weights_tmp = weights_vec.clone();
                        // This agent cannot make a link to itself; set its weight to 0.
                        weights_tmp[agent_idx] = 0;
                        // Friends are ineligible for a new link; set friends' weights to 0.
                        links.values().for_each(|&(key0, key1)| {
                            if key0 == agent_key {
                                weights_tmp[idx_map[key1]] = 0;
                            }
                            if key1 == agent_key {
                                weights_tmp[idx_map[key0]] = 0;
                            }
                        });
                        match WeightedIndex::new(weights_tmp) {
                            Ok(mut dist) => {
                                let mut k = 0;
                                loop {
                                    let friend_idx = dist.sample(&mut rng);
                                    links.insert((agent_key, keys_vec[friend_idx]));
                                    weights_vec[agent_idx] += 1;
                                    weights_vec[friend_idx] += 1;
                                    k += 1;
                                    if k == new_links {
                                        break;
                                    }
                                    // Make friend ineligible for a new link; set its weight to 0.
                                    if dist.update_weights(&[(friend_idx, &0)]).is_err() {
                                        break;
                                    }
                                }
                            }
                            Err(WeightedError::AllWeightsZero) => {}
                            Err(e) => {
                                panic!("Internal error OsXJWc0sHx: {}. Please debug.", e)
                            }
                        }
                    }
                });
            // Model measurements: Network
            #[cfg(feature = "net-metrics")]
            {
                time_step_results.d_s = match keys_vec
                    .iter()
                    .zip(weights_vec.iter())
                    .filter(|(&k, _w)| health[k] == Health::S)
                    .max_by_key(|(_k, &w)| w)
                {
                    Some((_k, &w)) => w,
                    None => 0,
                };
                time_step_results.d_i = match keys_vec
                    .iter()
                    .zip(weights_vec.iter())
                    .filter(|(&k, _w)| health[k] == Health::I)
                    .max_by_key(|(_k, &w)| w)
                {
                    Some((_k, &w)) => w,
                    None => 0,
                };
            }
            #[cfg(feature = "net-graphics")]
            {
                for weight in weights_vec {
                    *time_step_results
                        .degree_histogram
                        .entry(weight)
                        .or_insert(0) += 1;
                }
                for (&weight, &frequency) in &time_step_results.degree_histogram {
                    if compress_histogram {
                        scenario.histogram_degrees_set.insert(weight);
                    } else if scenario.histogram_max_degree < weight {
                        scenario.histogram_max_degree = weight;
                    }
                    if scenario.histogram_height < frequency {
                        scenario.histogram_height = frequency;
                    }
                }
            }
        }
        // Model measurements: agents
        {
            time_step_results.time_step = time_step;
            #[cfg(feature = "agent-metrics")]
            {
                time_step_results.n = health.len() as u32;
                health.values().for_each(|h| match h {
                    Health::S => time_step_results.s += 1,
                    Health::I => time_step_results.i += 1,
                });
            }
            #[cfg(feature = "landscape-metrics")]
            {
                time_step_results.c_i =
                    cell_health.iter().filter(|&&h| h == Health::I).count() as u32;
            }
            #[cfg(feature = "graphics")]
            {
                if scenario.agent_time_series_height < time_step_results.n {
                    scenario.agent_time_series_height = time_step_results.n;
                }
            }
            #[cfg(feature = "landscape-graphics")]
            {
                if scenario.cell_time_series_height < time_step_results.c_i {
                    scenario.cell_time_series_height = time_step_results.c_i;
                }
                time_step_results.cell_health = cell_health.clone();
            }
        }
        // Dynamics: infection spreads
        {
            // Model state: Agent health the next time step
            let mut next_health = SecondaryMap::with_capacity(health.capacity());
            #[cfg(feature = "net")]
            links.values().for_each(|&(key0, key1)| {
                let h0 = health[key0];
                let h1 = health[key1];
                if h0 == Health::S && h1 == Health::I && infection_distro.sample(&mut rng) {
                    next_health.insert(key0, Health::I);
                }
                if h1 == Health::S && h0 == Health::I && infection_distro.sample(&mut rng) {
                    next_health.insert(key1, Health::I);
                }
            });
            if time_step == 0 {
                health.iter().for_each(|(k, &h)| {
                    if h == Health::S && initial_infection_distro.sample(&mut rng) {
                        next_health.insert(k, Health::I);
                    }
                });
            }
            health.iter().for_each(|(k, &h)| {
                // Choose a random cell to visit
                #[cfg(feature = "landscape")]
                let x = visit_distro.sample(&mut rng) as i32;
                #[cfg(feature = "landscape")]
                let y = visit_distro.sample(&mut rng) as i32;
                #[cfg(feature = "landscape")]
                let idx = coord.index(x, y);
                match h {
                    Health::S => {
                        #[cfg(feature = "landscape")]
                        {
                            if cell_health[idx] == Health::I && infection_distro.sample(&mut rng) {
                                // Cell infects agent
                                next_health.insert(k, Health::I);
                            }
                        }
                    }
                    Health::I => {
                        #[cfg(feature = "landscape")]
                        {
                            if cell_health[idx] == Health::S && infection_distro.sample(&mut rng) {
                                // Agent infects cell
                                next_cell_health[idx] = Health::I;
                            }
                        }
                        if recovery_distro.sample(&mut rng) {
                            next_health.insert(k, Health::S);
                        }
                    }
                };
            });
            // Dynamics: Disease spreads across cells and infectious cells recover
            #[cfg(feature = "landscape")]
            coord.for_each8(
                |this_cell_index, neighbors| match cell_health[this_cell_index] {
                    Health::S => {
                        for neighbor_index in neighbors {
                            if cell_health[*neighbor_index] == Health::I
                                && infection_distro.sample(&mut rng)
                            {
                                next_cell_health[this_cell_index] = Health::I;
                                break;
                            }
                        }
                    }
                    Health::I => {
                        if recovery_distro.sample(&mut rng) {
                            next_cell_health[this_cell_index] = Health::S;
                        }
                    }
                },
            );
            // Dynamics: After spreading the infection, some infectious agents die
            health.retain(|_agent_key, h| match h {
                Health::S => true,
                Health::I => survival_distro.sample(&mut rng),
            });
            // Dynamics: Remaining agents update in parallel
            next_health.iter().for_each(|(k, &next_h)| {
                if let Some(h) = health.get_mut(k) {
                    *h = next_h;
                }
            });
            // Dynamics: cells update in parallel
            #[cfg(feature = "landscape")]
            {
                cell_health = next_cell_health.clone();
            }
        }
        // Dynamics: Prune network
        #[cfg(feature = "net")]
        links.retain(|_link_key, (key0, key1)| {
            health.contains_key(*key0) && health.contains_key(*key1)
        });
        // Dynamics: New agents emerge
        let nb = health
            .values()
            .filter(|&&h| h == Health::S && birth_distro.sample(&mut rng))
            .count();
        for _ in 0..nb {
            health.insert(Health::S);
        }
        // end-similar-code 3
        let time_series_len = core::cmp::max(time_step, 10);
        // begin-similar-code 4
        #[cfg(feature = "graphics")]
        let mut agent_time_series_height = 0;
        #[cfg(feature = "landscape-graphics")]
        let mut cell_time_series_height = 0;
        #[cfg(feature = "net-graphics")]
        let mut histogram_degrees_set = BTreeSet::new();
        #[cfg(feature = "net-graphics")]
        let mut histogram_max_degree = 0;
        #[cfg(feature = "net-graphics")]
        let mut histogram_height = 0;
        // end-similar-code 4
        // begin-similar-code 5
        #[cfg(feature = "graphics")]
        {
            #[cfg(feature = "net-graphics")]
            {
                if compress_histogram {
                    for degree in scenario.histogram_degrees_set.iter() {
                        histogram_degrees_set.insert(degree);
                    }
                } else if histogram_max_degree < scenario.histogram_max_degree {
                    histogram_max_degree = scenario.histogram_max_degree;
                }
                if histogram_height < scenario.histogram_height {
                    histogram_height = scenario.histogram_height;
                }
            }
            if agent_time_series_height < scenario.agent_time_series_height {
                agent_time_series_height = scenario.agent_time_series_height;
            }
        }
        #[cfg(feature = "landscape-graphics")]
        {
            if cell_time_series_height < scenario.cell_time_series_height {
                cell_time_series_height = scenario.cell_time_series_height;
            }
        }
        // end-similar-code 5
        #[cfg(feature = "landscape-graphics")]
        if cell_time_series_height == 0 {
            cell_time_series_height = 10;
        }
        // begin-similar-code 6
        #[cfg(feature = "net-graphics")]
        {
            if compress_histogram {
                assert!(!histogram_degrees_set.is_empty());
            }
            assert!(histogram_height > 0);
        }
        assert!(agent_time_series_height > 0);
        #[cfg(feature = "landscape-graphics")]
        assert!(cell_time_series_height > 0);
        #[cfg(feature = "net-graphics")]
        {
            // A little extra space in the chart:
            histogram_height += 1;
            histogram_max_degree += 1;
        }
        agent_time_series_height += 1;
        #[cfg(feature = "landscape-graphics")]
        {
            cell_time_series_height += 1;
        }
        #[cfg(feature = "net-graphics")]
        let x_degree: std::vec::Vec<_> = histogram_degrees_set.iter().enumerate().collect();
        let figure_margin = 5;
        #[cfg(feature = "net-graphics")]
        let bar_margin = 3;
        let thick_stroke = 4;
        let text_size0 = 30;
        let text_size1 = 17;
        let x_label_area_size = 40;
        #[cfg(feature = "net-graphics")]
        let x_label_offset = 1;
        let y_label_area_size = 60;
        // end-similar-code 6
        let dark_figures = js_dark_figures();
        let canvas_backend =
            plotters_canvas::CanvasBackend::new("abm-canvas").expect("cannot find canvas");
        use plotters::drawing::IntoDrawingArea;
        let drawing_area = canvas_backend.into_drawing_area();
        // begin-similar-code 7
        let background_color = if dark_figures { &BLACK } else { &WHITE };
        let _transparent_color = background_color.mix(0.);
        let color0 = if dark_figures { &WHITE } else { &BLACK };
        let color01 = color0.mix(0.1);
        let color02 = color0.mix(0.2);
        let color1 = if dark_figures {
            &plotters::style::RGBColor(255, 192, 0)
        } else {
            &RED
        };
        let color2 = &plotters::style::RGBColor(0, 176, 80);
        let color3 = &plotters::style::RGBColor(32, 56, 100);
        let color_s = color2;
        let color_i = color3;
        let color0t = color0.stroke_width(thick_stroke);
        let _color1t = color1.stroke_width(thick_stroke);
        let color2t = color2.stroke_width(thick_stroke);
        let color3t = color3.stroke_width(thick_stroke);
        let _color_st = color2t;
        let color_it = color3t;
        let _fill0 = color0.filled();
        let _fill01 = color01.filled();
        let _fill02 = color02.filled();
        let _fill1 = color1.filled();
        let _fill2 = color2.filled();
        let _fill3 = color3.filled();
        let text0 = ("Calibri", text_size0).into_font().color(color0);
        let text1 = ("Calibri", text_size1).into_font().color(color0);
        drawing_area.fill(background_color).unwrap();
        let (left_area, right_area) = drawing_area.split_horizontally(1920 - 1080);
        let left_panels = left_area.split_evenly((4, 1));
        left_panels[0]
            .draw_text(
                &format!("infection_probability = {}", scenario.infection_probability),
                &text0,
                (50, 10),
            )
            .unwrap();
        #[cfg(feature = "net-graphics")]
        {
            left_panels[0]
                .draw_text(
                    &format!("d_s Max degree of susceptibles: {}", time_step_results.d_s),
                    &text0,
                    (50, 100),
                )
                .unwrap();
            left_panels[0]
                .draw_text(
                    &format!(
                        "d_i Max degree of infectious agents: {}",
                        time_step_results.d_i
                    ),
                    &text0,
                    (50, 140),
                )
                .unwrap();
        }
        left_panels[0]
            .draw_text(
                &format!("time: {}", time_step_results.time_step),
                &text0,
                (500, 10),
            )
            .unwrap();
        #[cfg(feature = "net-graphics")]
        {
            let x_range = if compress_histogram {
                0..x_degree.len() as i32
            } else {
                0..histogram_max_degree
            };
            let mut chart = ChartBuilder::on(&left_panels[1])
                .x_label_area_size(x_label_area_size)
                .y_label_area_size(y_label_area_size)
                .margin(figure_margin)
                .caption("Network degree of agents", text0.clone())
                .build_cartesian_2d(x_range, 0..histogram_height)
                .unwrap();
            chart
                .configure_mesh()
                .light_line_style(&color01)
                .bold_line_style(&color02)
                .y_desc("Number of agents")
                .x_desc(if compress_histogram {
                    "Network degree (removing zeroes)"
                } else {
                    "Network degree"
                })
                .axis_style(color0)
                .axis_desc_style(text1.clone())
                .label_style(text1.clone())
                .x_label_offset(x_label_offset)
                .x_label_formatter(&|x_position| {
                    if compress_histogram {
                        match x_degree.get(*x_position as usize) {
                            Some(x_deg) => format!("{}", x_deg.1),
                            None => format!(""),
                        }
                    } else {
                        format!("{}", x_position)
                    }
                })
                .draw()
                .unwrap();
            chart
                .draw_series(
                    Histogram::vertical(&chart)
                        .style(background_color.filled())
                        .margin(bar_margin)
                        .data(
                            time_step_results
                                .degree_histogram
                                .iter()
                                .map(|(degree, weight)| {
                                    (
                                        if compress_histogram {
                                            x_degree
                                                .iter()
                                                .find(|&(_, &deg)| deg == degree)
                                                .unwrap()
                                                .0
                                                as i32
                                        } else {
                                            *degree
                                        },
                                        *weight,
                                    )
                                }),
                        ),
                )
                .unwrap();
            chart
                .draw_series(
                    Histogram::vertical(&chart)
                        .style(color0)
                        .margin(bar_margin)
                        .data(
                            time_step_results
                                .degree_histogram
                                .iter()
                                .map(|(degree, weight)| {
                                    (
                                        if compress_histogram {
                                            x_degree
                                                .iter()
                                                .find(|&(_, &deg)| deg == degree)
                                                .unwrap()
                                                .0
                                                as i32
                                        } else {
                                            *degree
                                        },
                                        *weight,
                                    )
                                }),
                        ),
                )
                .unwrap();
        }
        {
            let mut chart = ChartBuilder::on(&left_panels[2])
                .x_label_area_size(x_label_area_size)
                .y_label_area_size(y_label_area_size)
                .margin(figure_margin)
                .caption("Populations of agents", text0.clone())
                .build_cartesian_2d(0..(time_series_len as u32), 0..agent_time_series_height)
                .unwrap();
            chart
                .configure_mesh()
                .light_line_style(&color01)
                .bold_line_style(&color02)
                .y_desc("Number of agents")
                .x_desc("Time")
                .axis_style(color0)
                .axis_desc_style(text1.clone())
                .label_style(text1.clone())
                .draw()
                .unwrap();
            chart
                .draw_series(LineSeries::new(
                    scenario
                        .time_series
                        .iter()
                        .skip_while(|tsr| tsr.time_step < time_step_results.time_step)
                        .map(|time_step_results| {
                            (time_step_results.time_step, time_step_results.n)
                        }),
                    color0,
                ))
                .unwrap();
            chart
                .draw_series(LineSeries::new(
                    scenario
                        .time_series
                        .iter()
                        .take_while(|tsr| tsr.time_step <= time_step_results.time_step)
                        .map(|time_step_results| {
                            (time_step_results.time_step, time_step_results.n)
                        }),
                    color0t.clone(),
                ))
                .unwrap()
                .label("n Number of agents")
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color0t.clone()));
            chart
                .draw_series(LineSeries::new(
                    scenario
                        .time_series
                        .iter()
                        .skip_while(|tsr| tsr.time_step < time_step_results.time_step)
                        .map(|time_step_results| {
                            (time_step_results.time_step, time_step_results.i)
                        }),
                    color_i,
                ))
                .unwrap();
            chart
                .draw_series(LineSeries::new(
                    scenario
                        .time_series
                        .iter()
                        .take_while(|tsr| tsr.time_step <= time_step_results.time_step)
                        .map(|time_step_results| {
                            (time_step_results.time_step, time_step_results.i)
                        }),
                    color_it.clone(),
                ))
                .unwrap()
                .label("i Infected agents")
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color_it.clone()));
            chart
                .configure_series_labels()
                .label_font(text1.clone())
                .border_style(color0)
                .draw()
                .unwrap();
        }
        #[cfg(feature = "landscape")]
        {
            let mut chart = ChartBuilder::on(&left_panels[3])
                .x_label_area_size(x_label_area_size)
                .y_label_area_size(y_label_area_size)
                .margin(figure_margin)
                .caption("Infection of cells", text0.clone())
                .build_cartesian_2d(0..(time_series_len as u32), 0..cell_time_series_height)
                .unwrap();
            chart
                .configure_mesh()
                .light_line_style(&color01)
                .bold_line_style(&color02)
                .y_desc("Number of infected cells")
                .x_desc("Time")
                .axis_style(color0)
                .axis_desc_style(text1.clone())
                .label_style(text1)
                .draw()
                .unwrap();
            chart
                .draw_series(LineSeries::new(
                    scenario
                        .time_series
                        .iter()
                        .skip_while(|tsr| tsr.time_step < time_step_results.time_step)
                        .map(|time_step_results| {
                            (time_step_results.time_step, time_step_results.c_i)
                        }),
                    color_i,
                ))
                .unwrap();
            chart
                .draw_series(LineSeries::new(
                    scenario
                        .time_series
                        .iter()
                        .take_while(|tsr| tsr.time_step <= time_step_results.time_step)
                        .map(|time_step_results| {
                            (time_step_results.time_step, time_step_results.c_i)
                        }),
                    color_it,
                ))
                .unwrap();
        }
        #[cfg(feature = "landscape")]
        {
            let landscape = right_area.margin(10, 10, 10, 10);
            let cells = landscape.split_evenly((coord.height() as usize, coord.width() as usize));
            cells
                .iter()
                .zip(time_step_results.cell_health.iter())
                .for_each(|(cell, health)| {
                    cell.fill(match health {
                        Health::S => color_s,
                        Health::I => color_i,
                    })
                    .unwrap();
                });
        }
        // end-similar-code 7

        time_step += 1;
    }) as Box<dyn FnMut()>);
    js_scenario(rs_step_closure.as_ref().unchecked_ref());
    rs_step_closure.forget();
}
