"use strict";

function es_init() {
    // Webpack requires WebAssembly to be a dynamic import for now.
    import("/pkg/index.js").then(module => module.rs_init(), console.error);
    var abmmenuheader = document.getElementById("abmmenuheader");
    {
        var left1 = 0, top1 = 0, left2 = 0, top2 = 0;
        var abmmenu = document.getElementById("abmmenu");
        abmmenuheader.onmousedown = abm_mouse_down;
        document.getElementById("abmmenutitle").innerText = "Move";

        function abm_mouse_down(e) {
            e = e || window.event;
            e.preventDefault();
            left2 = e.clientX;
            top2 = e.clientY;
            document.onmouseup = abm_mouse_up;
            document.onmousemove = abm_drag;
        }

        function abm_drag(e) {
            e = e || window.event;
            e.preventDefault();
            left1 = left2 - e.clientX;
            top1 = top2 - e.clientY;
            left2 = e.clientX;
            top2 = e.clientY;
            abmmenu.style.left = (abmmenu.offsetLeft - left1) + "px";
            abmmenu.style.top = (abmmenu.offsetTop - top1) + "px";
        }

        function abm_mouse_up() {
            document.onmouseup = null;
            document.onmousemove = null;
        }
    }
}

document.addEventListener('DOMContentLoaded', es_init, false);
