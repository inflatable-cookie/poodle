# Value Track: Workstation UI Patterns

Status: complete (findings documented)
Created: 2026-03-11
Updated: 2026-03-11
Priority: high (informs g01.012)

## Purpose

Research DAW/pro-tool UI patterns for panels, docks, split views, and workspace management to inform:
- Workstation shell composites (g01.012)
- Panel system design
- Dock/region layout primitives
- Persistence and state management

---

## Key Findings

### VS Code Layout Architecture

VS Code uses a **6-area layout model**:

| Area | Description | Pug Equivalent |
|------|-------------|----------------|
| **Editor** | Main content area | Document/workspace surface |
| **Primary Side Bar** | Explorer, Search, etc. | Left panel group |
| **Secondary Side Bar** | Chat, custom views | Right panel group |
| **Panel** | Terminal, Output, Problems | Bottom panel group |
| **Activity Bar** | View switcher | Leftmost icon bar |
| **Status Bar** | Info, context | Bottom status area |

**Key Capabilities:**
- Drag and drop views between regions
- Primary/Secondary Side Bar can be left or right
- Panel can be bottom, left, or right
- Views can be grouped (tabs)
- Layout persistence across sessions

### Dock/Panel System Patterns

#### Common Panel Types (DAW/Pro Tools)

| Panel Type | Purpose | Examples |
|------------|---------|----------|
| **Inspector** | Properties/details | Logic Pro Inspector, VS Code Outline |
| **Browser** | Asset/library navigation | Ableton Browser, VS Code Explorer |
| **Mixer** | Channel/audio controls | All DAWs (Loophole-specific) |
| **Timeline** | Time-based editing | All DAWs (Loophole-specific) |
| **Utility** | Tools, meters, misc | VS Code Terminal, Debug console |
| **History** | Undo/redo stack | Photoshop History, VS Code Timeline |

#### Docking Behaviors

**VS Code Pattern:**
- Views drag between side bars and panel
- Drop targets: center (replace), edges (split)
- Views can be tabbed within a container
- Each view has a header with title and actions

**DAW Patterns:**
- Panels often dock to screen edges
- Some panels float (mixer, plugin windows)
- Dual-monitor support common
- Workspace layouts save/restore

#### Split View Mechanics

**SplitContainer Model (WinForms/Avalonia):**
```
SplitContainer
├── Panel1 (collapsible)
├── Splitter (draggable)
└── Panel2 (collapsible)
```

**Properties:**
- `SplitterDistance` - Position of divider
- `Panel1Collapsed` / `Panel2Collapsed` - Hide panels
- `IsSplitterFixed` - Lock divider position
- Orientation: horizontal or vertical

**Nested Splits:**
- Splits can be nested for complex layouts
- Common pattern: 3-panel (left, center, right) or (top, center, bottom)

### Panel Anatomy

**Standard Panel Structure:**
```
[Panel Root]
├── [Header]
│   ├── [Icon] (optional)
│   ├── [Title]
│   ├── [Actions] (optional)
│   ├── [Collapse Button]
│   └── [Context Menu]
├── [Content Area]
│   └── [Scrollable Content]
└── [Resize Handle] (if resizable)
```

**Header Actions (common):**
- Collapse/expand
- Close/remove
- Maximize/restore
- Pin (keep visible)
- Settings/options
- Context menu

### Persistence Patterns

**State to Persist:**
- Panel visibility (visible/hidden)
- Panel size (width/height)
- Panel position (docked location)
- Split ratios
- Tab selection
- Panel order

**VS Code Approach:**
- Layout state saved automatically
- Restored on window reopen
- Settings stored in user profile
- Workspaces have own layout state

**Storage Format:**
```json
{
  "layout": {
    "panels": {
      "left": {
        "width": 250,
        "visible": true,
        "tabs": ["explorer", "search"],
        "activeTab": "explorer"
      },
      "bottom": {
        "height": 200,
        "visible": false
      }
    },
    "splits": {
      "main": {
        "orientation": "horizontal",
        "ratio": 0.6
      }
    }
  }
}
```

### Dockview.dev Reference

**Features (from dockview research):**
- Draggable, resizable, dockable panels
- Tabbed panel groups
- Grid-based layout
- Persists layout state
- Multiple themes
- API for programmatic control

**Key APIs:**
- `panel()` - Create panel definition
- `dock_view()` - Create layout container
- Add/remove tab callbacks
- Event system for layout changes

---

## Recommendations for Pug

### Pug's Workstation Layer Scope

**In Scope (Layer 3 - Workstation Shell):**
- Panel surface/Shell components
- Dock/region layout primitives
- Split view with draggable dividers
- Tab strip for panel organization
- Persistence hooks (structure only)

**Out of Scope (App-Specific):**
- Loophole transport bars
- DAW-specific panels (mixer, timeline)
- App-specific workspace logic

### Proposed Component Suite

```
Layer 3 - Workstation Shell
├── Panel
│   ├── Panel.Header
│   ├── Panel.Content
│   └── Panel.Actions
├── PanelGroup
│   └── Tabbed panel container
├── SplitView
│   ├── SplitView.Panel
│   └── SplitView.Divider
├── DockRegion
│   └── Drop targets for docking
├── Workspace
│   └── Layout persistence context
└── ActivityBar
    └── View switcher
```

### Panel Component Contract

**Props:**
- `title: string`
- `icon?: Component`
- `collapsible?: boolean`
- `defaultCollapsed?: boolean`
- `onCollapse?: (collapsed: boolean) => void`
- `actions?: PanelAction[]`

**Slots/Parts:**
- Header, Content, Actions, Resize handle

**States:**
- Expanded/collapsed
- Focused/unfocused
- Active/inactive (in tab group)

### SplitView Component Contract

**Props:**
- `orientation: 'horizontal' | 'vertical'`
- `defaultSplit: number` (0-1 ratio)
- `minSize?: number` (px or %)
- `onResize?: (ratio: number) => void`

**Parts:**
- Panel1, Panel2, Divider

**States:**
- Resizing/not resizing
- Panel1 collapsed
- Panel2 collapsed

### Persistence Strategy

**Pug Provides:**
- Layout structure types
- Serialization format
- Change events

**Apps Handle:**
- Storage mechanism
- Restore logic
- Multiple workspace support

---

## Related

- Source hub: [hub-gpui](../source-hubs/hub-gpui.md) (GPUI panel capabilities)
- Milestone: [g01.012](../../roadmaps/g01/012-workstation-shell-composites-and-panel-system-baseline.md)
- External: [dockview.dev](https://dockview.dev/) - Reference implementation

---

## Next Task

Create contract documents for Panel and SplitView components.
