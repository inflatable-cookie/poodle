import "@poodle/svelte-tokens/styles.css";
import "../../../tokens/artifacts/css/poodle-themes.css";

import { mount } from "svelte";
import { AgentTranscript, type TranscriptItem } from "@poodle/svelte";

// Nucleus-like: long assistant messages (300-800px rendered) with short tool
// rows between — real heights far above the 120px estimate.
const paragraph =
  "The parser now matches the legacy output on every fixture in the corpus, including the pathological ones with nested directives. ".repeat(
    4,
  );

const items: TranscriptItem[] = [];
for (let i = 0; i < 60; i += 1) {
  if (i % 5 === 4) {
    items.push({ kind: "tool-call", id: `t${i}`, label: "Ran command", detail: `step ${i}`, status: "success" });
  } else {
    const reps = 1 + (i % 6); // 1..6 paragraphs -> ~150..900px
    items.push({
      kind: "message",
      id: `m${i}`,
      role: "assistant",
      markdown: Array.from({ length: reps }, () => paragraph).join("\n\n"),
    });
  }
}

mount(AgentTranscript as never, {
  target: document.getElementById("app")!,
  props: { items },
});
