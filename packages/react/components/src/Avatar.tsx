import "@poodle/styles/avatar.css";

export interface AvatarProps {
  src?: string | null;
  alt?: string | null;
  initials?: string | null;
  ariaLabel?: string | null;
  decorative?: boolean;
  size?: "xs" | "sm" | "md" | "lg" | "xl";
  shape?: "circle" | "rounded";
  tone?: "neutral" | "accent";
}

export function Avatar({
  src = null,
  alt = null,
  initials = null,
  ariaLabel = null,
  decorative = false,
  size = "md",
  shape = "circle",
  tone = "neutral",
}: AvatarProps) {
  const fallbackText = (initials ?? "?").trim().slice(0, 3).toUpperCase() || "?";
  const accessibleLabel = decorative ? undefined : (ariaLabel ?? alt ?? initials ?? "Avatar");

  return (
    <span
      className="poodle-avatar"
      data-size={size}
      data-shape={shape}
      data-tone={tone}
      aria-hidden={decorative ? "true" : undefined}
      aria-label={src ? undefined : accessibleLabel}
      role={!decorative && !src ? "img" : undefined}
    >
      {src ? (
        <img src={src} alt={decorative ? "" : (alt ?? accessibleLabel)} />
      ) : (
        <span className="poodle-avatar__initials">{fallbackText}</span>
      )}
    </span>
  );
}
