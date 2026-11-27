# Shared native specimens

Status: proof plus first batch of 14 form and action controls (17 routes total)

[Shared specimen definitions](../../../packages/specimens/src/lib.rs) own the
catalogue slug, display title, ordered panes and stable axis rows (`AxisRow`).
Each axis row carries a component-specific string value and display label so
routes with custom public domains need no mount changes, and row values do not
shift when another row changes.

A pane builds a `poodle_node::Node` from the host's `RenderContext`. Static
axis rows render one representative each. Interactive Examples use typed state
and callbacks that enqueue typed events. The session drains events through pure
reducers, reusing the component's headless transitions. Controlled editors
(Select, TextInput, NumberInput and CodeInput) emit raw ordered input intents
into the reducer rather than transitions precomputed against the painted tree,
so inputs batched before a repaint apply in order against live session state.
Pages without host effects return `State` alone; pages with effects return
`(State, Option<Effect>)`.

The GPUI preview keeps one session, chooses the active pane, and converts its
node tree at the backend boundary. It is the one native host
([vision](../vision.md#direction)). After input it drains the session,
rebuilds when state changes, and applies effects to the rebuilt tree. The only
effect so far is focusing a node by ID. The shell owns navigation, tab
controls, scrolling, and capture framing.

Svelte remains the reference for page content and behaviour. Shared native
pages use the same representative examples and public axis values. The
snapshots and reducer tests cover the native composition independently of a
backend. Preview construction and mounted interaction checks supply separate
evidence for the host integration.
