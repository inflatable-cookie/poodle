import "@inflatable-cookie/poodle-styles/media-thumbnail.css";

import type { CSSProperties, ReactNode } from "react";

import { Icon } from "./Icon";
import { Spinner } from "./Spinner";
import { resolveSemanticControlSize, resolveSupportingVisualSize, useUiPresentation } from "./presentation";
import type { AspectRatio, MediaKind, MediaState } from "./types";

export interface MediaThumbnailProps {
  kind?: MediaKind;
  state?: MediaState;
  aspectRatio?: AspectRatio;
  title?: string | null;
  badge?: string | null;
  meta?: string | null;
  ariaLabel?: string | null;
  stateTitle?: string | null;
  stateMessage?: string | null;
  presentation?: "default" | "compact";
  fit?: "cover" | "contain";
  frameWidth?: "fill" | "xl" | number | string | null;
  frameMinHeight?: number | string | null;
  frameMaxHeight?: number | string | null;
  children?: ReactNode;
}

export function MediaThumbnail({
  kind = "image",
  state = "ready",
  aspectRatio = "landscape",
  title = null,
  badge = null,
  meta = null,
  ariaLabel = null,
  stateTitle = null,
  stateMessage = null,
  presentation = "default",
  fit = "cover",
  frameWidth = "fill",
  frameMinHeight = null,
  frameMaxHeight = null,
  children,
}: MediaThumbnailProps) {
  const uiPresentation = useUiPresentation();

  const resolvedKind = kind === "pdf" || kind === "other" ? "document" : kind;
  const rootStyle: CSSProperties =
    frameWidth === null || frameWidth === undefined || frameWidth === "fill"
      ? { inlineSize: "100%" }
      : frameWidth === "xl"
        ? { inlineSize: "min(100%, 24rem)" }
        : { inlineSize: typeof frameWidth === "number" ? `${frameWidth}px` : frameWidth };
  const frameStyle: CSSProperties = {
    ...(frameMinHeight === null || frameMinHeight === undefined
      ? {}
      : { minBlockSize: typeof frameMinHeight === "number" ? `${frameMinHeight}px` : frameMinHeight }),
    ...(frameMaxHeight === null || frameMaxHeight === undefined
      ? {}
      : { maxBlockSize: typeof frameMaxHeight === "number" ? `${frameMaxHeight}px` : frameMaxHeight }),
  };
  const resolvedStateTitle =
    stateTitle ?? (state === "loading" ? "Loading preview" : state === "error" ? "Preview unavailable" : "No preview");
  const fallbackIcon =
    resolvedKind === "audio"
      ? "music"
      : resolvedKind === "video"
        ? "play"
        : resolvedKind === "document"
          ? "file-text"
          : resolvedKind === "embed"
            ? "external-link"
            : "image";
  const resolvedVisualSize = resolveSemanticControlSize(uiPresentation.sizeScale, "control");
  const resolvedSupportingSize = resolveSupportingVisualSize(resolvedVisualSize);
  const resolvedSpinnerSize =
    presentation === "compact" ? resolveSupportingVisualSize(resolvedSupportingSize) : resolvedSupportingSize;

  return (
    <figure
      className="poodle-media-thumbnail"
      data-kind={resolvedKind}
      data-state={state}
      data-aspect-ratio={aspectRatio}
      data-presentation={presentation}
      data-fit={fit}
      aria-label={ariaLabel ?? title ?? undefined}
      aria-busy={state === "loading"}
      style={rootStyle}
    >
      <div className="poodle-media-thumbnail__frame" style={frameStyle}>
        {state === "ready" ? (
          <>
            {children ?? (
              <div className="poodle-media-thumbnail__placeholder" aria-hidden="true">
                <Icon name={fallbackIcon} size={resolvedVisualSize} />
              </div>
            )}

            {resolvedKind === "audio" || resolvedKind === "video" ? (
              <span className="poodle-media-thumbnail__play" aria-hidden="true">
                <Icon name={resolvedKind === "audio" ? "music" : "play"} size={resolvedSupportingSize} />
              </span>
            ) : null}
          </>
        ) : (
          <div className="poodle-media-thumbnail__state">
            {state === "loading" ? (
              <span className="poodle-media-thumbnail__spinner" aria-hidden="true">
                <Spinner variant="grid" size={resolvedSpinnerSize} tone="accent" />
              </span>
            ) : null}
            <strong>{resolvedStateTitle}</strong>
            {stateMessage && presentation !== "compact" ? <p>{stateMessage}</p> : null}
          </div>
        )}

        {badge ? <span className="poodle-media-thumbnail__badge">{badge}</span> : null}
      </div>

      {presentation !== "compact" && (title || meta) ? (
        <figcaption className="poodle-media-thumbnail__caption">
          {title ? <strong>{title}</strong> : null}
          {meta ? <span>{meta}</span> : null}
        </figcaption>
      ) : null}
    </figure>
  );
}
