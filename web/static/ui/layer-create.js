/**
 * The view for building a layer.
 */
export class LayerCreate {
    constructor(id, store) {
        this.id = id;
        this.layer_create_id = `layer-create-${this.id}`;
    }

    _edit(e) {
        console.log(`Editing layer ${this.id}`);
    }

    // Draw the layer.
    draw() {
        return `<div class="layer-create drawable" id="${this.layer_create_id}"></div>`
    }

    // Set up a bunch of event callbacks.
    set_event_callbacks() {
        var layer_create = document.getElementById(this.layer_create_id);

        layer_create.onclick = e => {
            this._edit(e);
        }
    }
}