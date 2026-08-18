import type { ReactNode } from "react";
import {
  Avatar,
  Callout,
  EmptyState,
  Pill,
  Spinner,
  type ControlDensity,
  type ControlSize,
} from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "./SpecimenGroup";
import { SpecimenLayout } from "./SpecimenLayout";
import { specimenScenes } from "../generated/specimens/specimen-scenes";

type FixtureValue = string | boolean | number | null;

interface FixtureInstance {
  component: string;
  caption?: string;
  props: Record<string, FixtureValue>;
}

interface FixtureGroup {
  label: string;
  instances: FixtureInstance[];
}

type FixtureScene = {
  id: string;
  name: string;
  description: string;
  tabs: readonly string[];
  sizeAxis: readonly string[];
  densityAxis: readonly string[];
  groups: readonly FixtureGroup[];
};

/**
 * The fixture renderer (g14-b005 tranche one): renders one display-specimen
 * scene in the React preview. The scene is generated data — groups,
 * instances, typed prop bindings, matrix axes. `content` bindings project
 * to children; every other binding forwards as a prop.
 */
export function SceneSpecimen({ slug }: { slug?: string }) {
  const scene = slug
    ? (specimenScenes[slug as keyof typeof specimenScenes] as unknown as FixtureScene | undefined)
    : undefined;

  // The fixture props are data; each component validates its own at runtime
  // where it matters, so the map is deliberately untyped.
  const componentMap: Record<string, (props: Record<string, FixtureValue>) => ReactNode> = {
    callout: ({ content, ...props }: any) => <Callout {...props}>{String(content)}</Callout>,
    pill: ({ content, ...props }: any) => <Pill {...props}>{String(content)}</Pill>,
    spinner: (props: any) => <Spinner {...props} />,
    avatar: (props: any) => <Avatar {...props} />,
    "empty-state": (props: any) => <EmptyState {...props} />,
  };

  function renderInstance(instance: FixtureInstance) {
    const render = componentMap[instance.component];
    return render ? render(instance.props) : null;
  }

  /** The matrix renders the scene's first instance at each axis value. */
  const matrixInstance = scene?.groups.flatMap((group) => group.instances)[0];

  if (!scene) return null;

  const sizes = (size: ControlSize): ReactNode =>
    matrixInstance ? renderInstance({ ...matrixInstance, props: { ...matrixInstance.props, size } }) : null;

  const densities = (density: ControlDensity): ReactNode =>
    matrixInstance ? renderInstance({ ...matrixInstance, props: { ...matrixInstance.props, density } }) : null;

  return (
    // The scene's declared tabs decide which axis renderers exist; an axis
    // the scene does not declare never reaches SpecimenLayout, so its tab
    // cannot be advertised.
    <SpecimenLayout
      sizes={scene.tabs.includes("sizes") ? sizes : undefined}
      densities={scene.tabs.includes("densities") ? densities : undefined}
    >
      {scene.groups.map((group) => (
        <SpecimenGroup key={group.label} label={group.label}>
          <div className="poodle-specimen__row">
            {group.instances.map((instance, index) => (
              <div key={index} className="poodle-specimen__cell">
                {renderInstance(instance)}
              </div>
            ))}
          </div>
        </SpecimenGroup>
      ))}
    </SpecimenLayout>
  );
}
