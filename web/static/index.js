function stop() {
    console.log("Stopping playback");
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