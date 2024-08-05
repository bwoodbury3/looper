/**
 * Backing store for segments.
 */

var m_segments = {};
var m_callbacks = [];

var _ids = 0;

export var SegmentTypes = {
    input: "input",
    output: "output",
};

/**
 * A Segment.
 */
export class Segment {
    /**
     * Constructor.
     *
     * @param {number} start The start measure of the segment.
     * @param {number} stop The stop measure of the segment.
     * @param {string} type The segment type see (SegmentTypes).
     */
    constructor(start, stop, type) {
        this.id = _ids++;
        this.start = start;
        this.stop = stop;
        this.type = type;
    }

    /**
     * Copy all data from another segment.
     *
     * @param {Segment} other The other segment.
     */
    copy_from(other) {
        this.start = other.start;
        this.stop = other.stop;
        this.type = other.type;
    }

    /**
     * @returns The duration of the segment.
     */
    segment_duration() {
        return this.stop - this.start;
    }
}

/**
 * Trigger the relevant callbacks.
 *
 * @param {any} layer_id The layer ID.
 */
function trigger_callbacks(layer_id) {
    if (layer_id in m_callbacks) {
        for (var callback of m_callbacks[layer_id]) {
            callback();
        }
    }
}

/**
 * Trigger all of the callbacks.
 */
function trigger_all_callbacks() {
    for (var layer_id in m_callbacks) {
        for (var callback of m_callbacks[layer_id]) {
            callback();
        }
    }
}

/**
 * Add a segment to the layer.
 *
 * @param {any} layer_id The layer ID.
 * @param {Segment} segment The segment.
 */
export function add_segment(layer_id, segment) {
    if (!(layer_id in m_segments)) {
        m_segments[layer_id] = [];
    }
    m_segments[layer_id].push(segment);
    trigger_callbacks(layer_id);
}

/**
 * Update the segment provided. If no segment with that ID exists, this will fail.
 *
 * @param {any} layer_id The layer ID.
 * @param {Segment} segment The segment to update.
 *
 * @returns false only if that segment did not previously exist.
 */
export function update_segment(layer_id, segment) {
    if (!(layer_id in m_segments)) {
        console.log(`ERROR: Could not find update segment for layer=${layer_id}!`);
        return false;
    }

    var segments = m_segments[layer_id];
    for (var candidate of segments) {
        if (candidate.id === segment.id) {
            candidate.copy_from(segment);
            trigger_callbacks(layer_id);
            return true;
        }
    }

    console.log(`ERROR: Could not find update segment for layer=${layer_id}!`);
    return false;
}

/**
 * Get all segments.
 *
 * @returns All segments indexed by layer ID.
 */
export function get_all_segments() {
    return m_segments;
}

/**
 * Get all segments for a layer.
 *
 * @param {any} layer_id The layer ID.
 * @returns {Segment} The list of segments.
 */
export function get_segments(layer_id) {
    if (!(layer_id in m_segments)) {
        m_segments[layer_id] = [];
    }
    return m_segments[layer_id];
}

/**
 * Get the first segment that contains the specified measure.
 *
 * @param {any} layer_id The layer ID.
 * @param {number} measure The measure.
 * @return {Segment} The segment that was found, or null.
 */
export function get_segment_at_measure(layer_id, measure) {
    var segments = get_segments(layer_id);
    for (var segment of segments) {
        if (segment.start < measure && measure < segment.stop) {
            return segment;
        }
    }
    return null;
}

/**
 * Delete all segments.
 */
export function clear_segments() {
    m_segments = {};
    trigger_all_callbacks();
}

/**
 * Clear all callbacks.
 */
export function clear_segment_callbacks() {
    m_callbacks = {};
}

/**
 * Attach a callback that runs when any of the layer data changes.
 *
 * @param {() => void} callback The callback function to run.
 */
export function on_segment_update(layer_id, callback) {
    if (!(layer_id in m_callbacks)) {
        m_callbacks[layer_id] = [];
    }
    m_callbacks[layer_id].push(callback);
}