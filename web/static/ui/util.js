// Return a list of values as a list of options.
export function options_list(values) {
    var options = "";
    for (const value of values) {
        options += `<option>${value}</option>`
    }
    return options;
}