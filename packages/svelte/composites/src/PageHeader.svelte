<script lang="ts">
  import type { Snippet } from "svelte";

  import { Callout, Icon } from "@poodle/svelte-primitives";

  interface Props {
    title?: string | null;
    section?: string | null;
    count?: number | null;
    subtitle?: string | null;
    eyebrow?: string | null;
    backHref?: string | null;
    backLabel?: string | null;
    backIsContextual?: boolean;
    bannerMessage?: string | null;
    bannerTone?: "neutral" | "info" | "success" | "warning" | "danger";
    align?: "start" | "between";
    ariaLabel?: string | null;
    level?: 1 | 2 | 3 | 4 | 5 | 6;
    children?: Snippet;
    breadcrumbs?: Snippet;
    meta?: Snippet;
    actions?: Snippet;
    banner?: Snippet;
  }

  let {
    title = null,
    section = null,
    count = null,
    subtitle = null,
    eyebrow = null,
    backHref = null,
    backLabel = null,
    backIsContextual = false,
    bannerMessage = null,
    bannerTone = "warning",
    align = "between",
    ariaLabel = null,
    level = 2,
    children,
    breadcrumbs,
    meta,
    actions,
    banner,
  }: Props = $props();

  const primaryTitle = $derived(title ?? section ?? "");
  const hasSectionTitleSplit = $derived(Boolean(section && title));
  const headingTag = $derived(`h${level}` as `h${1 | 2 | 3 | 4 | 5 | 6}`);
</script>

<header class="page-header" data-align={align} aria-label={ariaLabel ?? undefined}>
  <div class="page-header__content">
    {#if breadcrumbs}
      <div class="page-header__breadcrumbs">
        {@render breadcrumbs()}
      </div>
    {/if}

    <div class="page-header__title-block">
      {#if eyebrow}
        <p class="page-header__eyebrow">{eyebrow}</p>
      {/if}
      {#if hasSectionTitleSplit}
        <p class="page-header__section">{section}</p>
      {/if}
      <svelte:element this={headingTag} class="page-header__title">
        <span>{primaryTitle}</span>
        {#if count !== null}
          <span class="page-header__count" aria-label={`${count}`}>{count}</span>
        {/if}
      </svelte:element>
      {#if hasSectionTitleSplit}
        <p class="page-header__section-title">{title}</p>
      {/if}
      {#if subtitle}
        <p class="page-header__subtitle">{subtitle}</p>
      {/if}
      {#if meta}
        <div class="page-header__meta">
          {@render meta()}
        </div>
      {/if}
      {#if children}
        <div class="page-header__body">
          {@render children()}
        </div>
      {/if}
    </div>
  </div>

  {#if backHref || actions}
    <div class="page-header__actions-row">
      {#if backHref}
        <a class="page-header__back" href={backHref} aria-label={backLabel ?? "Go back"}>
          <Icon name="arrow-left" />
          {#if backIsContextual}
            <span class="page-header__context-dot" aria-hidden="true"></span>
          {/if}
        </a>
      {/if}
      {#if actions}
        <div class="page-header__actions">
          {@render actions()}
        </div>
      {/if}
    </div>
  {/if}

  {#if banner}
    <div class="page-header__banner">
      {@render banner()}
    </div>
  {:else if bannerMessage}
    <div class="page-header__banner">
      <Callout tone={bannerTone} message={bannerMessage} announceMode="polite" />
    </div>
  {/if}
</header>

<style>
  .page-header {
    --poodle-recipe-page-header-padding-block-start: 0;
    --poodle-recipe-page-header-padding-inline: 0;
    --poodle-recipe-page-header-padding-block-end: calc(var(--poodle-space-stack-md) + 0.125rem);
    --poodle-recipe-page-header-fill: transparent;
    --poodle-recipe-page-header-border: transparent;
    --poodle-recipe-page-header-shadow: none;
    --poodle-recipe-page-header-radius: var(--poodle-radius-surface);
    display: grid;
    gap: var(--poodle-space-stack-md);
    align-items: end;
    padding:
      var(--poodle-recipe-page-header-padding-block-start)
      var(--poodle-recipe-page-header-padding-inline)
      var(--poodle-recipe-page-header-padding-block-end);
    border: 0.0625rem solid var(--poodle-recipe-page-header-border);
    border-radius: var(--poodle-recipe-page-header-radius);
    background: var(--poodle-recipe-page-header-fill);
    box-shadow: var(--poodle-recipe-page-header-shadow);
  }

  .page-header[data-align="between"] {
    grid-template-columns: minmax(0, 1fr) auto;
  }

  .page-header__content {
    display: grid;
    gap: var(--poodle-space-stack-md);
  }

  .page-header__back {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 0.25rem;
    width: 1.75rem;
    height: 1.75rem;
    border-radius: var(--poodle-radius-control);
    color: var(--poodle-color-text-secondary);
    text-decoration: none;
    transition: background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .page-header__back:hover {
    background: color-mix(in srgb, var(--poodle-color-text-secondary) 12%, transparent);
    color: var(--poodle-color-text-primary);
  }

  .page-header__back:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.0625rem;
  }

  .page-header__context-dot {
    width: 0.375rem;
    height: 0.375rem;
    border-radius: 999px;
    background: var(--poodle-color-fill-info-strong, var(--poodle-color-border-info));
    flex: none;
  }

  .page-header__title-block {
    display: grid;
    gap: var(--poodle-space-inline-sm);
  }

  .page-header__title,
  .page-header__subtitle,
  .page-header__body,
  .page-header__eyebrow,
  .page-header__section,
  .page-header__section-title {
    margin: 0;
  }

  .page-header__title {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    font-family: var(--poodle-typography-heading-family);
    font-size: 1.75rem;
    line-height: 1.1;
    font-weight: 700;
  }

  .page-header__section {
    color: var(--poodle-color-text-secondary);
    font-size: 0.75rem;
    font-weight: 700;
    letter-spacing: 0.08em;
    text-transform: uppercase;
  }

  .page-header__section-title {
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-heading-family);
    font-size: 1rem;
    line-height: 1.25;
    font-weight: 600;
  }

  .page-header__count {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 1.75rem;
    min-height: 1.75rem;
    padding: 0 0.5rem;
    border-radius: 999px;
    background: color-mix(in srgb, var(--poodle-color-fill-secondary) 72%, transparent);
    color: var(--poodle-color-text-secondary);
    font-size: 0.875rem;
    font-weight: 600;
    line-height: 1;
  }

  .page-header__eyebrow {
    color: var(--poodle-color-text-secondary);
    font-size: 0.6875rem;
    font-weight: 600;
    letter-spacing: 0.12em;
    text-transform: uppercase;
  }

  .page-header__subtitle {
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-typography-body-size);
    line-height: var(--poodle-typography-body-lineHeight);
  }

  .page-header__meta {
    margin-top: 0.125rem;
  }

  .page-header__body {
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-typography-body-size);
    line-height: var(--poodle-typography-body-lineHeight);
  }

  .page-header__actions-row {
    display: flex;
    align-items: center;
    gap: var(--poodle-space-inline-md);
  }

  .page-header__actions {
    display: flex;
    flex-wrap: wrap;
    gap: 0.375rem;
    align-items: center;
    margin-left: auto;
  }

  .page-header__banner {
    grid-column: 1 / -1;
  }

  @media (max-width: 45rem) {
    .page-header[data-align="between"] {
      grid-template-columns: 1fr;
    }

    .page-header__actions-row {
      flex-wrap: wrap;
    }

    .page-header__actions {
      margin-left: 0;
    }
  }
</style>
