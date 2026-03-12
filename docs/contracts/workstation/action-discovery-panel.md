# ActionDiscoveryPanel

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `ActionDiscoveryPanel`
- Layer: `workstation`
- Summary: an inline workstation discovery surface for suggested, recent, or scoped command groups outside the modal palette
- In scope: grouped action sections, invocation hinting, actionable rows, and empty discovery posture
- Out of scope: global modal launch behavior, full ranking engines, persistence of recents, or app-specific command generation

## 2. Accessibility

- grouped action sections need visible headings and accessible action names
- grouped sections should remain addressable as navigation/discovery structure rather than anonymous stacks
- inline discovery actions must be keyboard reachable without requiring the modal palette
- empty discovery states must stay explicit and avoid collapsing into blank shells
- GPUI-native accessibility mapping notes: GPUI must preserve grouped discovery structure and direct action reachability even where inline workstation panels are fully custom-rendered

## 3. Next Task

Use `ActionDiscoveryPanel` when workstation surfaces need visible suggested or recent actions without forcing all discovery through the modal launcher.
