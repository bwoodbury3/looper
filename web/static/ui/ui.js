import {Layer} from "/static/ui/layer.js";

var ui_main = document.getElementById("ui");

// Add the audio layers.
var layers = [];
layers.push(new Layer());
layers.push(new Layer());
layers.push(new Layer());
layers.push(new Layer());
layers.push(new Layer());
layers.push(new Layer());
layers.push(new Layer());
layers.push(new Layer());
layers.push(new Layer());
layers.push(new Layer());
layers.push(new Layer());

// Draw the UI
function draw() {
    for (const layer of layers) {
        ui_main.innerHTML += layer.draw();
    }
    for (const layer of layers) {
        layer.set_event_callbacks();
    }

}

draw();