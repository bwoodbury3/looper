import {get_data} from "/static/ui/ui.js";

function stop(e) {
    console.log("Stopping playback");

    fetch("/api/stop", {
        method: "POST",
        body: "",
        headers: {
            "Content-type": "application/json; charset=UTF-8"
        }
    });
}

function pause(e) {
    console.log("Pausing playback");

    /* TODO */
}

function play(e) {
    console.log("Starting playback");

    var data = get_data();

    fetch("/api/play", {
        method: "POST",
        body: JSON.stringify({
            config: {
                tempo: {
                    bpm: 100,
                    beats_per_measure: 4,
                    beat_duration: 4,
                },
            },
            devices: data.devices,
        }),
        headers: {
            "Content-type": "application/json; charset=UTF-8"
        }
    });
}

function volume() {
    var volume_slider = document.getElementById("volume-slider");
    console.log("Setting volume:", volume_slider.value);

    /* TODO */
}

function set_callbacks() {
    var stop_button = document.getElementById("stop-button");
    stop_button.onclick = stop;

    var pause_button = document.getElementById("pause-button");
    pause_button.onclick = pause;

    var play_button = document.getElementById("play-button");
    play_button.onclick = play;
}

set_callbacks();