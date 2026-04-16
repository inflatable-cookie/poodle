# g10.014 GPUI Overlay Architecture, Navigation, and Slider Fidelity

Status: pending
Owner: Poodle core
Depends on: g10.013
Updated: 2026-04-16

## Purpose

Close the remaining high and medium-priority GPUI component gaps from the April 2026
audit. This milestone targets the items that either (a) require shared infrastructure
(a floating overlay utility for Tooltip and Popover), (b) are missing meaningful
interaction behaviour (keyboard navigation, events), or (c) have ARIA gaps that make
components non-functional for assistive technology.

g10.013 covers the self-contained token/formula fixes. This milestone covers everything
that touches layout architecture, keyboard handling, or cross-component utilities.

---

## 1. Floating overlay utility (Tooltip + Popover shared root problem)

**Files:**
- `packages/gpui/components/src/primitives/tooltip.rs`
- `packages/gpui/components/src/primitives/popover.rs`

Both components have the same root problem: the overlay surface is a sibling `div` in
a `flex-col` — it pushes surrounding layout, cannot appear above other content, and the
`placement` prop is silently ignored (it only affects the element-ID hash).

These are two manifestations of one missing piece: a GPUI floating overlay primitive.

**Direction:**
- Implement a shared `floating_overlay` helper (in a new file, e.g.
  `packages/gpui/components/src/primitives/floating_overlay.rs`) that:
  - Uses GPUI `absolute` positioning relative to a known anchor rect
  - Reads the anchor bounds via `on_children_prepainted` (the same pattern used for
    Slider drag)
  - Computes position per `placement` (top/bottom/left/right + start/center/end
    alignment)
  - Applies a simple viewport-clamp so the surface stays on screen
- Wire Tooltip and Popover to use this helper
- Popover additionally needs: `dismiss_on_outside_interact` wired to an outside-click
  handler, `role="dialog"` and `aria-expanded` on the trigger, `initialFocus` prop
  acted on (shift focus into surface on open)

Note: full portal layering (surface escapes ancestor clip regions) is deferred unless
product asks. The anchor-rect + absolute approach handles the common case.

---

## 2. TabStrip — layout tokens, ARIA, keyboard

**File:** `packages/gpui/components/src/primitives/tab_strip.rs`

The most complete gap set of any component in the audit:

**Layout (lines 136–140):** All spacing is hardcoded — `gap(px(6.0))`, `px(10.0)`,
`py(px(6.0))`, `text_size(px(12.0))`. The size and density props on the spec have no
effect. Replace with token-resolved values; size and density must visibly change the
rendered tab strip.

**ARIA (contract §5):**
- `role="tablist"` on root — absent
- `role="tab"` on each tab item — absent
- `aria-selected` on active tab — absent
- `aria-disabled` on disabled tabs — absent

**Keyboard (contract §5):**
- `←`/`→` arrow key navigation already partially present; verify it skips disabled tabs
- `Home` / `End` — jump to first/last tab — absent
- `Delete` — close closable tab — absent

---

## 3. Slider — anatomy, track height, and valueCommit

**File:** `packages/gpui/components/src/primitives/slider.rs`

**Non-contract anatomy (lines 199–207):** A min/current/max label row is rendered below
the track. This element is not in the contract anatomy (Root → Track → Fill → Control
only). Remove it. If the preview specimen needs value display, add it in the specimen
file, not the component.

**Track height hardcoded (lines 128–134):** `match effective_size { Xs => 4.0, Sm => 5.0, ... }`
— replace with token-resolved track height per the contract size table.

**`valueCommit` event absent:** No `on_mouse_up` handler exists. The contract requires
an `on_value_commit` callback that fires on pointer release (distinct from `on_change`
which fires on every drag step). Add it.

**ARIA (medium):** `aria-valuemin`, `aria-valuemax`, `aria-valuenow`, `aria-valuetext`,
`aria-disabled` are all absent. Document as known GPUI delta if the fluent `div` API
cannot carry these. If GPUI 0.2.2 can express them via `element_state` or a custom
`Element`, implement them.

---

## 4. RangeSlider — thumb colour, keyboard, and valueCommit

**File:** `packages/gpui/components/src/primitives/range_slider.rs`

**Thumb colour wrong (lines 219–229):** Both thumbs use accent fill and accent border.
Single-thumb Slider uses `elevated` background + `border-default`. RangeSlider should
match — the accent-coloured thumbs are a deviation from the contract with no justification.

**Keyboard navigation absent:** Arrow keys have no effect on range values. Contract
requires:
- Focus on low thumb: `←`/`↓` decrements low, `→`/`↑` increments low
- Focus on high thumb: same for high
- `Tab` cycles between thumbs

**`valueCommit` absent:** Same as Slider — add `on_value_commit` firing on mouse-up.

**Vertical orientation:** Acknowledged as not implemented in code (lines 341–343).
Either implement or add an explicit `#[allow]` comment documenting it as a known delta
with a forward reference to a future ticket.

---

## 5. Tabs — close affordance, ARIA, icon gap token

**File:** `packages/gpui/components/src/primitives/tabs.rs`

**Close affordance (line 443):** Closable card tabs use a `"×"` Unicode literal. Replace
with `Icon::from_spec(IconSpec::new("x"), theme)` so the icon scales correctly and
inherits colour.

**ARIA:** `role="tablist"` on root and `role="tab"` + `aria-selected` on items are absent.
Same requirement as TabStrip.

**Icon gap hardcoded (line 38):** `gap(px(6.0))` in `build_tab_label`. Replace with
`space.inline.xs` token.

---

## 6. Select — option groups

**File:** `packages/gpui/components/src/primitives/select.rs`

If `ChoiceOption` carries group/section metadata, all options currently render flat with
no section headers. The Svelte groups options visually with a section label and separator.
Implement section-header rendering in the dropdown.

This is lower priority than the fake-search fix in g10.013 but belongs in the same
component; do it in the same pass if scope allows.

---

## Execution checklist

**Floating overlay:**
- [ ] Implement `floating_overlay` helper with anchor-rect positioning and placement enum
- [ ] Wire Tooltip to use floating_overlay; verify all placement values render correctly
- [ ] Wire Popover to use floating_overlay
- [ ] Popover: wire `dismiss_on_outside_interact`
- [ ] Popover: add `role="dialog"` and `aria-expanded` on trigger
- [ ] Popover: act on `initialFocus` prop (shift focus into surface on open)

**TabStrip:**
- [ ] Replace all hardcoded layout literals with token-resolved values
- [ ] Size and density props must visibly change the rendered output
- [ ] Add `role="tablist"`, `role="tab"`, `aria-selected`, `aria-disabled`
- [ ] Add `Home` / `End` keyboard shortcuts
- [ ] Add `Delete` to close closable tabs
- [ ] Verify arrow-key navigation skips disabled tabs

**Slider:**
- [ ] Remove min/current/max label row from component (move to specimen if needed)
- [ ] Replace hardcoded track-height match with token-resolved values
- [ ] Add `on_value_commit` callback firing on mouse-up
- [ ] Document or implement ARIA value attributes

**RangeSlider:**
- [ ] Fix thumb colour: elevated background + border-default (match single Slider)
- [ ] Implement keyboard navigation for both thumbs
- [ ] Add `on_value_commit` firing on mouse-up
- [ ] Document vertical orientation as a known delta or implement it

**Tabs:**
- [ ] Replace `"×"` close literal with `Icon::from_spec(IconSpec::new("x"), theme)`
- [ ] Add `role="tablist"`, `role="tab"`, `aria-selected`
- [ ] Replace `gap(px(6.0))` in `build_tab_label` with `space.inline.xs` token

**Select:**
- [ ] Implement option group / section-header rendering if ChoiceOption carries group metadata

## Next task

Start with the floating overlay utility — it unblocks both Tooltip and Popover in one
pass. Then TabStrip (most complete gap set), then Slider/RangeSlider as a pair, then
the Tabs polish items.
