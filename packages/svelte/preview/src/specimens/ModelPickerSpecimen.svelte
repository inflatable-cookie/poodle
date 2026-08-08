<script lang="ts">
  import {
    ModelPicker,
    type ModelCapabilityAxis,
    type ModelOption,
    type ModelSelection,
  } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

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

  let value = $state<ModelSelection>({
    model: "atlas-pro",
    axes: { effort: "high", fast: false, context: "1m" },
  });

  let outlinedValue = $state<ModelSelection>({ model: "atlas", axes: { effort: "medium" } });
  let sizeValue = $state<ModelSelection>({ model: "atlas", axes: { effort: "low" } });
  let densityValue = $state<ModelSelection>({ model: "atlas", axes: { effort: "low" } });
</script>

<SpecimenLayout>
  <SpecimenGroup label="Cross-provider list (switch model — the axes rail follows)">
    <ModelPicker {models} {axes} bind:value />
    <pre>{JSON.stringify(value, null, 2)}</pre>
  </SpecimenGroup>

  <SpecimenGroup label="Different axes per model (one picker per model)">
    <div class="poodle-model-picker-specimen__matrix">
      {#each perModel as model (model.value)}
        <div class="poodle-model-picker-specimen__row">
          <ModelPicker {models} {axes} value={{ model: model.value, axes: {} }} />
          <span class="poodle-model-picker-specimen__exposes">{exposes[model.value]}</span>
        </div>
      {/each}
    </div>
    <p>Each trigger summarises only its model's axes; open any of them to see that model's rail.</p>
  </SpecimenGroup>

  <SpecimenGroup label="Model marks: registry icon vs arbitrary image">
    <div class="poodle-model-picker-specimen__matrix">
      <div class="poodle-model-picker-specimen__row">
        <ModelPicker {models} {axes} value={{ model: "atlas", axes: {} }} />
        <span class="poodle-model-picker-specimen__exposes">
          icon: "sparkles" — a name from the icon registry
        </span>
      </div>
      <div class="poodle-model-picker-specimen__row">
        <ModelPicker {models} {axes} value={{ model: "corvid-1", axes: {} }} />
        <span class="poodle-model-picker-specimen__exposes">
          image: &lbrace; src, alt &rbrace; — any image URL (provider logo, data URI, asset path)
        </span>
      </div>
    </div>
    <p>An <code>image</code> wins over <code>icon</code> when a model sets both.</p>
  </SpecimenGroup>

  <SpecimenGroup label="Rebound axis (same key, provider's own levels, forced to a list)">
    <ModelPicker {models} {axes} value={{ model: "corvid-1", axes: { effort: "deep" } }} />
  </SpecimenGroup>

  <SpecimenGroup label="Many-level axis (7 levels → list) with a relabelled key">
    <ModelPicker {models} {axes} value={{ model: "corvid-ultra", axes: { effort: "very-high" } }} />
  </SpecimenGroup>

  <SpecimenGroup label="Model with no axes at all">
    <ModelPicker {models} {axes} value={{ model: "corvid-mini", axes: {} }} />
  </SpecimenGroup>

  <SpecimenGroup label="Emphasis: default vs subdued">
    <div class="poodle-model-picker-specimen__matrix">
      <div class="poodle-model-picker-specimen__row">
        <ModelPicker {models} {axes} value={{ model: "atlas-pro", axes: {} }} />
        <span class="poodle-model-picker-specimen__exposes">default — full-strength trigger</span>
      </div>
      <div class="poodle-model-picker-specimen__row">
        <ModelPicker {models} {axes} value={{ model: "atlas-pro", axes: {} }} emphasis="subdued" />
        <span class="poodle-model-picker-specimen__exposes">
          subdued — recedes beside a more important control; hover or focus restores it
        </span>
      </div>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Outlined trigger">
    <ModelPicker {models} {axes} variant="outlined" bind:value={outlinedValue} />
  </SpecimenGroup>

  <SpecimenGroup label="Summary suppressed">
    <ModelPicker {models} {axes} showAxisSummary={false} value={{ model: "atlas", axes: {} }} />
  </SpecimenGroup>

  <SpecimenGroup label="Descriptions hidden">
    <ModelPicker {models} {axes} showModelDescriptions={false} value={{ model: "atlas", axes: {} }} />
  </SpecimenGroup>

  <SpecimenGroup label="No model selected">
    <ModelPicker {models} {axes} value={{ model: "", axes: {} }} />
  </SpecimenGroup>

  <SpecimenGroup label="Models only (no axes declared)">
    <ModelPicker {models} value={{ model: "atlas", axes: {} }} />
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <ModelPicker {models} {axes} disabled value={{ model: "atlas", axes: { effort: "low" } }} />
  </SpecimenGroup>

  {#snippet sizes(size)}
    <ModelPicker {models} {axes} {size} bind:value={sizeValue} />
  {/snippet}

  {#snippet densities(density)}
    <ModelPicker {models} {axes} {density} bind:value={densityValue} />
  {/snippet}
</SpecimenLayout>

<style>
  pre {
    margin: 0;
    font-size: 0.75rem;
    max-height: 10rem;
    overflow: auto;
  }

  .poodle-model-picker-specimen__matrix {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .poodle-model-picker-specimen__row {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    flex-wrap: wrap;
  }

  .poodle-model-picker-specimen__exposes {
    font-size: 0.75rem;
    opacity: 0.7;
  }
</style>
