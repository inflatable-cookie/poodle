import { useState } from "react";
import { Tabs, type TabItem } from "@inflatable-cookie/poodle-react";
import {
  projectCorpus,
  tabsCases,
  tabsInterface,
  type ProjectedInstance,
} from "@inflatable-cookie/poodle-core/conformance";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

// g14.004: the shared corpus owns groups, fixtures, collection order, and axes.
const groups = projectCorpus(tabsCases, tabsInterface);
const residualItems: TabItem[] = [
  { value: "editor", label: "Editor", icon: "code" },
  { value: "preview", label: "Preview", icon: "eye", count: 12, separator: true },
  { value: "terminal", label: "Terminal", icon: "terminal", closable: true },
];

function propsOf(instance: ProjectedInstance): Record<string, unknown> {
  const props: Record<string, unknown> = { ...instance.props };
  for (const key of Object.keys(props)) {
    if (props[key] === null) delete props[key];
  }
  return props;
}

function ProjectedTabs({ instance, onChange }: {
  instance: ProjectedInstance;
  onChange: (value: string) => void;
}) {
  const props = propsOf(instance);
  const [value, setValue] = useState(String(props.value ?? props.defaultValue ?? ""));
  return <Tabs {...(props as never)} value={value} onValueChange={(next) => {
    setValue(next);
    onChange(next);
  }} />;
}

export function TabsSpecimen() {
  const [valueLog, setValueLog] = useState("No tab change yet.");

  return (
    <SpecimenLayout>
      {groups.map((group) => (
        <SpecimenGroup key={group.label} label={group.label}>
          {group.instances.map((instance) => (
            <div
              key={instance.caseId + instance.caption}
              style={{ display: "flex", alignItems: "center", gap: "0.75rem" }}
            >
              <span
                style={{
                  color: "var(--poodle-color-text-secondary, #c9d4e0)",
                  fontSize: "0.75rem",
                  minWidth: "12rem",
                }}
              >
                {instance.caption}
              </span>
              <ProjectedTabs
                instance={instance}
                onChange={(value) => setValueLog(`${instance.caption}: ${value}`)}
              />
            </div>
          ))}
        </SpecimenGroup>
      ))}
      <SpecimenGroup label="Residual visual and operator coverage">
        <div style={{ resize: "horizontal", overflow: "auto", width: "24rem", minWidth: "12rem" }}>
          <Tabs items={residualItems} overflowStrategy="shed" collapseWhenOverflow ariaLabel="Overflow shedding" />
        </div>
        <Tabs items={residualItems} variant="card" activeEdge="outline" activeFill="solid" defaultValue="editor" reorderable onClose={(value) => setValueLog(`Closed: ${value}`)} ariaLabel="Closable files" />
        <Tabs items={residualItems} variant="block" activeEdge="underline" activeFill="none" fullWidth defaultValue="editor" ariaLabel="Full-width workspace">
          {(activeValue) => <p>Panel: {activeValue}</p>}
        </Tabs>
        <Tabs items={residualItems} variant="pill" defaultValue="editor" size="lg" density="comfortable" ariaLabel="Large comfortable tabs" />
      </SpecimenGroup>
      <SpecimenGroup label="Interaction" bare>
        <span style={{ fontSize: "0.75rem" }}>{valueLog}</span>
      </SpecimenGroup>
    </SpecimenLayout>
  );
}
