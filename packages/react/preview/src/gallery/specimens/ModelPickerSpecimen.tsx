import { useState } from "react";
import {
  ModelPicker,
  type ModelCapabilityAxis,
  type ModelOption,
  type ModelSelection,
} from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

// ── Model marks ────────────────────────────────────────────────────────
// Two ways to put a mark beside a model:
//
//   icon:  "sparkles"          → a name from the icon registry (lucide via
//                                IconProvider), inheriting the current colour
//   image: { src, alt? }       → any image URL — a provider logo, a data URI,
//                                an asset path. Wins over `icon` when both set
//
// The logos below are inline data-URI SVGs so the preview stays offline; a real
// app would point `src` at its own asset.
const corvidLogo =
  "data:image/svg+xml;utf8," +
  encodeURIComponent(
    `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
       <rect width="24" height="24" rx="6" fill="#3b6ef5"/>
       <path d="M7 15.5 12 6l5 9.5H7z" fill="#fff"/>
     </svg>`,
  );
const corvidAltLogo =
  "data:image/svg+xml;utf8," +
  encodeURIComponent(
    `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
       <rect width="24" height="24" rx="6" fill="#12b886"/>
       <circle cx="12" cy="12" r="5.5" fill="none" stroke="#fff" stroke-width="2.5"/>
     </svg>`,
  );

// Five axes declared once, by key. No model uses all of them.
const axes: ModelCapabilityAxis[] = [
  {
    key: "effort",
    label: "Effort",
    kind: "select",
    options: [
      { value: "low", label: "Low" },
      { value: "medium", label: "Medium" },
      { value: "high", label: "High" },
    ],
    defaultValue: "medium",
  },
  {
    key: "fast",
    label: "Fast mode",
    kind: "toggle",
    description: "Trades a little depth for latency",
    onLabel: "Fast",
    offLabel: "Normal",
  },
  {
    key: "context",
    label: "Context window",
    kind: "select",
    options: [
      { value: "200k", label: "200K" },
      { value: "1m", label: "1M" },
    ],
    defaultValue: "200k",
  },
  {
    key: "verbosity",
    label: "Verbosity",
    kind: "select",
    options: [
      { value: "terse", label: "Terse" },
      { value: "normal", label: "Normal" },
      { value: "chatty", label: "Chatty" },
    ],
    defaultValue: "normal",
  },
  {
    key: "thinking",
    label: "Extended thinking",
    kind: "toggle",
    onLabel: "Thinking",
    offLabel: "Direct",
  },
];

// Two providers in one list — the cross-harness case. Every model exposes a
// different set: some reference shared axes by key, some rebind `effort` to
// their own levels, one has no knobs at all.
const models: ModelOption[] = [
  {
    value: "atlas-pro",
    label: "Atlas Pro",
    description: "Deepest reasoning, slowest responses",
    badge: "1M",
    icon: "sparkles",
    group: "Atlas",
    axes: ["effort", "fast", "context"],
  },
  {
    value: "atlas",
    label: "Atlas",
    description: "Balanced quality and latency",
    icon: "sparkles",
    group: "Atlas",
    // No long-context option on this tier.
    axes: ["effort", "fast"],
  },
  {
    value: "corvid-1",
    label: "Corvid 1",
    description: "Other provider, its own effort levels",
    image: { src: corvidLogo, alt: "Corvid" },
    group: "Corvid",
    axes: [
      // Same `effort` key, this provider's vocabulary, forced to a list.
      {
        key: "effort",
        control: "list",
        options: [
          { value: "minimal", label: "Minimal" },
          { value: "balanced", label: "Balanced" },
          { value: "deep", label: "Deep" },
        ],
        defaultValue: "balanced",
      },
      "verbosity",
    ],
  },
  {
    value: "corvid-ultra",
    label: "Corvid Ultra",
    description: "Seven-level scale plus a thinking toggle",
    badge: "Preview",
    image: { src: corvidLogo, alt: "Corvid" },
    group: "Corvid",
    axes: [
      // Past three options the axis renders as a list on its own. The binding
      // also relabels the shared key for this provider's vocabulary.
      {
        key: "effort",
        label: "Thinking budget",
        options: [
          { value: "minimal", label: "Minimal" },
          { value: "very-low", label: "Very low" },
          { value: "low", label: "Low" },
          { value: "medium", label: "Medium" },
          { value: "high", label: "High" },
          { value: "very-high", label: "Very high" },
          { value: "max", label: "Maximum" },
        ],
        defaultValue: "high",
      },
      "thinking",
    ],
  },
  {
    value: "corvid-mini",
    label: "Corvid Mini",
    description: "No knobs at all",
    image: { src: corvidAltLogo, alt: "Corvid Mini" },
    group: "Corvid",
    axes: [],
  },
  {
    value: "legacy-1",
    label: "Legacy 1",
    description: "Retired — kept for reproducibility",
    group: "Archive",
    disabled: true,
  },
];

// What each model exposes, for the caption beside the per-model pickers.
const exposes: Record<string, string> = {
  "atlas-pro": "Effort · Fast mode · Context window",
  atlas: "Effort · Fast mode",
  "corvid-1": "Effort (own levels, list) · Verbosity",
  "corvid-ultra": "Thinking budget (7 levels) · Extended thinking",
  "corvid-mini": "— none —",
};
const perModel = models.filter((model) => !model.disabled);

const matrix = { display: "flex", flexDirection: "column", gap: "0.25rem" } as const;
const row = { display: "flex", alignItems: "center", gap: "0.75rem", flexWrap: "wrap" } as const;
const caption = { fontSize: "0.75rem", opacity: 0.7 } as const;

export function ModelPickerSpecimen() {
  const [value, setValue] = useState<ModelSelection>({
    model: "atlas-pro",
    axes: { effort: "high", fast: false, context: "1m" },
  });
  const [outlinedValue, setOutlinedValue] = useState<ModelSelection>({
    model: "atlas",
    axes: { effort: "medium" },
  });
  const [sizeValue, setSizeValue] = useState<ModelSelection>({
    model: "atlas",
    axes: { effort: "low" },
  });
  const [densityValue, setDensityValue] = useState<ModelSelection>({
    model: "atlas",
    axes: { effort: "low" },
  });

  return (
    <SpecimenLayout
      sizes={(size) => (
        <ModelPicker
          models={models}
          axes={axes}
          size={size}
          value={sizeValue}
          onChange={setSizeValue}
        />
      )}
      densities={(density) => (
        <ModelPicker
          models={models}
          axes={axes}
          density={density}
          value={densityValue}
          onChange={setDensityValue}
        />
      )}
    >
      <SpecimenGroup label="Cross-provider list (switch model — the axes rail follows)">
        <ModelPicker models={models} axes={axes} value={value} onChange={setValue} />
        <pre style={{ margin: 0, fontSize: "0.75rem", maxHeight: "10rem", overflow: "auto" }}>
          {JSON.stringify(value, null, 2)}
        </pre>
      </SpecimenGroup>

      <SpecimenGroup label="Different axes per model (one picker per model)">
        <div style={matrix}>
          {perModel.map((model) => (
            <div key={model.value} style={row}>
              <ModelPicker models={models} axes={axes} value={{ model: model.value, axes: {} }} />
              <span style={caption}>{exposes[model.value]}</span>
            </div>
          ))}
        </div>
        <p>
          Each trigger summarises only its model's axes; open any of them to see that model's rail.
        </p>
      </SpecimenGroup>

      <SpecimenGroup label="Model marks: registry icon vs arbitrary image">
        <div style={matrix}>
          <div style={row}>
            <ModelPicker models={models} axes={axes} value={{ model: "atlas", axes: {} }} />
            <span style={caption}>icon: "sparkles" — a name from the icon registry</span>
          </div>
          <div style={row}>
            <ModelPicker models={models} axes={axes} value={{ model: "corvid-1", axes: {} }} />
            <span style={caption}>
              image: {"{ src, alt }"} — any image URL (provider logo, data URI, asset path)
            </span>
          </div>
        </div>
        <p>
          An <code>image</code> wins over <code>icon</code> when a model sets both.
        </p>
      </SpecimenGroup>

      <SpecimenGroup label="Rebound axis (same key, provider's own levels, forced to a list)">
        <ModelPicker
          models={models}
          axes={axes}
          value={{ model: "corvid-1", axes: { effort: "deep" } }}
        />
      </SpecimenGroup>

      <SpecimenGroup label="Many-level axis (7 levels → list) with a relabelled key">
        <ModelPicker
          models={models}
          axes={axes}
          value={{ model: "corvid-ultra", axes: { effort: "very-high" } }}
        />
      </SpecimenGroup>

      <SpecimenGroup label="Model with no axes at all">
        <ModelPicker models={models} axes={axes} value={{ model: "corvid-mini", axes: {} }} />
      </SpecimenGroup>

      <SpecimenGroup label="Emphasis: default vs subdued">
        <div style={matrix}>
          <div style={row}>
            <ModelPicker models={models} axes={axes} value={{ model: "atlas-pro", axes: {} }} />
            <span style={caption}>default — full-strength trigger</span>
          </div>
          <div style={row}>
            <ModelPicker
              models={models}
              axes={axes}
              value={{ model: "atlas-pro", axes: {} }}
              emphasis="subdued"
            />
            <span style={caption}>
              subdued — recedes beside a more important control; hover or focus restores it
            </span>
          </div>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Outlined trigger">
        <ModelPicker
          models={models}
          axes={axes}
          variant="outlined"
          value={outlinedValue}
          onChange={setOutlinedValue}
        />
      </SpecimenGroup>

      <SpecimenGroup label="Summary suppressed">
        <ModelPicker
          models={models}
          axes={axes}
          showAxisSummary={false}
          value={{ model: "atlas", axes: {} }}
        />
      </SpecimenGroup>

      <SpecimenGroup label="Descriptions hidden">
        <ModelPicker
          models={models}
          axes={axes}
          showModelDescriptions={false}
          value={{ model: "atlas", axes: {} }}
        />
      </SpecimenGroup>

      <SpecimenGroup label="No model selected">
        <ModelPicker models={models} axes={axes} value={{ model: "", axes: {} }} />
      </SpecimenGroup>

      <SpecimenGroup label="Models only (no axes declared)">
        <ModelPicker models={models} value={{ model: "atlas", axes: {} }} />
      </SpecimenGroup>

      <SpecimenGroup label="Disabled">
        <ModelPicker
          models={models}
          axes={axes}
          disabled
          value={{ model: "atlas", axes: { effort: "low" } }}
        />
      </SpecimenGroup>
    </SpecimenLayout>
  );
}
