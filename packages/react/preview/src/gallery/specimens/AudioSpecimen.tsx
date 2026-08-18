import type { ReactNode } from "react";

export function AudioSpecimenPage({ children }: { children: ReactNode }) {
  return <div style={{ display: "grid", gap: "1.5rem" }}>{children}</div>;
}

export function AudioSpecimenRow({ children }: { children: ReactNode }) {
  return <div style={{ display: "flex", alignItems: "center", gap: "1rem", flexWrap: "wrap" }}>{children}</div>;
}
