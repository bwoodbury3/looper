import {devices, device_names} from "/static/ui/devices.js"

var _ids = 0;

export class Layer {
    constructor() {
        this.id = _ids++;
        this.layer_id = `layer-${this.id}`;
        this.layer_settings_id = `layer-settings-${this.id}`;
        this.layer_create_id = `layer-create-${this.id}`;
        this.layer_vertical_drag_id = `layer-vertical-drag-${this.id}`;
        this.settings_dialog_id = `layer-settings-dialog-${this.id}`;
        this.settings_name_id = `settings-name-${this.id}`;
        this.settings_device_type_id = `settings-device-type-${this.id}`;
        this.settings_additional_fields_id = `settings-remaining-fields-${this.id}`;
        this.settings_submit_id = `settings-submit-${this.id}`;
        this.is_resizing = false;

        // Defaults
        this.layer_name = `Layer-${this.id}`;
        this.device_type = null;
        this.data = {};
    }

    // Return a list of values as a list of options.
    _options_list(values) {
        var options = "";
        for (const value of values) {
            options += `<option>${value}</option>`
        }
        return options;
    }

    // Callback to set settings.
    _settings(e) {
        // Show the dialog.
        var dialog = document.getElementById(this.settings_dialog_id);
        dialog.innerHTML = this._draw_settings_dialog();
        var modal = new bootstrap.Modal(dialog);
        modal.show();

        // Listen for changes in the settings device type.
        var settings_device_type = document.getElementById(this.settings_device_type_id);
        var settings_additional_fields = document.getElementById(this.settings_additional_fields_id);
        settings_device_type.addEventListener("change", e => {
            settings_additional_fields.innerHTML = this._draw_additional_settings(settings_device_type.value);
        });

        // Listen for the user hitting the submit button.
        var submit_button = document.getElementById(this.settings_submit_id);
        submit_button.onclick = e => {
            // Add all form data to our internal data object.
            this.layer_name = document.getElementById(this.settings_name_id).value;
            this.device_type = document.getElementById(this.settings_device_type_id).value;

            this.data = {};
            if (this.device_type in devices) {
                var schema = devices[this.device_type];
                for (var field of schema.required_fields) {
                    var value = document.getElementById(`layer-${this.id}-${field.name}`).value;
                    var type = field.type;
                    if (type === "int")
                        this.data[field.name] = parseInt(value);
                    else if (type === "float")
                        this.data[field.name] = parseFloat(value);
                    else
                        this.data[field.name] = value;
                }
                for (var field of schema.optional_fields) {
                    var value = document.getElementById(`layer-${this.id}-${field.name}`).value;
                    var type = field.type;
                    if (type === "int")
                        this.data[field.name] = parseInt(value);
                    else if (type === "float")
                        this.data[field.name] = parseFloat(value);
                    else
                        this.data[field.name] = value;
                }
                modal.hide();
            }

            // Update the settings summary.
            document.getElementById(this.layer_settings_id).innerHTML = this.draw_settings_summary();
        };
    }

    // Draw the settings dialog.
    _draw_settings_dialog() {
        var device_type = this.device_type ? this.device_type : "Please select one...";
        return `
<div class="modal-dialog modal-lg">
    <div class="modal-content">
        <div class="modal-header">
            <h5 class="modal-title">Edit Layer</h5>
            <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
        </div>
        <div class="modal-body">
            <form>
                <div class="form-group row mb-3">
                    <label class="col-3 col-form-label">Layer Name</label>
                    <div class="col-9">
                        <input class="form-control" id="${this.settings_name_id}" value="${this.layer_name}" type="text">
                    </div>
                </div>

                <div class="form-group row mb-3">
                    <label class="col-3 col-form-label">Device Type</label>
                    <div class="col-9">
                        <select class="form-control" id="${this.settings_device_type_id}">
                            <option selected>${device_type}</option>
                            ${this._options_list(device_names())}
                        </select>
                    </div>
                </div>

                <div id="${this.settings_additional_fields_id}">
                    ${this._draw_additional_settings(this.device_type)}
                </div>
            </form>
        </div>
        <div class="modal-footer">
            <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Close</button>
            <button type="button" class="btn btn-primary" id="${this.settings_submit_id}">Save changes</button>
        </div>
    </div>
</div>
`
    }

    // Render a single device input field.
    _draw_field(field) {
        var field_id = `layer-${this.id}-${field.name}`;

        var input = `<p>I don't know how to handle field "${field.name}" (${field.type})</p>`;
        if (field.type === "string") {
            var value = this.data[field.name] ? this.data[field.name] : "";
            input = `<input class="form-control" id="${field_id}" value="${value}" type="text">`;
        } else if (field.type === "float") {
            var value = this.data[field.name] ? this.data[field.name] : 1.0;
            input = `<input class="form-control" id="${field_id}" value="${value}" type="number">`;
        } else if (field.type === "int") {
            var value = this.data[field.name] ? this.data[field.name] : 1;
            input = `<input class="form-control" id="${field_id}" value="${value}" type="number" step="1">`;
        } else if (field.type === "choice") {
            var value = this.data[field.name] ? this.data[field.name] : "Please select one...";
            input = `
<select class="form-control" id="${field_id}">
    <option selected>${value}</option>
    ${this._options_list(field.choices)}
</select>
`
        }
        return `
<div class="form-group row mb-3">
    <label class="col-3 col-form-label">${field.name}</label>
    <div class="col-9">
        ${input}
    </div>
</div>
`
    }

    // Draw the remaining fields required for this device type.
    _draw_additional_settings(device_type) {
        if (!(device_type in devices)) {
            return "";
        }

        var form = "<hr/>";
        var schema = devices[device_type];
        for (var field of schema.required_fields) {
            form += this._draw_field(field);
        }
        form += "<hr/>";
        for (var field of schema.optional_fields) {
            form += this._draw_field(field);
        }
        return form;
    }

    // Draw the layer.
    draw() {
        return `
<div id=${this.layer_id} class="layer">
    <div class="layer-settings p-2" id="${this.layer_settings_id}">
        ${this.draw_settings_summary()}
    </div>
    <div class="layer-create" id="${this.layer_create_id}"></div>
    <div class="layer-vertical-drag" id="${this.layer_vertical_drag_id}"></div>
    <div id="${this.settings_dialog_id}" class="modal fade"></div>
</div>
`
    }

    // Draw a summary of this layer in the box on the left.
    draw_settings_summary() {
        if (!this.device_type) {
            return "";
        }

        return `
<div>
    <h5>${this.layer_name}</h5>
    <p>${this.device_type}</p>
</div>
`
    }

    // Set up a bunch of event callbacks.
    set_event_callbacks() {
        var layer = document.getElementById(this.layer_id);
        var layer_settings = document.getElementById(this.layer_settings_id);
        var layer_vertical_drag = document.getElementById(this.layer_vertical_drag_id);

        layer_vertical_drag.onmousedown = e => {
            this.is_resizing = true;
        };

        window.addEventListener("mousemove", e => {
            if (this.is_resizing) {
                var mouse_pos = e.clientY + layer.parentElement.scrollTop;
                var height = Math.max(mouse_pos - layer.offsetTop, 100);
                layer.style.height = `${height}px`;
            }
        });

        window.addEventListener("mouseup", e => {
            this.is_resizing = false;
        });

        layer_settings.onclick = e => {
            this._settings(e);
        }
    }

    // Get the configured data for this layer.
    get_data() {
        if (!this.device_type) {
            return {};
        }

        return {
            name: this.layer_name,
            type: this.device_type,
            ...this.data
        };
    }
};
