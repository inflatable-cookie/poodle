# Shared native specimens

Status: proof on Button, Select and DataTable

[Shared specimen definitions](../../../packages/specimens/src/lib.rs) own the
catalogue slug, ordered panes and exact axis values. A pane builds a
`poodle_node::Node` from the host's `RenderContext`. Static axis rows render
one representative each. Interactive Examples use typed state and callbacks
that enqueue typed events. The session drains events through pure reducers,
reusing the component's headless transitions. Select, for example, consumes the
complete transition already produced by its renderer, including changes to
highlight and caret.

A preview keeps one session, chooses the active pane, and converts its node
tree at the backend boundary. After input it drains the session, rebuilds when
state changes, and applies effects to the rebuilt tree. The only effect so far
is focusing a node by ID. The shell owns navigation, tab controls, scrolling,
and capture framing.

Svelte remains the reference for page content and behaviour. Shared native
pages use the same representative examples and public axis values. The
snapshots and reducer tests cover the native composition independently of a
backend. Preview construction and mounted interaction checks supply separate
evidence for the host integration.
