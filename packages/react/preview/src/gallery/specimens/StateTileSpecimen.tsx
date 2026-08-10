import { StateTile } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";

export function StateTileSpecimen() {
  return <div className="poodle-specimen">
    <SpecimenGroup label="States">
      <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(11rem, 1fr))", gap: "0.75rem" }}>
        <StateTile label="Queued jobs" value="14" />
        <StateTile label="Success rate" value="99.8%" trend="up" trendLabel="Up 1.2%" />
        <StateTile label="Errors" value="3" trend="down" trendLabel="Down 4" />
        <StateTile label="Capacity" value="72%" trend="steady" hasSparkline sparkline={<span aria-label="Stable over seven days">▁▂▂▃▂▃▃</span>} />
      </div>
    </SpecimenGroup>
  </div>;
}
