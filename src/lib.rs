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

mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, EventTarget, HtmlElement, Window};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Use the tags begin-similar-code and end-similar-code to mark a block of code that is similar between rust-agent-based-models and wasm-agent-based-models.
// begin-similar-code

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

// end-similar-code

#[wasm_bindgen]
pub fn rs_init() {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    let window = web_sys::window().expect("Trying to communicate with the web browser window");
    let document = window
        .document()
        .expect("Trying to communicate with the HTML document");

    let abm_message = |msg: &str| {
        // let message_element: &'static HtmlElement =
        document
            .get_element_by_id("abmmessage")
            .expect("Setting up Messages")
            .dyn_ref::<HtmlElement>()
            .expect("Setting up the Messages element")
            .set_inner_text(msg);
    };
    abm_message("Hello, wasm-agent-based-models!");

    // begin-similar-code
    // Model parameter: Initial number of agents
    let n0: usize = 1000;
    // Model parameter: Scale-free network parameter: new links per agent
    #[cfg(feature = "net")]
    let net_k: usize = 7;
    // Model parameter: Dimensions of the virtual landscape, in number of cells
    #[cfg(feature = "landscape")]
    let coord = WrappingCoords2d::new(100, 100).unwrap();
    let birth_distro = Bernoulli::new(0.01).unwrap();
    let initial_infection_distro = Bernoulli::new(0.3).unwrap();
    // Normal distribution to choose cells in the landscape
    #[cfg(feature = "landscape")]
    let visit_distro = Normal::new(50.0f32, 10f32).unwrap();
    #[cfg(feature = "net")]
    let link_distro = Bernoulli::new(0.01).unwrap();
    let recovery_distro = Bernoulli::new(0.8).unwrap();
    let survival_distro = Bernoulli::new(0.8).unwrap();
    // end-similar-code
    let mut scenario = Scenario::default();
    {
        // begin-similar-code
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
        // end-similar-code
        let model_mutex = std::sync::Mutex::new(0);
        let time_step_closure = Closure::wrap(Box::new(move || {
            let mut time_step_guard = model_mutex.lock().unwrap();
            let mut time_step_results: TimeStepResults = Default::default();

            // begin-similar-code
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
                time_step_results.time_step = *time_step_guard as u32;
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
                if *time_step_guard == 0 {
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
                                if cell_health[idx] == Health::I
                                    && infection_distro.sample(&mut rng)
                                {
                                    // Cell infects agent
                                    next_health.insert(k, Health::I);
                                }
                            }
                        }
                        Health::I => {
                            #[cfg(feature = "landscape")]
                            {
                                if cell_health[idx] == Health::S
                                    && infection_distro.sample(&mut rng)
                                {
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
            // end-similar-code
            *time_step_guard = 1;
        }) as Box<dyn FnMut()>);
        // .set_onclick(Some(time_step_closure.as_ref().unchecked_ref()));
        document
            .get_element_by_id("abmmenuheader")
            .expect("Trying to find the Step button")
            .dyn_ref::<EventTarget>()
            .expect("Setting click event")
            .add_event_listener_with_callback(
                &"click".to_owned(),
                time_step_closure.as_ref().unchecked_ref(),
            )
            .unwrap();
        time_step_closure.forget();
    }
}
