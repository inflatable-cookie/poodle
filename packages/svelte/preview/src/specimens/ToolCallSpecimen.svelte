<script lang="ts">
  import { ToolCall } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  const longDetail =
    "CP_LATEX_SOURCE='../book/F/F1616 Fluid flow and body shape/F1616.tex' CP_LATEX_CODE=F1616 CP_LATEX_OUTPUT=/tmp/out.json";
</script>

<SpecimenLayout>
  <SpecimenGroup title="Status" description="Success, running and failure. Only the label takes the danger colour — the detail is already the dimmest thing in the row.">
      <ToolCall id="ok" label="Ran command" detail="effigy cp-api/test:latex" status="success" />
      <ToolCall id="run" label="Ran command" detail="cargo build --release" status="running" />
      <ToolCall id="err" label="Ran command" detail="effigy check:gpui" status="error" />
  </SpecimenGroup>

  <SpecimenGroup title="Kinds" description="The icon is derived from the label, with an override for hosts using their own vocabulary.">
      <ToolCall id="k1" label="File change" detail="packages/styles/src/tool-call.css" />
      <ToolCall id="k2" label="Searched" detail="ResizeObserver" />
      <ToolCall id="k3" label="Something else" detail="falls back to the default glyph" />
      <ToolCall id="k4" label="Something else" detail="with an explicit icon" icon="sparkles" />
  </SpecimenGroup>

  <SpecimenGroup title="Truncation" description="A long detail ellipsises rather than pushing the status indicator out of the row. The accessible name keeps the whole command.">
    <div style="max-width: 26rem;">
          <ToolCall id="long" label="Ran command" detail={longDetail} />
    </div>
  </SpecimenGroup>

  <SpecimenGroup title="Output" description="A row with no output is not a disabled button — it is not in the tab order at all.">
      <ToolCall id="no-output" label="Ran command" detail="no output, not interactive" />
      <ToolCall id="with-output" label="Ran command" detail="bun test" output={"272 pass\n0 fail"} expanded />
  </SpecimenGroup>



  {#snippet sizes(size)}
    <ToolCall id={`sz-${size}`} label="Ran command" detail={`size ${size}`} {size} />
  {/snippet}

  {#snippet densities(density)}
    <ToolCall id={`dn-${density}`} label="Ran command" detail={`density ${density}`} {density} />
  {/snippet}
</SpecimenLayout>
