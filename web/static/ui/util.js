// Return a list of values as a list of options.
export function options_list(values) {
    var options = "";
    for (const value of values) {
        options += `<option>${value}</option>`
    }
    return options;
}

export function num_trailing_zeros(num) {
    var count = 0;
    var _num = num;
    while ((_num & 1) === 0) {
        _num = _num >> 1;
        count += 1;
    }
    return count;
}

// A bunch of constants that shouldn't really be constants.
export const constants = {
    PIXELS_PER_MEASURE: 100,

    // 16th note resolution
    RULER_RESOLUTION: 16,
    MAX_MEASURES: 500,
}