<script lang="ts">
  import type { Snippet } from "svelte";

  import Callout from "./Callout.svelte";
  import Icon from "./Icon.svelte";
  import Pill from "./Pill.svelte";

  interface Props {
    title?: string | null;
    section?: string | null;
    count?: number | null;
    subtitle?: string | null;
    showSubtitleWithBreadcrumbs?: boolean;
    eyebrow?: string | null;
    posture?: "default" | "entity-detail";
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
    showSubtitleWithBreadcrumbs = false,
    eyebrow = null,
    posture = "default",
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

  const hasSectionTitleSplit = $derived(Boolean(section && title));
  const isEntityDetailPosture = $derived(posture === "entity-detail" && hasSectionTitleSplit);
  const primaryTitle = $derived(
    isEntityDetailPosture ? section ?? title ?? "" : title ?? section ?? ""
  );
  const resolvedSubtitle = $derived(
    isEntityDetailPosture ? title ?? subtitle ?? null : subtitle
  );
  const hasPrimaryHeading = $derived(Boolean(primaryTitle || count !== null));
  const showTopBreadcrumbs = $derived(Boolean(breadcrumbs && !isEntityDetailPosture));
  const showSubtitleText = $derived(
    Boolean(resolvedSubtitle) &&
      (!isEntityDetailPosture || !breadcrumbs || showSubtitleWithBreadcrumbs)
  );
  const isCompactSubtitleHeader = $derived(
    !hasPrimaryHeading &&
      Boolean(resolvedSubtitle) &&
      !eyebrow &&
      !showTopBreadcrumbs &&
      !meta &&
      !children
  );
  const headingTag = $derived(`h${level}` as `h${1 | 2 | 3 | 4 | 5 | 6}`);
</script>

<header
  class="poodle-page-header"
  data-align={align}
  data-level={level}
  data-has-title={hasPrimaryHeading}
  data-compact-subtitle-header={isCompactSubtitleHeader}
  aria-label={ariaLabel ?? undefined}
>
  <div class="poodle-page-header__content">
    {#if showTopBreadcrumbs && breadcrumbs}
      <div class="poodle-page-header__breadcrumbs">
        {@render breadcrumbs()}
      </div>
    {/if}

    <div class="poodle-page-header__title-block">
      {#if eyebrow}
        <p class="poodle-page-header__eyebrow">{eyebrow}</p>
      {/if}
      {#if hasSectionTitleSplit && !isEntityDetailPosture}
        <p class="poodle-page-header__section">{section}</p>
      {/if}
      {#if hasPrimaryHeading}
        <svelte:element this={headingTag} class="poodle-page-header__title">
          {#if primaryTitle}
            <span>{primaryTitle}</span>
          {/if}
          {#if count !== null}
            <span class="poodle-page-header__count">
              <Pill tone="neutral" appearance="subtle" size="lg" ariaLabel={`${count}`}>
                {count}
              </Pill>
            </span>
          {/if}
        </svelte:element>
      {/if}
      {#if hasSectionTitleSplit && !isEntityDetailPosture}
        <p class="poodle-page-header__section-title">{title}</p>
      {/if}
      {#if showSubtitleText}
        <p class="poodle-page-header__subtitle">{resolvedSubtitle}</p>
      {/if}
      {#if isEntityDetailPosture && breadcrumbs}
        <div class="poodle-page-header__breadcrumbs poodle-page-header__breadcrumbs--subtitle">
          {@render breadcrumbs()}
        </div>
      {/if}
      {#if meta}
        <div class="poodle-page-header__meta">
          {@render meta()}
        </div>
      {/if}
      {#if children}
        <div class="poodle-page-header__body">
          {@render children()}
        </div>
      {/if}
    </div>
  </div>

  {#if backHref || actions}
    <div class="poodle-page-header__actions-row">
      {#if backHref}
        <a class="poodle-page-header__back" href={backHref}>
          <Icon name="arrow-left" />
          <span>{backLabel ?? "Back"}</span>
          {#if backIsContextual}
            <span class="poodle-page-header__context-dot" aria-hidden="true"></span>
          {/if}
        </a>
      {/if}
      {#if actions}
        <div class="poodle-page-header__actions">
          {@render actions()}
        </div>
      {/if}
    </div>
  {/if}

  {#if banner}
    <div class="poodle-page-header__banner">
      {@render banner()}
    </div>
  {:else if bannerMessage}
    <div class="poodle-page-header__banner">
      <Callout tone={bannerTone} message={bannerMessage} announceMode="polite" />
    </div>
  {/if}
</header>

<style>
  .poodle-page-header {
    --poodle-recipe-page-header-padding-block-start: 0;
    --poodle-recipe-page-header-padding-inline: 0;
    --poodle-recipe-page-header-padding-block-end: calc(var(--poodle-space-stack-md) + 0.125rem);
    --poodle-recipe-page-header-fill: transparent;
    --poodle-recipe-page-header-border: transparent;
    --poodle-recipe-page-header-shadow: none;
    --poodle-recipe-page-header-radius: var(--poodle-radius-surface);
    display: grid;
    gap: var(--poodle-space-stack-md);
    align-items: start;
    padding:
      var(--poodle-recipe-page-header-padding-block-start)
      var(--poodle-recipe-page-header-padding-inline)
      var(--poodle-recipe-page-header-padding-block-end);
    border: 0.0625rem solid var(--poodle-recipe-page-header-border);
    border-radius: var(--poodle-recipe-page-header-radius);
    background: var(--poodle-recipe-page-header-fill);
    box-shadow: var(--poodle-recipe-page-header-shadow);
  }

  .poodle-page-header[data-align="between"] {
    grid-template-columns: minmax(0, 1fr) auto;
  }

  .poodle-page-header[data-compact-subtitle-header="true"][data-align="between"] {
    align-items: center;
  }

  .poodle-page-header__content {
    display: grid;
    gap: var(--poodle-space-stack-md);
  }

  .poodle-page-header__breadcrumbs {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    min-width: 0;
  }

  .poodle-page-header[data-compact-subtitle-header="true"] .poodle-page-header__content,
  .poodle-page-header[data-compact-subtitle-header="true"] .poodle-page-header__title-block {
    gap: 0;
  }

  .poodle-page-header__back {
    width: fit-content;
    display: inline-flex;
    align-items: center;
    gap: 0.35rem;
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.2;
    text-decoration: none;
  }

  .poodle-page-header__back:hover {
    color: var(--poodle-color-text-primary);
  }

  .poodle-page-header__context-dot {
    width: 0.375rem;
    height: 0.375rem;
    border-radius: 999px;
    background: var(--poodle-color-status-success, #22c55e);
    flex: none;
  }

  .poodle-page-header__title-block {
    display: grid;
    gap: var(--poodle-space-inline-sm);
  }

  .poodle-page-header__title,
  .poodle-page-header__subtitle,
  .poodle-page-header__body,
  .poodle-page-header__eyebrow,
  .poodle-page-header__section,
  .poodle-page-header__section-title {
    margin: 0;
  }

  .poodle-page-header__title {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    font-family: var(--poodle-typography-heading-family);
    font-size: 1.75rem;
    line-height: 1.1;
    font-weight: 700;
  }

  .poodle-page-header__section {
    color: var(--poodle-color-text-secondary);
    font-size: 0.75rem;
    font-weight: 700;
    letter-spacing: 0.08em;
    text-transform: uppercase;
  }

  .poodle-page-header__section-title {
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-heading-family);
    font-size: 1rem;
    line-height: 1.25;
    font-weight: 600;
  }

  .poodle-page-header__count {
    display: inline-flex;
    align-items: center;
  }

  .poodle-page-header__eyebrow {
    color: var(--poodle-color-text-secondary);
    font-size: 0.6875rem;
    font-weight: 600;
    letter-spacing: 0.12em;
    text-transform: uppercase;
  }

  .poodle-page-header__subtitle {
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-typography-body-size);
    line-height: var(--poodle-typography-body-lineHeight);
  }

  .poodle-page-header[data-has-title="false"] .poodle-page-header__subtitle {
    color: var(--poodle-color-text-primary);
    font-family: var(--poodle-typography-heading-family);
    font-size: 1rem;
    line-height: 1.2;
    font-weight: 600;
  }

  .poodle-page-header[data-level="3"] .poodle-page-header__title,
  .poodle-page-header[data-level="4"] .poodle-page-header__title,
  .poodle-page-header[data-level="5"] .poodle-page-header__title,
  .poodle-page-header[data-level="6"] .poodle-page-header__title {
    font-size: 1.25rem;
    line-height: 1.15;
  }

  .poodle-page-header[data-level="3"] .poodle-page-header__section-title,
  .poodle-page-header[data-level="4"] .poodle-page-header__section-title,
  .poodle-page-header[data-level="5"] .poodle-page-header__section-title,
  .poodle-page-header[data-level="6"] .poodle-page-header__section-title {
    font-size: 0.875rem;
  }

  .poodle-page-header[data-level="3"] .poodle-page-header__subtitle,
  .poodle-page-header[data-level="3"] .poodle-page-header__body,
  .poodle-page-header[data-level="4"] .poodle-page-header__subtitle,
  .poodle-page-header[data-level="4"] .poodle-page-header__body,
  .poodle-page-header[data-level="5"] .poodle-page-header__subtitle,
  .poodle-page-header[data-level="5"] .poodle-page-header__body,
  .poodle-page-header[data-level="6"] .poodle-page-header__subtitle,
  .poodle-page-header[data-level="6"] .poodle-page-header__body {
    font-size: 0.875rem;
  }

  .poodle-page-header[data-level="3"][data-has-title="false"] .poodle-page-header__subtitle,
  .poodle-page-header[data-level="4"][data-has-title="false"] .poodle-page-header__subtitle,
  .poodle-page-header[data-level="5"][data-has-title="false"] .poodle-page-header__subtitle,
  .poodle-page-header[data-level="6"][data-has-title="false"] .poodle-page-header__subtitle {
    font-size: 0.9375rem;
    line-height: 1.15;
  }

  .poodle-page-header__meta {
    margin-top: 0.125rem;
  }

  .poodle-page-header__body {
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-typography-body-size);
    line-height: var(--poodle-typography-body-lineHeight);
  }

  .poodle-page-header__actions-row {
    display: flex;
    align-items: center;
    gap: var(--poodle-space-inline-md);
  }

  .poodle-page-header[data-compact-subtitle-header="true"] .poodle-page-header__actions-row {
    align-self: center;
  }

  .poodle-page-header__actions {
    display: flex;
    flex-wrap: wrap;
    gap: 0.375rem;
    align-items: center;
    margin-left: auto;
  }

  .poodle-page-header__actions:empty {
    display: none;
  }

  .poodle-page-header__banner {
    grid-column: 1 / -1;
    margin-top: 0.75rem;
  }

  @media (max-width: 45rem) {
    .poodle-page-header[data-align="between"] {
      grid-template-columns: 1fr;
    }

    .poodle-page-header__actions-row {
      flex-wrap: wrap;
    }

    .poodle-page-header__actions {
      margin-left: 0;
    }
  }
</style>
