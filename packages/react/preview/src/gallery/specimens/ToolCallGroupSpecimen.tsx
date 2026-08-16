import { ToolCallGroup, type TranscriptToolCall } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const call = (id: string, detail: string, status: TranscriptToolCall["status"] = "success"): TranscriptToolCall => ({
  kind: "tool-call",
  id,
  label: "Ran command",
  detail,
  status,
});

const three = [call("a", "nl -ba src/lexer.rs"), call("b", "effigy cp-api/test:latex"), call("c", "sed -n '430,560p' cp_html.rs")];
const many = Array.from({ length: 30 }, (_, i) => call(`m${i}`, `step ${i + 1} of the sweep`));
// The case run status exists for: the failure is *not* the newest call, so a
// collapsed run would otherwise look entirely healthy.
const buriedFailure = [call("f1", "cargo check"), call("f2", "effigy check:gpui", "error"), call("f3", "bun test")];
const running = [call("r1", "cargo build"), call("r2", "cargo test", "running")];

export function ToolCallGroupSpecimen() {
  return (
    <SpecimenLayout
      sizes={(size) => <ToolCallGroup id={`sz-${size}`} calls={three} size={size} />}
      densities={(density) => <ToolCallGroup id={`dn-${density}`} calls={three} density={density} />}
    >
      <SpecimenGroup label="Single call">
        <ToolCallGroup id="single" calls={[call("only", "bun test")]} />
      </SpecimenGroup>

      <SpecimenGroup label="Collapsed and expanded">
        <ToolCallGroup id="three" calls={three} />
      </SpecimenGroup>

      <SpecimenGroup label="A long run">
        <ToolCallGroup id="many" calls={many} />
      </SpecimenGroup>

      <SpecimenGroup label="Buried failure">
        <ToolCallGroup id="buried" calls={buriedFailure} />
      </SpecimenGroup>

      <SpecimenGroup label="Running">
        <ToolCallGroup id="running" calls={running} />
      </SpecimenGroup>
    </SpecimenLayout>
  );
}