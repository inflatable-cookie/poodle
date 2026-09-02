<script lang="ts">
  import { AgentTranscript } from "@inflatable-cookie/poodle-svelte/markdown";
  import type { TranscriptItem } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  const call = (id: string, detail: string, status: "running" | "success" | "error" = "success"): TranscriptItem => ({
    kind: "tool-call",
    id,
    label: "Ran command",
    detail,
    status,
  });

  const message = (id: string, markdown: string): TranscriptItem => ({
    kind: "message",
    id,
    role: "assistant",
    markdown,
  });

  // The worked turn from the reference design. Note the changed-files card
  // splits the surrounding commands into two runs rather than being absorbed:
  // they happened either side of an edit, not as one stretch of work.
  const turn: TranscriptItem[] = [
    message("m1", "The latest fixes hold: 41 parser tests pass. AO415 now matches in text and structure; its only remaining delta is an extra CSS class on `<sup>`."),
    call("t1", "effigy cp-api/test:latex"),
    call("t2", "nl -ba cp-api/crates/latex/src/lexer.rs"),
    call("t3", "sed -n '145,175p' lexer.rs"),
    message("m2", "AO415 and RO418 now both reach full semantic parity, taking the verified set to at least 25 of 95 before a fresh sweep."),
    call("t4", "cargo test -p latex"),
    call("t5", "effigy cp-api/test:latex"),
    {
      kind: "changed-files",
      id: "diff",
      files: [
        { path: "cp-api/crates/latex/src/lexer.rs", additions: 271, deletions: 10 },
        { path: "cp-api/tools/export_fixture.rs", additions: 89, deletions: 1 },
        { path: "cp-api/effigy.toml", additions: 1, deletions: 0 },
        { path: "cp-docs/book-port.md", additions: 15, deletions: 5 },
      ],
    },
    call("t6", "jq -r .body_html /tmp/G0216.legacy.json"),
    { kind: "activity", id: "act", label: "Working for 1h 1m" },
  ];

  const simple: TranscriptItem[] = [
    message("s0", "Running the gate."),
    call("s1", "effigy check:gpui"),
  ];

  const thirty: TranscriptItem[] = [
    message("t30m", "Running the remaining checks."),
    ...Array.from({ length: 30 }, (_, index) => call(`t30-${index + 1}`, `check ${index + 1}`)),
  ];

  const streaming: TranscriptItem[] = [
    message("st1", "Reading the parser now"),
    { kind: "message", id: "st2", role: "assistant", markdown: "The corpus-wide patterns were genuine legacy", isStreaming: true },
  ];

  const withFailure: TranscriptItem[] = [
    message("f0", "Running the gate."),
    call("f1", "cargo check"),
    call("f2", "effigy check:gpui", "error"),
    call("f3", "bun test"),
  ];

  // Long enough that windowing matters; each block is a different height, which
  // is the case a uniform-row virtualizer cannot handle.
  const long: TranscriptItem[] = Array.from({ length: 120 }, (_, i) =>
    i % 3 === 0
      ? message(`lm${i}`, `Block ${i}. ${"A sentence that makes this block taller than a tool row. ".repeat((i % 4) + 1)}`)
      : call(`lc${i}`, `step ${i} of a long session`),
  );

  let turnRuns = $state<string[]>([]);
  let turnCalls = $state<string[]>([]);
  let turnFiles = $state<string[]>([]);
  let thirtyExpanded = $state<string[]>(["t30-1"]);
</script>

<SpecimenLayout>
  <SpecimenGroup label="A worked turn" description="A realistic turn: messages, tool runs, changed files, and the activity footer.">
    <div style="height: 30rem; border: 1px solid var(--poodle-color-border-subtle); border-radius: var(--poodle-radius-surface);">
      <AgentTranscript
        items={turn}
        virtualized={false}
        bind:expandedToolRuns={turnRuns}
        bind:expandedToolCalls={turnCalls}
        bind:expandedChangedFiles={turnFiles}
      />
    </div>
  </SpecimenGroup>

  <SpecimenGroup
    label="Tool run states"
    description="Contiguous calls collapse into one run. Expand a thirty-call run to read every row; a failure anywhere marks the whole run."
  >
    <div style="height: 10rem; border: 1px solid var(--poodle-color-border-subtle); border-radius: var(--poodle-radius-surface);">
      <AgentTranscript items={simple} virtualized={false} />
    </div>
    <div style="height: 14rem; border: 1px solid var(--poodle-color-border-subtle); border-radius: var(--poodle-radius-surface);">
      <AgentTranscript items={thirty} virtualized={false} />
    </div>
    <div style="height: 22rem; border: 1px solid var(--poodle-color-border-subtle); border-radius: var(--poodle-radius-surface);">
      <AgentTranscript items={thirty} virtualized={false} bind:expandedToolRuns={thirtyExpanded} />
    </div>
    <div style="height: 14rem; border: 1px solid var(--poodle-color-border-subtle); border-radius: var(--poodle-radius-surface);">
      <AgentTranscript items={withFailure} virtualized={false} />
    </div>
  </SpecimenGroup>

  <SpecimenGroup
    label="Streaming and detached scroll"
    description="The caret marks a message still arriving. Scroll away from the bottom to reveal jump-to-latest."
  >
    <div style="height: 12rem; border: 1px solid var(--poodle-color-border-subtle); border-radius: var(--poodle-radius-surface);">
      <AgentTranscript items={streaming} virtualized={false} />
    </div>
    <div style="height: 16rem; border: 1px solid var(--poodle-color-border-subtle); border-radius: var(--poodle-radius-surface);">
      <AgentTranscript items={long} virtualized={false} />
    </div>
  </SpecimenGroup>

  <SpecimenGroup
    label="Long transcript rendering"
    description="The same mixed-height content, windowed and unwindowed."
  >
    <div style="height: 26rem; border: 1px solid var(--poodle-color-border-subtle); border-radius: var(--poodle-radius-surface);">
      <AgentTranscript items={long} />
    </div>
    <div style="height: 26rem; border: 1px solid var(--poodle-color-border-subtle); border-radius: var(--poodle-radius-surface);">
      <AgentTranscript items={long} virtualized={false} />
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Empty">
    <div style="height: 10rem; border: 1px solid var(--poodle-color-border-subtle); border-radius: var(--poodle-radius-surface);">
      <AgentTranscript items={[]} />
    </div>
  </SpecimenGroup>

  {#snippet sizes(size)}
    <div style="height: 12rem; border: 1px solid var(--poodle-color-border-subtle); border-radius: var(--poodle-radius-surface);">
      <AgentTranscript items={withFailure} virtualized={false} {size} />
    </div>
  {/snippet}

  {#snippet densities(density)}
    <div style="height: 12rem; border: 1px solid var(--poodle-color-border-subtle); border-radius: var(--poodle-radius-surface);">
      <AgentTranscript items={withFailure} virtualized={false} {density} />
    </div>
  {/snippet}
</SpecimenLayout>
