# 009 Node backend conformance

Status: pure pilot
Updated: 2026-11-25
Owner: Poodle core

The [conformance kit](../../../packages/contracts/conformance/) exercises the
boundary between `poodle-node` and a native backend. Fixtures construct the
real Rust nodes with logical viewports and expected observations. Backends
mount them through their production converters and read the resulting layout,
accessibility tree, and paint output.

The inventory uses exhaustive Rust enum matches and struct patterns. Changes
to a covered declaration require a fixture or a classification. The pilot
covers all node kinds, roles, and accessibility fields, plus representative
layout. The progress ring and resolved icon cases check positive and negative
samples of emitted geometry. The accessibility focus cases check the
published focus target and sequential tab-stop count. Callbacks, tooltip
transitions, and drag registrations are inventoried for a later dispatch
pass. The activation fixture carries an action and recording closure and
reports `not_run` in the pure pilot.

The kit owns backend vocabulary evidence. Component roster coverage stays
with the [census](../../evidence/gpui/gpui-functionality-census.md). Component
construction, machine behavior, and web parity continue to use the existing
contract tests, A1/A2 receipts, and machine vectors. The pilot receipts live
beside the parity ledger and do not feed it yet.

## Running and recording

An external backend adds `poodle-conformance` as a development dependency
from Poodle Git at the full commit it consumes. Pin its own `poodle-node`
dependency to that same commit so the fixture trees have the same Rust
type identity. Iterate `cases()`, implement `Backend::mount` and
`Backend::read`, and use `Case::run` and `Receipt::new` to write the run.
Normalize values from engine output to the fixture's probe vocabulary.
Report unavailable observer channels as `unsupported`.

A [`poodle-node-conformance.v1` receipt](../../evidence/conformance/receipt-v1.schema.json) carries the vocabulary and case
manifest SHA-256 digests, the Poodle and backend source commits, the runner
lockfile SHA-256, and the platform. Each case carries its ID, proof level,
status, and observations in check order. A mismatch records the target,
probe, expected value, and actual value. `Receipt::from_json` rejects stale
stamps, unknown or missing IDs, duplicate rows, and mismatched proof levels.
Read a historical receipt with the kit at its stamped commit.

The GPUI runner targets macOS and Linux. Run the headless test at the clean
source commit and write the receipt in a subsequent commit.

```sh
POODLE_GPUI_CONFORMANCE_RECEIPT=/absolute/path/to/gpui.json \
  effigy test:gpui-node-backend -- --locked
```

The runner uses the existing headless mount with a test-window delegate
that activates AccessKit and captures its tree updates and GPUI's presented
scene. Geometry samples check submitted quads and tessellated paths in
logical coordinates. They carry scene-level proof without claiming pixel,
compositor, or operating-system assistive-technology coverage. The runner
uses distinct fixture rectangles to match AccessKit nodes to the backend's
measured element bounds.

## Cost and stop rule

Keep the corpus rooted in the compiler-visible node vocabulary. A runner
that needs a component-specific branch stops the pilot. Count authored,
generated, runner and wiring lines, and name any replaced probes in each
pilot PR. This pilot reuses the headless mount and replaces no existing probes.

The prior cross-runtime component pilot was rejected in `g14.008` and its
machinery removed in `g14.021`. It added 22,746 lines against 472 replaced,
and the comparator exposed 1,205 differences after HistoryCenter was added
to its scope. The [historical cost rules](../specs/archive/066-executable-component-conformance.md#cost-and-replacement-rules)
record the reasons for that decision.
