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

## Final attempt — certified and published 2026-09-05

Candidate `85609d941a208ff2f854e9f7c0e457089cc77d0e` was verified from a clean
detached checkout. Frozen install passed; `lucide-static` resolved at 1.31.0;
local `effigy release gates` passed after a temporary local reachability branch
was created and removed.

The mandatory branch dry run passed:
https://github.com/inflatable-cookie/poodle/actions/runs/33930305831

The lightweight tag was then created and pushed:

```text
git rev-parse v0.3.0
85609d941a208ff2f854e9f7c0e457089cc77d0e
git ls-remote --tags origin refs/tags/v0.3.0
85609d941a208ff2f854e9f7c0e457089cc77d0e refs/tags/v0.3.0
```

The tag dry run passed:
https://github.com/inflatable-cookie/poodle/actions/runs/33934223827

The publish run passed:
https://github.com/inflatable-cookie/poodle/actions/runs/33952493234

The `packed-tarballs` artifact was 930019 bytes. Its downloaded archive
SHA-256 was
`dc04659cd7a716f58a6a59f448c97ec5959855828d35de89e0e33acd0c4fea49`.

Registry proof:

```text
@inflatable-cookie/poodle-core latest = 0.3.0
@inflatable-cookie/poodle-svelte latest = 0.3.0
```

A fresh temporary npm consumer installed exact `@inflatable-cookie/poodle-core`
and `@inflatable-cookie/poodle-svelte` 0.3.0 with Svelte 5.56.8. The compiled
subpaths `@inflatable-cookie/poodle-core/icons` and
`@inflatable-cookie/poodle-svelte/types` resolved successfully.
