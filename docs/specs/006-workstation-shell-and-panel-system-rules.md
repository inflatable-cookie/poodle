# 006 Workstation Shell And Panel System Rules

Status: active
Updated: 2026-03-11
Depends on: `002-component-contract-template-and-parity-rules.md`, `003-accessibility-and-assistive-technology-baseline.md`, `004-overlay-focus-dismissal-and-layering-rules.md`, `005-product-composite-composition-and-information-architecture-rules.md`

## Purpose

Freeze the baseline rules for workstation-shell contracts so Flint can support
desktop and pro-tool applications without turning the shared layer into
Loophole-specific workspace logic.

## Core Rule

The workstation layer owns reusable shell structure, not app-specific
workstation behavior.

It may define:

- shell headers
- panel headers and panel tabs
- dock regions
- split views
- workspace shell region expectations
- shell-level utility overlays such as command-palette posture

It may not define:

- DAW transport bars
- timelines
- mixer strips
- clip editors
- plugin/device-chain editors
- or app-specific command registries

## Panel System Rule

The workstation layer may define panel-system semantics at the contract level:

- active panel selection
- panel tab naming
- collapse and expand posture
- shell-level move, reorder, rename, or close intents
- dock-region structure

It does not own the full orchestration engine yet.

That means the baseline layer may expose:

- callbacks or intents
- shell state models
- and persistence-relevant structure

while leaving:

- drag/drop engines
- persistence backends
- and cross-window orchestration policy

to host code or later milestones.

## Workspace Region Rule

Workstation shells must preserve named, addressable region structure.

At minimum that can include:

- app header
- project header
- surface tabs
- left/right/bottom dock regions
- center work area
- utility overlay host

Hosts may vary the exact region mix.
They may not collapse everything into an unlabeled custom canvas.

## Split And Resize Rule

Resizable shell regions must support:

- documented orientation
- divider semantics
- size or ratio meaning
- keyboard resize support when the divider is focusable
- and focus continuity during resize or collapse

Pointer-only split behavior is not sufficient.

## Collapse And Focus Rule

When shell regions collapse or expand:

- the region name must remain discoverable
- the remaining affordance to restore the region must remain reachable
- focus must move to a deterministic fallback if the collapsed region contained
  the focused element
- and the new state must be conveyed through control semantics, not only visual
  change

This is especially strict for GPUI, where custom shell code can easily drop the
equivalent accessibility state.

## Surface And Panel Tab Rule

Surface tabs and panel tabs are both workstation shell navigation, but they are
not interchangeable.

- `SurfaceTabs` switch top-level workspace surfaces
- `PanelTabs` switch panels within a dock or panel group

Contracts must keep their:

- naming
- close/move/reorder intents
- focus restoration behavior
- and contextual menu posture

explicitly distinct.

## Persistence Boundary Rule

Workstation contracts may define the shape of persistence-relevant data such as:

- region sizes
- collapsed state
- tab order
- active panel
- active surface

They must not require one storage backend or one sync mechanism.

Persistence mechanics remain:

- app-owned
- Underlay-bridge-owned
- or future orchestration-layer work

depending on the milestone.

## Command Palette Shell Rule

The workstation baseline may define command-palette shell posture:

- modal overlay role
- initial focus to query input
- shell-level invocation and dismissal
- focus restoration

It does not yet define full command-discovery depth such as:

- ranking
- search semantics
- grouping
- or action-history heuristics

Those remain future work.

## Accessibility Rule

Workstation shell contracts must define:

- named-region hierarchy
- tab and panel naming
- collapse semantics
- keyboard reachability in dense shell environments
- focus restoration after close, collapse, move, or modal utility overlays
- and non-color-only state cues for dirty, active, collapsed, or disabled shell
  states

GPUI must provide equivalent native accessibility tree structure and keyboard
semantics.

## GPUI-Specific Rule

GPUI implementations must explicitly preserve:

- shell-region hierarchy
- panel and surface tab semantics
- resizable divider accessibility
- modal utility overlay behavior
- collapse-state exposure
- and focus continuity across shell transitions

These are not optional platform deltas.

## Contract Author Checklist

Workstation-shell authors must define:

- what shell scope the component owns
- what panel-system behavior is in scope
- what remains host-owned or orchestration-owned
- region naming and accessibility structure
- collapse/resize/focus behavior where relevant
- persistence-relevant state shape
- GPUI-native accessibility expectations

## Seed Evidence

The first contracts that explicitly exercise this baseline are:

- `docs/contracts/workstation/panel-surface.md`
- `docs/contracts/workstation/app-header.md`
- `docs/contracts/workstation/project-header.md`
- `docs/contracts/workstation/panel-header.md`
- `docs/contracts/workstation/panel-tabs.md`
- `docs/contracts/workstation/surface-tabs.md`
- `docs/contracts/workstation/dock-region.md`
- `docs/contracts/workstation/split-view.md`
- `docs/contracts/workstation/workspace-shell.md`
- `docs/contracts/workstation/command-palette-shell.md`

## Next Task

Use this workstation baseline in later `g02` workstation depth tranches without
reopening the shell semantics already closed in `g01`.
