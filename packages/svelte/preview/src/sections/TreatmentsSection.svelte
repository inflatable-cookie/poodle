<script lang="ts">
  import { Eyebrow, Separator, Surface, Button, Pill } from "@pug/svelte-primitives";

  const treatmentRoles = [
    {
      name: "interactive",
      description: "General interactive surfaces — secondary buttons, toggles, menu triggers.",
      variables: [
        { name: "--pug-treatment-interactive-radius", purpose: "Border radius" },
        { name: "--pug-treatment-interactive-fill", purpose: "Resting background" },
        { name: "--pug-treatment-interactive-fill-active", purpose: "Hover/active background" },
        { name: "--pug-treatment-interactive-border", purpose: "Resting border color" },
        { name: "--pug-treatment-interactive-border-active", purpose: "Hover/active border color" },
        { name: "--pug-treatment-interactive-shadow", purpose: "Resting shadow" },
        { name: "--pug-treatment-interactive-shadow-active", purpose: "Hover/active shadow" },
      ],
      components: ["Button (secondary)", "IconButton (secondary)", "SplitButton", "Toggle", "ToggleGroup", "SegmentedControl (track)", "Tabs (card items)"],
    },
    {
      name: "interactive-primary",
      description: "Primary action buttons and prominent call-to-action surfaces.",
      variables: [
        { name: "--pug-treatment-interactive-primary-radius", purpose: "Border radius" },
        { name: "--pug-treatment-interactive-primary-fill", purpose: "Resting background" },
        { name: "--pug-treatment-interactive-primary-fill-hover", purpose: "Hover background" },
        { name: "--pug-treatment-interactive-primary-border", purpose: "Resting border color" },
        { name: "--pug-treatment-interactive-primary-shadow", purpose: "Shadow" },
        { name: "--pug-treatment-interactive-primary-text", purpose: "Text/icon color" },
      ],
      components: ["Button (primary)", "SplitButton (primary)", "IconButton (primary)"],
    },
    {
      name: "interactive-subtle",
      description: "Text inputs, selects, search fields — controls with subtle chrome.",
      variables: [
        { name: "--pug-treatment-interactive-subtle-radius", purpose: "Border radius" },
        { name: "--pug-treatment-interactive-subtle-fill", purpose: "Resting background" },
        { name: "--pug-treatment-interactive-subtle-fill-hover", purpose: "Hover background" },
        { name: "--pug-treatment-interactive-subtle-fill-focus", purpose: "Focus background" },
        { name: "--pug-treatment-interactive-subtle-border", purpose: "Resting border" },
        { name: "--pug-treatment-interactive-subtle-border-hover", purpose: "Hover border" },
        { name: "--pug-treatment-interactive-subtle-border-focus", purpose: "Focus border" },
        { name: "--pug-treatment-interactive-subtle-shadow", purpose: "Resting shadow" },
        { name: "--pug-treatment-interactive-subtle-shadow-hover", purpose: "Hover shadow" },
        { name: "--pug-treatment-interactive-subtle-shadow-focus", purpose: "Focus shadow" },
      ],
      components: ["TextInput", "TextArea", "SearchField", "Select", "Combobox (input)"],
    },
    {
      name: "surface",
      description: "Panel backgrounds, card frames, and container surfaces.",
      variables: [
        { name: "--pug-treatment-surface-radius", purpose: "Border radius" },
        { name: "--pug-treatment-surface-fill", purpose: "Background" },
        { name: "--pug-treatment-surface-border", purpose: "Border color" },
        { name: "--pug-treatment-surface-shadow", purpose: "Shadow" },
        { name: "--pug-treatment-surface-hover-fill", purpose: "Hover background" },
        { name: "--pug-treatment-surface-hover-border", purpose: "Hover border" },
        { name: "--pug-treatment-surface-hover-shadow", purpose: "Hover shadow" },
        { name: "--pug-treatment-surface-header-fill", purpose: "Card/section header fill" },
        { name: "--pug-treatment-surface-divider", purpose: "Internal divider color" },
      ],
      components: ["Surface", "Card (default/outlined)", "MetricTile"],
    },
    {
      name: "surface-elevated",
      description: "Elevated surfaces — dialogs, drawers, popovers, elevated cards.",
      variables: [
        { name: "--pug-treatment-surface-elevated-radius", purpose: "Border radius" },
        { name: "--pug-treatment-surface-elevated-fill", purpose: "Background" },
        { name: "--pug-treatment-surface-elevated-border", purpose: "Border color" },
        { name: "--pug-treatment-surface-elevated-shadow", purpose: "Shadow" },
      ],
      components: ["Surface (elevated)", "Card (elevated)", "Dialog", "Drawer", "Popover", "Menu (overlay)", "HoverCard", "Tooltip", "Combobox (list)", "SplitButton (menu)"],
    },
    {
      name: "focus-ring",
      description: "Focus state treatment for keyboard navigation indicators. Currently uses accent token directly; reserved for future divergence.",
      variables: [],
      components: [],
    },
  ];
</script>

<div class="treatments-section">
  <article class="treatments-page">
    <header class="treatments-page__hero">
      <h1 class="treatments-page__title">Treatment System</h1>
      <p class="treatments-page__description">
        Treatments sit between canonical semantic tokens and app-owned wrappers.
        They let downstream consumers apply cohesive visual branding across component
        families without redefining token meaning.
      </p>
    </header>

    <Separator />

    <section class="treatments-page__section">
      <h2 class="treatments-page__heading">Three-Layer Architecture</h2>
      <div class="layer-stack">
        <div class="layer-card">
          <Eyebrow>Layer 1</Eyebrow>
          <strong>Canonical Semantic Tokens</strong>
          <p>Typed, narrow values: color, spacing, radius. Never broadened to hold gradients or web-only effects.</p>
        </div>
        <div class="layer-card layer-card--active">
          <Eyebrow>Layer 2</Eyebrow>
          <strong>Appearance Recipes &amp; Treatment Roles</strong>
          <p>Grouped visual overrides scoped to component families. May include gradients, layered shadows, and other web-only effects.</p>
        </div>
        <div class="layer-card">
          <Eyebrow>Layer 3</Eyebrow>
          <strong>App-Owned Wrappers &amp; Composites</strong>
          <p>Structural brand expression built by composing Pug primitives.</p>
        </div>
      </div>
    </section>

    <Separator />

    <section class="treatments-page__section">
      <h2 class="treatments-page__heading">How Components Consume Treatments</h2>
      <p class="treatments-page__body">
        Components reference treatment variables using CSS custom property fallbacks.
        The treatment variable is tried first; if undefined, the semantic token value is used.
      </p>
      <pre class="treatments-page__code"><code>{`.text-input {
  background: var(
    --pug-treatment-interactive-subtle-fill,
    var(--pug-color-background-surface)
  );
}`}</code></pre>
      <p class="treatments-page__body">
        This means components render with standard token values by default, and
        treatment values take precedence only when explicitly set. Components never
        need to know which specific treatment is active.
      </p>
    </section>

    <Separator />

    <section class="treatments-page__section">
      <h2 class="treatments-page__heading">Treatment Roles</h2>
      <p class="treatments-page__body">
        Six family-level roles are defined. Components map these into local CSS aliases
        rather than inventing per-component vocabularies.
      </p>

      {#each treatmentRoles as role}
        <div class="role-card">
          <div class="role-card__header">
            <h3 class="role-card__name">{role.name}</h3>
            {#if role.components.length > 0}
              <div class="role-card__pills">
                {#each role.components as comp}
                  <Pill size="xxs" tone="neutral" appearance="subtle">{comp}</Pill>
                {/each}
              </div>
            {/if}
          </div>
          <p class="role-card__description">{role.description}</p>
          {#if role.variables.length > 0}
            <div class="role-card__table-wrap">
              <table class="role-card__table">
                <thead>
                  <tr>
                    <th>Variable</th>
                    <th>Purpose</th>
                  </tr>
                </thead>
                <tbody>
                  {#each role.variables as v}
                    <tr>
                      <td class="role-card__var"><code>{v.name}</code></td>
                      <td>{v.purpose}</td>
                    </tr>
                  {/each}
                </tbody>
              </table>
            </div>
          {/if}
        </div>
      {/each}
    </section>

    <Separator />

    <section class="treatments-page__section">
      <h2 class="treatments-page__heading">Applying a Treatment</h2>
      <p class="treatments-page__body">
        Set a <code>data-appearance-treatment</code> attribute on a container element.
        All descendants inherit treatment values through the CSS cascade.
      </p>
      <pre class="treatments-page__code"><code>{`<div data-appearance-treatment="brand-raised">
  <!-- All Pug components inside inherit treatment values -->
</div>`}</code></pre>
      <p class="treatments-page__body">
        Then define CSS that overrides treatment variables when that attribute is present:
      </p>
      <pre class="treatments-page__code"><code>{`[data-appearance-treatment="brand-raised"] {
  --pug-treatment-interactive-fill:
    linear-gradient(180deg, rgba(255,255,255,0.14), transparent),
    var(--pug-color-background-elevated);
  --pug-treatment-interactive-primary-fill:
    linear-gradient(180deg, rgba(255,255,255,0.24), transparent),
    var(--pug-color-accent-base);
  --pug-treatment-surface-fill:
    linear-gradient(180deg, rgba(255,255,255,0.14), transparent),
    var(--pug-color-background-panel);
  /* ... all other treatment variables */
}`}</code></pre>
    </section>

    <Separator />

    <section class="treatments-page__section">
      <h2 class="treatments-page__heading">Creating a New Treatment</h2>
      <div class="step-list">
        <div class="step">
          <div class="step__number">1</div>
          <div class="step__content">
            <strong>Define treatment variables</strong>
            <p>Create a CSS rule block that sets treatment variables for every role you want to affect. Unset variables fall through to semantic token defaults.</p>
          </div>
        </div>
        <div class="step">
          <div class="step__number">2</div>
          <div class="step__content">
            <strong>Add theme-specific adjustments</strong>
            <p>Treatments may need per-theme overrides, especially for shadows which look different on light versus dark backgrounds.</p>
          </div>
        </div>
        <div class="step">
          <div class="step__number">3</div>
          <div class="step__content">
            <strong>Register in the preview app</strong>
            <p>Add the treatment name to <code>DisplayControls.svelte</code> and <code>App.svelte</code> to make it selectable during development.</p>
          </div>
        </div>
      </div>
    </section>

    <Separator />

    <section class="treatments-page__section">
      <h2 class="treatments-page__heading">Rules</h2>
      <div class="rule-list">
        <div class="rule">
          <strong>Token purity</strong>
          <p>Semantic tokens must remain typed and narrow. Do not broaden a color token to hold a gradient.</p>
        </div>
        <div class="rule">
          <strong>Family-level roles</strong>
          <p>Prefer shared treatment roles over per-component treatment variables.</p>
        </div>
        <div class="rule">
          <strong>Fallback chain</strong>
          <p>Every treatment variable reference must include a semantic token fallback.</p>
        </div>
        <div class="rule">
          <strong>Gradient rule</strong>
          <p>Gradients are valid appearance treatments, not canonical colors.</p>
        </div>
        <div class="rule">
          <strong>Safe override boundary</strong>
          <p>Downstream apps may scope recipe overrides to subtrees. They must not redefine semantic token meaning.</p>
        </div>
      </div>
    </section>
  </article>
</div>

<style>
  .treatments-section {
    flex: 1;
    overflow-y: auto;
  }

  .treatments-page {
    padding: 1.5rem 2rem;
    max-width: 52rem;
  }

  .treatments-page__hero {
    margin-bottom: 1.5rem;
  }

  .treatments-page__title {
    font-size: 2rem;
    font-weight: 700;
    color: var(--pug-color-text-primary);
    margin: 0 0 0.75rem;
  }

  .treatments-page__description {
    font-size: 1rem;
    line-height: 1.6;
    color: var(--pug-color-text-secondary);
    margin: 0;
  }

  .treatments-page__section {
    padding: 1.5rem 0;
  }

  .treatments-page__heading {
    font-size: 1.125rem;
    font-weight: 600;
    color: var(--pug-color-text-primary);
    margin: 0 0 1rem;
  }

  .treatments-page__body {
    font-size: 0.875rem;
    line-height: 1.6;
    color: var(--pug-color-text-secondary);
    margin: 0 0 1rem;
  }

  .treatments-page__body code {
    font-family: "SF Mono", "Fira Code", monospace;
    font-size: 0.8125rem;
    padding: 0.0625rem 0.25rem;
    border-radius: 0.1875rem;
    background: color-mix(in srgb, var(--pug-color-background-canvas) 80%, transparent);
  }

  .treatments-page__code {
    padding: 0.75rem 1rem;
    border-radius: var(--pug-radius-surface);
    background: color-mix(in srgb, var(--pug-color-background-canvas) 90%, transparent);
    border: 0.0625rem solid color-mix(in srgb, var(--pug-color-border-subtle) 50%, transparent);
    font-family: "SF Mono", "Fira Code", monospace;
    font-size: 0.8125rem;
    line-height: 1.6;
    color: var(--pug-color-text-primary);
    overflow-x: auto;
    margin: 0 0 1rem;
    white-space: pre;
  }

  .layer-stack {
    display: grid;
    gap: 0.75rem;
  }

  .layer-card {
    display: grid;
    gap: 0.25rem;
    padding: 1rem;
    border: 0.0625rem solid color-mix(in srgb, var(--pug-color-border-subtle) 40%, transparent);
    border-radius: var(--pug-radius-surface);
    background: color-mix(in srgb, var(--pug-color-background-surface) 60%, transparent);
  }

  .layer-card--active {
    border-color: color-mix(in srgb, var(--pug-color-accent-base) 32%, var(--pug-color-border-default));
    background: color-mix(in srgb, var(--pug-color-accent-base) 6%, var(--pug-color-background-surface));
  }

  .layer-card strong {
    font-size: 0.9375rem;
  }

  .layer-card p {
    margin: 0;
    font-size: 0.8125rem;
    color: var(--pug-color-text-secondary);
    line-height: 1.5;
  }

  .role-card {
    display: grid;
    gap: 0.5rem;
    padding: 1rem;
    margin-bottom: 0.75rem;
    border: 0.0625rem solid color-mix(in srgb, var(--pug-color-border-subtle) 40%, transparent);
    border-radius: var(--pug-radius-surface);
  }

  .role-card__header {
    display: flex;
    flex-wrap: wrap;
    align-items: baseline;
    gap: 0.75rem;
  }

  .role-card__name {
    font-family: "SF Mono", "Fira Code", monospace;
    font-size: 0.9375rem;
    font-weight: 600;
    margin: 0;
    color: var(--pug-color-text-primary);
  }

  .role-card__pills {
    display: flex;
    flex-wrap: wrap;
    gap: 0.25rem;
  }

  .role-card__description {
    margin: 0;
    font-size: 0.8125rem;
    color: var(--pug-color-text-secondary);
    line-height: 1.5;
  }

  .role-card__table-wrap {
    overflow-x: auto;
  }

  .role-card__table {
    width: 100%;
    border-collapse: collapse;
    font-size: 0.8125rem;
    line-height: 1.5;
  }

  .role-card__table th {
    text-align: left;
    font-weight: 600;
    color: var(--pug-color-text-secondary);
    padding: 0.375rem 0.75rem;
    border-bottom: 0.0625rem solid var(--pug-color-border-subtle);
    font-size: 0.6875rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .role-card__table td {
    padding: 0.375rem 0.75rem;
    border-bottom: 0.0625rem solid color-mix(in srgb, var(--pug-color-border-subtle) 40%, transparent);
    color: var(--pug-color-text-secondary);
  }

  .role-card__table tbody tr:last-child td {
    border-bottom: none;
  }

  .role-card__var code {
    font-family: "SF Mono", "Fira Code", monospace;
    font-size: 0.75rem;
    color: var(--pug-color-text-primary);
    white-space: nowrap;
  }

  .step-list {
    display: grid;
    gap: 1rem;
  }

  .step {
    display: grid;
    grid-template-columns: 2rem 1fr;
    gap: 0.75rem;
    align-items: start;
  }

  .step__number {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 2rem;
    height: 2rem;
    border-radius: 999px;
    background: color-mix(in srgb, var(--pug-color-accent-base) 16%, transparent);
    color: var(--pug-color-text-primary);
    font-size: 0.8125rem;
    font-weight: 700;
  }

  .step__content {
    display: grid;
    gap: 0.25rem;
  }

  .step__content strong {
    font-size: 0.9375rem;
  }

  .step__content p {
    margin: 0;
    font-size: 0.8125rem;
    color: var(--pug-color-text-secondary);
    line-height: 1.5;
  }

  .step__content code {
    font-family: "SF Mono", "Fira Code", monospace;
    font-size: 0.75rem;
    padding: 0.0625rem 0.25rem;
    border-radius: 0.1875rem;
    background: color-mix(in srgb, var(--pug-color-background-canvas) 80%, transparent);
  }

  .rule-list {
    display: grid;
    gap: 0.75rem;
  }

  .rule {
    display: grid;
    gap: 0.25rem;
    padding: 0.875rem;
    border-radius: var(--pug-radius-surface);
    background: color-mix(in srgb, var(--pug-color-background-surface) 60%, transparent);
  }

  .rule strong {
    font-size: 0.875rem;
  }

  .rule p {
    margin: 0;
    font-size: 0.8125rem;
    color: var(--pug-color-text-secondary);
    line-height: 1.5;
  }
</style>
