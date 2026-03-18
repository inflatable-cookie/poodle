# Roadmap Generation Index

- `g01`
  - Status: completed
  - Range: `001` to `014`
  - Notes: repository/program bootstrap, token model, contract system, Svelte
    and GPUI substrates, primitive suite, workstation shells, Underlay bridge,
    and first parity baseline

- `g02`
  - Status: completed
  - Range: `001` to `016`
  - Notes: advanced composites, product/workstation depth, documentation and
    examples, preview/docs cleanup, API cleanup, packaging, and release
    baseline; `g02.001` forms/validation, `g02.002` table/bulk-action,
    `g02.003` browse-shell, `g02.004` detail-display, `g02.005`
    picker/relation workflow, `g02.006` media/embed, and `g02.007`
    hardening baselines landed; `g02.008` command palette depth, `g02.009`
    shell-depth, `g02.010` dock/split/tab orchestration, `g02.011`
    advanced accessibility hardening, `g02.012` docs/example
    discoverability, `g02.013` preview/docs usability hardening, `g02.014`
    package API and parity-debt cleanup, and `g02.015` packaging/release
    baseline are now in place

- `g03`
  - Status: completed
  - Range: `001` to `014`
  - Notes: migration policy, parity automation, docs publishing, first real
    downstream adoption tranches, ecosystem validation, change control, and
    mature extension support are now explicit; `g03.001` freezes token
    evolution and compatibility policy, `g03.002` adds the parity registry and
    generated evidence artifact, `g03.003` and `g03.004` harden docs/publish
    and performance posture, `g03.005` freezes recipe-based downstream styling
    overrides, `g03.006` through `g03.009` make extension, adoption, and GPUI
    validation boundaries explicit, and `g03.010` through `g03.014` complete
    the accessibility, release, acceptance, onboarding, and closeout baselines

- `g04`
  - Status: completed
  - Range: `001` to `018`
  - Notes: Underlay component parity, new component families, feature depth
    for existing components, and specialist editing/media surfaces; `g04.001`
    formalizes the Underlay parity gap register and implementation priority,
    `g04.002` adds AlertDialog, FormDialog, and ConfirmAction dialog patterns,
    `g04.003` adds FileUpload, MediaPicker, EmbedInput, and EmbedPreview,
    `g04.004` adds SplitButton and CardRadioGroup with Card selection
    extensions, `g04.005` deepens TextInput, Field, InlineEditableField, and
    SlugField, `g04.006` adds TimeAgo and DurationInput, `g04.007` adds
    ReorderableList, AutonomousList, and OrderBy, `g04.008` adds Code display
    and ColorPicker, `g04.009` adds NavCard, NavCardGrid, and ListCard,
    `g04.010` adds Skeleton presets and PageLoading, `g04.011` deepens
    DataTable and Select, `g04.012` adds LogList and StateTile trends,
    `g04.013` adds MarkdownEditor, `g04.014` adds AudioPlayer and
    VideoPlayer, `g04.015` explores block editor feasibility, `g04.016`
    hardens remaining feature gaps, `g04.017` ensures specimen and docs
    coverage, and `g04.018` closes the generation

- `g05`
  - Status: completed
  - Range: `001` to `014`
  - Notes: GPUI foundation, spec crates, cross-runtime parity baseline, and
    demo alignment; `g05.001` contract audit and parity priority matrix,
    `g05.002` theme runtime and token application, `g05.003` through `g05.006`
    primitive spec crates (structural, action, selection, overlay), `g05.007`
    and `g05.008` composite spec crates (form, data, browse, workstation),
    `g05.009` workstation shell specs, `g05.010` accessibility proof,
    `g05.011` parity report and delta register, `g05.012` through `g05.014`
    demo-app audit, contract, and Svelte rebuild; deferred GPUI g04-surface
    parity, expanded component coverage, downstream adoption proof, and docs
    promotion to `g06` to enable shared multi-renderer contract redesign

- `g06`
  - Status: completed
  - Range: `001` to `015`
  - Notes: shared multi-renderer contract layer; `g06.001` architecture audit
    and extraction plan, `g06.002` crate restructuring and rename
    (`pug-gpui-*` → `pug-*`), `g06.003` typed token resolution system
    (`[f32; 4]` colors, `f32` pixel values), `g06.004` layout intent
    abstraction, `g06.005` event model and interaction abstraction, `g06.006`
    style descriptor intermediate representation, `g06.007` renderer adapter
    trait definition, `g06.008` through `g06.012` spec expansion to full
    124-component Svelte surface (53 new specs), `g06.013` Jetstream rendering
    constraint document, `g06.014` multi-renderer parity validation tooling,
    and `g06.015` closes the generation

- `g07`
  - Status: completed
  - Range: `001` to `015`
  - Notes: GPUI rendering build-out; `g07.001` adapter crate setup and theme
    integration, `g07.002` through `g07.006` primitive rendering batches
    (structural, action, selection, overlay, informational), `g07.007` through
    `g07.009` composite rendering batches (form, data, editing/navigation),
    `g07.010` workstation shell updates, `g07.011` cross-runtime parity report,
    `g07.012` GPUI demo-app parity, `g07.013` downstream reference-app proof,
    `g07.014` published docs and evaluator onboarding, and `g07.015` closes
    the generation; can run in parallel with `g08` after `g06` completes

- `g08`
  - Status: completed
  - Range: `001` to `014`
  - Notes: Jetstream rendering build-out; `g08.001` adapter crate setup and
    token bridge, `g08.002` theme construction from Pug tokens with 19 semantic
    color constants, `g08.003` through `g08.008` component rendering batches
    (8 structural, 4 action, 8 input, 8 selection, 22 feedback/informational,
    13 overlay), `g08.009` and `g08.010` composite rendering (5 form, 36
    data/editing/operational), workstation rendering (13 shell specs),
    `g08.011` component subset review (full parity — 0 unsupported),
    `g08.012` integration demo scene with 4 game screens (32 nodes),
    `g08.013` cross-runtime parity report for Jetstream target, and `g08.014`
    closes the generation; 142 tests passing

- `g09`
  - Status: completed
  - Range: `001` to `018`
  - Notes: GPUI first-class component build-out, preview parity, and visual
    fidelity; `g09.001` component gap audit (Svelte surface vs GPUI
    components), `g09.002` and `g09.003` missing primitive components
    (structural, informational, input, temporal), `g09.004` and `g09.005`
    missing composite components (form, data, editing, media, operational),
    `g09.006` missing workstation components, `g09.007` through `g09.012`
    specimen upgrades replacing all hand-built mockups with real Pug component
    instances, `g09.013` preview app shell (section tabs, sidebar, per-component
    pages), `g09.014` display controls (theme, density, control size, route
    state), `g09.015` shared 6-screen demo app, `g09.016` systematic visual
    parity audit (Svelte vs GPUI), `g09.017` cross-runtime parity report
    update, and `g09.018` generation closeout; goal is GPUI preview app
    that looks and works identically to the Svelte version

- `g10`
  - Status: completed
  - Range: `001` to `016`
  - Notes: Jetstream first-class component build-out, preview app, and visual
    fidelity; `g10.001` preview app scaffold and navigation shell, `g10.002`
    component registry and specimen framework, `g10.003` through `g10.009`
    per-component specimen build-out (structural, action, input, selection,
    feedback, overlay, date/time, composites, workstation), `g10.010` display
    controls (theme, density, control size), `g10.011` demo scene expansion
    to 6-screen parity, `g10.012` visual parity audit (Svelte/GPUI vs
    Jetstream), `g10.013` delta register update and native adaptation
    documentation, `g10.014` cross-runtime parity report refresh, `g10.015`
    accessibility and input model verification (keyboard + gamepad), and
    `g10.016` generation closeout; goal is Jetstream preview app with full
    component coverage and visual fidelity matching Svelte and GPUI

- `g11`
  - Status: completed
  - Range: `001` to `013`
  - Notes: workstation contracts and Svelte implementation; `g11.001` audits
    downstream workstation pressure and freezes the generalized target shape,
    `g11.002` through `g11.009` define workstation contracts (window hosts,
    region grammar, strip rails, resize/collapse, docks, tabs, panel variants,
    hosted surfaces) with formal contract files in `docs/contracts/workstation/`,
    `g11.010` and `g11.011` implement the new surface in Svelte, `g11.012`
    refreshes docs/specimens and performs downstream adoption proof, and
    `g11.013` closes the generation; GPUI and Jetstream implementations are
    deferred to `g12` and `g13` respectively

- `g12`
  - Status: planned
  - Range: `001` to `007`
  - Notes: GPUI workstation parity; `g12.001` audits g11 contracts against
    current GPUI surface, `g12.002` and `g12.003` implement workstation
    substrate in GPUI (windows/regions/strips then docks/tabs/panels/hosted
    surfaces), `g12.004` adds GPUI preview specimens and demo coverage,
    `g12.005` produces Svelte/GPUI parity evidence and delta register,
    `g12.006` performs downstream adoption proof against Loophole Spark, and
    `g12.007` closes the generation

- `g13`
  - Status: planned
  - Range: `001` to `007`
  - Notes: Jetstream workstation parity; `g13.001` audits g11 contracts
    against Jetstream rendering constraints, `g13.002` and `g13.003` implement
    feasible workstation substrate in Jetstream, `g13.004` adds preview
    specimens and demo coverage, `g13.005` produces cross-runtime parity
    evidence (Svelte/GPUI/Jetstream), `g13.006` updates the delta register
    with native adaptations and exclusions, and `g13.007` closes the
    generation

## Next Task

Open `g11.001` and begin the downstream workstation gap audit and generalized
target-shape freeze. Keep the generation focused on generalized workstation
substrate work rather than downstream app semantics.
