import "@inflatable-cookie/poodle-core/styles/media-preview.css";

import type { ReactNode } from "react";

import { Card } from "./Card";
import { MediaThumbnail } from "./MediaThumbnail";
import { UiPresentationProvider, resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type {
  AspectRatio,
  CardVariant,
  ControlDensity,
  ControlSize,
  MediaKind,
  MediaState,
  SemanticControlSizeRole,
} from "./types";

export interface MediaPreviewProps {
  title: string;
  description?: string | null;
  eyebrow?: string | null;
  caption?: string | null;
  meta?: string[];
  badge?: string | null;
  thumbnailMeta?: string | null;
  kind?: MediaKind;
  state?: MediaState;
  aspectRatio?: AspectRatio;
  variant?: CardVariant;
  ariaLabel?: string | null;
  stateTitle?: string | null;
  stateMessage?: string | null;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  mediaContent?: ReactNode;
  children?: ReactNode;
}

export function MediaPreview({
  title,
  description = null,
  eyebrow = null,
  caption = null,
  meta = [],
  badge = null,
  thumbnailMeta = null,
  kind = "image",
  state = "ready",
  aspectRatio = "landscape",
  variant = "default",
  ariaLabel = null,
  stateTitle = null,
  stateMessage = null,
  size = null,
  sizeRole = "control",
  density = null,
  mediaContent,
  children,
}: MediaPreviewProps) {
  const uiPresentation = useUiPresentation();
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;

  return (
    <UiPresentationProvider sizeScale={resolvedSize} density={resolvedDensity}>
      <div className="poodle-media-preview" data-size={resolvedSize} data-density={resolvedDensity}>
        <Card
          variant={variant}
          media={true}
          ariaLabel={ariaLabel ?? title}
          density={resolvedDensity}
          mediaContent={
            <MediaThumbnail
              kind={kind}
              state={state}
              aspectRatio={aspectRatio}
              title={null}
              badge={badge}
              meta={null}
              ariaLabel={title}
              stateTitle={stateTitle}
              stateMessage={stateMessage}
            >
              {mediaContent}
            </MediaThumbnail>
          }
          header={
            <div className="poodle-media-preview__header">
              <div className="poodle-media-preview__heading">
                {eyebrow ? <p className="poodle-media-preview__eyebrow">{eyebrow}</p> : null}
                <h3>{title}</h3>
                {description ? <p className="poodle-media-preview__description">{description}</p> : null}
              </div>

              {thumbnailMeta || meta.length > 0 ? (
                <ul className="poodle-media-preview__meta" aria-label="preview metadata">
                  {thumbnailMeta ? <li>{thumbnailMeta}</li> : null}
                  {meta.map((item) => (
                    <li key={item}>{item}</li>
                  ))}
                </ul>
              ) : null}
            </div>
          }
        >
          <div className="poodle-media-preview__body">
            {caption ? <p className="poodle-media-preview__caption">{caption}</p> : null}
            {children}
          </div>
        </Card>
      </div>
    </UiPresentationProvider>
  );
}
