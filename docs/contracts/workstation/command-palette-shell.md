# CommandPaletteShell

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `CommandPaletteShell`
- Layer: `workstation`
- Summary: a shell-level command launcher overlay that hosts query input,
  result-list content, and shell-level dismissal/restoration behavior
- In scope: modal shell posture, initial focus to query input, result list
  viewport, close behavior, shell-level invocation/restoration
- Out of scope: fuzzy-search algorithm, command registry model, result ranking,
  action-discovery depth

## 2. Anatomy

```text
[Root Overlay]
  ├── [Backdrop]
  └── [Palette Surface]
        ├── [Query Input]
        └── [Results Viewport]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root Overlay | yes | shell-level modal overlay | overlay state |
| Backdrop | yes | background scrim | overlay tone, motion |
| Palette Surface | yes | centered command shell | surface, border, radius, elevation |
| Query Input | yes | command search field | control tokens, focus |
| Results Viewport | yes | command results host | scroll, separator, spacing |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `open` | `boolean` | `false` | no | controlled open state |
| `defaultOpen` | `boolean` | `false` | no | uncontrolled initial state |
| `ariaLabel` | `string \| null` | `null` | no | shell label, required when no visible title exists |
| `onOpenChange` | `(open: boolean) => void` | none | no | open-state callback |
| `onRequestClose` | `() => void` | none | no | close-intent callback |

### Controlled And Uncontrolled

- controlled: `open` plus `onOpenChange`
- uncontrolled: `defaultOpen`
- query state and results are host-owned or future deeper composite concerns

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| closed | default | palette hidden |
| opening | open transition begins | overlay enters and query input becomes target |
| open | open state true | modal palette visible |
| closing | dismissal begins | overlay exits and focus restores |

### Component States

Closed, opening, open, and closing states are required.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onOpenChange` | palette opens or closes | boolean | state callback |
| `onRequestClose` | user requests dismissal | none | escape, backdrop, or command commit intent |

## 6. Accessibility

### Semantics

- Role: dialog-like command launcher surface
- Required attributes: accessible name, modal semantics, query input focus
  target, results relationship
- Optional attributes: description text and shortcut hints
- Labeling rules: the query input label supplements the palette label; it does
  not replace it

### Keyboard

| Key | Behavior |
|-----|----------|
| shell invocation shortcut | opens the palette and focuses the query input |
| `Escape` | closes the palette and restores focus |
| `Tab` | moves between query input and result content according to palette design |
| result-list keys | remain owned by the result-view contract or host |

### Focus And Announcement

- focus entry: opening moves focus to the query input
- focus trap: modal shell behavior keeps focus within the palette while open
- focus restoration: close returns focus to the invoking shell control or prior
  focused element
- live-region behavior: none by default; result changes are future depth work
- GPUI-native accessibility mapping notes: GPUI must preserve modal semantics,
  initial query focus, and restoration; palette behavior may not depend on web
  dialog defaults

## 7. Layout

### Sizing

- palette surface centers within the shell and constrains width/height
- results viewport scrolls independently when long

### Composition

- parent expectations: `WorkspaceShell` utility overlays host
- child expectations: `SearchField`, list or result-shell content, modal
  overlay primitives
- resizing rules: query input stays visible while results scroll

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Backdrop | overlay tone and motion roles | modal emphasis |
| Palette Surface | surface, border, radius, elevation, and overlay roles | shell |
| Query Input | search field/control roles | command query |
| Results Viewport | `ScrollShell`, separator, and spacing roles | results host |

## 9. Svelte Notes

- expected substrate: `Dialog`, `SearchField`, and list-shell or result content
- wrapper strategy: algorithm and command wiring remain external to this shell
  posture contract

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::workstation::command_palette_shell`
- implementation-only details: GPUI may use a dedicated overlay entity, but
  modal semantics, initial focus, and restoration remain required

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] modal shell semantics and initial query focus match
- [ ] dismissal and focus restoration match
- [ ] query/results region relationship matches

### Tier 2: Visual Parity

- [ ] palette prominence and internal hierarchy use comparable token roles

### Tier 3: Implementation Freedom

- [ ] result virtualization and ranking stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| exact result-list implementation may differ | result engines differ by runtime | allowed | keep shell posture and focus semantics strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: shell command launchers, action discovery surfaces
- future follow-up: connect deeper workspace-shell orchestration and
  command-history behavior in later `g02` workstation milestones

## Next Task

Use `CommandPaletteShell` as the shell-level launcher posture and layer richer
`CommandPalette` discovery semantics on top of it instead of reopening modal
shell behavior.
