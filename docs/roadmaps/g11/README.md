# g11 Workstation Contracts, Svelte Implementation, And Downstream Adoption Proof

Status: complete
Updated: 2026-03-17

## Context

`g09` and `g10` made GPUI and Jetstream preview applications first-class,
which proved broad component parity and specimen coverage. However, downstream
workstation-style applications still hit a real ceiling: the current Pug
workstation layer is good at panels and splits, but not yet good enough at
window ownership, strip rails, region snapshots, collapse behavior, richer
dock semantics, or hosted external surfaces.

That gap is now visible from real downstream pressure instead of speculation.
Loophole Aura's current-versus-legacy shell audit shows the missing semantics
clearly: downstream teams can build bespoke frames and DAW workflows locally,
but Pug is still undersupplying the generalized workstation substrate those
applications want to reuse.

`g11` fixes that without turning Pug into an app-specific shell library. The
goal is not to add DAW widgets. The goal is to deepen the reusable workstation
substrate so downstream applications can build serious multi-region products on
top of it with less local workaround code and less renderer drift.

This generation focuses on contract design and Svelte implementation. GPUI
workstation parity follows in `g12` and Jetstream workstation parity in `g13`,
each as dedicated generations against the same contracts produced here.

## Starting State

- `WorkspaceShell`, `SplitView`, `DockRegion`, `PanelSurface`, `SurfaceTabs`,
  `PanelTabs`, `AppHeader`, `ProjectHeader`, and `ShellStatusBar` exist
- Svelte workstation support is ahead of GPUI and Jetstream but all are bounded
  by the same missing semantics
- Pug does not yet provide:
  - explicit workspace window hosts and active-surface-per-window ownership
  - a richer region snapshot covering `topStrip`, `bottomStrip`, `leftStrip`,
    `rightStrip`, `left`, `right`, `centerTop`, and `centerBottom`
  - strip rail components with compact and collapsed variants
  - workstation-level resize/collapse affordances
  - deeper dock behavior for tab placement, collapsed-tab posture, and active
    panel emphasis
  - a dedicated hosted external-surface model for plugin/editor-like content
- downstream applications still need too much local shell glue to get from
  Pug primitives to a credible workstation interface

## Exit State

- Pug has documented, generalized workstation contracts following the 12-section
  contract template for all new workstation components
- layout snapshots support the full region grammar needed by downstream
  workstation applications, including collapse and tab-placement state
- strip rails, resize/collapse affordances, richer docks, stronger panel
  variants, and hosted external surfaces exist as generalized workstation
  substrate implemented in Svelte
- Svelte specimens and docs demonstrate the new surface
- downstream adoption proof demonstrates that Pug can now carry more of a real
  workstation shell without absorbing downstream app semantics
- GPUI and Jetstream implementations are deferred to `g12` and `g13`
  respectively, building against the same contracts

## Non-Goals

- no DAW-specific timeline, browser, mixer, inspector, or plugin workflows in
  Pug core
- no Loophole-specific app frame, toolbar, or project-management semantics
- no GPUI or Jetstream implementation in this generation (deferred to `g12` and
  `g13`)

## Milestone Status

| ID | Milestone | Depends On | Class | Status |
|----|-----------|------------|-------|--------|
| 001 | Workstation downstream gap audit and generalized target-shape freeze | g10.016 | Foundation | Complete |
| 002 | Workspace window host model and surface-ownership contract | 001 | Foundation | Complete |
| 003 | Region grammar and layout-snapshot expansion | 002 | Foundation | Complete |
| 004 | Strip rail family and orientation variants | 003 | Foundation | Complete |
| 005 | Resize handles, split dividers, and collapse affordances | 004 | Foundation | Complete |
| 006 | Dock-region depth, collapsed posture, and active-panel emphasis | 005 | Core build | Complete |
| 007 | Window-aware surface tabs and panel-tab orchestration | 006 | Core build | Complete |
| 008 | Panel variants and utility-versus-focused surface treatments | 007 | Core build | Complete |
| 009 | Hosted external-surface and plugin-editor container contract | 008 | Core build | Complete |
| 010 | Svelte workstation implementation batch 1 — windows, regions, strips | 009 | Implementation | Complete |
| 011 | Svelte workstation implementation batch 2 — docks, tabs, panels, hosted surfaces | 010 | Implementation | Complete |
| 012 | Docs, specimens, and downstream reference adoption proof | 011 | Hardening | Complete |
| 013 | Generation closeout and next-program cutover | 012 | Closure | Complete |

## Dependency Shape

```text
g10.016 Previous Generation Complete
  -> 001 Downstream Audit
      -> 002 Window Host Model
          -> 003 Region Grammar + Layout Snapshot
              -> 004 Strip Rails
                  -> 005 Resize + Collapse Affordances
                      -> 006 Dock Depth
                          -> 007 Window-Aware Tabs
                              -> 008 Panel Variants
                                  -> 009 Hosted External Surfaces
                                      -> 010 Svelte Batch 1
                                          -> 011 Svelte Batch 2
                                              -> 012 Docs + Proof
                                                  -> 013 Closeout
```

## Execution Lanes

### Lane A: Contract and Surface Design

`001 -> 002 -> 003 -> 004 -> 005 -> 006 -> 007 -> 008 -> 009`

### Lane B: Svelte Implementation

`010 -> 011`

### Lane C: Evidence and Closeout

`012 -> 013`

## Milestone Details

### 001 — Workstation Downstream Gap Audit and Generalized Target-Shape Freeze

Audit real downstream workstation consumers, especially Loophole Aura's
current-versus-legacy shell analysis, and translate that pressure into a
generalized target shape for Pug. The result should distinguish:
- reusable workstation substrate gaps that belong in Pug
- downstream application semantics that must stay out of Pug
- Svelte-ready versus GPUI-ready work

### 002 — Workspace Window Host Model and Surface-Ownership Contract

Define the generalized contract for:
- workspace window hosts
- active surface per window
- surface movement between windows
- window-local surface ordering and identity

This must stay product-neutral while still being strong enough for real
multi-window workstation applications. Produce a contract file in
`docs/contracts/workstation/` following the 12-section template.

### 003 — Region Grammar and Layout-Snapshot Expansion

Expand workstation layout snapshots so they can express:
- `topStrip`, `bottomStrip`, `leftStrip`, `rightStrip`
- `left`, `right`, `centerTop`, `centerBottom`
- collapse state
- tab placement and active-panel state

The snapshot should remain renderer-agnostic and suitable for all consumers.
Produce or update contract files in `docs/contracts/workstation/` following the
12-section template.

### 004 — Strip Rail Family and Orientation Variants

Add generalized strip rail components for all four edges with:
- icon-first and mixed-content modes
- active, idle, compact, and collapsed variants
- orientation-aware spacing, borders, and affordances

Produce a contract file in `docs/contracts/workstation/` following the
12-section template.

### 005 — Resize Handles, Split Dividers, and Collapse Affordances

Add the workstation interaction primitives needed to adjust layout without
local ad hoc shell code:
- resize handles
- split dividers
- region collapse/expand affordances
- clearer contract for user-adjusted versus programmatic layout changes

Produce contract files in `docs/contracts/workstation/` following the
12-section template.

### 006 — Dock-Region Depth, Collapsed Posture, and Active-Panel Emphasis

Deepen `DockRegion` so it can express:
- tab placement options
- collapsed-tab posture
- stronger active-panel emphasis
- quieter inactive panels
- dock-local panel status without app-specific logic

Update the existing dock-region contract in `docs/contracts/workstation/`
following the 12-section template.

### 007 — Window-Aware Surface Tabs and Panel-Tab Orchestration

Upgrade surface tabs and panel tabs so they understand window ownership and
can coexist cleanly:
- surface tabs for top-level workspace surfaces
- panel tabs for dock-local panel switching
- explicit active/inactive semantics
- ordering, move, and close behaviors that stop short of app-specific policy

Update the existing tab contracts in `docs/contracts/workstation/` following
the 12-section template.

### 008 — Panel Variants and Utility-Versus-Focused Surface Treatments

Add a generalized variant system for workstation panels:
- quieter support/utility panels
- balanced standard panels
- stronger focused/detail panels

This should reduce downstream need for one-off visual treatments in browser,
inspector, and focused-editor contexts. Update the panel-surface contract in
`docs/contracts/workstation/` following the 12-section template.

### 009 — Hosted External-Surface and Plugin-Editor Container Contract

Define the generalized container for hosted external surfaces:
- embedded external surface host
- detached/external window host relationship
- title/identity/focus posture
- bounded status/failure states

This is specifically a generalized host container, not a plugin workflow.
Produce a contract file in `docs/contracts/workstation/` following the
12-section template.

### 010 — Svelte Workstation Implementation Batch 1

Implement the new workstation substrate in Svelte for:
- window host primitives
- expanded region snapshots
- strip rail components
- resize/collapse affordances

### 011 — Svelte Workstation Implementation Batch 2

Complete the Svelte side by implementing:
- deeper docks
- window-aware surface and panel tabs
- panel variants
- hosted external-surface containers
- specimen and example coverage for the new surface

### 012 — Docs, Specimens, and Downstream Reference Adoption Proof

Refresh documentation, specimen coverage, and one downstream reference adoption
proof. The proof should demonstrate that the new Pug workstation substrate
meaningfully reduces downstream shell glue without requiring Pug to absorb
downstream app semantics. Svelte-only parity evidence at this stage.

### 013 — Generation Closeout and Next-Program Cutover

Close the generation with an explicit record of:
- what workstation substrate is now genuinely shared
- what still belongs downstream
- what remains deferred for `g12` (GPUI) and `g13` (Jetstream)

## Cross-Project Dependencies

| Dependency | Direction | Description |
|-----------|-----------|-------------|
| Loophole Aura/Chorus workstation audit | Downstream -> Pug | Real workstation pressure and target-shape evidence |
| Pug g10 | Pug -> Pug g11 | Preview-app and parity groundwork must already be in place |
| Svelte workstation package | Internal | Primary implementation target for this generation |

## Next Task

Execute `g11.001` and freeze the downstream-driven but still generalized
workstation target shape before opening any new component or implementation
batch.
