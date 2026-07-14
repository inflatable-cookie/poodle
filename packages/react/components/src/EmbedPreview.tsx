import "@poodle/styles/embed-preview.css";

import type { CSSProperties } from "react";

import { Skeleton } from "./Skeleton";
import { TextLink } from "./TextLink";
import type { ParsedEmbed } from "./types";

export interface EmbedPreviewProps {
  parsed?: ParsedEmbed | null;
  trustedHtml?: string | null;
  aspectRatio?: number | "auto";
  loading?: boolean;
  error?: string | null;
  emptyMessage?: string;
}

function getEmbedUrl(embed: ParsedEmbed): string | null {
  switch (embed.provider) {
    case "youtube":
      return `https://www.youtube-nocookie.com/embed/${embed.id}`;
    case "vimeo":
      return `https://player.vimeo.com/video/${embed.id}`;
    default:
      return embed.originalUrl ?? null;
  }
}

export function EmbedPreview({
  parsed = null,
  trustedHtml = null,
  aspectRatio = 16 / 9,
  loading = false,
  error = null,
  emptyMessage = "No embed to preview",
}: EmbedPreviewProps) {
  const embedUrl = parsed ? getEmbedUrl(parsed) : null;
  const isAudio = parsed?.provider === "audioboom";
  const effectiveAspectRatio = isAudio ? "auto" : aspectRatio;
  const hasFixedAspectRatio = effectiveAspectRatio !== "auto";
  const containerStyle: CSSProperties | undefined = hasFixedAspectRatio
    ? { aspectRatio: `${effectiveAspectRatio}` }
    : undefined;

  return (
    <div className="poodle-embed-preview">
      {loading ? (
        <div className="poodle-embed-preview__loading">
          <Skeleton shape="block" />
          <span className="poodle-embed-preview__loading-text">Loading preview...</span>
        </div>
      ) : error ? (
        <div className="poodle-embed-preview__error">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5">
            <circle cx="12" cy="12" r="10" />
            <path d="M12 8v4m0 4h.01" strokeLinecap="round" />
          </svg>
          <span>{error}</span>
        </div>
      ) : !parsed && !trustedHtml ? (
        <div className="poodle-embed-preview__empty">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5">
            <rect x="2" y="4" width="20" height="16" rx="2" />
            <path d="M10 9l5 3-5 3V9z" />
          </svg>
          <span>{emptyMessage}</span>
        </div>
      ) : parsed && embedUrl ? (
        <div className="poodle-embed-preview__container" data-fixed-aspect={hasFixedAspectRatio} style={containerStyle}>
          <iframe
            src={embedUrl}
            title={`${parsed.provider} embed`}
            frameBorder="0"
            allowFullScreen
            loading="lazy"
            sandbox="allow-scripts allow-same-origin allow-popups"
            className="poodle-embed-preview__iframe"
          />
        </div>
      ) : parsed?.originalEmbed ? (
        <div
          className="poodle-embed-preview__container"
          data-fixed-aspect={hasFixedAspectRatio}
          style={containerStyle}
          dangerouslySetInnerHTML={{ __html: parsed.originalEmbed }}
        />
      ) : trustedHtml ? (
        <div
          className="poodle-embed-preview__container"
          data-fixed-aspect={hasFixedAspectRatio}
          style={containerStyle}
          dangerouslySetInnerHTML={{ __html: trustedHtml }}
        />
      ) : parsed ? (
        <div className="poodle-embed-preview__fallback">
          <TextLink href={parsed.originalUrl} target="_blank" rel="noopener noreferrer">
            {parsed.originalUrl ?? parsed.id}
          </TextLink>
        </div>
      ) : null}
    </div>
  );
}
