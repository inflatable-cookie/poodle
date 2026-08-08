import "@inflatable-cookie/poodle-styles/media-browse-panel.css";

import { Button } from "./Button";
import { Callout } from "./Callout";
import { MediaThumbnail } from "./MediaThumbnail";
import { UiPresentationProvider, resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, MediaKind, MediaPickerItem, SemanticControlSizeRole } from "./types";

export interface MediaBrowsePanelProps {
  loading?: boolean;
  error?: string | null;
  items?: MediaPickerItem[];
  hasMore?: boolean;
  emptyMessage?: string;
  loadMoreLabel?: string;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onSelect?: ((item: MediaPickerItem) => void) | undefined;
  onLoadMore?: (() => void) | undefined;
}

function toMediaKind(kind?: MediaKind): MediaKind {
  return kind ?? "image";
}

export function MediaBrowsePanel({
  loading = false,
  error = null,
  items = [],
  hasMore = false,
  emptyMessage = "No media found",
  loadMoreLabel = "Load more",
  size = null,
  sizeRole = "control",
  density = null,
  onSelect = undefined,
  onLoadMore = undefined,
}: MediaBrowsePanelProps) {
  const uiPresentation = useUiPresentation();

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;

  return (
    <UiPresentationProvider sizeScale={resolvedSize} density={resolvedDensity}>
      <div className="poodle-media-browse-panel" data-size={resolvedSize} data-density={resolvedDensity}>
        {loading && items.length === 0 ? (
          <div className="poodle-media-browse-panel__state">
            <p>Loading media...</p>
          </div>
        ) : error ? (
          <Callout tone="danger" message={error} announceMode="polite" />
        ) : items.length === 0 ? (
          <div className="poodle-media-browse-panel__state">
            <p>{emptyMessage}</p>
          </div>
        ) : (
          <>
            <div className="poodle-media-browse-panel__grid">
              {items.map((item) => (
                <button key={item.id} type="button" className="poodle-media-browse-panel__item" onClick={() => onSelect?.(item)}>
                  <MediaThumbnail kind={toMediaKind(item.kind)} presentation="compact" aspectRatio="square" ariaLabel={item.label}>
                    {item.thumbnailUrl ? (
                      <img src={item.thumbnailUrl} alt={item.label} className="poodle-media-browse-panel__image" />
                    ) : null}
                  </MediaThumbnail>
                  <span className="poodle-media-browse-panel__label">{item.label}</span>
                  {item.meta ? <span className="poodle-media-browse-panel__meta">{item.meta}</span> : null}
                </button>
              ))}
            </div>

            {hasMore ? (
              <div className="poodle-media-browse-panel__actions">
                <Button variant="secondary" size={resolvedSize} onClick={() => onLoadMore?.()} disabled={loading}>
                  {loading ? "Loading..." : loadMoreLabel}
                </Button>
              </div>
            ) : null}
          </>
        )}
      </div>
    </UiPresentationProvider>
  );
}
