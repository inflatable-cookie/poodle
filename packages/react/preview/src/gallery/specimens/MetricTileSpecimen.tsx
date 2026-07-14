import type { CSSProperties } from "react";
import { MetricTile } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const gridStyle: CSSProperties = {
  display: "grid",
  gridTemplateColumns: "repeat(auto-fit, minmax(10rem, 1fr))",
  gap: "0.75rem",
};

export function MetricTileSpecimen() {
  return (
    <SpecimenLayout
      showSizes={false}
      densities={(density) => (
        <div style={{ width: "min(100%, 14rem)" }}>
          <MetricTile
            label="Requests/min"
            value="1,204"
            trend="up"
            trendLabel="+5%"
            sparklineData={[800, 920, 850, 1100, 980, 1050, 1204]}
            density={density}
          />
        </div>
      )}
    >
      <div className="poodle-specimen">
        <SpecimenGroup label="Basic tiles">
          <div style={gridStyle}>
            <MetricTile label="Components" value="85" />
            <MetricTile label="Coverage" value="94%" />
            <MetricTile label="Open issues" value="12" />
            <MetricTile label="Build time" value="1.8s" />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="With trend indicators">
          <div style={gridStyle}>
            <MetricTile label="Active users" value="2,847" trend="up" trendLabel="+12.3%" />
            <MetricTile label="Error rate" value="0.04%" trend="down" trendLabel="-8%" />
            <MetricTile label="Latency" value="42ms" trend="flat" trendLabel="No change" />
            <MetricTile label="Revenue" value="$14.2k" trend="up" trendLabel="+3.1%" />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="With sparklines">
          <div style={gridStyle}>
            <MetricTile
              label="Requests/min"
              value="1,204"
              trend="up"
              trendLabel="+5%"
              sparklineData={[800, 920, 850, 1100, 980, 1050, 1204]}
            />
            <MetricTile
              label="CPU usage"
              value="62%"
              trend="down"
              trendLabel="-4%"
              sparklineData={[75, 72, 68, 70, 65, 63, 62]}
            />
            <MetricTile
              label="Memory"
              value="4.2 GB"
              sparklineData={[3.8, 3.9, 4.0, 4.1, 4.0, 4.1, 4.2]}
            />
          </div>
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}
