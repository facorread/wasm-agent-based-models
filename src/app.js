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
import { MDCTextField } from '@material/textfield';

function js_get_id(id) {
    let elem = document.getElementById(id);
    if (elem === null) {
        // console.error("Fatal: Element " + id + " was not found in index.html");
        throw new Error("Fatal: Element " + id + " was not found in index.html");
    }
    return elem;
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

// Links a continuous slider with a textfield to show the numerical value continuously.
// Set the minimum and maximum values in index.html.
class JsSliderValue {
    constructor(initial_value, use_logarithmic_slider, slider_id, text_id) {
        let slider_element = js_get_id(slider_id);
        this.slider = new MDCSlider(slider_element);
        let text_element = js_get_id(text_id);
        this.text = new MDCTextField(text_element);

        if (this.slider.foundation.min < this.text.min) {
            js_error("Error: The slider has a minimum of " + this.slider.foundation.min + ". Please review index.html to change it to a value larger than " + this.text.min + ", which is the minimum of the text field.");
            throw new Error("Error: The slider has a minimum of " + this.slider.foundation.min + ". Please review index.html to change it to a value larger than " + this.text.min + ", which is the minimum of the text field.");
        }
        if (this.slider.foundation.max > this.text.max) {
            js_error("Error: The slider has a maximum of " + this.slider.foundation.max + ". Please review index.html to change it to a value smaller than " + this.text.max + ", which is the maximum of the text field.");
            throw new Error("Error: The slider has a maximum of " + this.slider.foundation.max + ". Please review index.html to change it to a value smaller than " + this.text.max + ", which is the maximum of the text field.");
        }
        this.logarithmic = use_logarithmic_slider;
        if (use_logarithmic_slider) {
            if (this.slider.foundation.min <= 0) {
                js_error("Error: The logarithmic slider has a minimum of " + this.slider.foundation.min + ". Please review index.html to change it to a value larger than 0.");
                throw new Error("Error: The logarithmic slider has a minimum of " + this.slider.foundation.min + ". Please review index.html to change it to a value larger than 0.");
            }
            if (this.slider.foundation.max <= 0) {
                js_error("Error: The logarithmic slider has a maximum of " + this.slider.foundation.max + ". Please review index.html to change it to a value larger than 0.");
                throw new Error("Error: The logarithmic slider has a maximum of " + this.slider.foundation.max + ". Please review index.html to change it to a value larger than 0.");
            }
            this.log_min = Math.log(this.slider.foundation.min);
            this.log_scale = (Math.log(this.slider.foundation.max) - this.log_min) / (this.slider.foundation.max - this.slider.foundation.min);
        }
        this.numeric_value = initial_value;
        this.private_set_slider_value(initial_value);
        this.text.value = initial_value.toString();
        this.text.valid = true;
        slider_element.addEventListener("MDCSlider:input", slider_listener, false);
        text_element.addEventListener("input", text_listener, false);
        text_element.addEventListener("change", text_change_listener, false);
        let myself = this;
        text_listener();
        text_change_listener();
        function slider_listener(event) {
            if (myself.logarithmic) {
                if (myself.slider.getValue() < myself.slider.foundation.max) {
                    myself.numeric_value = Math.exp(myself.log_min + myself.log_scale * (myself.slider.getValue() - myself.slider.foundation.min));
                } else {
                    myself.numeric_value = myself.slider.foundation.max;
                }
            } else {
                myself.numeric_value = myself.slider.getValue();
            }
            if (myself.text.step != 0) {
                myself.numeric_value -= ((myself.numeric_value - myself.text.min) % myself.text.step);
                if (myself.numeric_value < myself.text.min) {
                    myself.numeric_value = myself.text.min
                }
            }
            myself.text.value = myself.numeric_value;
            myself.text.valid = true;
        }
        function text_listener(event) {
            if (myself.text.valid) {
                myself.numeric_value = parseFloat(myself.text.value);
                let new_slider_position;
                if (myself.logarithmic) {
                    if (myself.numeric_value > 0) {
                        new_slider_position = ((Math.log(myself.numeric_value) - myself.log_min) / myself.log_scale) + myself.slider.foundation.min;
                    } else {
                        new_slider_position = myself.slider.foundation.min;
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
            if (!myself.text.valid) {
                myself.text.valid = false;
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
    private_set_slider_value(new_value) {
        if (new_value < this.slider.foundation.min) {
            this.slider.setValue(this.slider.foundation.min);
        } else if (new_value > this.slider.foundation.max) {
            this.slider.setValue(this.slider.foundation.max);
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
}

export class JsAbmParams {
    constructor() {
        this.nAgents0 = new JsSliderValue(1000, true, "abm-n-agents-slider", "abm-n-agents-text");
        // this.layout();
    }
    layout() {
        this.nAgents0.layout();
    }
    get js_n0() {
        return this.nAgents0.value;
    }
}

function js_init() {
    let abm_params = new JsAbmParams;
    {
        for (let el of document.getElementsByClassName("mdc-fab")) {
            const fabRipple = new MDCRipple(el);
        }
        for (let el of document.getElementsByClassName("mdc-icon-button")) {
            const iconButtonRipple = new MDCRipple(el);
            iconButtonRipple.unbounded = true;
        }
        // const lineRipple = new MDCLineRipple(document.querySelector('.mdc-line-ripple'));

        // Webpack requires WebAssembly to be a dynamic import for now.
        import("/pkg/index.js").then(module => module.rs_init(abm_params), console.error);
    }
    {
        let left_offset = 0, top_offset = 0, abm_card = js_get_id("abm-card");
        let drag_zones = Array.prototype.slice.call(document.getElementsByClassName("abm-draggable"));
        drag_zones.forEach(e => e.addEventListener("pointerdown", abm_pointer_down, false));

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
            abm_params.layout();
        }
    }
}

window.addEventListener('load', js_init);
