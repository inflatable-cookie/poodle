/**
 * Primitive capability roster (spec 066, g14.002): the single typed authority
 * for reusable substrate capabilities beneath component cases.
 *
 * Component interfaces declare capability names that must exist in this
 * roster. Profile-owned rows certify the channel here and leave semantic
 * proof to cards 003–007. Jetstream is program-deferred outside these rows.
 */

export type CapabilityFamily =
  | "structure"
  | "layout"
  | "surface"
  | "content"
  | "semantic"
  | "interaction"
  | "accessibility"
  | "overlay"
  | "input";

/** Who owns semantic proof for this capability. */
export type CapabilityOwner =
  | "g14.002"
  | "g14.003"
  | "g14.004"
  | "g14.005"
  | "g14.006"
  | "g14.007";

export type ObservationField =
  | "parts.present"
  | "parts.role"
  | "parts.name"
  | "parts.text"
  | "parts.icon"
  | "parts.states"
  | "parts.tokenRoles"
  | "parts.focusable"
  | "parts.focused"
  | "parts.focusVisible"
  | "parts.geometry"
  | "parts.channels"
  | "trace"
  | "node.field"
  | "node.a11y"
  | "gpui.focus"
  | "gpui.event"
  | "gpui.layout"
  | "image.regional";

export interface PrimitiveCapability {
  /** Stable ID. Component interfaces must use IDs from this roster. */
  id: string;
  family: CapabilityFamily;
  /** Public vocabulary items this row covers (no unclassified leftovers). */
  covers: readonly string[];
  owner: CapabilityOwner;
  /** Observation fields required when this card owns the row. */
  requiredObservations: readonly ObservationField[];
  /** Governing contract when a platform exclusion applies. */
  governingContract?: string;
  notes?: string;
}

/**
 * Finite roster. Every public poodle-node / layout / style / observation
 * vocabulary item maps to exactly one row, a profile deferral, or retirement.
 */
export const PRIMITIVE_CAPABILITIES = [
  // ── 1. structure ─────────────────────────────────────────────────────────
  {
    id: "structure.identity",
    family: "structure",
    covers: [
      "Node.id",
      "Node.children",
      "NodeKind::Container",
      "NodeKind::Text",
      "NodeKind::Icon",
      "NodeKind::Image",
      "NodeKind::Progress",
      "NodeKind::Button",
      "NodeKind::Input",
      "Node::intrinsic_text",
      "component-observation.v1.parts.*.present",
    ],
    owner: "g14.002",
    requiredObservations: ["parts.present", "node.field"],
  },
  {
    id: "structure.part-resolution",
    family: "structure",
    covers: [
      "PartDecl.resolve.web.self",
      "PartDecl.resolve.web.class",
      "PartDecl.resolve.web.icon",
      "PartDecl.resolve.native.self",
      "PartDecl.resolve.native.root-label",
      "PartDecl.resolve.native.first-text",
      "PartDecl.resolve.native.icon-side",
      "PartDecl.resolve.native.icon-named",
    ],
    owner: "g14.002",
    requiredObservations: ["parts.present", "parts.text", "parts.icon"],
  },

  // ── 2. layout ────────────────────────────────────────────────────────────
  {
    id: "layout.intent",
    family: "layout",
    covers: [
      "LayoutDirection",
      "LayoutSizing",
      "LayoutEdges",
      "LayoutSpacing.gap",
      "LayoutSpacing.padding",
      "LayoutSpacing.margin",
      "MainAxisAlignment",
      "CrossAxisAlignment",
      "LayoutAlignment",
      "LayoutOverflow",
      "LayoutIntent",
      "StyleDescriptor.layout",
    ],
    owner: "g14.002",
    requiredObservations: ["node.field", "parts.geometry"],
  },
  {
    id: "layout.geometry",
    family: "layout",
    covers: [
      "NodeStyle.fill_width",
      "NodeStyle.fill_height",
      "NodeStyle.flex_none",
      "NodeStyle.self_stretch",
      "NodeStyle.flex_grow",
      "NodeStyle.flex_fill",
      "NodeStyle.flex_shrink_zero",
      "NodeStyle.flex_basis",
      "NodeStyle.flex_basis_pct",
      "NodeStyle.width_pct",
      "NodeStyle.flex_wrap",
      "NodeStyle.min_width",
      "NodeStyle.max_width",
      "NodeStyle.min_height",
      "NodeStyle.max_height",
      "component-observation.v1.parts.*.geometry.height",
      "component-observation.v1.parts.*.geometry.minWidth",
      "component-observation.v1.parts.*.geometry.paddingLeft",
      "component-observation.v1.parts.*.geometry.paddingRight",
      "component-observation.v1.parts.*.geometry.radius",
      "component-observation.v1.parts.*.geometry.borderWidth",
    ],
    owner: "g14.002",
    requiredObservations: ["parts.geometry", "node.field"],
  },
  {
    id: "layout.position",
    family: "layout",
    covers: [
      "NodePosition::InFlow",
      "NodePosition::Relative",
      "NodePosition::Absolute",
      "NodePosition::Absolute.top",
      "NodePosition::Absolute.left",
      "NodePosition::Absolute.right",
      "NodePosition::Absolute.bottom",
    ],
    owner: "g14.002",
    requiredObservations: ["node.field"],
    notes: "Absolute placement result semantics owned by g14.005.",
  },

  // ── 3. surface ───────────────────────────────────────────────────────────
  {
    id: "surface.channels",
    family: "surface",
    covers: [
      "StyleDescriptor.background",
      "StyleDescriptor.text_color",
      "StyleDescriptor.border",
      "StyleDescriptor.corner_radii",
      "StyleDescriptor.opacity",
      "BorderDescriptor",
      "CornerRadii",
      "ColorValue",
      "component-observation.v1.parts.*.channels.background",
      "component-observation.v1.parts.*.channels.borderColor",
      "component-observation.v1.parts.*.channels.color",
      "component-observation.v1.parts.*.channels.opacity",
    ],
    owner: "g14.002",
    requiredObservations: ["parts.channels", "node.field"],
  },
  {
    id: "surface.extended",
    family: "surface",
    covers: [
      "StyleDescriptor.icon_color",
      "StyleDescriptor.shadow",
      "StyleDescriptor.visible",
      "StyleDescriptor.cursor",
      "StyleDescriptor.focus_ring_color",
      "StyleDescriptor.focus_ring_width",
      "CursorHint",
      "ShadowValue",
      "NodeStyle.shadow_layers",
      "ShadowLayer",
      "NodeStyle.border_color_top",
      "NodeStyle.border_color_left",
      "NodeStyle.border_color_bottom",
      "NodeStyle.border_dashed",
      "NodeStyle.border_bottom_width",
      "NodeStyle.border_right_width",
      "NodeStyle.border_top_width",
      "NodeStyle.border_left_width",
      "NodeStyle.gradient",
      "NodeStyle.grayscale",
    ],
    owner: "g14.002",
    requiredObservations: ["node.field"],
  },
  {
    id: "surface.state-patches",
    family: "surface",
    covers: [
      "StylePatch",
      "StylePatch.background",
      "StylePatch.border_color",
      "StylePatch.text_color",
      "StylePatch.opacity",
      "NodeStyle.hover",
      "NodeStyle.active",
      "NodeStyle.focus",
    ],
    owner: "g14.002",
    requiredObservations: ["parts.focusVisible", "node.field"],
  },
  {
    id: "surface.animation",
    family: "surface",
    covers: [
      "NodeStyle.animation",
      "NodeAnimation",
      "AnimProperty",
      "AnimEasing",
      "AnimLoop",
      "AnimKeyframe",
    ],
    owner: "g14.002",
    requiredObservations: ["node.field"],
    notes: "Declaration channel only; timing-driven visuals stay skipped in capture gates.",
  },

  // ── 4. content ───────────────────────────────────────────────────────────
  {
    id: "content.text-icon",
    family: "content",
    covers: [
      "NodeKind::Text.content",
      "NodeKind::Icon.name",
      "NodeKind::Icon.size",
      "NodeKind::Image.source",
      "NodeKind::Progress.fraction",
      "NodeKind::Button.label",
      "component-observation.v1.parts.*.text",
      "component-observation.v1.parts.*.icon",
    ],
    owner: "g14.002",
    requiredObservations: ["parts.text", "parts.icon", "node.field"],
  },
  {
    id: "content.typography",
    family: "content",
    covers: [
      "StyleDescriptor.typography",
      "TypographyDescriptor",
      "FontFamily",
      "NodeStyle.text_ellipsis",
      "NodeStyle.text_underline",
      "NodeStyle.text_underline_color",
      "NodeStyle.text_size",
      "NodeStyle.text_weight",
      "NodeStyle.text_italic",
      "NodeStyle.font_family",
      "NodeStyle.line_height",
      "NodeStyle.letter_spacing_em",
      "NodeStyle.text_align",
      "TextAlign",
      "NodeStyle.text_wrap",
      "NodeStyle.no_wrap",
    ],
    owner: "g14.002",
    requiredObservations: ["node.field"],
  },

  // ── 5. semantic ──────────────────────────────────────────────────────────
  {
    id: "semantic.token-roles",
    family: "semantic",
    covers: [
      "Node.roles",
      "TokenRoleDecl",
      "component-observation.v1.parts.*.tokenRoles",
    ],
    owner: "g14.002",
    requiredObservations: ["parts.tokenRoles"],
  },
  {
    id: "toggle",
    family: "semantic",
    covers: [
      "NodeA11y.toggled",
      "NodeToggled",
      "component-observation.v1.parts.*.states.pressed",
    ],
    owner: "g14.002",
    requiredObservations: ["parts.states"],
    notes: "Component capability id retained from Button. Mixed-state semantics defer to g14.003.",
  },
  {
    id: "semantic.disabled",
    family: "semantic",
    covers: [
      "Interaction.disabled",
      "component-observation.v1.parts.*.states.disabled",
    ],
    owner: "g14.002",
    requiredObservations: ["parts.states"],
  },
  {
    id: "semantic.selected",
    family: "semantic",
    covers: ["NodeA11y.selected"],
    owner: "g14.004",
    requiredObservations: ["node.a11y"],
  },
  {
    id: "semantic.expanded",
    family: "semantic",
    covers: ["NodeA11y.expanded"],
    owner: "g14.005",
    requiredObservations: ["node.a11y"],
  },

  // ── 6. interaction ───────────────────────────────────────────────────────
  {
    id: "focus",
    family: "interaction",
    covers: [
      "Interaction.focusable",
      "Interaction.on_focus_change",
      "component-observation.v1.parts.*.focusable",
      "component-observation.v1.parts.*.focused",
      "component-observation.v1.parts.*.focusVisible",
    ],
    owner: "g14.002",
    requiredObservations: ["parts.focusable", "parts.focused", "gpui.focus"],
  },
  {
    id: "activate",
    family: "interaction",
    covers: [
      "Interaction.on_activate",
      "component-observation.v1.trace",
    ],
    owner: "g14.002",
    requiredObservations: ["trace", "gpui.event"],
  },
  {
    id: "interaction.activate-modified",
    family: "interaction",
    covers: ["Interaction.on_activate_modified", "NodeModifiers"],
    owner: "g14.004",
    requiredObservations: ["node.field", "gpui.event"],
  },
  {
    id: "interaction.key",
    family: "interaction",
    covers: ["Interaction.on_key", "NodeKey"],
    owner: "g14.004",
    requiredObservations: ["node.field", "gpui.event"],
  },
  {
    id: "interaction.scrub",
    family: "interaction",
    covers: ["Interaction.on_scrub", "ScrubPhase"],
    owner: "g14.003",
    requiredObservations: ["node.field", "gpui.event"],
  },
  {
    id: "interaction.drag-drop",
    family: "interaction",
    covers: [
      "Interaction.on_drag",
      "NodeDragEvent",
      "NodeDragPhase",
      "Interaction.drag_payload",
      "Interaction.drop_zone",
      "Interaction.on_drop_hover",
      "Interaction.on_drop",
      "NodeDropEvent",
      "DropEdge",
    ],
    owner: "g14.007",
    requiredObservations: ["node.field"],
  },
  {
    id: "interaction.context",
    family: "interaction",
    covers: ["Interaction.on_context", "NodePoint"],
    owner: "g14.007",
    requiredObservations: ["node.field"],
  },

  // ── 7. accessibility ─────────────────────────────────────────────────────
  {
    id: "accessibility.projection",
    family: "accessibility",
    covers: [
      "NodeA11y.role",
      "NodeA11y.label",
      "NodeA11y.level",
      "NodeRole",
      "component-observation.v1.parts.*.role",
      "component-observation.v1.parts.*.name",
    ],
    owner: "g14.002",
    requiredObservations: ["parts.role", "parts.name", "node.a11y"],
    notes:
      "Certifies DOM + NodeA11y projection and that GPUI receives metadata. Mounted GPUI AT tree is contract-003 forced acceptance outside passing rows.",
    governingContract: "docs/contracts/003-native-accessibility.md",
  },

  // ── 8. overlay ───────────────────────────────────────────────────────────
  {
    id: "overlay.intent",
    family: "overlay",
    covers: ["NodeStyle.overlay"],
    owner: "g14.005",
    requiredObservations: ["node.field", "image.regional"],
    notes: "Channel certified as deferred; placement/dismissal/focus transfer owned by g14.005.",
  },

  // ── 9. input ─────────────────────────────────────────────────────────────
  {
    id: "input.value",
    family: "input",
    covers: [
      "NodeKind::Input.value",
      "NodeKind::Input.placeholder",
      "Node.caret",
      "NodeCaret",
      "SelectGranularity",
    ],
    owner: "g14.006",
    requiredObservations: ["node.field"],
  },
  {
    id: "input.editing",
    family: "input",
    covers: [
      "Interaction.on_text_change",
      "Interaction.on_edit_key",
      "Interaction.on_select_range",
      "Interaction.on_edit_insert",
      "Interaction.on_submit",
      "Interaction.on_cancel",
    ],
    owner: "g14.006",
    requiredObservations: ["node.field", "gpui.event"],
  },
] as const satisfies readonly PrimitiveCapability[];

export type PrimitiveCapabilityId = (typeof PRIMITIVE_CAPABILITIES)[number]["id"];

export const PRIMITIVE_CAPABILITY_IDS: readonly PrimitiveCapabilityId[] =
  PRIMITIVE_CAPABILITIES.map((row) => row.id);

const CAPABILITY_ID_SET = new Set<string>(PRIMITIVE_CAPABILITY_IDS);

/** True when `id` is a known roster capability. */
export function isPrimitiveCapabilityId(id: string): id is PrimitiveCapabilityId {
  return CAPABILITY_ID_SET.has(id);
}

/** Throws when any capability name is unknown. Used by interface validation. */
export function assertKnownCapabilities(names: readonly string[], where: string): void {
  for (const name of names) {
    if (!isPrimitiveCapabilityId(name)) {
      throw new Error(
        `${where}: unknown capability '${name}' — must be one of the primitive roster`,
      );
    }
  }
}

/** Rows this card must execute across Svelte, React, Rust, and GPUI. */
export function ownedPrimitiveCapabilities(): readonly PrimitiveCapability[] {
  return PRIMITIVE_CAPABILITIES.filter((row) => row.owner === "g14.002");
}

export function serializePrimitiveRoster(): unknown {
  return {
    schema: "primitive-capability-roster.v1",
    jetstream: "program-deferred",
    gpuiMountedAccessibility: {
      status: "forced-acceptance",
      contract: "docs/contracts/003-native-accessibility.md",
      note: "GPUI 0.2.2 has no mounted accessibility tree for Poodle content. Omission is deliberate and observable; do not schedule a parallel tree.",
    },
    capabilities: PRIMITIVE_CAPABILITIES.map((row) => ({ ...row })),
  };
}
