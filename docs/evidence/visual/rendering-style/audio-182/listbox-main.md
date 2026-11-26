# Listbox comparison on main

The two unlisted listbox size failures predate this change. On main
`03cd8e7e8e860dda9a4d7991621a926d2b1eda9b`, the focused visual report
compared two pairs and exited 1:

```text
listbox [eclipse-compact-md] size — svelte 992x692 vs react 992x686
listbox [iceberg-compact-md] size — svelte 992x692 vs react 992x686
```

Reproduction used a fresh `git archive` of that commit, with dependencies
and web distributions built via `effigy bootstrap:deps` and `effigy web:dist`.
The fixture's Git HEAD was set to the archived commit for the build receipt.
Only the fixture's visual harness was adjusted: the two preview ports changed
from 4174/4180 to 4378/4380 to avoid other checkouts, and the `visual:report`
selector appended `--slug=listbox` to focus the sweep. Component sources,
styles, specimens, and visual debt entries were unmodified. Running
`effigy visual:report` in that fixture produced the output above; no process
already listening on those ports was reused.

The full report on the PR's audio styles compared 346 pairs and likewise
reported these two unlisted listbox differences; the other failures were in
the existing visual debt inventory.
