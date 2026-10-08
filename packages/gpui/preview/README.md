# GPUI Preview

The preview app renders the shared Poodle specimen catalogue through GPUI.
Run it from the repository root with `effigy gpui:preview`.

## Specimen Capture

Render a component's Examples pane without preview chrome or specimen tabs:

```sh
effigy gpui:run -- --capture-mode specimen --component button \
  --screenshot button.png --capture-receipt button.json
```

Capture mode uses a titleless, shadowless window whose content frame is 1280 ×
900 logical pixels with 24 pixels of padding. The receipt records the effective
`deviceScale`; pair it with the Svelte frame's `data-capture-device-scale`
marker before pixel comparison. Shared dimensions and scale notes are in the
[preview specimen capture guide](../../../docs/guides/preview-specimen-capture.md).

The frame values are shared with Svelte through
[`packages/preview-capture/specimen-frame.json`](../../preview-capture/specimen-frame.json).
