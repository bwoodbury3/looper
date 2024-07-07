/**
 * Schema for devices.
 *
 * Devices have three types of fields:
 *   - Required fields: Static inputs that are required to configure the device.
 *   - Optional fields: Static inputs that are optional and have sane defaults.
 *   - Input channel(s): A list of source channels that this device reads from.
 *   - Output channel(s): A list of dest channel that this device writes to.
 *   - Input Segment(s): A list of segments that describe when a channel is reading.
 *   - Output Segment(s): A list of segments that descibe when a channel is writing.
 *
 * If any field is omitted it means that it's not supported.
 *
 * If a list has 'max_count: -1' it means that it can support an unlimited size.
 */
export var devices = {
    VirtualInstrument: {
        required_fields: [
            {
                name: "instrument",
                type: "choice",
                choices: [
                    "drums1",
                ],
            },
            {
                name: "output_channel",
                type: "string",
            },
        ],
        optional_fields: [
            {
                name: "volume",
                type: "float",
            },
        ],
        output_channels: {
            min_count: 1,
            max_count: 1,
        },
        output_segments: {
            min_count: 0,
            max_count: -1,
        },
    },
    Metronome: {
        required_fields: [
            {
                name: "output_channel",
                type: "string",
            },
        ],
        optional_fields: [],
        output_channels: {
            min_count: 1,
            max_count: 1,
        },
        output_segments: {
            min_count: 0,
            max_count: -1,
        },
    },
    AudioOutput: {
        required_fields: [
            {
                name: "device",
                type: "string",
            },
            {
                name: "input_channel",
                type: "string",
            },
        ],
        optional_fields: [],
        input_channels: {
            min_count: 1,
            max_count: 1,
        },
        input_segments: {
            min_count: 0,
            max_count: -1,
        },
    },
    AudioInput: {
        required_fields: [
            {
                name: "Device Name",
                type: "string",
            },
            {
                name: "output_channel",
                type: "string",
            },
        ],
        optional_fields: [],
        output_channels: {
            min_count: 1,
            max_count: 1,
        },
        output_segments: {
            min_count: 0,
            max_count: -1,
        },
    },
};

// Convenience helper functions.
export var schema_query = {
    // Get all available device names.
    device_names: function() {
        return Object.keys(devices);
    },

    // Whether the provided device name is a supported device.
    is_supported: function(device_name) {
        return device_name in devices;
    },

    // Whether a device has inputs.
    has_inputs: function(device_name) {
        return "input_channels" in devices[device_name];
    },

    // Whether a device has outputs.
    has_outputs: function (device_name) {
        return "output_channels" in devices[device_name];
    },

    // Whether a device has input segments.
    has_input_segments: function(device_name) {
        return "input_segments" in devices[device_name];
    },

    // Whether a device has output segments.
    has_output_segments: function(device_name) {
        return "output_segments" in devices[device_name];
    },
}