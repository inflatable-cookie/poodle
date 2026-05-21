<script lang="ts">
  import type { Snippet } from "svelte";

  import { default as Callout } from "./Callout.svelte";
  import { default as Icon } from "./Icon.svelte";
  import { default as Pill } from "./Pill.svelte";
  import { default as UiPresentationProvider } from "./UiPresentationProvider.svelte";
  import { getUiPresentation, resolveSemanticControlSize, resolveSupportingVisualSize } from "./presentation";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

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
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole | null;
    density?: ControlDensity | null;
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
    size = null,
    sizeRole = null,
    density = null,
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
  const uiPresentation = getUiPresentation();
  const resolvedSize = $derived(
    size ?? (sizeRole ? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole) : $uiPresentation.sizeScale)
  );
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const countPillSize = $derived(resolveSupportingVisualSize(resolvedSize));
</script>

<UiPresentationProvider sizeScale={resolvedSize} density={resolvedDensity}>
  <header
    class="poodle-page-header"
    data-align={align}
    data-level={level}
    data-size={resolvedSize}
    data-density={resolvedDensity}
    data-has-title={hasPrimaryHeading}
    data-compact-subtitle-header={isCompactSubtitleHeader}
    aria-label={ariaLabel ?? undefined}
  >
    <div class="poodle-page-header__content">
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
                <Pill tone="neutral" appearance="subtle" size={countPillSize} ariaLabel={`${count}`}>
                  {count}
                </Pill>
              </span>
            {/if}
          </svelte:element>
        {/if}
        {#if showSubtitleText}
          <p class="poodle-page-header__subtitle">{resolvedSubtitle}</p>
        {/if}
        {#if showTopBreadcrumbs && breadcrumbs}
          <div class="poodle-page-header__breadcrumbs poodle-page-header__breadcrumbs--stacked">
            {@render breadcrumbs()}
          </div>
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
            <Icon name="arrow-left" size={countPillSize} />
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
</UiPresentationProvider>

<style>
  .poodle-page-header {
    --poodle-page-header-gap: var(--poodle-space-stack-md);
    --poodle-page-header-content-gap: var(--poodle-space-stack-md);
    --poodle-page-header-title-gap: var(--poodle-space-inline-sm);
    --poodle-page-header-title-size: 1.75rem;
    --poodle-page-header-title-compact-size: 1.25rem;
    --poodle-page-header-section-size: 0.75rem;
    --poodle-page-header-section-title-size: 1rem;
    --poodle-page-header-section-title-compact-size: 0.875rem;
    --poodle-page-header-eyebrow-size: 0.6875rem;
    --poodle-page-header-subtitle-size: 1rem;
    --poodle-page-header-subtitle-compact-size: 1rem;
    --poodle-page-header-body-size: var(--poodle-typography-body-size);
    --poodle-page-header-back-size: 0.8125rem;
    --poodle-page-header-actions-gap: 0.375rem;
    --poodle-page-header-actions-row-gap: var(--poodle-space-inline-md);
    --poodle-page-header-banner-margin-top: 0.75rem;
    --poodle-recipe-page-header-padding-block-start: 0;
    --poodle-recipe-page-header-padding-inline: 0;
    --poodle-recipe-page-header-padding-block-end: calc(var(--poodle-space-stack-md) + 0.125rem);
    --poodle-recipe-page-header-fill: transparent;
    --poodle-recipe-page-header-border: transparent;
    --poodle-recipe-page-header-shadow: none;
    --poodle-recipe-page-header-radius: var(--poodle-radius-surface);
    display: grid;
    gap: var(--poodle-page-header-gap);
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
    gap: var(--poodle-page-header-content-gap);
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
    font-size: var(--poodle-page-header-back-size);
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
    gap: var(--poodle-page-header-title-gap);
  }

  .poodle-page-header__title,
  .poodle-page-header__subtitle,
  .poodle-page-header__body,
  .poodle-page-header__eyebrow,
  .poodle-page-header__section {
    margin: 0;
  }

  .poodle-page-header__title {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    font-family: var(--poodle-typography-heading-family);
    font-size: var(--poodle-page-header-title-size);
    line-height: 1.1;
    font-weight: 700;
  }

  .poodle-page-header__section {
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-page-header-section-size);
    font-weight: 700;
    letter-spacing: 0.08em;
    text-transform: uppercase;
  }

  .poodle-page-header__count {
    display: inline-flex;
    align-items: center;
  }

  .poodle-page-header__eyebrow {
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-page-header-eyebrow-size);
    font-weight: 600;
    letter-spacing: 0.12em;
    text-transform: uppercase;
  }

  .poodle-page-header__subtitle {
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-page-header-subtitle-size);
    line-height: var(--poodle-typography-body-lineHeight);
  }

  .poodle-page-header[data-has-title="false"] .poodle-page-header__subtitle {
    color: var(--poodle-color-text-primary);
    font-family: var(--poodle-typography-heading-family);
    font-size: var(--poodle-page-header-subtitle-compact-size);
    line-height: 1.2;
    font-weight: 600;
  }

  .poodle-page-header[data-level="3"] .poodle-page-header__title,
  .poodle-page-header[data-level="4"] .poodle-page-header__title,
  .poodle-page-header[data-level="5"] .poodle-page-header__title,
  .poodle-page-header[data-level="6"] .poodle-page-header__title {
    font-size: var(--poodle-page-header-title-compact-size);
    line-height: 1.15;
  }

  .poodle-page-header[data-level="3"] .poodle-page-header__subtitle,
  .poodle-page-header[data-level="3"] .poodle-page-header__body,
  .poodle-page-header[data-level="4"] .poodle-page-header__subtitle,
  .poodle-page-header[data-level="4"] .poodle-page-header__body,
  .poodle-page-header[data-level="5"] .poodle-page-header__subtitle,
  .poodle-page-header[data-level="5"] .poodle-page-header__body,
  .poodle-page-header[data-level="6"] .poodle-page-header__subtitle,
  .poodle-page-header[data-level="6"] .poodle-page-header__body {
    font-size: var(--poodle-page-header-body-size);
  }

  .poodle-page-header[data-level="3"][data-has-title="false"] .poodle-page-header__subtitle,
  .poodle-page-header[data-level="4"][data-has-title="false"] .poodle-page-header__subtitle,
  .poodle-page-header[data-level="5"][data-has-title="false"] .poodle-page-header__subtitle,
  .poodle-page-header[data-level="6"][data-has-title="false"] .poodle-page-header__subtitle {
    font-size: var(--poodle-page-header-subtitle-compact-size);
    line-height: 1.15;
  }

  .poodle-page-header__meta {
    margin-top: 0.125rem;
  }

  .poodle-page-header__body {
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-page-header-body-size);
    line-height: var(--poodle-typography-body-lineHeight);
  }

  .poodle-page-header__actions-row {
    display: flex;
    align-items: center;
    gap: var(--poodle-page-header-actions-row-gap);
  }

  .poodle-page-header[data-compact-subtitle-header="true"] .poodle-page-header__actions-row {
    align-self: center;
  }

  .poodle-page-header__actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--poodle-page-header-actions-gap);
    align-items: center;
    margin-left: auto;
  }

  .poodle-page-header__actions:empty {
    display: none;
  }

  .poodle-page-header__banner {
    grid-column: 1 / -1;
    margin-top: var(--poodle-page-header-banner-margin-top);
  }

  .poodle-page-header[data-size="xs"] {
    --poodle-page-header-title-size: 1.375rem;
    --poodle-page-header-title-compact-size: 1.0625rem;
    --poodle-page-header-section-size: 0.6875rem;
    --poodle-page-header-section-title-size: 0.875rem;
    --poodle-page-header-section-title-compact-size: 0.8125rem;
    --poodle-page-header-eyebrow-size: 0.625rem;
    --poodle-page-header-subtitle-size: 0.875rem;
    --poodle-page-header-subtitle-compact-size: 0.875rem;
    --poodle-page-header-body-size: 0.8125rem;
    --poodle-page-header-back-size: 0.75rem;
  }

  .poodle-page-header[data-size="sm"] {
    --poodle-page-header-title-size: 1.5rem;
    --poodle-page-header-title-compact-size: 1.125rem;
    --poodle-page-header-section-size: 0.71875rem;
    --poodle-page-header-section-title-size: 0.9375rem;
    --poodle-page-header-section-title-compact-size: 0.84375rem;
    --poodle-page-header-eyebrow-size: 0.65625rem;
    --poodle-page-header-subtitle-size: 0.9375rem;
    --poodle-page-header-subtitle-compact-size: 0.9375rem;
    --poodle-page-header-body-size: 0.875rem;
    --poodle-page-header-back-size: 0.78125rem;
  }

  .poodle-page-header[data-size="md"] {
    --poodle-page-header-title-size: 1.75rem;
    --poodle-page-header-title-compact-size: 1.25rem;
    --poodle-page-header-section-size: 0.75rem;
    --poodle-page-header-section-title-size: 1rem;
    --poodle-page-header-section-title-compact-size: 0.875rem;
    --poodle-page-header-eyebrow-size: 0.6875rem;
    --poodle-page-header-subtitle-size: 1rem;
    --poodle-page-header-subtitle-compact-size: 1rem;
    --poodle-page-header-body-size: var(--poodle-typography-body-size);
    --poodle-page-header-back-size: 0.8125rem;
  }

  .poodle-page-header[data-size="lg"] {
    --poodle-page-header-title-size: 2rem;
    --poodle-page-header-title-compact-size: 1.375rem;
    --poodle-page-header-section-size: 0.8125rem;
    --poodle-page-header-section-title-size: 1.125rem;
    --poodle-page-header-section-title-compact-size: 0.9375rem;
    --poodle-page-header-eyebrow-size: 0.75rem;
    --poodle-page-header-subtitle-size: 1.0625rem;
    --poodle-page-header-subtitle-compact-size: 1.0625rem;
    --poodle-page-header-body-size: 0.9375rem;
    --poodle-page-header-back-size: 0.875rem;
  }

  .poodle-page-header[data-size="xl"] {
    --poodle-page-header-title-size: 2.25rem;
    --poodle-page-header-title-compact-size: 1.5rem;
    --poodle-page-header-section-size: 0.875rem;
    --poodle-page-header-section-title-size: 1.25rem;
    --poodle-page-header-section-title-compact-size: 1rem;
    --poodle-page-header-eyebrow-size: 0.8125rem;
    --poodle-page-header-subtitle-size: 1.125rem;
    --poodle-page-header-subtitle-compact-size: 1.125rem;
    --poodle-page-header-body-size: 1rem;
    --poodle-page-header-back-size: 0.9375rem;
  }

  .poodle-page-header[data-density="compact"] {
    --poodle-page-header-gap: 0.75rem;
    --poodle-page-header-content-gap: 0.75rem;
    --poodle-page-header-title-gap: 0.375rem;
    --poodle-page-header-actions-gap: 0.25rem;
    --poodle-page-header-actions-row-gap: 0.75rem;
    --poodle-page-header-banner-margin-top: 0.5rem;
  }

  .poodle-page-header[data-density="default"] {
    --poodle-page-header-gap: var(--poodle-space-stack-md);
    --poodle-page-header-content-gap: var(--poodle-space-stack-md);
    --poodle-page-header-title-gap: var(--poodle-space-inline-sm);
    --poodle-page-header-actions-gap: 0.375rem;
    --poodle-page-header-actions-row-gap: var(--poodle-space-inline-md);
    --poodle-page-header-banner-margin-top: 0.75rem;
  }

  .poodle-page-header[data-density="comfortable"] {
    --poodle-page-header-gap: 1.125rem;
    --poodle-page-header-content-gap: 1.125rem;
    --poodle-page-header-title-gap: 0.625rem;
    --poodle-page-header-actions-gap: 0.5rem;
    --poodle-page-header-actions-row-gap: 1rem;
    --poodle-page-header-banner-margin-top: 1rem;
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
