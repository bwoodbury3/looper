import {io} from "/static/socket.io-client.js";

import {
    clear_blocks,
    clear_block_callbacks,
    get_all_blocks,
    on_blocks_changed,
    update_block,
    Block,
} from '/static/model/blocks.js';
import {
    add_segment,
    clear_segments,
    clear_segment_callbacks,
    get_all_segments,
    Segment,
} from '/static/model/segments.js';
import {Layer} from "/static/ui/layer.js";
import {Ruler} from "/static/ui/ruler.js";
import {VerticalBar} from "/static/ui/vertical-bar.js";
import {constants} from "/static/ui/util.js";

var ui_main = document.getElementById("ui");
var ruler = new Ruler();
var layers = [];
var mouse_bar = new VerticalBar(0);
var time_bar = new VerticalBar(1);

var last_monitor_data = {};

/**
 * Draw the UI
 */
export function draw_ui() {
    layers = [];
    var m_blocks = get_all_blocks();

    ui_main.innerHTML = "";

    /* Draw child content */
    ui_main.innerHTML += ruler.draw();
    for (const layer of layers) {
        ui_main.innerHTML += layer.draw();
    }
    ui_main.innerHTML += mouse_bar.draw();
    ui_main.innerHTML += time_bar.draw();
}

/**
 * Set callbacks
 */
export function set_callbacks() {
    clear_block_callbacks();
    clear_segment_callbacks();

    /* Set event callbacks */
    ruler.set_event_callbacks();
    mouse_bar.set_event_callbacks();
    for (const layer of layers) {
        layer.set_event_callbacks();
    }

    on_blocks_changed(draw_ui);
}

/**
 * Initialize UI with default data.
 */
function init_default() {
    layers = [];
    for (var i = 0; i < 10; i++) {
        layers.push(new Layer(i));
    }
}

/**
 * Load in external data to the UI.
 *
 * @param {data} data The data to load.
 */
export function load_project_data(data) {
    clear_blocks();
    clear_segments();

    for (var block_id in data.blocks) {
        var block_data = data.blocks[block_id];
        var block = new Block(
            block_id,
            block_data.name,
            block_data.type,
            block_data.data
        );
        update_block(block_id, block);
        layers.push(new Layer(block_id));
    }

    for (var block_id in data.segments) {
        var segments = data.segments[block_id];
        for (var segment_data of segments) {
            var segment = new Segment(
                segment_data.start,
                segment_data.stop,
                segment_data.type
            );
            add_segment(block_id, segment);
        }
    }
}

/**
 * Get all data from the UI.
 */
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

/**
 * Get all state that needs to be saved to disk.
 */
export function get_save_data() {
    return {
        blocks: get_all_blocks(),
        segments: get_all_segments(),
    };
}

init_default();
draw_ui();
set_callbacks();

// Websocket for interfacing with the backend.
const socket = io("ws://localhost:1080", {
    reconnectionDelayMax: 10000,
});

// Detect keypress and send to the server.
document.onkeydown = e => {
    var data = {key: e.key};
    socket.emit("keypress", data);
}

socket.on("playback_monitor", (monitor_data) => {
    // Move the progress bar as needed.
    if (last_monitor_data.current_measure !== monitor_data.current_measure) {
        var left = constants.PIXELS_PER_MEASURE * monitor_data.current_measure;
        time_bar.set_left(left);
    }

    // Gray out buttons as needed.
    if (last_monitor_data.playing !== monitor_data.playing) {
        if (monitor_data.playing) {
            var play_button = document.getElementById("play-button");
            play_button.classList.add("disabled");
            play_button.classList.add("grayed-out");

            var stop_button = document.getElementById("stop-button");
            stop_button.classList.remove("disabled");
            stop_button.classList.remove("grayed-out");

            var pause_button = document.getElementById("pause-button");
            pause_button.classList.remove("disabled");
            pause_button.classList.remove("grayed-out");
        } else {
            var play_button = document.getElementById("play-button");
            play_button.classList.remove("disabled");
            play_button.classList.remove("grayed-out");

            var stop_button = document.getElementById("stop-button");
            stop_button.classList.add("disabled");
            stop_button.classList.add("grayed-out");

            var pause_button = document.getElementById("pause-button");
            pause_button.classList.add("disabled");
            pause_button.classList.add("grayed-out");
        }
    }

    last_monitor_data = monitor_data;
});
