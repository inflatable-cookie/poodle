import type { CSSProperties } from "react";
import { Icon, IconProvider, type IconSet } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import iconNodes from "lucide-static/icon-nodes.json";

// A small custom icon set to demonstrate swappability
const customIcons: IconSet = {
  "check": [["path", { "d": "M20 6 9 17l-5-5" }]],
  "star": [["polygon", { "points": "12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" }]],
  "heart": [["path", { "d": "M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z" }]],
  "x": [["path", { "d": "M18 6 6 18" }], ["path", { "d": "m6 6 12 12" }]],
};

const sampleNames = ["rocket", "flame", "shield-check", "globe", "compass", "anchor", "cpu", "database"];

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
const iconRow: CSSProperties = { display: "flex", gap: "1rem", alignItems: "flex-start", flexWrap: "wrap" };
const labeledIcon: CSSProperties = {
  display: "flex",
  flexDirection: "column",
  alignItems: "center",
  gap: "0.25rem",
  minWidth: "4rem",
};
const label: CSSProperties = {
  fontSize: "0.5625rem",
  fontFamily: "var(--poodle-typography-code-family)",
  color: "var(--poodle-color-text-muted)",
  textAlign: "center",
  wordBreak: "break-all",
  lineHeight: 1.3,
};

export function IconProviderSpecimen() {
  return (
    <div className="poodle-specimen">
      <SpecimenGroup label="Full Lucide set — catalogue tools only">
        <p style={hint}>
          Wrap a subtree in <code style={hintCode}>{"<IconProvider icons={iconNodes}>"}</code> to
          make all {Object.keys(iconNodes).length} Lucide icons available by name.
          This full-catalogue form is for icon browsers, not application bundles.
          Icons resolve from this set first, then Poodle's default Lucide set.
        </p>
        <IconProvider icons={iconNodes as unknown as IconSet}>
          <div style={iconRow}>
            {sampleNames.map((name) => (
              <div key={name} style={labeledIcon}>
                <Icon icon={name} />
                <span style={label}>{name}</span>
              </div>
            ))}
          </div>
        </IconProvider>
      </SpecimenGroup>

      <SpecimenGroup label="Custom icon set">
        <p style={hint}>
          Any <code style={hintCode}>Record&lt;string, IconNodes&gt;</code> works as an icon set.
          You can supply a Phosphor equivalent, a subset, or your own custom icons.
        </p>
        <IconProvider icons={customIcons}>
          <div style={iconRow}>
            {Object.keys(customIcons).map((name) => (
              <div key={name} style={labeledIcon}>
                <Icon icon={name} />
                <span style={label}>{name}</span>
              </div>
            ))}
          </div>
        </IconProvider>
      </SpecimenGroup>

      <SpecimenGroup label="Without IconProvider — default Lucide set">
        <p style={hint}>
          Without any <code style={hintCode}>IconProvider</code>, Poodle's component-owned
          string names resolve from its scoped 54-icon Lucide default.
        </p>
        <div style={iconRow}>
          <div style={labeledIcon}>
            <Icon icon="check" />
            <span style={label}>check</span>
          </div>
          <div style={labeledIcon}>
            <Icon icon="chevron-down" />
            <span style={label}>chevron-down</span>
          </div>
          <div style={labeledIcon}>
            <Icon icon="x" />
            <span style={label}>x</span>
          </div>
          <div style={labeledIcon}>
            <Icon icon="search" />
            <span style={label}>search</span>
          </div>
          <div style={labeledIcon}>
            <Icon icon="plus" />
            <span style={label}>plus</span>
          </div>
        </div>
      </SpecimenGroup>
    </div>
  );
}
