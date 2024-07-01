import {Layer} from "/static/ui/layer.js";
import {project} from "/static/ui/project.js";

var ui_main = document.getElementById("ui");
var layers = [];

// Draw the UI
export function draw_ui() {
    ui_main.innerHTML = "";
    for (const layer of layers) {
        ui_main.innerHTML += layer.draw();
    }
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