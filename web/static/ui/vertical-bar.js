var computed_style = getComputedStyle(document.body);
var settings_width = parseInt(computed_style.getPropertyValue("--settings-width"));

export class VerticalBar {
    constructor(id) {
        this.id = id;
        this.vertical_bar_id = `vertical-bar-${this.id}`;
    }

    _handle_move(e) {
        var vertical_bar = document.getElementById(this.vertical_bar_id);
        var parent = vertical_bar.parentElement;
        var left = Math.max(e.clientX - parent.getBoundingClientRect().x, settings_width);
        vertical_bar.style.left = `${left}px`;
    }

    draw() {
        return `<div class="vertical-bar drawable" id="${this.vertical_bar_id}"></div>`;
    }

    set_event_callbacks() {
        var vertical_bar = document.getElementById(this.vertical_bar_id);
        vertical_bar.parentElement.onmousemove = e => {
            this._handle_move(e);
        }
    }
}