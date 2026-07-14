import { useState } from "react";
import {
  Card,
  CardRadioGroup,
  CardToggleGroup,
  Icon,
  MetricTile,
  NavCard,
  Pill,
  StatusBar,
  Surface,
  type CardRadioItem,
  type CardToggleItem,
} from "@poodle/react";
import { registerSpecimen, SpecimenSection } from "../harness";

const radioItems: CardRadioItem[] = [
  { value: "starter", label: "Starter", description: "For personal projects" },
  { value: "team", label: "Team", description: "Shared workspaces and roles" },
  { value: "enterprise", label: "Enterprise", description: "SSO and audit logs", disabled: true },
  { value: "custom", label: "Custom", description: "Talk to us" },
];

const toggleItems: CardToggleItem[] = [
  { value: "all", label: "All", count: 128 },
  { value: "open", label: "Open", count: 12, description: "Awaiting triage" },
  { value: "closed", label: "Closed", count: 116 },
];

function CardsTilesDemo() {
  const [lastEvent, setLastEvent] = useState("");
  const [plan, setPlan] = useState<string>("starter");
  const [filter, setFilter] = useState<string | null>("all");

  return (
    <>
      <SpecimenSection title="Card">
        <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(14rem, 1fr))", gap: "0.75rem" }}>
          <Card header={<strong>Default card</strong>} footer={<Pill tone="info">Footer</Pill>}>
            Body content resolves from tokens.
          </Card>
          <Card variant="elevated" interactive ariaLabel="Elevated card">
            Elevated interactive card.
          </Card>
          <Card variant="outlined" selected ariaLabel="Selected card">
            Outlined selected card.
          </Card>
        </div>
      </SpecimenSection>

      <SpecimenSection title="Surface">
        <Surface tone="elevated" border="default" elevated asRole="region" label="Elevated surface">
          <p style={{ margin: 0 }}>Elevated surface region.</p>
        </Surface>
      </SpecimenSection>

      <SpecimenSection title="NavCard">
        <div style={{ display: "flex", flexDirection: "column", gap: "0.5rem", maxWidth: "24rem" }}>
          <NavCard
            title="Getting started"
            description="Install and wire your first component"
            badge="New"
            icon={<Icon name="book-open" />}
            onClick={() => setLastEvent("navcard:getting-started")}
          />
          <NavCard title="API reference" href="#api" icon={<Icon name="code" />} />
          <NavCard title="Legacy guide" description="Archived" disabled />
        </div>
      </SpecimenSection>

      <SpecimenSection title="MetricTile">
        <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(10rem, 1fr))", gap: "0.75rem" }}>
          <MetricTile label="Revenue" value="$48.2k" trend="up" trendLabel="+12%" sparklineData={[3, 5, 4, 7, 6, 9, 11]} />
          <MetricTile label="Churn" value="2.1%" trend="down" trendLabel="-0.4%" />
          <MetricTile label="Sessions" value="8,204" trend="flat" trendLabel="±0%" />
        </div>
      </SpecimenSection>

      <SpecimenSection title="CardRadioGroup">
        <CardRadioGroup
          items={radioItems}
          value={plan}
          columns={2}
          ariaLabel="Plan"
          onValueChange={(value) => {
            setPlan(value);
            setLastEvent(`plan:${value}`);
          }}
        />
      </SpecimenSection>

      <SpecimenSection title="CardToggleGroup">
        <CardToggleGroup
          items={toggleItems}
          value={filter}
          columns={3}
          allowDeactivation
          ariaLabel="Filter"
          onValueChange={(value) => {
            setFilter(value);
            setLastEvent(`filter:${value ?? "none"}`);
          }}
        />
      </SpecimenSection>

      <SpecimenSection title="StatusBar">
        <StatusBar summary="3 items selected" trailing={<Pill tone="success">Synced</Pill>} />
        <StatusBar chrome ariaLabel="App status" leading={<span>Ready</span>} trailing={<span>UTF-8</span>} />
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
  slug: "cards-tiles",
  title: "Card / NavCard / MetricTile / StatusBar",
  render: () => <CardsTilesDemo />,
});
