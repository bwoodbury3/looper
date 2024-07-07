import {constants, num_trailing_zeros} from "/static/ui/util.js";

const NOTCH_COLOR = "#dddddd";

// Draw a line from [x0, y0] to [x1, y1]
function line(ctx, x0, y0, x1, y1) {
    ctx.beginPath();
    ctx.moveTo(x0, y0);
    ctx.lineTo(x1, y1);
    ctx.stroke();
}

// Draw some text at a location.
function text(ctx, x, y, text, text_align = "center", font = "14px Arial") {
    ctx.textAlign = text_align;
    ctx.font = font;

    ctx.stroke();
    ctx.fillText(text, x, y);
}

// Draw a notch at point x with the provided height.
function notch(ctx, x, height, annotation = null) {
    line(ctx, x, ctx.canvas.height, x, ctx.canvas.height - height);

    if (annotation !== null) {
        text(ctx, x, 16, annotation)
    }
}

export class Ruler {
    constructor() {
        this.settings_header_id = "ruler-settings-header";
        this.ruler_id = "ruler";
    }

    refresh() {
        var canvas = document.getElementById(this.ruler_id);
        canvas.width = canvas.parentElement.offsetWidth;
        canvas.height = canvas.parentElement.offsetHeight;

        var ctx = canvas.getContext("2d");
        ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
        ctx.strokeStyle = NOTCH_COLOR;
        ctx.fillStyle = NOTCH_COLOR;

        var distance_between_notches =
            parseFloat(constants.PIXELS_PER_MEASURE) /
            constants.RULER_RESOLUTION;

        for (var m = 0; m < constants.MAX_MEASURES; m++) {
            var measure_start = m * constants.PIXELS_PER_MEASURE;

            // Draw all of the in-between notches
            for (var n = 1; n <= constants.RULER_RESOLUTION; n++) {
                var height = (num_trailing_zeros(n) + 1) * 4;
                var x = measure_start + distance_between_notches * n;
                if (n === constants.RULER_RESOLUTION) {
                    notch(ctx, x, height, `${m + 1}`);
                } else {
                    notch(ctx, x, height);
                }
            }
        }
    }

    // Initialize the
    draw() {
        return `
<div class="ruler-row">
    <div class="settings-header settings-column-width p-2" id="${this.settings_header_id}">
        <p><b>Layers</b></p>
    </div>
    <div class="ruler">
        <canvas id="${this.ruler_id}"></canvas>
    </div>
</div>
`;
    }

    set_event_callbacks() {
        this.refresh();
    }
}