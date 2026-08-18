import type { ReactNode } from "react";
import { SpecimenGroup } from "../SpecimenGroup";

export function AudioSpecimenPage({ children }: { children: ReactNode }) {
  return <div style={{ display: "grid", gap: "1.5rem" }}>{children}</div>;
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
    <SpecimenGroup label="Sizes — xs / sm / md / lg / xl"><AudioSpecimenRow>{sizes.map((size) => <AxisSample key={size} label={size}>{render({ size }, `${size} size`)}</AxisSample>)}</AudioSpecimenRow></SpecimenGroup>
    <SpecimenGroup label="Densities — compact / default / comfortable"><AudioSpecimenRow>{densities.map((density) => <AxisSample key={density} label={density}>{render({ density }, `${density} density`)}</AxisSample>)}</AudioSpecimenRow></SpecimenGroup>
  </>;
}
