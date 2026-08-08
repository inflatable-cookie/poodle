import "@inflatable-cookie/poodle-core/styles/page-header.css";

import { createElement, type ReactNode } from "react";

import { Callout } from "./Callout";
import { Icon } from "./Icon";
import { Pill } from "./Pill";
import {
  UiPresentationProvider,
  resolveSemanticControlSize,
  resolveSupportingVisualSize,
  useUiPresentation,
} from "./presentation";
import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

export interface PageHeaderProps {
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
  children?: ReactNode;
  titleContent?: ReactNode;
  subtitleContent?: ReactNode;
  breadcrumbs?: ReactNode;
  meta?: ReactNode;
  actions?: ReactNode;
  banner?: ReactNode;
}

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

export function PageHeader({
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
}: PageHeaderProps) {
  const uiPresentation = useUiPresentation();

  const hasSectionTitleSplit = Boolean(section && title);
  const isEntityDetailPosture = posture === "entity-detail" && hasSectionTitleSplit;
  const primaryTitle = isEntityDetailPosture ? (section ?? title ?? "") : (title ?? section ?? "");
  const resolvedSubtitle = isEntityDetailPosture ? (title ?? subtitle ?? null) : subtitle;
  const hasPrimaryHeading = Boolean(primaryTitle || titleContent || count !== null);
  const showTopBreadcrumbs = Boolean(breadcrumbs && !isEntityDetailPosture);
  const showSubtitleText =
    Boolean(resolvedSubtitle || subtitleContent) &&
    (!isEntityDetailPosture || !breadcrumbs || showSubtitleWithBreadcrumbs);
  const hasSecondaryContent = showSubtitleText || Boolean(breadcrumbs) || Boolean(meta) || Boolean(children);
  const isCompactSubtitleHeader =
    !hasPrimaryHeading && Boolean(resolvedSubtitle) && !eyebrow && !showTopBreadcrumbs && !meta && !children;

  const resolvedSize =
    size ?? (sizeRole ? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole) : uiPresentation.sizeScale);
  const resolvedDensity = density ?? uiPresentation.density;
  const countPillSize = resolveSupportingVisualSize(resolvedSize);
  const resolvedBackDisplayLabel = resolveBackDisplayLabel(backLabel);
  const resolvedBackAriaLabel = resolveBackAriaLabel(backLabel);

  const heading = hasPrimaryHeading
    ? createElement(
        `h${level}`,
        { className: "poodle-page-header__title" },
        titleContent ?? (primaryTitle ? <span>{primaryTitle}</span> : null),
        count !== null ? (
          <span key="count" className="poodle-page-header__count">
            <Pill tone="neutral" appearance="subtle" size={countPillSize} ariaLabel={`${count}`}>
              {count}
            </Pill>
          </span>
        ) : null,
      )
    : null;

  return (
    <UiPresentationProvider sizeScale={resolvedSize} density={resolvedDensity}>
      <header
        className="poodle-page-header"
        data-align={align}
        data-level={level}
        data-size={resolvedSize}
        data-density={resolvedDensity}
        data-has-title={hasPrimaryHeading}
        data-compact-subtitle-header={isCompactSubtitleHeader}
        aria-label={ariaLabel ?? undefined}
      >
        <div className="poodle-page-header__top-row">
          <div className="poodle-page-header__content poodle-page-header__content--top">
            {eyebrow ? <p className="poodle-page-header__eyebrow">{eyebrow}</p> : null}
            {hasSectionTitleSplit && !isEntityDetailPosture ? (
              <p className="poodle-page-header__section">{section}</p>
            ) : null}
            {heading}
          </div>

          {backHref || actions ? (
            <div className="poodle-page-header__actions-row">
              {backHref ? (
                <>
                  <a className="poodle-page-header__back poodle-page-header__back--text" href={backHref}>
                    <Icon name="arrow-left" size={countPillSize} />
                    <span>{resolvedBackDisplayLabel}</span>
                    {backIsContextual ? (
                      <span className="poodle-page-header__context-dot" aria-hidden="true" />
                    ) : null}
                  </a>
                  <a
                    className="poodle-page-header__back poodle-page-header__back--icon"
                    href={backHref}
                    aria-label={resolvedBackAriaLabel}
                    title={resolvedBackAriaLabel}
                    data-contextual={backIsContextual || undefined}
                  >
                    <Icon name="arrow-left" size={countPillSize} />
                    {backIsContextual ? (
                      <span
                        className="poodle-page-header__context-dot poodle-page-header__context-dot--overlay"
                        aria-hidden="true"
                      />
                    ) : null}
                  </a>
                </>
              ) : null}
              {actions ? <div className="poodle-page-header__actions">{actions}</div> : null}
            </div>
          ) : null}
        </div>

        {hasSecondaryContent ? (
          <div className="poodle-page-header__content poodle-page-header__content--secondary">
            {showSubtitleText ? (
              <div className="poodle-page-header__subtitle">{subtitleContent ?? resolvedSubtitle}</div>
            ) : null}
            {showTopBreadcrumbs && breadcrumbs ? (
              <div className="poodle-page-header__breadcrumbs poodle-page-header__breadcrumbs--stacked">
                {breadcrumbs}
              </div>
            ) : null}
            {isEntityDetailPosture && breadcrumbs ? (
              <div className="poodle-page-header__breadcrumbs poodle-page-header__breadcrumbs--subtitle">
                {breadcrumbs}
              </div>
            ) : null}
            {meta ? <div className="poodle-page-header__meta">{meta}</div> : null}
            {children ? <div className="poodle-page-header__body">{children}</div> : null}
          </div>
        ) : null}

        {banner ? (
          <div className="poodle-page-header__banner">{banner}</div>
        ) : bannerMessage ? (
          <div className="poodle-page-header__banner">
            <Callout tone={bannerTone} message={bannerMessage} announceMode="polite" />
          </div>
        ) : null}
      </header>
    </UiPresentationProvider>
  );
}
