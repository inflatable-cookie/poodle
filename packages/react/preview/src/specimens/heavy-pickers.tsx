import { useState } from "react";
import {
  ColorPicker,
  FileUpload,
  RelationPicker,
  type DrillDownConfig,
  type PickerItem,
} from "@poodle/react";
import { registerSpecimen, SpecimenSection } from "../harness";

const candidates: PickerItem[] = [
  { id: "u1", label: "Ada Lovelace", description: "Engineering", meta: "London" },
  { id: "u2", label: "Grace Hopper", description: "Compilers", meta: "New York" },
  { id: "u3", label: "Alan Turing", description: "Research", meta: "Manchester", disabled: true },
  { id: "u4", label: "Katherine Johnson", description: "Trajectories", meta: "Hampton" },
];

const drillConfig: DrillDownConfig = {
  levels: [
    {
      key: "workspace",
      label: "Workspaces",
      items: [
        { id: "w1", label: "Marketing", count: 3 },
        { id: "w2", label: "Engineering", count: 2 },
      ],
    },
  ],
  finalItems: (query, context) =>
    candidates
      .filter(() => context.workspace === "w2")
      .filter((item) => item.label.toLowerCase().includes(query.toLowerCase())),
};

function HeavyPickersDemo() {
  const [lastEvent, setLastEvent] = useState("");
  const [color, setColor] = useState("#6366f1");
  const [selectedIds, setSelectedIds] = useState<string[]>(["u1"]);

  return (
    <>
      <SpecimenSection title="ColorPicker">
        <ColorPicker
          value={color}
          swatches={["#ef4444", "#f59e0b", "#22c55e", "#6366f1"]}
          onChange={(value) => {
            setColor(value);
            setLastEvent(`color:${value}`);
          }}
        />
        <p data-testid="color-value">{color}</p>
      </SpecimenSection>

      <SpecimenSection title="FileUpload">
        <FileUpload
          accept="image/*,.pdf"
          multiple
          maxSize={5 * 1024 * 1024}
          onUpload={(files) => setLastEvent(`upload:${files.map((f) => f.name).join(",")}`)}
          onError={(event) => setLastEvent(`upload-error:${event.message}`)}
          onRemove={(item) => setLastEvent(`remove:${item.file.name}`)}
        />
      </SpecimenSection>

      <SpecimenSection title="RelationPicker (flat, multiple)">
        <RelationPicker
          title="Assign reviewers"
          description="Pick teammates for this review"
          items={candidates}
          selectedIds={selectedIds}
          onSelectionChange={(ids) => {
            setSelectedIds(ids);
            setLastEvent(`relation:${ids.join(",") || "none"}`);
          }}
          onConfirm={(ids) => setLastEvent(`confirm:${ids.join(",")}`)}
          onCancel={() => setLastEvent("relation:cancel")}
          footerNote="Reviewers get notified."
        />
      </SpecimenSection>

      <SpecimenSection title="RelationPicker (drill-down)">
        <RelationPicker
          title="Pick from workspace"
          drillDown={drillConfig}
          showFooter={false}
          onSelectionChange={(ids) => setLastEvent(`drill-pick:${ids.join(",")}`)}
        />
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
  slug: "heavy-pickers",
  title: "ColorPicker / FileUpload / RelationPicker",
  render: () => <HeavyPickersDemo />,
});
