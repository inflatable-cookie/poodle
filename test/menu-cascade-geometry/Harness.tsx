import { Menu } from "../../packages/react/components/src/Menu";
import { items, geometry, action } from "./items";
export function Harness() {
  return <section data-framework="react"><div className="scroller"><div className="spacer"/><div className="host">
    <Menu items={items} placement="top-end" triggerAriaLabel="Options react" ariaLabel="Root react" trigger="Options" onAction={action} onSurfaceGeometryChange={geometry}/>
  </div><div className="tail"/></div></section>;
}
