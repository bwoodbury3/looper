import {
    draw_ui,
    get_play_data,
    get_save_data,
    load_project_data,
    set_callbacks,
} from "/static/ui/ui.js";
import {options_list} from "/static/ui/util.js";

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
    var data = get_play_data();
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

function save(e) {
    var dialog = document.getElementById("navbar-save-modal");

    // Draw the project save dialog
    dialog.innerHTML = _draw_save_modal();
    var modal = new bootstrap.Modal(dialog);
    modal.show();

    // Bind to the save project button
    var load_button = document.getElementById("navbar-save-button");
    load_button.onclick = e => {
        var project_file = document.getElementById("save-value").value;
        console.log(`Saving file ${project_file}`);

        fetch("/api/save", {
            method: "POST",
            body: JSON.stringify({
                name: project_file,
                data: get_save_data(),
            }),
            headers: {
                "Content-type": "application/json; charset=UTF-8"
            }
        })
            .then(response => modal.hide());
    }
}

function load(e) {
    fetch("/api/projects")
        .then(response => response.json())
        .then(data => {
            var dialog = document.getElementById("navbar-load-modal");

            // Draw the project load dialog
            dialog.innerHTML = _draw_load_modal(data.projects);
            var modal = new bootstrap.Modal(dialog);
            modal.show();

            // Bind to the load project button
            var load_button = document.getElementById("navbar-load-button");
            load_button.onclick = e => {
                var project_file = document.getElementById("load-value").value;
                fetch("/api/load", {
                    method: "POST",
                    body: JSON.stringify({name: project_file}),
                    headers: {
                        "Content-type": "application/json; charset=UTF-8"
                    }
                })
                    .then(response => response.json())
                    .then(content => {
                        load_project_data(content);
                        draw_ui();
                        set_callbacks();
                        modal.hide();
                    });
            }
        });
}

function _draw_save_modal() {
    return `
<div class="modal-dialog modal-lg">
    <div class="modal-content">
        <div class="modal-header">
            <h5 class="modal-title">Save Project</h5>
            <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
        </div>
        <div class="modal-body">
            <form>
                <div class="form-group row mb-3">
                    <label class="col-3 col-form-label">Project Name</label>
                    <div class="col-9">
                        <input class="form-control" id="save-value" type="text">
                    </div>
                </div>
            </form>
        </div>
        <div class="modal-footer">
            <button type="button" class="btn btn-primary" id="navbar-save-button">Save</button>
        </div>
    </div>
</div>
`;
}

function _draw_load_modal(project_names) {
    return `
<div class="modal-dialog modal-lg">
    <div class="modal-content">
        <div class="modal-header">
            <h5 class="modal-title">Load Project</h5>
            <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
        </div>
        <div class="modal-body">
            <form>
                <div class="form-group row mb-3">
                    <label class="col-3 col-form-label">Project Name</label>
                    <div class="col-9">
                        <select class="form-control" id="load-value">
                            <option selected>Choose one...</option>
                            ${options_list(project_names)}
                        </select>
                    </div>
                </div>
            </form>
        </div>
        <div class="modal-footer">
            <button type="button" class="btn btn-primary" id="navbar-load-button">Load</button>
        </div>
    </div>
</div>
`;
}

function set_callbacks() {
    var stop_button = document.getElementById("stop-button");
    stop_button.onclick = stop;

    var pause_button = document.getElementById("pause-button");
    pause_button.onclick = pause;

    var play_button = document.getElementById("play-button");
    play_button.onclick = play;

    var save_button = document.getElementById("navbar-save");
    save_button.onclick = save;

    var load_button = document.getElementById("navbar-load");
    load_button.onclick = load;
}

set_callbacks();