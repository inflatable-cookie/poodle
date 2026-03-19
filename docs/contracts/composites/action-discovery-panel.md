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

## 3. Specimen Definitions

### Grouped Actions

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Grouped actions | `items` with 7 actions across 3 groups (File: Save/Open File/Close Tab, Edit: Find in Files/Find and Replace, View: Toggle Terminal/Toggle Sidebar), each with shortcut hints | Grouped action list with section headings (File, Edit, View), action rows showing title and shortcut label |

### With Descriptions And Badges

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With descriptions and badges | `items` with 3 actions across 2 groups (CI/CD: Deploy to Production with description and "Dangerous" badge, Open Preview with description and shortcut; Tools: Run Linter with shortcut) | Grouped action list with description text below titles, badge pill on dangerous action, shortcut hints on applicable rows |

### Empty State

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Empty state | `items=[]`, `state="empty"` | Empty discovery posture with no action rows visible |

## 4. Next Task

Use `ActionDiscoveryPanel` when workstation surfaces need visible suggested or recent actions without forcing all discovery through the modal launcher.
