<script lang="ts">
  import { ChangedFiles, type ChangedFile } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

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

  const scopes: ChangedFile[] = [
    { path: "cp-api/src/main.rs", additions: 4, deletions: 1 },
    { path: "cp-docs/book.md", additions: 2, deletions: 0 },
    { path: "packages/core/index.ts", additions: 8, deletions: 3 },
    { path: "apps/preview/App.svelte", additions: 5, deletions: 1 },
    { path: "tools/export.ts", additions: 1, deletions: 0 },
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

  let workedExpanded = $state(false);
</script>

<SpecimenLayout>
  <SpecimenGroup
    label="Worked change set"
    description="Collapsed gives scope counts and a few chips; expanded gives the tree with counts rolled up from descendants."
  >
    <ChangedFiles id="worked" files={worked} bind:expanded={workedExpanded} />
    <ChangedFiles id="worked-open" files={worked} expanded />
  </SpecimenGroup>

  <SpecimenGroup
    label="Paths and scopes"
    description="A path with no forks costs one row, not four. Files across several scopes keep their scope chips."
  >
    <ChangedFiles id="deep" files={deep} expanded />
    <ChangedFiles id="scopes" files={scopes} />
  </SpecimenGroup>

  <SpecimenGroup label="Count variations" description="A single file, additions only, and deletions only.">
    <ChangedFiles id="single" files={single} />
    <ChangedFiles id="adds" files={additionsOnly} />
    <ChangedFiles id="dels" files={deletionsOnly} />
  </SpecimenGroup>

  <SpecimenGroup
    label="Overflow and actions"
    description="Long filenames ellipsise in their chip; files beyond the limit hide behind 'Show all'. The diff action can be withheld."
  >
    <ChangedFiles id="long" files={longName} chipLimit={2} />
    <ChangedFiles id="nodiff" files={single} showOpenDiff={false} />
  </SpecimenGroup>

  {#snippet sizes(size)}
    <ChangedFiles id={`sz-${size}`} files={worked} {size} />
  {/snippet}

  {#snippet densities(density)}
    <ChangedFiles id={`dn-${density}`} files={worked} {density} />
  {/snippet}
</SpecimenLayout>
