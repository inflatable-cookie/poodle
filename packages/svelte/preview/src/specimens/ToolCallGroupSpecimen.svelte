<script lang="ts">
  import { ToolCallGroup, type TranscriptToolCall } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

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

  let expandedThree = $state(false);
  let expandedMany = $state(false);
</script>

<SpecimenLayout>
  <SpecimenGroup title="Single call" description="One call renders no toggle at all — not a disabled one, none, so there is no stray tab stop.">
    <ToolCallGroup id="single" calls={[call("only", "bun test")]} />
  </SpecimenGroup>

  <SpecimenGroup title="Collapsed and expanded" description="Collapsed shows the newest call. Expanded lists chronologically and ends on that same call, so the row you were reading does not move.">
    <ToolCallGroup id="three" calls={three} bind:expanded={expandedThree} />
  </SpecimenGroup>

  <SpecimenGroup title="A long run" description="Thirty calls behind one row.">
    <ToolCallGroup id="many" calls={many} bind:expanded={expandedMany} />
  </SpecimenGroup>

  <SpecimenGroup title="Buried failure" description="The failure is not the newest call. Without run status it would be invisible until someone expanded.">
    <ToolCallGroup id="buried" calls={buriedFailure} />
  </SpecimenGroup>

  <SpecimenGroup title="Running" description="Running ranks below error: a run that already broke is not in progress.">
    <ToolCallGroup id="running" calls={running} />
  </SpecimenGroup>



  {#snippet sizes(size)}
    <ToolCallGroup id={`sz-${size}`} calls={three} {size} />
  {/snippet}

  {#snippet densities(density)}
    <ToolCallGroup id={`dn-${density}`} calls={three} {density} />
  {/snippet}
</SpecimenLayout>
