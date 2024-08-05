import {
    get_block,
} from "/static/model/blocks.js";
import {
    add_segment,
    get_segments,
    get_segment_at_measure,
    on_segment_update,
    update_segment,
    Segment,
} from "/static/model/segments.js";
import {schema_query} from "/static/model/devices.js";
import {constants, options_list} from "/static/ui/util.js";

const SEGMENT_PADDING_Y = 10;

const SEGMENT_RADIUS = 5;
const SEGMENT_BORDER = "#bcc4d1";
const INPUT_SEGMENT_FILL = "#4c576b"
const OUTPUT_SEGMENT_FILL = "#63738f";

/**
 * The view for building a layer.
 */
export class LayerCreate {
    constructor(id) {
        this.id = id;
        this.layer_create_id = `layer-create-${this.id}`;
        this.layer_create_canvas_id = `layer-create-canvas-${this.id}`;
        this.segment_dialog_id = `segment-dialog-${this.id}`;
        this.segment_type_id = `segment-type-${this.id}`;
        this.measure_start_id = `measure-start-${this.id}`;
        this.measure_stop_id = `measure-stop-${this.id}`;
        this.segment_submit_id = `segment-submit-${this.id}`;
    }

    // Get the relative mouse position inside of the layer.
    _relative_mouse_position(e) {
        var element = document.getElementById(this.layer_create_canvas_id);
        var x = e.clientX - element.getBoundingClientRect().x;
        var y = e.clientY - element.getBoundingClientRect().y;
        return {x: x, y: y};
    }

    // Get the mouse position as a measure.
    _get_mouse_measure(e) {
        var pos = this._relative_mouse_position(e);
        return pos.x / constants.PIXELS_PER_MEASURE;
    }

    // Callback for creating a new segment.
    _new(e) {
        // Prevent child targets from triggering this event.
        if (e.target.id !== this.layer_create_canvas_id) {
            return;
        }

        // This layer isn't configured yet.
        var block = get_block(this.id);
        if (!block) {
            return;
        }

        // This layer has no input or output segments.
        if (!schema_query.has_input_segments(block.type) &&
            !schema_query.has_output_segments(block.type)) {
            return;
        }

        // Show the dialog.
        var dialog = document.getElementById(this.segment_dialog_id);
        dialog.innerHTML = this._draw_segment_dialog({});
        var modal = new bootstrap.Modal(dialog);
        modal.show();

        // Listen for the user to submit the form.
        var submit_button = document.getElementById(this.segment_submit_id);
        submit_button.onclick = e => {
            var segment_type = document.getElementById(this.segment_type_id).value;
            var segment_start = parseFloat(document.getElementById(this.measure_start_id).value);
            var segment_stop = parseFloat(document.getElementById(this.measure_stop_id).value);

            add_segment(this.id, new Segment(segment_start, segment_stop, segment_type));
            modal.hide();
        }
    }

    _edit(e, segment) {
        // This is a coding error. How could we have segments if there's no block?
        var block = get_block(this.id);
        if (!block) {
            console.log(`ERROR: Segment found for layer ${this.id}`)
            return;
        }

        // This layer has no input or output segments.
        if (!schema_query.has_input_segments(block.type) &&
            !schema_query.has_output_segments(block.type)) {
            return;
        }

        // Show the dialog.
        var dialog = document.getElementById(this.segment_dialog_id);
        dialog.innerHTML = this._draw_segment_dialog(segment);
        var modal = new bootstrap.Modal(dialog);
        modal.show();

        // Listen for the user to submit the form.
        var submit_button = document.getElementById(this.segment_submit_id);
        submit_button.onclick = e => {
            segment.type = document.getElementById(this.segment_type_id).value;
            segment.start = parseFloat(document.getElementById(this.measure_start_id).value);
            segment.stop = parseFloat(document.getElementById(this.measure_stop_id).value);
            update_segment(this.id, segment);

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

        const segments = get_segments(this.id);

        for (var segment of segments) {
            ctx.strokeStyle = SEGMENT_BORDER;
            if (segment.type === "input") {
                ctx.fillStyle = INPUT_SEGMENT_FILL;
            } else {
                ctx.fillStyle = OUTPUT_SEGMENT_FILL;
            }
            ctx.beginPath();
            ctx.roundRect(constants.PIXELS_PER_MEASURE * segment.start,
                          SEGMENT_PADDING_Y,
                          constants.PIXELS_PER_MEASURE * segment.segment_duration(),
                          canvas.height - (2 * SEGMENT_PADDING_Y),
                          SEGMENT_RADIUS);
            ctx.fill();
            ctx.stroke();
        }
    }

    // Drag a dialog that lets the user interact with the segment.
    _draw_segment_dialog(segment) {
        // This is a coding error. How could we have segments if there's no block?
        var block = get_block(this.id);
        if (!block) {
            console.log(`ERROR: Segment found for layer ${this.id}`)
            return;
        }

        var default_type = segment.type ?? "";
        var default_start = segment.start ?? "";
        var default_stop = segment.stop ?? "";

        var possible_types = [];
        if (schema_query.has_input_segments(block.type)) {
            possible_types.push("input");
        }
        if (schema_query.has_output_segments(block.type)) {
            possible_types.push("output");
        }
        possible_types = possible_types.filter(item => item !== default_type);

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
                            <option selected>${default_type}</option>
                            ${options_list(possible_types)}
                        </select>
                    </div>
                </div>
                <div class="form-group row mb-3">
                    <label class="col-3 col-form-label">Measure Start</label>
                    <div class="col-9">
                        <input class="form-control" type="number" value="${default_start}" id="${this.measure_start_id}">
                    </div>
                </div>
                <div class="form-group row mb-3">
                    <label class="col-3 col-form-label">Measure Stop</label>
                    <div class="col-9">
                        <input class="form-control" type="number" value="${default_stop}" id="${this.measure_stop_id}">
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
<div class="layer-create" id="${this.layer_create_id}">
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
            // Prevent child targets from triggering this event.
            if (e.target.id !== this.layer_create_canvas_id) {
                return;
            }

            // Get the segment corresponding to this click.
            var segment = get_segment_at_measure(this.id, this._get_mouse_measure(e));
            if (!segment) {
                this._new(e);
            } else {
                this._edit(e, segment);
            }
        }

        on_segment_update(this.id, e => this._refresh_canvas());
    }
}