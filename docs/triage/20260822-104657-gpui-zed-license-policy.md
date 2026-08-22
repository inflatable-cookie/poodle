# GPUI/Zed dependency licence policy

Status: open triage
Found: PR #66 review, 2026-08-22
Candidate home: a bounded release-gap card before `g15.050`

`effigy release gates` reaches the GPUI node-backend licence audit. Current
`cargo deny` policy rejects `libbz2-rs-sys` (`bzip2-1.0.6`) and the
GPL-3.0-or-later `zlog`, `ztracing`, and `ztracing_macro` crates in the pinned
Zed graph.

PR #66 attempted to make the gate green by adding crate-scoped licence
exceptions and five allowed Git sources. That is not release-automation
plumbing. It changes Poodle's distribution policy and needs an explicit
decision about which crates and artifacts ship, whether the licences are
compatible with the intended distribution, and whether the dependency graph
should instead be changed.

Do not weaken or bypass the gate. Route a separate evidence-led card after
`g15.049` and before the release candidate. The card should verify the actual
distributed artifact graph, record the licence/source rationale, and either
approve narrowly reviewed exceptions or remove/replace the dependencies.
