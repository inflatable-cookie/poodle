# 003 - Native Accessibility

Status: active
Owner: Poodle core
Applies to: every contract with ARIA requirements, on the GPUI target

## The Fact

Poodle's GPUI backend maps direct node roles, names, states, and values
through GPUI 1.22.0's AccessKit path. The live, non-activating AXUIElement
proof selects its window by title and walks only that window's subtree. The
latest run read 9 elements with no unnamed meaningful elements, and its
planted-control case detected an unnamed `AXButton` at depth 2. The clean
window stayed non-frontmost for 239 successful samples and the planted window
for 237; neither had failed reads.
It verifies Button role, name, and toggled `AXValue`; disclosure Button role
and name; Checkbox role, name, and nonempty `AXValue`; Slider role, name,
range, orientation, and value text; and a selected Tab's role, name, and
state. The census A2 hold is lifted for Button, Checkbox, Slider, and Tabs,
the rows with these live assertions. The proof does not verify the disclosure
Button's `AXExpanded` state or distinguish a mixed Checkbox from a checked
one through `AXValue`. ID-based relationships (`controls`, `labelled_by`, and
`described_by`) are also not projected yet; the GPUI node backend has no
translation from their shared string IDs to AccessKit node IDs. Every other
portable component retains its A2 hold because there is no live platform-tree
assertion for it yet.

| Runtime | State | Evidence |
|---------|-------|----------|
| gpui-unofficial 1.22.0 | **AccessKit live; bounded platform proof.** The selector read named controls without foreground activation and detected its planted unnamed control. It verifies Button, Checkbox, Slider, and selected Tab signals, clearing A2 for Button, Checkbox, Slider, and Tabs. Disclosure expanded state, Checkbox mixed-state distinction, and ID-based relationships remain unverified; every other portable row retains its A2 hold. | `packages/gpui/native-accessibility-proof.json#livePlatformProofAttempt`; the census records the cleared rows and remaining holds. |

## What This Means For `aria_label`

**102 `poodle-specs` structs carry `aria_label`.** Where it goes now depends on
the target:

- **Svelte and React** consume it, held to that by the axe sweep.
- **GPUI** carries the metadata through the shared renderer into
  `poodle-node`, and its node backend projects direct roles, names, states,
  and values into AccessKit; ID-based relationships remain unprojected.
  `effigy test:gpui-ax` reads a bounded live platform tree, checks names and
  representative semantics, and verifies a planted unnamed control is found.
  It clears A2 for Button, Checkbox, Slider, and Tabs. The disclosure Button's
  expanded state, Checkbox's mixed-state distinction, and ID-based
  relationships remain unverified.

## Update 2026-09-05

Upstream gpui merged AccessKit on 2026-05-27 (zed-industries/zed#56065).
crates.io `gpui` is still 0.2.2, but `gpui-unofficial` republishes upstream
unmodified at every Zed release tag (Apache-2.0; the previously GPL
`ztracing`/`zlog` crates were relicensed Apache upstream on 2026-09-01 and are
clean from republish 1.19.0-pre). The operator chose to evaluate that route
instead of building a fork-free adapter. The feasibility spike succeeded at
the API boundary; the roll-up and current
`gpui-unofficial` adoption gates (Nucleus lead)
preserve the result. "No API to build against" is no longer true of upstream,
only of the crates.io 0.2.2 pin. At that time adoption still waited on a
buildable published `gpui-apple` crate and live platform-tree proof.

## Update 2026-10-06

Operator ruling (Tom, 2026-10-06): switch Poodle's GPUI dependency from
crates.io `gpui` 0.2.2 to `gpui-unofficial`, pinned to an exact stable release
(1.22.0 at the time of the ruling), then adopt its AccessKit path. The dependency switch and the node-metadata wiring into AccessKit both landed.
A bounded live, non-activating platform-tree
selector. After macOS Accessibility trust was granted to Paseo, the selector
read clean and planted-control trees while proving neither window became
frontmost. The live proof clears A2 for Button, Checkbox, Slider, and Tabs;
other portable rows retain their holds because no live platform-tree
assertion exists for them yet. Button disclosure expansion, Checkbox mixed
state, and ID-based relationships remain unverified. Re-checked the same day:
`gpui-apple-gpui-unofficial` 1.22.0 bundles its
own `gpui` source and resolves it inside its crate directory, clearing the
September sibling-path gate; the `bzip2-1.0.6` licence is allowed in
`deny.toml`.
Pre-release (`-pre`) tags are not pinned; upgrades are deliberate, one stable
tag at a time. GPUI 1.22.0 still has only `Definite`/`Auto` lengths and
`Normal`/`Nowrap` white space, so the two recorded layout and wrapping Known
Deltas remain upstream items.

## Consequences For Planning

- **Do not claim GPUI A2 from the dependency switch or mapping code alone.**
  The live GPUI proof clears A2 only for Button, Checkbox, Slider, and Tabs,
  for the signals recorded above. Every other portable row retains its census
  hold because it has no live platform-tree assertion yet. Disclosure
  expansion, mixed Checkbox state, and ID-based relationships are still
  unverified. The in-memory test platform exposes no live accessibility tree;
  do not use its snapshots as platform proof.
- **Continue component-level accessibility work below A2.** Poodle can still
  prove roles, labels, state, value, keyboard operation, and focus in its
  mounted node/backend path. The live tree verifies bounded Button,
  Checkbox, Slider, Tab, and disabled-control examples. Button expanded and
  Checkbox mixed semantics remain unverified at the platform boundary, as do
  ID-based relationships.
- **Do not read the GPUI accessibility artifacts as runtime proof.**
  `packages/gpui/native-accessibility-proof.json` is explicit about this in its
  own non-goals — it forbids claiming "mounted assistive-technology proof for
  sections that still only have spec-level or crate-test evidence". Its evidence
  is spec-level and crate-test level. That is the correct reading.
## Extending The GPUI Proof

The source API and Poodle's direct role/name/state/value mapping are in place.
The trusted AXUIElement proof read both the clean and planted-control trees
without bringing either window to the foreground. It clears A2 for Button,
Checkbox, Slider, and Tabs: Button role/name/toggled state, disclosure Button
role/name, Checkbox role/name/nonempty value, Slider role/name/range/
orientation/value, and selected Tab role/name/state. The disabled Button and
planted unnamed-control checks also pass. The proof does not expose disclosure
`AXExpanded`, distinguish mixed from checked in Checkbox `AXValue`, or
translate ID-based relationships. Every other portable component retains
its A2 hold because there is no live platform-tree assertion for it yet.

To extend the proof, add a named fixture for the component and assert its
contracted role, accessible name, and relevant state or value in the live
AXUIElement tree. Keep the clean-tree name audit, planted unnamed-control
case, and non-activation monitor passing; then add only that component to the
proof artifact's `a2ClearedComponents` and regenerate the census. Do not
infer platform evidence from headless node snapshots.

Component-level node semantics, keyboard, focus, and assistive-technology
quality remain payable work and are governed by the active GPUI runway.

## The 48 Contracts This Governs

**48 component contracts carry ARIA requirements inside their GPUI Notes
section.** Poodle's GPUI backend maps direct node roles, names, states, and
values through AccessKit. The live proof verified bounded Button, Checkbox,
Slider, and Tab signals and clears those census rows; every other portable
row retains A2. ID-based relationships remain unprojected.
`checkbox.md` is
representative: it requires the indeterminate state to be "accessible to
assistive technology as `aria-checked="mixed"`", and requires exposing "state,
and accessible name through the native accessibility tree". Poodle content
is projected into AccessKit and the live AXUIElement tree verifies bounded
named controls. Its assertions clear the Button, Checkbox, Slider, and Tabs
rows while leaving the disclosure-expanded and mixed-Checkbox state gaps
explicit.

Those requirements are not deleted or softened. They describe what each
component must do and remain the specification its implementation is measured
against. The mapping is implemented; the current platform evidence clears A2
for the asserted Button, Checkbox, Slider, and Tabs properties, with the
unverified Button expanded and Checkbox mixed-state details recorded above.
