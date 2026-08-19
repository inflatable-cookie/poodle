import { useState } from "react";
import { AgentTranscript, type TranscriptItem } from "@inflatable-cookie/poodle-react";

import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const call = (
  id: string,
  detail: string,
  status: "running" | "success" | "error" = "success",
): TranscriptItem => ({ kind: "tool-call", id, label: "Ran command", detail, status });

const message = (id: string, markdown: string): TranscriptItem => ({
  kind: "message",
  id,
  role: "assistant",
  markdown,
});

const turn: TranscriptItem[] = [
  message(
    "m1",
    "The latest fixes hold: 41 parser tests pass. AO415 now matches in text and structure; its only remaining delta is an extra CSS class on `<sup>`.",
  ),
  call("t1", "effigy cp-api/test:latex"),
  call("t2", "nl -ba cp-api/crates/latex/src/lexer.rs"),
  call("t3", "sed -n '145,175p' lexer.rs"),
  message(
    "m2",
    "AO415 and RO418 now both reach full semantic parity, taking the verified set to at least 25 of 95 before a fresh sweep.",
  ),
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
  {
    kind: "message",
    id: "st2",
    role: "assistant",
    markdown: "The corpus-wide patterns were genuine legacy",
    isStreaming: true,
  },
];

const withFailure: TranscriptItem[] = [
  message("f0", "Running the gate."),
  call("f1", "cargo check"),
  call("f2", "effigy check:gpui", "error"),
  call("f3", "bun test"),
];

// Long enough that windowing matters, with each block a different height —
// the case a uniform-row virtualizer cannot handle.
const long: TranscriptItem[] = Array.from({ length: 120 }, (_, i) =>
  i % 3 === 0
    ? message(
        `lm${i}`,
        `Block ${i}. ${"A sentence that makes this block taller than a tool row. ".repeat((i % 4) + 1)}`,
      )
    : call(`lc${i}`, `step ${i} of a long session`),
);

const frame = {
  height: "26rem",
  border: "1px solid var(--poodle-color-border-subtle)",
  borderRadius: "var(--poodle-radius-surface)",
};

export function AgentTranscriptSpecimen() {
  const [turnRuns, setTurnRuns] = useState<string[]>([]);
  const [turnCalls, setTurnCalls] = useState<string[]>([]);
  const [turnFiles, setTurnFiles] = useState<string[]>([]);
  const [thirtyExpanded, setThirtyExpanded] = useState<string[]>(["t30-1"]);

  return (
    <SpecimenLayout
      sizes={(size) => (
        <div style={{ ...frame, height: "12rem" }}>
          <AgentTranscript items={withFailure} virtualized={false} size={size} />
        </div>
      )}
      densities={(density) => (
        <div style={{ ...frame, height: "12rem" }}>
          <AgentTranscript items={withFailure} virtualized={false} density={density} />
        </div>
      )}
    >
      <SpecimenGroup
        label="A worked turn"
        description="A realistic turn: messages, tool runs, changed files, and the activity footer."
      >
        <div style={{ ...frame, height: "30rem" }}>
          <AgentTranscript
            items={turn}
            virtualized={false}
            expandedToolRuns={turnRuns}
            onToolRunToggle={(id) =>
              setTurnRuns((current) =>
                current.includes(id) ? current.filter((value) => value !== id) : [...current, id],
              )
            }
            expandedToolCalls={turnCalls}
            onToolCallToggle={(id) =>
              setTurnCalls((current) =>
                current.includes(id) ? current.filter((value) => value !== id) : [...current, id],
              )
            }
            expandedChangedFiles={turnFiles}
            onChangedFilesToggle={(id) =>
              setTurnFiles((current) =>
                current.includes(id) ? current.filter((value) => value !== id) : [...current, id],
              )
            }
          />
        </div>
      </SpecimenGroup>

      <SpecimenGroup
        label="Tool run states"
        description="Contiguous calls collapse into one run. Expand a thirty-call run to read every row; a failure anywhere marks the whole run."
      >
        <div style={{ ...frame, height: "10rem" }}>
          <AgentTranscript items={simple} virtualized={false} />
        </div>
        <div style={{ ...frame, height: "14rem" }}>
          <AgentTranscript items={thirty} virtualized={false} />
        </div>
        <div style={{ ...frame, height: "22rem" }}>
          <AgentTranscript
            items={thirty}
            virtualized={false}
            expandedToolRuns={thirtyExpanded}
            onToolRunToggle={(id) =>
              setThirtyExpanded((current) =>
                current.includes(id) ? current.filter((value) => value !== id) : [...current, id],
              )
            }
          />
        </div>
        <div style={{ ...frame, height: "14rem" }}>
          <AgentTranscript items={withFailure} virtualized={false} />
        </div>
      </SpecimenGroup>

      <SpecimenGroup
        label="Streaming and detached scroll"
        description="The caret marks a message still arriving. Scroll away from the bottom to reveal jump-to-latest."
      >
        <div style={{ ...frame, height: "12rem" }}>
          <AgentTranscript items={streaming} virtualized={false} />
        </div>
        <div style={{ ...frame, height: "16rem" }}>
          <AgentTranscript items={long} virtualized={false} />
        </div>
      </SpecimenGroup>

      <SpecimenGroup
        label="Long transcript rendering"
        description="The same mixed-height content, windowed and unwindowed."
      >
        <div style={frame}>
          <AgentTranscript items={long} />
        </div>
        <div style={frame}>
          <AgentTranscript items={long} virtualized={false} />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Empty">
        <div style={{ ...frame, height: "10rem" }}>
          <AgentTranscript items={[]} />
        </div>
      </SpecimenGroup>
    </SpecimenLayout>
  );
}
