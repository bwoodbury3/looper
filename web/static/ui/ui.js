import {io} from "/static/socket.io-client.js";

import {Layer} from "/static/ui/layer.js";
import {project} from "/static/ui/project.js";
import {Ruler} from "/static/ui/ruler.js";
import {VerticalBar} from "/static/ui/vertical-bar.js";

var ui_main = document.getElementById("ui");
var ruler = new Ruler();
var layers = [];
var vertical_bar = new VerticalBar(0);

// Draw the UI
export function draw_ui() {
    ui_main.innerHTML = "";

    /* Draw child content */
    ui_main.innerHTML += ruler.draw();
    for (const layer of layers) {
        ui_main.innerHTML += layer.draw();
    }
    ui_main.innerHTML += vertical_bar.draw();

    /* Set event callbacks */
    ruler.set_event_callbacks();
    vertical_bar.set_event_callbacks();
    for (const layer of layers) {
        layer.set_event_callbacks();
    }
}

// Initialize UI with default data.
function init_default() {
    layers = [];
    for (var i = 0; i < 10; i++) {
        var layer_data = {};
        project.layers.push(layer_data)
        layers.push(new Layer(layer_data));
    }
}

// Set the project data.
export function load_project_data(data) {
    layers = [];

    // Clear the project.
    for (var item in project) {
        delete project[item];
    }

    // Assign the data to the project.
    Object.assign(project, data);
    for (var layer_data of project.layers) {
        layers.push(new Layer(layer_data));
    }
}

// Get all data from the UI.
export function get_play_data() {
    var devices = [];
    for (const layer of layers) {
        var data = layer.get_data();
        if (Object.keys(data).length > 0) {
            devices.push(data);
        }
    }
    return {
        devices: devices,
    }
}

// Get all state that needs to be saved to disk.
export function get_save_data() {
    return project;
}

init_default();
draw_ui();

// Websocket for interfacing with the backend.
const socket = io("ws://localhost:1080", {
    reconnectionDelayMax: 10000,
});

// Detect keypress and send to the server.
document.onkeydown = e => {
    var data = {
        key: e.key,
    };
    socket.emit("keypress", data);
}