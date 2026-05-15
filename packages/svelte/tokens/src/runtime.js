export function applyThemeAttributes(element, options) {
    if (options.theme) {
        element.dataset.theme = options.theme;
    }
    if (options.density) {
        element.dataset.density = options.density;
    }
    if (options.controlSize) {
        element.dataset.controlSize = options.controlSize;
    }
}
