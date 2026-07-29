<script lang="ts">
  import { AgentTranscript, type TranscriptItem } from "@poodle/svelte";
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

  const streaming: TranscriptItem[] = [
    message("s1", "Reading the parser now"),
    { kind: "message", id: "s2", role: "assistant", markdown: "The corpus-wide patterns were genuine legacy", isStreaming: true },
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
</script>

<SpecimenLayout>
  <SpecimenGroup title="A worked turn" description="Message, run, message, run, changed files, run, activity. The changed-files card splits the runs on either side of it.">
    <div style="height: 30rem; border: 1px solid var(--poodle-color-border-subtle); border-radius: var(--poodle-radius-surface);">
      <AgentTranscript items={turn} virtualized={false} />
    </div>
  </SpecimenGroup>

  <SpecimenGroup title="Streaming" description="The last message carries its caret while tokens arrive.">
    <div style="height: 12rem; border: 1px solid var(--poodle-color-border-subtle); border-radius: var(--poodle-radius-surface);">
      <AgentTranscript items={streaming} virtualized={false} />
    </div>
  </SpecimenGroup>

  <SpecimenGroup title="A run containing a failure" description="Collapsed, the run still advertises that something in it broke.">
    <div style="height: 14rem; border: 1px solid var(--poodle-color-border-subtle); border-radius: var(--poodle-radius-surface);">
      <AgentTranscript items={withFailure} virtualized={false} />
    </div>
  </SpecimenGroup>

  <SpecimenGroup title="Empty">
    <div style="height: 10rem; border: 1px solid var(--poodle-color-border-subtle); border-radius: var(--poodle-radius-surface);">
      <AgentTranscript items={[]} />
    </div>
  </SpecimenGroup>

  <SpecimenGroup title="Windowed" description="120 blocks of varying height. Scroll up and the jump-to-latest pill appears; following re-arms only when you return to the bottom.">
    <div style="height: 26rem; border: 1px solid var(--poodle-color-border-subtle); border-radius: var(--poodle-radius-surface);">
      <AgentTranscript items={long} />
    </div>
  </SpecimenGroup>

  <SpecimenGroup title="Unwindowed" description="The same content with every block rendered. The result must be identical — only the cost differs.">
    <div style="height: 26rem; border: 1px solid var(--poodle-color-border-subtle); border-radius: var(--poodle-radius-surface);">
      <AgentTranscript items={long} virtualized={false} />
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
