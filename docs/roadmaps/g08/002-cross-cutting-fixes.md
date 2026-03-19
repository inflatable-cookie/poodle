# g08.002 Cross-Cutting Fixes: Disabled Opacity, Hover Colors, Geometry Tokens

Status: planned
Owner: Pug Core
Depends on: g08.001

## Contract Check

Before starting, re-verify that the token names used in the fixes below still
match the current contracts. Specifically check:
- `semantic.opacity.disabled` (or whatever the current disabled opacity token is)
- `semantic.size.control-height` and size variants (sm/md/lg)
- `semantic.space.*` padding and gap tokens
- `semantic.radius.control` and radius variants

## Goals

Fix the three systemic issues found across 18+ components so that batch fixes
(003–005) can follow a consistent, correct pattern.

## Execution Checklist

### Disabled Opacity (~18 components)

- [ ] Replace every instance of hardcoded `0.48` or `0.5` disabled opacity
      with `resolve_opacity(theme, spec.disabled_opacity_token())`
- [ ] Verify `disabled_opacity_token()` exists on each affected spec; add if
      missing

### Hardcoded Hover/Active Colors (~10 components)

- [ ] Replace `hsla(0.0, 0.0, 0.5, 0.04)` hover overlays with `color_mix`
      using elevated/surface tokens, matching the pattern in `button.rs`
- [ ] Ensure active states use `color_mix` at a stronger ratio

### Hardcoded Geometry (~15 components)

- [ ] Replace `px(36.0)` heights with token-resolved control height
- [ ] Replace `px(12.0)` horizontal padding with token-resolved padding
- [ ] Replace `px(6.0)` radii with token-resolved radius
- [ ] Replace `px(8.0)` gaps with token-resolved gap
- [ ] Verify each spec has the necessary token methods; add if missing

## Acceptance Criteria

- [ ] Zero instances of hardcoded `0.48`/`0.5` opacity in component files
- [ ] Zero instances of hardcoded `hsla(0.0, 0.0, ...)` hover colors
- [ ] All standard geometry values (height, padding, radius, gap) resolve
      from spec token methods
- [ ] All changes compile and render correctly in preview app
