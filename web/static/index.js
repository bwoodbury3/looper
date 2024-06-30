function stop() {
    console.log("Stopping playback");

    /* TODO */
}

function pause() {
    console.log("Pausing playback");

    /* TODO */
}

function play() {
    console.log("Starting playback");

    fetch("/api/play", {
        method: "POST",
        body: JSON.stringify({
            config: {
                tempo: {
                    bpm: 100,
                    beats_per_measure: 4,
                    beat_duration: 4,
                },
                devices: [],
            }
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