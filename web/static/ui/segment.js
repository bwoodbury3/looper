var _ids = 0;

/**
 * A Segment.
 */
export class Segment {
    constructor(store) {
        this.id = _ids++;

        // Initialize from backing store.
        this.store = store;
    }

    draw() {
        return "";
    }

    set_event_callbacks() {
        // Nothing to do.
    }

    measure_start() {
        return this.store.start;
    }

    measure_stop() {
        return this.store.stop;
    }

    measure_duration() {
        return this.store.stop - this.store.start;
    }
}