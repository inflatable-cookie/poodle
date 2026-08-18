import { useState } from "react";
import { XYPad } from "@inflatable-cookie/poodle-react";
import { AudioSpecimenRow as Row } from "./AudioSpecimen";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";
export function XYPadSpecimen() { const [position, setPosition] = useState({ x: .5, y: .5 }); return (
    <SpecimenLayout
      variantDirection="row"
      sizes={(size) => <XYPad x={0.4} y={0.6} size={size} ariaLabel={`XY pad ${size} size`} />}
      densities={(density) => <XYPad x={0.4} y={0.6} density={density} ariaLabel={`XY pad ${density} density`} />}
    >
      <div style={{ display: "grid", gap: "1.5rem" }}>
        <SpecimenGroup label="Centered / default"><XYPad x={position.x} y={position.y} onValueChange={(x, y) => setPosition({ x, y })} ariaLabel="Centered pad" /></SpecimenGroup>
  <SpecimenGroup label="Corners"><Row><XYPad x={0} y={0} ariaLabel="Lower-left corner" /><XYPad x={1} y={1} ariaLabel="Upper-right corner" /></Row></SpecimenGroup>
  <SpecimenGroup label="Independent nonlinear laws"><XYPad x={1000} y={.6} minX={20} maxX={20_000} lawX={{ type: "logarithmic" }} lawY={{ type: "exponential", exponent: 2 }} formatX={{ type: "hz" }} formatY={{ type: "percent" }} ariaLabel="Nonlinear pad" /></SpecimenGroup>
  <SpecimenGroup label="Coarse / fine drag (Shift)"><XYPad x={.4} y={.6} ariaLabel="Fine drag pad" /></SpecimenGroup>
  <SpecimenGroup label="Reset (double-click)"><XYPad x={.8} y={.2} defaultX={.5} defaultY={.5} ariaLabel="Reset pad" /></SpecimenGroup>
  <SpecimenGroup label="Automation state"><XYPad x={.7} y={.2} automation="writing" ariaLabel="Automated pad" /></SpecimenGroup>
  <SpecimenGroup label="Keyboard axis bounds"><XYPad x={0} y={1} ariaLabel="Bounded pad" /></SpecimenGroup>
  <SpecimenGroup label="Disabled"><XYPad x={.5} y={.5} disabled ariaLabel="Disabled pad" /></SpecimenGroup>
      </div>
    </SpecimenLayout>
  ); }
