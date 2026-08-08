import type { CSSProperties } from "react";
import { TimeAgo } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";

const rowStyle: CSSProperties = {
  display: "flex",
  alignItems: "baseline",
  gap: "0.5rem",
};

const labelStyle: CSSProperties = {
  fontSize: "0.8125rem",
  color: "var(--poodle-color-text-secondary)",
  minWidth: "8rem",
};

export function TimeAgoSpecimen() {
  const now = Date.now();
  const twoMinutesAgo = new Date(now - 2 * 60 * 1000);
  const threeHoursAgo = new Date(now - 3 * 60 * 60 * 1000);
  const twoDaysAgo = new Date(now - 2 * 24 * 60 * 60 * 1000);
  const inFiveMinutes = new Date(now + 5 * 60 * 1000);

  return (
    <div className="poodle-specimen">
      <SpecimenGroup label="Recent timestamps">
        <div style={rowStyle}>
          <span style={labelStyle}>2 minutes ago:</span>
          <TimeAgo datetime={twoMinutesAgo} />
        </div>
        <div style={rowStyle}>
          <span style={labelStyle}>3 hours ago:</span>
          <TimeAgo datetime={threeHoursAgo} />
        </div>
        <div style={rowStyle}>
          <span style={labelStyle}>2 days ago:</span>
          <TimeAgo datetime={twoDaysAgo} />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Future timestamp">
        <div style={rowStyle}>
          <span style={labelStyle}>In 5 minutes:</span>
          <TimeAgo datetime={inFiveMinutes} />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Long format">
        <div style={rowStyle}>
          <span style={labelStyle}>2 minutes ago:</span>
          <TimeAgo datetime={twoMinutesAgo} short={false} />
        </div>
        <div style={rowStyle}>
          <span style={labelStyle}>2 days ago:</span>
          <TimeAgo datetime={twoDaysAgo} short={false} />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Static (live updates off)">
        <TimeAgo datetime={twoMinutesAgo} live={false} />
      </SpecimenGroup>

      <SpecimenGroup label="Inherited typography">
        <p className="poodle-specimen__inline-copy">
          Activity finished <TimeAgo datetime={twoMinutesAgo} typography="inherit" /> in running prose.
        </p>
      </SpecimenGroup>

      <SpecimenGroup label="From ISO string">
        <TimeAgo datetime="2026-03-14T00:00:00Z" />
      </SpecimenGroup>
    </div>
  );
}
