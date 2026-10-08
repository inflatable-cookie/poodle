# Preview specimen capture

Svelte and GPUI use the same logical frame for component specimen captures:
1280 × 900 logical pixels, with 24 pixels of inset padding. The values live in
[`packages/preview-capture/specimen-frame.json`](../../packages/preview-capture/specimen-frame.json)
and are imported by both preview implementations.

The reference browser scale is 1 device pixel per logical pixel. A Svelte
capture reports `window.devicePixelRatio` on its readiness marker; a GPUI axis
receipt records the window's effective `scale_factor()`. The lab should use
the receipt and marker values to confirm each pair has the same device scale.
Captures become ready only after web fonts have loaded and the specimen has
committed a frame.
