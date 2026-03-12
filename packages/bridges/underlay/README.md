# Underlay Bridge

Underlay bridge scaffolding for token aliases, theme translation, and
wrapper-preservation rules that ingest Pug artifacts without exposing Pug
directly to Underlay app code.

## Package Shape

```text
packages/bridges/underlay/
  README.md
  package.json
  css/
    pug-to-underlay.css
  ts/
    index.ts
    token-map.ts
    theme-map.ts
    component-wrappers.ts
```

## Ownership Rule

- Pug owns canonical token meaning and component contracts.
- The bridge owns alias maps and wrapper-preservation guidance.
- Underlay owns app-facing APIs and rollout.

## Zero-Leak Goal

Underlay apps should not need:

- direct Pug imports
- Pug token variable names
- Pug component names
- Pug-specific prop names

## Next Task

Use this bridge package as the baseline while `g01.014` defines parity
evidence and the downstream extension contract.
