<script lang="ts">
  import { Icon, IconProvider, Eyebrow, defaultIconRegistry } from "@pug/svelte-primitives";

  const iconNames = Object.keys(defaultIconRegistry).sort();

  let copiedName = "";

  function copyName(name: string) {
    navigator.clipboard.writeText(name);
    copiedName = name;
    setTimeout(() => {
      if (copiedName === name) copiedName = "";
    }, 1200);
  }

  const overrideRegistry = {
    ...defaultIconRegistry,
    check: [["path", { d: "M20 6 9 17l-5-5" }]],
    x: [["path", { d: "M18 6 6 18" }], ["path", { d: "m6 6 12 12" }]],
    star: [
      ["polygon", { points: "12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" }],
    ],
  };
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Sizes</Eyebrow>
    <div class="size-row">
      <div class="size-demo">
        <span class="size-label">sm</span>
        <Icon name="star" size="sm" />
        <Icon name="heart" size="sm" />
        <Icon name="settings" size="sm" />
      </div>
      <div class="size-demo">
        <span class="size-label">md</span>
        <Icon name="star" size="md" />
        <Icon name="heart" size="md" />
        <Icon name="settings" size="md" />
      </div>
      <div class="size-demo">
        <span class="size-label">lg</span>
        <Icon name="star" size="lg" />
        <Icon name="heart" size="lg" />
        <Icon name="settings" size="lg" />
      </div>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Color inheritance</Eyebrow>
    <div class="color-row">
      <span class="color-demo" style="color: var(--pug-color-icon-primary)">
        <Icon name="check-circle" /> Primary
      </span>
      <span class="color-demo" style="color: var(--pug-color-icon-muted)">
        <Icon name="info" /> Muted
      </span>
      <span class="color-demo" style="color: var(--pug-color-accent-base)">
        <Icon name="zap" /> Accent
      </span>
      <span class="color-demo" style="color: var(--pug-color-status-danger)">
        <Icon name="triangle-alert" /> Danger
      </span>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>IconProvider override</Eyebrow>
    <div class="override-row">
      <div class="override-demo">
        <span class="override-label">Default</span>
        <Icon name="check" />
        <Icon name="x" />
        <Icon name="star" />
      </div>
      <IconProvider registry={overrideRegistry}>
        <div class="override-demo">
          <span class="override-label">Overridden</span>
          <Icon name="check" />
          <Icon name="x" />
          <Icon name="star" />
        </div>
      </IconProvider>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>All icons ({iconNames.length})</Eyebrow>
    <div class="icon-grid">
      {#each iconNames as name}
        <button
          class="icon-cell"
          class:copied={copiedName === name}
          on:click={() => copyName(name)}
          title={name}
        >
          <Icon {name} size="md" />
          <span class="icon-name">{name}</span>
        </button>
      {/each}
    </div>
  </div>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .size-row {
    display: flex;
    gap: 1.5rem;
    align-items: center;
  }

  .size-demo {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .size-label {
    font-size: 0.6875rem;
    font-family: var(--pug-typography-code-family);
    color: var(--pug-color-text-muted);
    min-width: 1.5rem;
  }

  .color-row {
    display: flex;
    gap: 1.25rem;
    align-items: center;
  }

  .color-demo {
    display: inline-flex;
    align-items: center;
    gap: 0.375rem;
    font-size: 0.75rem;
  }

  .override-row {
    display: flex;
    gap: 1.5rem;
    align-items: center;
  }

  .override-demo {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .override-label {
    font-size: 0.6875rem;
    font-family: var(--pug-typography-code-family);
    color: var(--pug-color-text-muted);
    min-width: 4rem;
  }

  .icon-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(6.5rem, 1fr));
    gap: 0.125rem;
  }

  .icon-cell {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.25rem;
    padding: 0.625rem 0.25rem;
    border: none;
    border-radius: var(--pug-radius-control);
    background: transparent;
    color: var(--pug-color-text-primary);
    cursor: pointer;
    transition: background 0.15s;
  }

  .icon-cell:hover {
    background: color-mix(in srgb, var(--pug-color-background-surface) 64%, transparent);
  }

  .icon-cell.copied {
    background: color-mix(in srgb, var(--pug-color-accent-base) 18%, transparent);
  }

  .icon-name {
    font-size: 0.5625rem;
    font-family: var(--pug-typography-code-family);
    color: var(--pug-color-text-muted);
    text-align: center;
    word-break: break-all;
    line-height: 1.3;
  }
</style>
