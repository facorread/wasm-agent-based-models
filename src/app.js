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

"use strict";

// Use dynamic import only when necessary. The static form is preferable for loading initial dependencies, and can benefit more readily from static analysis tools and tree shaking. https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/import
import { MDCRipple } from '@material/ripple';
import { MDCSlider } from '@material/slider';
import { MDCSwitch } from '@material/switch';
import { MDCTextField } from '@material/textfield';

const abm_passive_listener = {
    passive: true,
}

function js_get_id(id) {
    let elem = document.getElementById(id);
    if (elem === null) {
        // console.error("Fatal: Element " + id + " was not found in index.html");
        throw new Error("Fatal: Element " + id + " was not found in index.html");
    }
    return elem;
}

function js_get_class(class_name) {
    let collection = document.getElementsByClassName(class_name);
    if (collection.length == 0) {
        // console.error("Fatal: Element " + id + " was not found in index.html");
        throw new Error("Fatal: No elements of class " + class_name + " were found in index.html");
    }
    return collection;
}

function js_visually_busy(expensive_closure) {
    let abm_get_busy = js_get_class("abm-get-busy");
    let was_already_busy = (abm_get_busy[0].classList.contains("abm-busy"));
    if (was_already_busy) {
        expensive_closure();
    } else {
        for (let el of abm_get_busy) {
            el.classList.add("abm-busy");
        }
        setTimeout(function () {
            expensive_closure();
            for (let el of abm_get_busy) {
                el.classList.remove("abm-busy");
            }
        }, 100);
    }
}

function js_message_with_color(msg, show_error) {
    let abm_logs = js_get_id("abm-logs");
    let p = document.createElement("p");
    p.innerText = msg;
    if (show_error) {
        js_get_id("abm-simulation-log").setAttribute("open", "");
        p.style.color = "red";
    }
    abm_logs.appendChild(p);
    abm_logs.scrollTop = abm_logs.scrollHeight;
}

export function js_message(msg) {
    js_message_with_color(msg, false);
}

export function js_error(msg) {
    js_message_with_color(msg, true);
}

function set_min_max_step(elem, value, min, max, step) {
    elem.setAttribute("value", value);
    elem.setAttribute("min", min);
    elem.setAttribute("max", max);
    elem.setAttribute("step", step);
}

// Links a continuous slider with a textfield to show the numerical value continuously.
// Set the minimum and maximum values in index.html.
class JsSliderValue {
    constructor(initial_value, text_min, text_max, slider_min, slider_max, step_size, use_logarithmic_slider, slider_id, text_id) {
        // number: initial_value, text_min, text_max, slider_min, slider_max, step_size
        // bool: use_logarithmic_slider
        // string: slider_id, text_id

        // Set the HTML attributes before initializing the Material Components
        let text_element = js_get_id(text_id);
        let slider_element = js_get_id(slider_id);
        set_min_max_step(text_element, initial_value, text_min, text_max, step_size);
        this.numeric_value = initial_value;
        this.slider_min = slider_min;
        this.slider_max = slider_max;
        this.text_min = text_min;
        this.text_max = text_max;
        this.step_size = step_size;
        this.logarithmic = use_logarithmic_slider;
        if (!(step_size > 0)) {
            js_error("Error: JsSliderValue " + slider_id + " has a step of " + step_size + ". Please make it a valid positive value.");
            throw new Error("Error: JsSliderValue " + slider_id + " has a step of " + step_size + ". Please make it a valid positive value.");
        }
        if (slider_min >= slider_max) {
            js_error("Error: Slider " + slider_id + " has a minimum of " + slider_min + ". Please review app.js to change it to a value smaller than " + slider_max + ".");
            throw new Error("Error: Slider " + slider_id + " has a minimum of " + slider_min + ". Please review app.js to change it to a value smaller than " + slider_max + ".");
        }
        if (slider_min < text_min) {
            js_error("Error: Slider " + slider_id + " has a minimum of " + slider_min + ". Please review index.html to change it to a value larger than " + text_min + ", which is the minimum of the text field.");
            throw new Error("Error: Slider " + slider_id + " has a minimum of " + slider_min + ". Please review index.html to change it to a value larger than " + text_min + ", which is the minimum of the text field.");
        }
        if (slider_max > text_max) {
            js_error("Error: Slider " + slider_id + " has a maximum of " + slider_max + ". Please review index.html to change it to a value smaller than " + text_max + ", which is the maximum of the text field.");
            throw new Error("Error: Slider " + slider_id + " has a maximum of " + slider_max + ". Please review index.html to change it to a value smaller than " + text_max + ", which is the maximum of the text field.");
        }
        if (use_logarithmic_slider) {
            if (slider_min <= 0) {
                js_error("Error: Logarithmic slider " + slider_id + " has a minimum of " + slider_min + ". Please review index.html to change it to a value larger than 0.");
                throw new Error("Error: Logarithmic slider " + slider_id + " has a minimum of " + slider_min + ". Please review index.html to change it to a value larger than 0.");
            }
            if (slider_max <= 0) {
                js_error("Error: Logarithmic slider " + slider_id + " has a maximum of " + slider_max + ". Please review index.html to change it to a value larger than 0.");
                throw new Error("Error: Logarithmic " + slider_id + " slider has a maximum of " + slider_max + ". Please review index.html to change it to a value larger than 0.");
            }
            this.log_min = Math.log(slider_min);
            this.log_scale = (Math.log(slider_max) - this.log_min) / (slider_max - slider_min);
            let log_step_size = slider_max - this.private_calculate_slider_position(slider_max - step_size);
            set_min_max_step(slider_element, this.private_calculate_slider_position(initial_value), slider_min, slider_max, log_step_size);
        } else {
            set_min_max_step(slider_element, initial_value, slider_min, slider_max, step_size);
        }

        // console.log("initial_value " + initial_value + " text_min " + text_min + " text_max " + text_max + " slider_min " + slider_min + " slider_max " + slider_max + " step_size " + step_size + " use_logarithmic_slider " + use_logarithmic_slider + " text_id " + text_id + " slider_id " + slider_id);
        this.slider = new MDCSlider(slider_element);
        this.text = new MDCTextField(text_element);

        this.private_set_slider_value(initial_value);
        this.text.value = initial_value.toString();
        this.valid = true;
        slider_element.addEventListener("MDCSlider:input", slider_listener, abm_passive_listener);
        text_element.addEventListener("input", text_listener, abm_passive_listener);
        text_element.addEventListener("change", text_change_listener, abm_passive_listener);
        let myself = this;
        text_listener();
        text_change_listener();
        function slider_listener(event) {
            if (myself.logarithmic) {
                if (myself.slider.getValue() < myself.slider_max) {
                    myself.private_parse_slider_position();
                } else {
                    myself.numeric_value = myself.slider_max;
                }
            } else {
                myself.numeric_value = myself.slider.getValue();
            }
            myself.numeric_value = Math.round((myself.numeric_value - myself.text_min) / myself.step_size) * myself.step_size + myself.text_min;
            myself.text.value = myself.numeric_value;
            myself.valid = true;
        }
        function text_listener(event) {
            if (myself.valid) {
                myself.numeric_value = parseFloat(myself.text.value);
                let new_slider_position;
                if (myself.logarithmic) {
                    if (myself.numeric_value > 0) {
                        new_slider_position = myself.private_calculate_slider_position(myself.numeric_value);
                    } else {
                        new_slider_position = myself.slider_min;
                    }
                } else {
                    new_slider_position = myself.numeric_value;
                }
                myself.private_set_slider_value(new_slider_position);
            }
        }
        function text_change_listener() {
            let parsed_value = parseFloat(myself.text.value);
            myself.text.value = parsed_value;
            if (!myself.valid) {
                myself.valid = false;
                if (myself.text.value == "") {
                    // throw new Error("Error: Make sure to enter a valid number.");
                    js_error("Error: Make sure to enter a valid number.");
                } else {
                    // throw new Error("Error: Make sure to enter a valid number instead of '" + myself.text.value + "'");
                    js_error("Error: Make sure to enter a valid number instead of '" + myself.text.value + "'");
                }
            }
        }
    }
    private_calculate_slider_position(value) {
        return ((Math.log(value) - this.log_min) / this.log_scale) + this.slider_min;
    }
    private_parse_slider_position() {
        this.numeric_value = Math.exp(this.log_min + this.log_scale * (this.slider.getValue() - this.slider_min));
    }
    private_set_slider_value(new_value) {
        if (new_value < this.slider_min) {
            this.slider.setValue(this.slider_min);
        } else if (new_value > this.slider_max) {
            this.slider.setValue(this.slider_max);
        } else {
            this.slider.setValue(new_value);
        }
    }
    layout() {
        // let r = this.slider.foundation.rect;
        // console.log("Slider rect left=" + r.left + " top=" + r.top + " width=" + r.width + " height=" + r.height + ".");
        this.slider.layout();
        // r = this.slider.foundation.rect;
        // console.log("Slider rect left=" + r.left + " top=" + r.top + " width=" + r.width + " height=" + r.height + ".");
        this.text.layout()
    }
    set value(new_value) {
        this.numeric_value = new_value;
        this.private_set_value(new_value)
    }
    get value() {
        return this.numeric_value;
    }
    get valid() {
        return this.text.valid;
    }
    set valid(new_value) {
        this.text.valid = new_value;
    }
}

function layout() {
    window.abm.nAgents0.layout();
    window.abm.worldLength.layout();
    window.abm.dark_figures_switch.layout();
    window.abm.fps.layout();
}

export function js_n0() {
    return window.abm.nAgents0.value;
}

export function js_world_length() {
    return window.abm.worldLength.value;
}

export function js_infection_probability() {
    return window.abm.infection_probability.value;
}

export function js_dark_figures() {
    return window.abm.dark_figures_switch.checked;
}

function rs_deploy_scenario() {
    window.abm.rs_mod.rs_deploy_scenario();
}

function frame_duration() {
    // The delay argument is converted to a signed 32-bit integer https://developer.mozilla.org/en-US/docs/Web/API/WindowOrWorkerGlobalScope/setInterval#Delay_restrictions
    // We convert it to integer first for comparison purposes.
    return Math.trunc(1000.0 / window.abm.fps.value);
}

export function js_scenario(rs_step_closure) {
    // let abm_body = js_get_id("abm-body");
    let abm_canvas = js_get_id("abm-canvas");
    // let abm_canvas_context = abm_canvas.getContext("2d");
    let start_stop = js_get_id("start-stop");
    let start_stop_label = js_get_id("start-stop-label");
    let abm_running = false;
    let interval_id;
    let abm_frame_duration = frame_duration();
    let abm_allow_buttons = true; // Prevents spurious/anxious clicks
    let abm_allow_step = true; // Prevents simulation from going too fast
    function step_handler() {
        if (abm_allow_step) {
            abm_allow_step = false;
            // The following code was supposed to scale the canvas to fit into the browser window. Unfortunately, Chrome and plotters distort the output plots.
            // let width = abm_body.offsetWidth;
            // let height = abm_body.offsetHeight;
            // abm_canvas.width = width;
            // abm_canvas.height = height;
            // let scaleX = width / 1280;
            // let scaleY = height / 720;
            // let abm_scale = Math.min(scaleX, scaleY);
            // js_message("Width " + width + " Height " + height + " scaleX " + scaleX + " scaleY " + scaleY + " abm_scale " + abm_scale + ".");
            // abm_canvas_context.scale(scaleX, scaleY);
            let new_frame_duration = frame_duration();
            if (window.abm.fps.valid && (abm_frame_duration != new_frame_duration)) {
                abm_frame_duration = new_frame_duration;
                clearInterval(interval_id);
                interval_id = setInterval(step_handler, abm_frame_duration);
            }
            rs_step_closure();
            abm_allow_step = true;
        }
    }
    js_visually_busy(step_handler); // Shows the first frame of the sim
    function stop_impl() {
        abm_running = false;
        start_stop_label.innerText = "play_arrow";
    }
    stop_impl();
    function start_stop_handler(event) {
        if (abm_allow_buttons) {
            abm_allow_buttons = false;
            if (abm_running) {
                clearInterval(interval_id);
                stop_impl();
            } else {
                abm_running = true;
                abm_frame_duration = frame_duration();
                step_handler();
                interval_id = setInterval(step_handler, abm_frame_duration);
                start_stop_label.innerText = "pause";
            }
            abm_allow_buttons = true;
        }
    };
    start_stop.addEventListener("click", start_stop_handler, abm_passive_listener);
    let reset_button = js_get_id("abm-reset-button");
    function reset_button_handler(event) {
        if (abm_allow_buttons) {
            abm_allow_buttons = false;
            js_visually_busy(function () {
                if (abm_running) {
                    clearInterval(interval_id);
                    stop_impl();
                }
                start_stop.removeEventListener("click", start_stop_handler, abm_passive_listener);
                reset_button.removeEventListener("click", reset_button_handler, abm_passive_listener);
                // Restart simulation here
                rs_deploy_scenario();
            });
            abm_allow_buttons = true;
        }
    };
    reset_button.addEventListener("click", reset_button_handler, abm_passive_listener);
}

function js_init() {
    {
        js_visually_busy(function () {
            for (let el of document.getElementsByClassName("mdc-fab")) {
                const fabRipple = new MDCRipple(el);
            }
            for (let el of document.getElementsByClassName("mdc-icon-button")) {
                const iconButtonRipple = new MDCRipple(el);
                iconButtonRipple.unbounded = true;
            }
            // Switches should be handled individually
            // for (let el of document.getElementsByClassName("mdc-switch")) {
            //     const switchControl = new MDCSwitch(el);
            // }
            window.abm = {};
            window.abm.nAgents0 = new JsSliderValue(1000, 1, 2000, 1, 2000, 1, true, "abm-n-agents-slider", "abm-n-agents-text");
            window.abm.worldLength = new JsSliderValue(10, 2, 200, 2, 200, 1, false, "abm-world-length-slider", "abm-world-length-text");
            window.abm.fps = new JsSliderValue(1, 0.25, 100, 0.25, 100, 0.25, false, "abm-fps-slider", "abm-fps-text");
            window.abm.dark_figures_switch = new MDCSwitch(js_get_id("abm-dark-mode-switch"));
            window.abm.infection_probability = new JsSliderValue(0.5, 0, 1, 0, 1, 0.01, false, "abm-infection-probability-slider", "abm-infection-probability-text");

            // Webpack requires WebAssembly to be a dynamic import for now.
            import("/pkg/index.js").then(function (module) {
                window.abm.rs_mod = module;
                rs_deploy_scenario();
            }, console.error);

        });
    }
    {
        let left_offset = 0, top_offset = 0, abm_card = js_get_id("abm-card");
        let drag_zones = Array.prototype.slice.call(document.getElementsByClassName("abm-draggable"));
        drag_zones.forEach(e => e.addEventListener("pointerdown", abm_pointer_down));

        function abm_prevent_extra_click(event) {
            event.preventDefault();
        }

        function abm_pointer_down(event) {
            event.preventDefault();
            if ((event.target !== null) && event.target.classList.contains("abm-draggable")) {
                left_offset = abm_card.offsetLeft - event.clientX;
                top_offset = abm_card.offsetTop - event.clientY;
                document.addEventListener("pointermove", abm_pointer_move);
                document.addEventListener("pointerup", abm_pointer_up);
                drag_zones.forEach(e => e.removeEventListener("pointerdown", abm_pointer_down));
            }
        }

        function abm_pointer_move(event) {
            event.preventDefault();
            abm_card.style.left = (left_offset + event.clientX) + "px";
            abm_card.style.top = (top_offset + event.clientY) + "px";
            event.target.addEventListener("click", abm_prevent_extra_click, { once: true });
        }

        function abm_pointer_up(event) {
            event.preventDefault();
            document.removeEventListener("pointermove", abm_pointer_move);
            document.removeEventListener("pointerup", abm_pointer_up);
            drag_zones.forEach(e => e.addEventListener("pointerdown", abm_pointer_down));
            abm_layout();
        }
    }
}

window.addEventListener('load', js_init);
