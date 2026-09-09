# A1 divergence records

Executed A1 runs whose paired snapshots did not agree. A record here is not a
receipt and moves no ledger cell; it is the honest output of both extractors
for one shared scenario. The validator ignores this directory; only top-level
`*.json` receipts are evidence.

## Active records

No active divergence records. The final four g16.121 rows now have empty-diff
A1 receipts after applying the named-radio group law, using a programmatic
transcript append scenario, and aligning Menu's first enabled roving stop.

Reproduce the owned cohort with:

```sh
POODLE_NUCLEUS_RECEIPT_DIR=$PWD/target/nucleus-receipts \
  effigy regressions:native
```
