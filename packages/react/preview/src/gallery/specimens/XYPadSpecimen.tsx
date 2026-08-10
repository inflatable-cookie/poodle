import { useState } from "react";
import { XYPad } from "@inflatable-cookie/poodle-react";
import { AudioAxes, AudioSpecimenGroup as Group, AudioSpecimenPage as Page, AudioSpecimenRow as Row } from "./AudioSpecimen";
export function XYPadSpecimen() { const [position, setPosition] = useState({ x: .5, y: .5 }); return <Page>
  <Group title="Centered / default"><XYPad x={position.x} y={position.y} onValueChange={(x, y) => setPosition({ x, y })} ariaLabel="Centered pad" /></Group>
  <Group title="Corners"><Row><XYPad x={0} y={0} ariaLabel="Lower-left corner" /><XYPad x={1} y={1} ariaLabel="Upper-right corner" /></Row></Group>
  <Group title="Independent nonlinear laws"><XYPad x={1000} y={.6} minX={20} maxX={20_000} lawX={{ type: "logarithmic" }} lawY={{ type: "exponential", exponent: 2 }} formatX={{ type: "hz" }} formatY={{ type: "percent" }} ariaLabel="Nonlinear pad" /></Group>
  <Group title="Coarse / fine drag (Shift)"><XYPad x={.4} y={.6} ariaLabel="Fine drag pad" /></Group>
  <Group title="Reset (double-click)"><XYPad x={.8} y={.2} defaultX={.5} defaultY={.5} ariaLabel="Reset pad" /></Group>
  <Group title="Automation state"><XYPad x={.7} y={.2} automation="writing" ariaLabel="Automated pad" /></Group>
  <Group title="Keyboard axis bounds"><XYPad x={0} y={1} ariaLabel="Bounded pad" /></Group>
  <Group title="Disabled"><XYPad x={.5} y={.5} disabled ariaLabel="Disabled pad" /></Group>
  <AudioAxes render={(props, label) => <XYPad {...props} x={.4} y={.6} ariaLabel={`XY pad ${label}`} />} />
</Page>; }
