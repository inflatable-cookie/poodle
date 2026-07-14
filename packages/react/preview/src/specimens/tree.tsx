import { useState } from "react";
import { Tree, type TreeNode } from "@poodle/react";
import { registerSpecimen, SpecimenSection } from "../harness";

const NODES: TreeNode[] = [
  {
    value: "src",
    label: "src",
    isBranch: true,
    children: [
      { value: "main.ts", label: "main.ts" },
      { value: "util.ts", label: "util.ts" },
      {
        value: "lib",
        label: "lib",
        isBranch: true,
        children: [{ value: "core.ts", label: "core.ts" }],
      },
    ],
  },
  { value: "readme", label: "README.md" },
  { value: "hidden", label: "secrets.env", isDisabled: true },
];

function TreeSpecimen() {
  const [selected, setSelected] = useState<string[]>([]);
  const [checked, setChecked] = useState<string[]>([]);
  const [activated, setActivated] = useState("");
  return (
    <SpecimenSection title="Tree">
      <Tree
        nodes={NODES}
        defaultExpandedValues={["src"]}
        selectedValues={selected}
        onSelectionChange={setSelected}
        showCheckboxes
        checkedValues={checked}
        onCheckedChange={setChecked}
        onActivate={setActivated}
        ariaLabel="Files"
      />
      <span data-testid="tree-selected">sel: {selected.join(",")}</span>
      <span data-testid="tree-checked">chk: {checked.join(",")}</span>
      <span data-testid="tree-activated">act: {activated}</span>
    </SpecimenSection>
  );
}
registerSpecimen({ slug: "tree", title: "Tree", render: () => <TreeSpecimen /> });
