import { useState, type ReactNode } from "react";
import { Surface, Tabs, type TabItem } from "@inflatable-cookie/poodle-react";
import type { ControlDensity, ControlSize } from "@inflatable-cookie/poodle-react";

const CONTROL_SIZES: ControlSize[] = ["xs", "sm", "md", "lg", "xl"];
const CONTROL_DENSITIES: ControlDensity[] = ["compact", "default", "comfortable"];

export interface SpecimenLayoutProps {
  activeTab?: "examples" | "sizes" | "densities";
  bareVariants?: boolean;
  variantDirection?: "row" | "column";
  showSizes?: boolean;
  showDensities?: boolean;
  children?: ReactNode;
  sizes?: (size: ControlSize) => ReactNode;
  densities?: (density: ControlDensity) => ReactNode;
}

export function SpecimenLayout({
  activeTab: initialTab = "examples",
  bareVariants = false,
  variantDirection = "column",
  showSizes = true,
  showDensities = true,
  children,
  sizes,
  densities,
}: SpecimenLayoutProps) {
  const [activeTab, setActiveTab] = useState<"examples" | "sizes" | "densities">(initialTab);

  const tabs: TabItem[] = [
    { value: "examples", label: "Examples" },
    ...(showSizes ? [{ value: "sizes", label: "Sizes" }] : []),
    ...(showDensities ? [{ value: "densities", label: "Densities" }] : []),
  ];

  const variants = (nodes: ReactNode) =>
    bareVariants ? (
      <div className="poodle-specimen-layout__variants" data-direction={variantDirection}>
        {nodes}
      </div>
    ) : (
      <Surface tone="panel" border="subtle" padding="md">
        <div className="poodle-specimen-layout__variants" data-direction={variantDirection}>
          {nodes}
        </div>
      </Surface>
    );

  return (
    <div className="poodle-specimen-layout">
      <Tabs
        value={activeTab}
        items={tabs}
        variant="text"
        ariaLabel="Specimen view"
        onValueChange={(value) => setActiveTab(value as typeof activeTab)}
      />

      <div className="poodle-specimen-layout__content">
        {activeTab === "examples"
          ? children
          : activeTab === "sizes" && showSizes && sizes
            ? variants(CONTROL_SIZES.map((size) => <span key={size}>{sizes(size)}</span>))
            : activeTab === "densities" && showDensities && densities
              ? variants(CONTROL_DENSITIES.map((density) => <span key={density}>{densities(density)}</span>))
              : null}
      </div>
    </div>
  );
}
