<script lang="ts">
  import { Icon, IconProvider } from "@inflatable-cookie/poodle-svelte";
  import type { IconSet } from "@inflatable-cookie/poodle-svelte";
  import iconNodes from "lucide-static/icon-nodes.json";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  const catalogue = iconNodes as unknown as IconSet;
  const heart = catalogue.heart;
  const settings = catalogue.settings;
  const zap = catalogue.zap;
  const circleCheck = catalogue["circle-check"];
  const info = catalogue.info;
  const triangleAlert = catalogue["triangle-alert"];
  const star = catalogue.star;
  const search = catalogue.search;
  const pencil = catalogue.pencil;
  const allIconNames = Object.keys(catalogue).sort();
  const iconSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let copiedName = $state("");

  function copyName(name: string) {
    navigator.clipboard.writeText(name);
    copiedName = name;
    setTimeout(() => {
      if (copiedName === name) copiedName = "";
    }, 1200);
  }

  // The scoped Lucide default required by Poodle's own component chrome.
  const defaultLucideNames = [
    "arrow-down", "arrow-left", "arrow-right", "arrow-up", "arrow-up-down",
    "bold", "check", "check-check", "chevron-down", "chevron-left",
    "chevron-right", "chevron-up", "circle", "circle-x", "code",
    "columns-2", "columns-3", "diff", "dot", "download", "ellipsis",
    "external-link", "eye", "file", "file-pen", "file-text", "git-branch",
    "git-commit-horizontal", "grip-vertical", "heading", "image", "inbox",
    "info", "italic", "link", "list", "menu", "minus", "music", "pencil",
    "play", "plus", "quote", "refresh-cw", "search", "square", "star",
    "tag", "terminal", "trash-2", "trending-down", "trending-up",
    "triangle-alert", "x",
  ];
</script>

<SpecimenLayout showDensities={false}>
  <SpecimenGroup label="Direct import — tree-shakeable">
    <p class="poodle-hint">
      Generate an application set with <code>poodle-icons</code>. The generated
      module contains only the icons in its names file.
    </p>
    <div class="poodle-size-row">
      {#each iconSizes as size}
        <div class="poodle-size-demo">
          <span class="poodle-size-label">{size}</span>
          <Icon icon={star} {size} />
          <Icon icon={heart} {size} />
          <Icon icon={settings} {size} />
        </div>
      {/each}
    </div>
    <div class="poodle-code-hint">
      <code>poodle-icons --names icons.json --out icons.generated.ts</code>
      <br />
      <code>&lt;Icon icon={"{star}"} size="lg" /&gt;</code>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Color inheritance">
    <p class="poodle-hint">
      Icons inherit <code>currentColor</code> from their parent element.
    </p>
    <div class="poodle-color-row">
      <span class="poodle-color-demo" style="color: var(--poodle-color-icon-primary)">
        <Icon icon={circleCheck} /> Primary
      </span>
      <span class="poodle-color-demo" style="color: var(--poodle-color-icon-muted)">
        <Icon icon={info} /> Muted
      </span>
      <span class="poodle-color-demo" style="color: var(--poodle-color-accent-base)">
        <Icon icon={zap} /> Accent
      </span>
      <span class="poodle-color-demo" style="color: var(--poodle-color-status-danger)">
        <Icon icon={triangleAlert} /> Danger
      </span>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Accessibility">
    <p class="poodle-hint">
      Set <code>ariaLabel</code> for meaningful icons. Decorative icons
      are automatically <code>aria-hidden</code>.
    </p>
    <div class="poodle-color-row">
      <span class="poodle-color-demo">
        <Icon icon={search} ariaLabel="Search" /> with ariaLabel (role="img")
      </span>
      <span class="poodle-color-demo">
        <Icon icon={pencil} /> without ariaLabel (aria-hidden)
      </span>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Default Lucide icons ({defaultLucideNames.length})">
    <p class="poodle-hint">
      Poodle ships this scoped Lucide default so its own component chrome works
      without an <code>IconProvider</code>. Application icons remain operator-owned.
    </p>
    <div class="poodle-icon-grid poodle-icon-grid--compact">
      {#each defaultLucideNames as name}
        <button
          class="poodle-icon-cell"
          class:poodle-copied={copiedName === name}
          onclick={() => copyName(name)}
          title={name}
        >
          <Icon icon={name} />
          <span class="poodle-icon-name">{name}</span>
        </button>
      {/each}
    </div>
    <div class="poodle-code-hint">
      <code>&lt;Icon icon="chevron-down" sizeRole="chrome" /&gt;</code>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="All icons via IconProvider ({allIconNames.length})">
    <p class="poodle-hint">
      Wrap your app (or a subtree) in <code>&lt;IconProvider&gt;</code> with
      a full icon set to enable string-based lookups for any icon.
      Click any icon to copy its name.
    </p>
    <div class="poodle-code-hint">
      <code>import iconNodes from "lucide-static/icon-nodes.json";</code>
      <br />
      <code>&lt;IconProvider icons={"{iconNodes}"}&gt; ... &lt;/IconProvider&gt;</code>
    </div>
    <IconProvider icons={iconNodes as unknown as IconSet}>
      <div class="poodle-icon-grid">
        {#each allIconNames as name}
          <button
            class="poodle-icon-cell"
            class:poodle-copied={copiedName === name}
            onclick={() => copyName(name)}
            title={name}
          >
            <Icon icon={name} />
            <span class="poodle-icon-name">{name}</span>
          </button>
        {/each}
      </div>
    </IconProvider>
  </SpecimenGroup>

  {#snippet sizes(size)}
    <div class="poodle-size-demo">
      <Icon icon={star} {size} />
      <Icon icon={heart} {size} />
      <Icon icon={settings} {size} />
    </div>
  {/snippet}

</SpecimenLayout>

<style>
  .poodle-hint {
    font-size: 0.75rem;
    color: var(--poodle-color-text-secondary);
    line-height: 1.5;
    margin: 0;
  }

  .poodle-hint code {
    font-family: var(--poodle-typography-code-family);
    font-size: 0.6875rem;
    padding: 0.0625rem 0.25rem;
    border-radius: 0.1875rem;
    background: color-mix(in srgb, var(--poodle-color-background-surface) 64%, transparent);
  }

  .poodle-code-hint {
    font-family: var(--poodle-typography-code-family);
    font-size: 0.6875rem;
    color: var(--poodle-color-text-muted);
    line-height: 1.6;
    padding: 0.5rem 0.75rem;
    border-radius: var(--poodle-radius-control);
    background: color-mix(in srgb, var(--poodle-color-background-surface) 48%, transparent);
  }

  .poodle-code-hint code {
    font-family: inherit;
  }

  .poodle-size-row {
    display: flex;
    gap: 1.5rem;
    align-items: center;
  }

  .poodle-size-demo {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .poodle-size-label {
    font-size: 0.6875rem;
    font-family: var(--poodle-typography-code-family);
    color: var(--poodle-color-text-muted);
    min-width: 1.5rem;
  }

  .poodle-color-row {
    display: flex;
    gap: 1.25rem;
    align-items: center;
    flex-wrap: wrap;
  }

  .poodle-color-demo {
    display: inline-flex;
    align-items: center;
    gap: 0.375rem;
    font-size: 0.75rem;
  }

  .poodle-icon-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(6.5rem, 1fr));
    gap: 0.125rem;
  }

  .poodle-icon-grid--compact {
    grid-template-columns: repeat(auto-fill, minmax(5.5rem, 1fr));
  }

  .poodle-icon-cell {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.25rem;
    padding: 0.625rem 0.25rem;
    border: none;
    border-radius: var(--poodle-radius-control);
    background: transparent;
    color: var(--poodle-color-text-primary);
    cursor: pointer;
    transition: background 0.15s;
  }

  .poodle-icon-cell:hover {
    background: color-mix(in srgb, var(--poodle-color-background-surface) 64%, transparent);
  }

  .poodle-icon-cell.poodle-copied {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 18%, transparent);
  }

  .poodle-icon-name {
    font-size: 0.5625rem;
    font-family: var(--poodle-typography-code-family);
    color: var(--poodle-color-text-muted);
    text-align: center;
    word-break: break-all;
    line-height: 1.3;
  }
</style>
