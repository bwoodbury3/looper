// Schema for devices.
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
    },
    Metronome: {
        required_fields: [
            {
                name: "output_channel",
                type: "string",
            },
        ],
        optional_fields: [
            {
                name: "start_measure",
                type: "int",
            },
            {
                name: "stop_measure",
                type: "int",
            },
        ],
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
    },
    AudioInput: {
        required_fields: [
            {
                name: "device",
                type: "string",
            },
            {
                name: "output_channel",
                type: "string",
            },
        ],
        optional_fields: [],
    },
};

// Helper to get all available device names.
export function device_names() {
    return Object.keys(devices);
}