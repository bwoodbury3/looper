import {schema_query} from "/static/ui/devices.js";
import {Segment} from "/static/ui/segment.js";
import {constants, options_list} from "/static/ui/util.js";

const SEGMENT_PADDING_Y = 10;

const SEGMENT_RADIUS = 5;
const SEGMENT_BORDER = "#bcc4d1";
const SEGMENT_FILL = "#63738f";

/**
 * The view for building a layer.
 */
export class LayerCreate {
    constructor(id, store) {
        this.id = id;
        this.layer_create_id = `layer-create-${this.id}`;
        this.layer_create_canvas_id = `layer-create-canvas-${this.id}`;
        this.segment_dialog_id = `segment-dialog-${this.id}`;
        this.segment_type_id = `segment-type-${this.id}`;
        this.measure_start_id = `measure-start-${this.id}`;
        this.measure_stop_id = `measure-stop-${this.id}`;
        this.segment_submit_id = `segment-submit-${this.id}`;
        this.segments = [];

        // Initialize from backing store.
        this.store = store;
        if (!("segments" in this.store)) {
            this.store.segments = [];
        } else {
            for (var segment of this.store.segments) {
                this.segments.push(new Segment(segment));
            }
        }
    }

    // Callback for creating a new segment.
    _new(e) {
        // Prevent child targets from triggering this event.
        if (e.target.id !== this.layer_create_canvas_id) {
            return;
        }

        // This layer isn't configured yet.
        if (!this.store.schema || !this.store.device_type) {
            return;
        }

        // This layer has no input or output segments.
        if (!schema_query.has_input_segments(this.store.device_type) &&
            !schema_query.has_output_segments(this.store.device_type)) {
            return;
        }

        // Show the dialog.
        var dialog = document.getElementById(this.segment_dialog_id);
        dialog.innerHTML = this._draw_segment_dialog();
        var modal = new bootstrap.Modal(dialog);
        modal.show();

        // Listen for the user to submit the form.
        var submit_button = document.getElementById(this.segment_submit_id);
        submit_button.onclick = e => {
            var segment_type = document.getElementById(this.segment_type_id).value;
            var segment_start = parseFloat(document.getElementById(this.measure_start_id).value);
            var segment_stop = parseFloat(document.getElementById(this.measure_stop_id).value);

            var segment_data = {
                type: segment_type,
                start: segment_start,
                stop: segment_stop,
            };
            this.store.segments.push(segment_data);
            this.segments.push(new Segment(segment_data));
            modal.hide();

            this._refresh_canvas();
        }
    }

    // Redraw the canvas.
    _refresh_canvas() {
        var canvas = document.getElementById(this.layer_create_canvas_id);
        canvas.width = canvas.parentElement.offsetWidth;
        canvas.height = canvas.parentElement.offsetHeight;

        var ctx = canvas.getContext("2d");
        ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);

        for (var segment of this.segments) {
            ctx.strokeStyle = SEGMENT_BORDER;
            ctx.fillStyle = SEGMENT_FILL;
            ctx.beginPath();
            ctx.roundRect(constants.PIXELS_PER_MEASURE * segment.measure_start(),
                          SEGMENT_PADDING_Y,
                          constants.PIXELS_PER_MEASURE * segment.measure_duration(),
                          canvas.height - (2 * SEGMENT_PADDING_Y),
                          SEGMENT_RADIUS);
            ctx.fill();
            ctx.stroke();
        }
    }

    // Drag a dialog that lets the user interact with the segment.
    _draw_segment_dialog() {
        var possible_types = [];
        if (schema_query.has_input_segments(this.store.device_type)) {
            possible_types.push("input");
        }
        if (schema_query.has_output_segments(this.store.device_type)) {
            possible_types.push("output");
        }

        return `
<div class="modal-dialog modal-lg">
    <div class="modal-content">
        <div class="modal-header">
            <h5 class="modal-title">Audio Segment</h5>
            <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
        </div>
        <div class="modal-body">
            <form>
                <div class="form-group row mb-3">
                    <label class="col-3 col-form-label">Segment Type</label>
                    <div class="col-9">
                        <select class="form-control" id="${this.segment_type_id}">
                            <option selected>${possible_types[0]}</option>
                            ${options_list(possible_types.slice(1))}
                        </select>
                    </div>
                </div>
                <div class="form-group row mb-3">
                    <label class="col-3 col-form-label">Measure Start</label>
                    <div class="col-9">
                        <input class="form-control" type="number" id="${this.measure_start_id}">
                    </div>
                </div>
                <div class="form-group row mb-3">
                    <label class="col-3 col-form-label">Measure Stop</label>
                    <div class="col-9">
                        <input class="form-control" type="number" id="${this.measure_stop_id}">
                    </div>
                </div>
            </form>
        </div>
        <div class="modal-footer">
            <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Close</button>
            <button type="button" class="btn btn-primary" id="${this.segment_submit_id}">Save changes</button>
        </div>
    </div>
</div>
`
    }

    // Clear the state.
    clear() {
        // TODO
    }

    // Draw the layer.
    draw() {
        return `
<div class="layer-create drawable" id="${this.layer_create_id}">
    <canvas class="layer-create-canvas" id="${this.layer_create_canvas_id}"></canvas>
    <div id="${this.segment_dialog_id}" class="modal fade"></div>
</div>
`
    }

    // Set up a bunch of event callbacks.
    set_event_callbacks() {
        var layer_canvas = document.getElementById(this.layer_create_canvas_id);
        // Refresh the canvas here. I don't like it but it has to be done after
        // draw().
        this._refresh_canvas();

        layer_canvas.onclick = e => {
            this._new(e);
        }
    }
}