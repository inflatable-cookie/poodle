import { AgentTranscript, type TranscriptItem } from "@poodle/react";

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
        title="A run containing a failure"
        description="Collapsed, the run still advertises that something in it broke."
      >
        <div style={{ ...frame, height: "14rem" }}>
          <AgentTranscript items={withFailure} virtualized={false} />
        </div>
      </SpecimenGroup>

      <SpecimenGroup title="Empty">
        <div style={{ ...frame, height: "10rem" }}>
          <AgentTranscript items={[]} />
        </div>
      </SpecimenGroup>

      <SpecimenGroup
        title="Windowed"
        description="120 blocks of varying height. Scroll up and the jump-to-latest pill appears; following re-arms only when you return to the bottom."
      >
        <div style={frame}>
          <AgentTranscript items={long} />
        </div>
      </SpecimenGroup>

      <SpecimenGroup
        title="Unwindowed"
        description="The same content with every block rendered. The result must be identical — only the cost differs."
      >
        <div style={frame}>
          <AgentTranscript items={long} virtualized={false} />
        </div>
      </SpecimenGroup>
    </SpecimenLayout>
  );
}
