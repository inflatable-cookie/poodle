import { useState, type CSSProperties } from "react";
import { Icon, IconProvider, type IconSet } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";
import iconNodes from "lucide-static/icon-nodes.json";

const allIconNames = Object.keys(iconNodes as unknown as IconSet).sort();

// The 35 built-in internal icons that work without an IconProvider
const builtinNames = [
  "arrow-down", "arrow-right", "arrow-up", "check", "chevron-down",
  "chevron-left", "chevron-right", "chevron-up", "circle-alert",
  "circle-check", "circle-x", "columns-3", "download", "ellipsis",
  "ellipsis-vertical", "external-link", "file-text", "grip-vertical",
  "image", "inbox", "info", "list-filter", "loader", "lock-open",
  "minus", "music", "pencil", "play", "plus", "search", "star",
  "trending-down", "trending-up", "triangle-alert", "x",
];

const hint: CSSProperties = {
  fontSize: "0.75rem",
  color: "var(--poodle-color-text-secondary)",
  lineHeight: 1.5,
  margin: 0,
};

const hintCode: CSSProperties = {
  fontFamily: "var(--poodle-typography-code-family)",
  fontSize: "0.6875rem",
  padding: "0.0625rem 0.25rem",
  borderRadius: "0.1875rem",
  background: "color-mix(in srgb, var(--poodle-color-background-surface) 64%, transparent)",
};

const codeHint: CSSProperties = {
  fontFamily: "var(--poodle-typography-code-family)",
  fontSize: "0.6875rem",
  color: "var(--poodle-color-text-muted)",
  lineHeight: 1.6,
  padding: "0.5rem 0.75rem",
  borderRadius: "var(--poodle-radius-control)",
  background: "color-mix(in srgb, var(--poodle-color-background-surface) 48%, transparent)",
};

const sizeRow: CSSProperties = { display: "flex", gap: "1.5rem", alignItems: "center" };
const sizeDemo: CSSProperties = { display: "flex", alignItems: "center", gap: "0.5rem" };
const sizeLabel: CSSProperties = {
  fontSize: "0.6875rem",
  fontFamily: "var(--poodle-typography-code-family)",
  color: "var(--poodle-color-text-muted)",
  minWidth: "1.5rem",
};
const colorRow: CSSProperties = { display: "flex", gap: "1.25rem", alignItems: "center", flexWrap: "wrap" };
const colorDemo: CSSProperties = { display: "inline-flex", alignItems: "center", gap: "0.375rem", fontSize: "0.75rem" };

const iconGrid: CSSProperties = {
  display: "grid",
  gridTemplateColumns: "repeat(auto-fill, minmax(6.5rem, 1fr))",
  gap: "0.125rem",
};
const iconGridCompact: CSSProperties = {
  ...iconGrid,
  gridTemplateColumns: "repeat(auto-fill, minmax(5.5rem, 1fr))",
};
const iconCell: CSSProperties = {
  display: "flex",
  flexDirection: "column",
  alignItems: "center",
  gap: "0.25rem",
  padding: "0.625rem 0.25rem",
  border: "none",
  borderRadius: "var(--poodle-radius-control)",
  background: "transparent",
  color: "var(--poodle-color-text-primary)",
  cursor: "pointer",
  transition: "background 0.15s",
};
const iconCellCopied: CSSProperties = {
  ...iconCell,
  background: "color-mix(in srgb, var(--poodle-color-accent-base) 18%, transparent)",
};
const iconName: CSSProperties = {
  fontSize: "0.5625rem",
  fontFamily: "var(--poodle-typography-code-family)",
  color: "var(--poodle-color-text-muted)",
  textAlign: "center",
  wordBreak: "break-all",
  lineHeight: 1.3,
};

export function IconSpecimen() {
  const [copiedName, setCopiedName] = useState("");

  const copyName = (name: string) => {
    navigator.clipboard.writeText(name);
    setCopiedName(name);
    setTimeout(() => {
      setCopiedName((current) => (current === name ? "" : current));
    }, 1200);
  };

  return (
    <SpecimenLayout
      showDensities={false}
      sizes={(size) => (
        <div style={sizeDemo}>
          <Icon icon="star" size={size} />
          <Icon icon="heart" size={size} />
          <Icon icon="settings" size={size} />
        </div>
      )}
    >
      <SpecimenGroup label="Direct import — tree-shakeable">
        <p style={hint}>
          Import individual icons from <code style={hintCode}>@inflatable-cookie/poodle-core/icons</code>.
          Only the icons you use are included in the bundle.
        </p>
        <div style={sizeRow}>
          {(["xs", "sm", "md", "lg", "xl"] as const).map((size) => (
            <div key={size} style={sizeDemo}>
              <span style={sizeLabel}>{size}</span>
              <Icon icon="star" size={size} />
              <Icon icon="heart" size={size} />
              <Icon icon="settings" size={size} />
            </div>
          ))}
        </div>
        <div style={codeHint}>
          <code>{'import { star, heart, settings } from "@inflatable-cookie/poodle-core/icons";'}</code>
          <br />
          <code>{'<Icon icon={star} size="lg" />'}</code>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Color inheritance">
        <p style={hint}>
          Icons inherit <code style={hintCode}>currentColor</code> from their parent element.
        </p>
        <div style={colorRow}>
          <span style={{ ...colorDemo, color: "var(--poodle-color-icon-primary)" }}>
            <Icon icon="circle-check" /> Primary
          </span>
          <span style={{ ...colorDemo, color: "var(--poodle-color-icon-muted)" }}>
            <Icon icon="info" /> Muted
          </span>
          <span style={{ ...colorDemo, color: "var(--poodle-color-accent-base)" }}>
            <Icon icon="zap" /> Accent
          </span>
          <span style={{ ...colorDemo, color: "var(--poodle-color-status-danger)" }}>
            <Icon icon="triangle-alert" /> Danger
          </span>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Accessibility">
        <p style={hint}>
          Set <code style={hintCode}>ariaLabel</code> for meaningful icons. Decorative icons
          are automatically <code style={hintCode}>aria-hidden</code>.
        </p>
        <div style={colorRow}>
          <span style={colorDemo}>
            <Icon icon="search" ariaLabel="Search" /> with ariaLabel (role="img")
          </span>
          <span style={colorDemo}>
            <Icon icon="pencil" /> without ariaLabel (aria-hidden)
          </span>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label={`Built-in internal icons (${builtinNames.length})`}>
        <p style={hint}>
          These icons are embedded in the framework and work with string names
          without any <code style={hintCode}>IconProvider</code>. They cover component chrome
          (chevrons, check, x, plus, etc.).
        </p>
        <div style={iconGridCompact}>
          {builtinNames.map((name) => (
            <button
              key={name}
              style={copiedName === name ? iconCellCopied : iconCell}
              onClick={() => copyName(name)}
              title={name}
            >
              <Icon icon={name} />
              <span style={iconName}>{name}</span>
            </button>
          ))}
        </div>
        <div style={codeHint}>
          <code>{'<Icon icon="chevron-down" sizeRole="chrome" />'}</code>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label={`All icons via IconProvider (${allIconNames.length})`}>
        <p style={hint}>
          Wrap your app (or a subtree) in <code style={hintCode}>&lt;IconProvider&gt;</code> with
          a full icon set to enable string-based lookups for any icon.
          Click any icon to copy its name.
        </p>
        <div style={codeHint}>
          <code>{'import iconNodes from "lucide-static/icon-nodes.json";'}</code>
          <br />
          <code>{"<IconProvider icons={iconNodes}> ... </IconProvider>"}</code>
        </div>
        <IconProvider icons={iconNodes as unknown as IconSet}>
          <div style={iconGrid}>
            {allIconNames.map((name) => (
              <button
                key={name}
                style={copiedName === name ? iconCellCopied : iconCell}
                onClick={() => copyName(name)}
                title={name}
              >
                <Icon icon={name} />
                <span style={iconName}>{name}</span>
              </button>
            ))}
          </div>
        </IconProvider>
      </SpecimenGroup>
    </SpecimenLayout>
  );
}
