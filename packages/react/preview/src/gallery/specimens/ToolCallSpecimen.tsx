import { ToolCall } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const longDetail =
  "CP_LATEX_SOURCE='../book/F/F1616 Fluid flow and body shape/F1616.tex' CP_LATEX_CODE=F1616 CP_LATEX_OUTPUT=/tmp/out.json";

export function ToolCallSpecimen() {
  return (
    <SpecimenLayout
      sizes={(size) => <ToolCall id={`sz-${size}`} label="Ran command" detail={`size ${size}`} size={size} />}
      densities={(density) => (
        <ToolCall id={`dn-${density}`} label="Ran command" detail={`density ${density}`} density={density} />
      )}
    >
      <SpecimenGroup label="Status">
        <ToolCall id="ok" label="Ran command" detail="effigy cp-api/test:latex" status="success" />
        <ToolCall id="run" label="Ran command" detail="cargo build --release" status="running" />
        <ToolCall id="err" label="Ran command" detail="effigy check:gpui" status="error" />
      </SpecimenGroup>

      <SpecimenGroup label="Kinds">
        <ToolCall id="k1" label="File change" detail="packages/styles/src/tool-call.css" />
        <ToolCall id="k2" label="Searched" detail="ResizeObserver" />
        <ToolCall id="k3" label="Something else" detail="falls back to the default glyph" />
        <ToolCall id="k4" label="Something else" detail="with an explicit icon" icon="sparkles" />
      </SpecimenGroup>

      <SpecimenGroup label="Truncation">
        <div style={{ maxWidth: "26rem" }}>
          <ToolCall id="long" label="Ran command" detail={longDetail} />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Output">
        <ToolCall id="no-output" label="Ran command" detail="no output, not interactive" />
        <ToolCall id="with-output" label="Ran command" detail="bun test" output={"272 pass\n0 fail"} expanded />
      </SpecimenGroup>
    </SpecimenLayout>
  );
}