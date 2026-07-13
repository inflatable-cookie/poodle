<script lang="ts">
  import "./page-header.css";
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
    titleContent?: Snippet;
    subtitleContent?: Snippet;
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
    titleContent,
    subtitleContent,
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
  const hasPrimaryHeading = $derived(Boolean(primaryTitle || titleContent || count !== null));
  const showTopBreadcrumbs = $derived(Boolean(breadcrumbs && !isEntityDetailPosture));
  const showSubtitleText = $derived(
    Boolean(resolvedSubtitle || subtitleContent) &&
      (!isEntityDetailPosture || !breadcrumbs || showSubtitleWithBreadcrumbs)
  );
  const hasSecondaryContent = $derived(
    showSubtitleText || Boolean(breadcrumbs) || Boolean(meta) || Boolean(children)
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
  function resolveBackDisplayLabel(label: string | null): string {
    const trimmed = label?.trim() ?? "";
    if (!trimmed) return "Back";

    const stripped = trimmed.replace(/^back(?:\s+to)?\s+/i, "").trim();
    return stripped || "Back";
  }

  function resolveBackAriaLabel(label: string | null): string {
    const displayLabel = resolveBackDisplayLabel(label);
    return displayLabel === "Back" ? "Back" : `Back to ${displayLabel}`;
  }

  const resolvedSize = $derived(
    size ?? (sizeRole ? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole) : $uiPresentation.sizeScale)
  );
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const countPillSize = $derived(resolveSupportingVisualSize(resolvedSize));
  const resolvedBackDisplayLabel = $derived(resolveBackDisplayLabel(backLabel));
  const resolvedBackAriaLabel = $derived(resolveBackAriaLabel(backLabel));
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
    <div class="poodle-page-header__top-row">
      <div class="poodle-page-header__content poodle-page-header__content--top">
        {#if eyebrow}
          <p class="poodle-page-header__eyebrow">{eyebrow}</p>
        {/if}
        {#if hasSectionTitleSplit && !isEntityDetailPosture}
          <p class="poodle-page-header__section">{section}</p>
        {/if}
        {#if hasPrimaryHeading}
          <svelte:element this={headingTag} class="poodle-page-header__title">
            {#if titleContent}
              {@render titleContent()}
            {:else if primaryTitle}
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
      </div>

      {#if backHref || actions}
        <div class="poodle-page-header__actions-row">
          {#if backHref}
            <a class="poodle-page-header__back poodle-page-header__back--text" href={backHref}>
              <Icon name="arrow-left" size={countPillSize} />
              <span>{resolvedBackDisplayLabel}</span>
              {#if backIsContextual}
                <span class="poodle-page-header__context-dot" aria-hidden="true"></span>
              {/if}
            </a>
            <a
              class="poodle-page-header__back poodle-page-header__back--icon"
              href={backHref}
              aria-label={resolvedBackAriaLabel}
              title={resolvedBackAriaLabel}
              data-contextual={backIsContextual || undefined}
            >
              <Icon name="arrow-left" size={countPillSize} />
              {#if backIsContextual}
                <span class="poodle-page-header__context-dot poodle-page-header__context-dot--overlay" aria-hidden="true"></span>
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
    </div>

    {#if hasSecondaryContent}
      <div class="poodle-page-header__content poodle-page-header__content--secondary">
        {#if showSubtitleText}
          <div class="poodle-page-header__subtitle">
            {#if subtitleContent}{@render subtitleContent()}{:else}{resolvedSubtitle}{/if}
          </div>
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
