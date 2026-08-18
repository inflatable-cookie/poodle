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

  let value = $state<ModelSelection>({
    model: "atlas-pro",
    axes: { effort: "high", fast: false, context: "1m" },
  });

  let outlinedValue = $state<ModelSelection>({ model: "atlas", axes: { effort: "medium" } });
  let sizeValue = $state<ModelSelection>({ model: "atlas", axes: { effort: "low" } });
  let densityValue = $state<ModelSelection>({ model: "atlas", axes: { effort: "low" } });
</script>

<SpecimenLayout>
  <SpecimenGroup label="Cross-provider default">
    <p class="poodle-model-picker-specimen__note">
      Two providers and an archive group in one list. Open it: the models carry
      their own marks, badges and descriptions, and the axes rail follows
      whichever model is selected. The serialized selection is below.
    </p>
    <ModelPicker {models} {axes} bind:value />
    <pre>{JSON.stringify(value, null, 2)}</pre>
  </SpecimenGroup>

  <SpecimenGroup label="Axis control forms">
    <p class="poodle-model-picker-specimen__note">
      An axis renders as a segmented control up to three options and as a list
      beyond that; <code>control</code> forces either. A model may expose none.
    </p>
    <div class="poodle-model-picker-specimen__matrix">
      <div class="poodle-model-picker-specimen__row">
        <ModelPicker {models} {axes} value={{ model: "corvid-1", axes: { effort: "deep" } }} />
        <span class="poodle-model-picker-specimen__exposes">
          Rebound axis — same <code>effort</code> key, the provider's own levels, forced to a list
        </span>
      </div>
      <div class="poodle-model-picker-specimen__row">
        <ModelPicker {models} {axes} value={{ model: "corvid-ultra", axes: { effort: "very-high" } }} />
        <span class="poodle-model-picker-specimen__exposes">
          Seven levels → a list on its own, with the shared key relabelled
        </span>
      </div>
      <div class="poodle-model-picker-specimen__row">
        <ModelPicker {models} {axes} value={{ model: "corvid-mini", axes: {} }} />
        <span class="poodle-model-picker-specimen__exposes">
          No axes at all — no summary, and a single-column surface
        </span>
      </div>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Variants and emphasis">
    <div class="poodle-model-picker-specimen__matrix">
      <div class="poodle-model-picker-specimen__row">
        <ModelPicker {models} {axes} variant="outlined" bind:value={outlinedValue} />
        <span class="poodle-model-picker-specimen__exposes">outlined — a bordered trigger</span>
      </div>
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

  <SpecimenGroup label="What the trigger shows">
    <div class="poodle-model-picker-specimen__matrix">
      <div class="poodle-model-picker-specimen__row">
        <ModelPicker {models} {axes} showAxisSummary={false} value={{ model: "atlas", axes: {} }} />
        <span class="poodle-model-picker-specimen__exposes">axis summary suppressed</span>
      </div>
      <div class="poodle-model-picker-specimen__row">
        <ModelPicker {models} {axes} showModelDescriptions={false} value={{ model: "atlas", axes: {} }} />
        <span class="poodle-model-picker-specimen__exposes">model descriptions hidden in the list</span>
      </div>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Nothing selected, and disabled">
    <div class="poodle-model-picker-specimen__matrix">
      <div class="poodle-model-picker-specimen__row">
        <ModelPicker {models} {axes} value={{ model: "", axes: {} }} />
        <span class="poodle-model-picker-specimen__exposes">placeholder — no model chosen yet</span>
      </div>
      <div class="poodle-model-picker-specimen__row">
        <ModelPicker {models} {axes} disabled value={{ model: "atlas", axes: { effort: "low" } }} />
        <span class="poodle-model-picker-specimen__exposes">disabled — the trigger does not open</span>
      </div>
    </div>
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

  .poodle-model-picker-specimen__note {
    margin: 0 0 0.75rem;
    font-size: 0.875rem;
    opacity: 0.75;
  }
</style>
