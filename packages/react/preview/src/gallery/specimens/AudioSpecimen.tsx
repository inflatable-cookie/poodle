import type { ReactNode } from "react";

export function AudioSpecimenPage({ children }: { children: ReactNode }) {
  return <div style={{ display: "grid", gap: "1.5rem" }}>{children}</div>;
}

export function AudioSpecimenGroup({ title, children }: { title: string; children: ReactNode }) {
  return <section style={{ display: "grid", gap: ".75rem" }}><h3 style={{ margin: 0, color: "var(--poodle-color-text-secondary)", fontSize: ".75rem" }}>{title}</h3>{children}</section>;
}

export function AudioSpecimenRow({ children }: { children: ReactNode }) {
  return <div style={{ display: "flex", alignItems: "center", gap: "1rem", flexWrap: "wrap" }}>{children}</div>;
}

type AudioAxisProps = {
  size?: "xs" | "sm" | "md" | "lg" | "xl";
  density?: "compact" | "default" | "comfortable";
};

function AxisSample({ label, children }: { label: string; children: ReactNode }) {
  return <div style={{ display: "grid", justifyItems: "center", gap: ".375rem" }}><small>{label}</small>{children}</div>;
}

export function AudioAxes({ render }: { render: (props: AudioAxisProps, label: string) => ReactNode }) {
  const sizes: NonNullable<AudioAxisProps["size"]>[] = ["xs", "sm", "md", "lg", "xl"];
  const densities: NonNullable<AudioAxisProps["density"]>[] = ["compact", "default", "comfortable"];
  return <>
    <AudioSpecimenGroup title="Sizes — xs / sm / md / lg / xl"><AudioSpecimenRow>{sizes.map((size) => <AxisSample key={size} label={size}>{render({ size }, `${size} size`)}</AxisSample>)}</AudioSpecimenRow></AudioSpecimenGroup>
    <AudioSpecimenGroup title="Densities — compact / default / comfortable"><AudioSpecimenRow>{densities.map((density) => <AxisSample key={density} label={density}>{render({ density }, `${density} density`)}</AxisSample>)}</AudioSpecimenRow></AudioSpecimenGroup>
  </>;
}
