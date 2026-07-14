import { useState } from "react";
import {
  Button,
  DetailItem,
  DetailSection,
  DetailSectionGroup,
  DetailShell,
  Pill,
  ScrollShell,
  SplitView,
} from "@poodle/react";
import { registerSpecimen, SpecimenSection } from "../harness";

function WorkstationDemo() {
  const [lastEvent, setLastEvent] = useState("");
  const [ratio, setRatio] = useState(0.4);

  return (
    <>
      <SpecimenSection title="SplitView">
        <div style={{ height: "12rem", border: "1px solid var(--poodle-color-border-subtle)" }}>
          <SplitView
            ratio={ratio}
            showCollapsePrimary
            showCollapseSecondary
            onRatioChange={(next) => {
              setRatio(next);
              setLastEvent(`split:${next.toFixed(2)}`);
            }}
            onPrimaryCollapsedChange={(collapsed) => setLastEvent(`split:primary:${collapsed}`)}
            primary={<div data-testid="split-primary" style={{ padding: "0.5rem" }}>Primary pane</div>}
            secondary={<div data-testid="split-secondary" style={{ padding: "0.5rem" }}>Secondary pane</div>}
          />
        </div>
        <p data-testid="split-ratio">{ratio.toFixed(2)}</p>
      </SpecimenSection>

      <SpecimenSection title="ScrollShell">
        <div style={{ height: "8rem" }}>
          <ScrollShell direction="vertical" padding="sm" focusable label="Log output" onScroll={() => setLastEvent("scrolled")}>
            <div style={{ height: "40rem" }} data-testid="scroll-content">
              Tall content
            </div>
          </ScrollShell>
        </div>
      </SpecimenSection>

      <SpecimenSection title="DetailShell + sections">
        <DetailShell title="Entity detail" ariaLabel="Entity detail">
          <DetailSectionGroup>
            <DetailSection title="Overview" description="Core fields" actions={<Button size="sm" variant="ghost" onClick={() => setLastEvent("section:edit")}>Edit</Button>}>
              <dl style={{ margin: 0, display: "contents" }}>
                <DetailItem label="Name" value="Poodle" />
                <DetailItem label="Status" valueContent={<Pill tone="success">Active</Pill>} />
                <DetailItem label="Owner" value={null} />
                <DetailItem label="Notes" description="Internal notes field" value="Long-running parity effort" truncateValue />
              </dl>
            </DetailSection>
            <DetailSection title="Billing" separated={false}>
              <dl style={{ margin: 0, display: "contents" }}>
                <DetailItem label="Plan" value="Team" layout="stacked" presentation="simple" />
              </dl>
            </DetailSection>
          </DetailSectionGroup>
        </DetailShell>
        <DetailShell title="Loading detail" state="loading" stateTitle="Loading entity" />
        <DetailShell title="Broken detail" state="error" stateTitle="Could not load" stateMessage="Try again later." />
      </SpecimenSection>

      {lastEvent ? (
        <SpecimenSection title="Last event">
          <p data-testid="last-event">{lastEvent}</p>
        </SpecimenSection>
      ) : null}
    </>
  );
}

registerSpecimen({
  slug: "workstation",
  title: "SplitView / ScrollShell / Detail shells",
  render: () => <WorkstationDemo />,
});
