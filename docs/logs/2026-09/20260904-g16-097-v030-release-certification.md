# g16.097 — v0.3.0 Release Certification

Status: blocked serially on g16.104
Date: 2026-09-04
Card: `docs/roadmaps/g16/097-v030-release-certification.md`
Candidate under test: `eab436eefc1a65d0e0cde518a113a51c5d4d7f4e`

## Third attempt — retracted before publish

The operator-authorized tag `v0.3.0` pointed at the candidate above. Dry run
`33908714014` failed in `Release gates` before version agreement, pack, or
publish:

```text
git merge-base eab436eefc1a65d0e0cde518a113a51c5d4d7f4e origin/main
fatal: Not a valid object name origin/main
```

Run: https://github.com/inflatable-cookie/poodle/actions/runs/33908714014

No package was published. Registry proof after the failed run remained:

```text
@inflatable-cookie/poodle-core latest = 0.2.2
@inflatable-cookie/poodle-svelte latest = 0.2.2
```

The tag was then retracted under the operator-confirmed direction:

```text
git tag -d v0.3.0
Deleted tag 'v0.3.0' (was eab436eefc)
git push origin :refs/tags/v0.3.0
 - [deleted] v0.3.0
git ls-remote --tags origin refs/tags/v0.3.0
(empty)
```

No replacement tag exists. g16.104 must merge the checkout-base fix and pass
the mandated branch dry run before g16.097 resumes tagging or publishing.
