# g10.012 GPUI Runtime Truth And Deferred Work Closure

Status: active (execution)
Owner: Poodle core
Depends on: g10.011
Updated: 2026-04-13

## Purpose

Replace the retired `docs/roadmaps/g08/delta-register.md` as the **living** place
for:

- what is a **real** GPUI 0.2.2 / platform constraint vs Poodle implementation debt
- what is **accepted cosmetic** divergence from Svelte
- **executable backlog** to close functional gaps (no “deferred” without an owner path)

Normative process for intentional deltas (automation, harness): still
`docs/specs/058-cross-runtime-parity-report-delta-register-and-acceptance-harness-expansion.md`.

## Governing refs

- `docs/specs/002-component-contract-template-and-parity-rules.md`
- `docs/contracts/components/`
- GPUI in use: `gpui = "0.2.2"` (`packages/gpui/components/Cargo.toml`)

---

## 1. Corrected narratives (were wrong in g08 register)

- **Slider drag (former D-101, single-thumb):** Implementation used `window.bounds()`
  for ratio math, so values were wrong and interaction felt broken. The GPUI
  `div` API exposes `on_children_prepainted` to read the track’s layout bounds;
  mouse handlers now use those bounds. **RangeSlider** uses the same track-bounds
  pattern plus a keyed drag map so the active thumb survives `cx.notify()` rebuilds
  during drag (`packages/gpui/components/src/primitives/range_slider.rs`).

- **Dashed borders:** GPUI fluent `div` supports `.border_dashed()` (`BorderStyle::Dashed`).
  Prior “GPUI has no dashed border” comments were **incorrect**; Region, FileUpload,
  list not-live state, and preview specimens should use it.
- **Spinner motion:** `Spinner` ring + grid variants use `with_animation` (GPUI
  animation API). g08 **D-007** (“static loader only”) is **obsolete** for current
  Poodle code.
- **Focus chrome:** Many Poodle GPUI components already apply **visible** focus
  treatment via `.focus(|style| …)` (border/shadow). The old **D-001** blanket
  (“GPUI shows nothing”) conflated **browser `outline` / `:focus-visible`**
  parity with **any** focus indicator. Remaining gap is **contract-exact** ring
  geometry vs **assistive** naming (see §3).

---

## 2. True limits today (re-check when bumping GPUI)

- **Assistive technology / semantic tree:** Crates.io `gpui` 0.2.2 does not expose
  ARIA-like attributes on the same fluent `div()` path Poodle uses for primitives.
  Contract `role` / `aria-*` parity with Svelte is **not** a small local fix;
  it tracks **upstream** accessibility APIs (or a different component architecture).
- **Letter-spacing:** No styled API found for contract `letter-spacing` on labels;
  treat as **cosmetic delta** unless GPUI adds it.
- **CSS box-shadow fidelity:** GPUI uses preset shadows; fine-grained CSS shadow
  matching stays **cosmetic** unless API expands.

---

## 3. Implementation debt (Poodle must ship real behavior)

These are **not** “GPUI cannot”; they need proper engineering in `poodle-gpui-components`
(and sometimes preview state), tracked as production work:

| Area | Issue | Direction |
|------|--------|-----------|
| **TextInput / TextArea** | `IntoElement` + `on_key_down` hack; no IME, selection, clipboard | Adopt GPUI `EntityInputHandler` pattern (see `gpui/examples/input.rs`) or equivalent; likely requires **Entity-backed** input or custom `Element` bridge, not only `div().child(text)` |
| **Select** | Listbox was in document flow (pushed layout) | **Improved:** listbox is `absolute` below the trigger inside a `relative` root (`select.rs`). Viewport-aware horizontal flip / portal layering vs Svelte remains future work if needed |
| **Slider** | Was using `window.bounds()` for drag math (broken) | **Fixed:** track bounds via `on_children_prepainted` + pointer handlers |
| **RangeSlider** | Was display-only | **Fixed:** track bounds + thumb hit-test + `on_change` with `interaction_key`; preview wired |
| **Residual `px(...)`** | Composites, date grids, specimens | Continue tokenization per contract tables; add tokens where missing (**former D-107**). **Progress:** overlay/menu/select/datetime/file-upload tokens above; `Accordion` / `DateTimeRangePicker` / `TimeZoneSelect` spacing; `FileUpload` helper + browse padding via `typography.caption.size`, `space.{inline.xs,control.y,inline.md}`; `TimeZoneSelect` rows `space.inline.sm` + `space.control.y`; scroll caps in Select / picker shells |
| **Adapter vs components** | Adapter cannot depend on `poodle-gpui-components` | Host crate split or neutral mount layer for real `FormShell` / validation family previews (**former D-108 / D-109**) |

Exit for each row: **specimen + preview prove** the behavior (keyboard, pointer, focus),
not a visual stub.

---

## 4. Accepted cosmetic deltas (no milestone unless product asks)

- Font rasterization vs browser (hinting, subpixel)
- SVG mask tint vs inline SVG
- `color_mix` vs CSS `color-mix` rounding
- Primary button border via `color_mix_black` vs exact CSS mix (**D-100** class)
- Drawer min-width choices vs fluid web layout (**D-104** class)

---

## 5. Closed / merged elsewhere

- **D-105** tri-state switch: treat as resolved in tree; normal regression only.
- **D-106** region dashed: **closed** by using `.border_dashed()` (this milestone).

---

## Execution checklist

- [x] Retire g08 delta register as living doc; point here
- [x] Apply `border_dashed` where contract requires dashed outline
- [ ] TextInput / TextArea: EntityInputHandler-grade editing (batch milestone or g11)
- [x] Select: listbox absolutely positioned under trigger (viewport flip / portal deferred if product asks)
- [x] Slider: pointer drag uses real track layout bounds (`packages/gpui/components/src/primitives/slider.rs`)
- [x] RangeSlider: dual-thumb drag (`range_slider.rs` + stateful `packages/gpui/preview/src/specimens/range_slider.rs`)
- [ ] Token sweep for remaining literals (prioritize contract-facing surfaces; menu max-height done — see §3 **Residual** row)
- [ ] Adapter mount strategy for form/validation composites

## Next task

Execute §3 rows in priority order agreed with stakeholders; after TextInput overhaul,
re-run GPUI preview specimens for keyboard + IME smoke paths.
