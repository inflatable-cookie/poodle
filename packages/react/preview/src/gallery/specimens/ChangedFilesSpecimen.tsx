import { ChangedFiles, type ChangedFile } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

// The worked example from the reference design: 9 files, +376 −16.
const worked: ChangedFile[] = [
  { path: "cp-api/Cargo.toml", additions: 1, deletions: 0 },
  { path: "cp-api/crates/latex/src/lexer.rs", additions: 140, deletions: 6 },
  { path: "cp-api/crates/latex/src/parser.rs", additions: 131, deletions: 4 },
  { path: "cp-api/tools/export_fixture.rs", additions: 60, deletions: 1 },
  { path: "cp-api/tools/build.rs", additions: 29, deletions: 0 },
  { path: "cp-api/effigy.toml", additions: 1, deletions: 0 },
  { path: "cp-api/crates/latex/src/tests/lexer_tests.rs", additions: 0, deletions: 0 },
  { path: "cp-docs/book-port-and-production.md", additions: 14, deletions: 5 },
  { path: "cp-docs/notes.md", additions: 1, deletions: 0 },
];

// A chain with no forks collapses to one row rather than an indentation
// staircase four levels deep.
const deep: ChangedFile[] = [
  { path: "app/src/lib/features/editor/state/machine.ts", additions: 12, deletions: 3 },
];

const single: ChangedFile[] = [{ path: "README.md", additions: 4, deletions: 1 }];
const additionsOnly: ChangedFile[] = [{ path: "src/new.ts", additions: 88, deletions: 0 }];
const deletionsOnly: ChangedFile[] = [{ path: "src/old.ts", additions: 0, deletions: 45 }];
const longName: ChangedFile[] = [
  { path: "packages/contracts/components/src/a_very_long_component_spec_name.rs", additions: 3, deletions: 2 },
  { path: "b.rs", additions: 1, deletions: 0 },
  { path: "c.rs", additions: 1, deletions: 0 },
  { path: "d.rs", additions: 1, deletions: 0 },
];

export function ChangedFilesSpecimen() {
  return (
    <SpecimenLayout
      sizes={(size) => <ChangedFiles id={`sz-${size}`} files={worked} size={size} />}
      densities={(density) => <ChangedFiles id={`dn-${density}`} files={worked} density={density} />}
    >
      <SpecimenGroup
        label="Collapsed and expanded"
        description="Collapsed gives scope counts and a few chips; expanded gives the tree with counts rolled up from descendants."
      >
        <ChangedFiles id="worked" files={worked} />
      </SpecimenGroup>

      <SpecimenGroup label="Chain collapse" description="A path with no forks costs one row, not four.">
        <ChangedFiles id="deep" files={deep} expanded />
      </SpecimenGroup>

      <SpecimenGroup label="Single file">
        <ChangedFiles id="single" files={single} />
      </SpecimenGroup>

      <SpecimenGroup label="One-sided counts">
        <ChangedFiles id="adds" files={additionsOnly} />
        <ChangedFiles id="dels" files={deletionsOnly} />
      </SpecimenGroup>

      <SpecimenGroup
        label="Truncation and overflow"
        description="Long filenames ellipsise in their chip; files beyond the limit hide behind 'Show all'."
      >
        <ChangedFiles id="long" files={longName} />
      </SpecimenGroup>

      <SpecimenGroup label="Without the diff action">
        <ChangedFiles id="nodiff" files={single} showOpenDiff={false} />
      </SpecimenGroup>

      <SpecimenGroup
        label="Empty"
        description="No files renders nothing at all. A turn that changed nothing should not have a box saying so."
      >
        <ChangedFiles id="empty" files={[]} />
      </SpecimenGroup>
    </SpecimenLayout>
  );
}
