import { useEffect, useState, type ReactNode } from "react";
import { Surface, Tabs, type TabItem } from "@inflatable-cookie/poodle-react";
import type { ControlDensity, ControlSize } from "@inflatable-cookie/poodle-react";

const DEFAULT_CONTROL_SIZES: ControlSize[] = ["xs", "sm", "md", "lg", "xl"];
const DEFAULT_CONTROL_DENSITIES: ControlDensity[] = ["compact", "default", "comfortable"];

export interface SpecimenLayoutProps {
  activeTab?: "examples" | "sizes" | "densities";
  bareVariants?: boolean;
  variantDirection?: "row" | "column";
  showSizes?: boolean;
  showDensities?: boolean;
  children?: ReactNode;
  sizes?: (size: string) => ReactNode;
  densities?: (density: string) => ReactNode;
  sizeValues?: readonly string[];
  densityValues?: readonly string[];
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
  sizeValues = DEFAULT_CONTROL_SIZES,
  densityValues = DEFAULT_CONTROL_DENSITIES,
}: SpecimenLayoutProps) {
  const [activeTab, setActiveTab] = useState<"examples" | "sizes" | "densities">(initialTab);

  const tabs: TabItem[] = [
    { value: "examples", label: "Examples" },
    ...(showSizes && sizes ? [{ value: "sizes", label: "Sizes" }] : []),
    ...(showDensities && densities ? [{ value: "densities", label: "Densities" }] : []),
  ];

  const tabKey = tabs.map((tab) => tab.value).join("|");
  useEffect(() => {
    const values = tabKey.split("|");
    setActiveTab((current) => (values.includes(current) ? current : "examples"));
  }, [tabKey]);

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
        variant="card"
        ariaLabel="Specimen view"
        onValueChange={(value) => setActiveTab(value as typeof activeTab)}
      />

      <div className="poodle-specimen-layout__content">
        {activeTab === "examples"
          ? children
          : activeTab === "sizes" && showSizes && sizes
            ? variants(
                sizeValues.map((size) => (
                  <div key={size} className="poodle-specimen-layout__variant" data-axis-size={size}>
                    {sizes(size)}
                  </div>
                )),
              )
            : activeTab === "densities" && showDensities && densities
              ? variants(
                  densityValues.map((density) => (
                    <div
                      key={density}
                      className="poodle-specimen-layout__variant"
                      data-axis-density={density}
                    >
                      {densities(density)}
                    </div>
                  )),
                )
              : null}
      </div>
    </div>
  );
}
