/**
 * Specimen projection (spec 066): the corpus's specimen metadata defines
 * groups, captions, and axes once; each runtime renders that structure with
 * its own component. This module turns the serialized corpus + interface
 * into the projection data every web specimen page renders. The native
 * previews render the same structure from the same JSON.
 */

import type {
  SerializedCase,
  SerializedComponentCases,
  SerializedComponentInterface,
} from "./define";

export interface ProjectedInstance {
  caseId: string;
  caption: string;
  label: string;
  props: Record<
    string,
    string | boolean | number | readonly [number, number] |
      readonly Record<string, string | boolean | number>[] | null
  >;
  leadingIcon: string | null;
  trailingIcon: string | null;
  /** Whether this instance is an interactive (behaviour) case. */
  interactive: boolean;
}

export interface ProjectedGroup {
  label: string;
  instances: ProjectedInstance[];
}

function enumValues(iface: SerializedComponentInterface, propName: string): string[] {
  const prop = iface.props.find((p) => p.name === propName);
  return prop?.type.kind === "enum" && prop.type.values ? [...prop.type.values] : [];
}

function instanceFor(
  caseData: SerializedCase,
  caption: string,
  props: Record<
    string,
    string | boolean | number | readonly [number, number] |
      readonly Record<string, string | boolean | number>[] | null
  >,
): ProjectedInstance {
  return {
    caseId: caseData.id,
    caption,
    label: caseData.fixture.regions.label ?? "",
    props,
    leadingIcon: caseData.fixture.regions.leading ?? null,
    trailingIcon: caseData.fixture.regions.trailing ?? null,
    interactive: caseData.steps.some((step) => step.kind === "action"),
  };
}

/**
 * Groups + instances for the four specimen pages. Axis expansion: a case
 * declaring a size or density axis yields one instance per enum value, unless
 * the fixture already fixes that axis.
 */
export function projectCorpus(
  cases: SerializedComponentCases,
  iface: SerializedComponentInterface,
): ProjectedGroup[] {
  const groups: ProjectedGroup[] = [];
  for (const caseData of cases.cases) {
    let group = groups.find((g) => g.label === caseData.specimen.group);
    if (!group) {
      group = { label: caseData.specimen.group, instances: [] };
      groups.push(group);
    }
    const base = instanceFor(caseData, caseData.specimen.caption, { ...caseData.fixture.props });
    group.instances.push(base);
    for (const axis of caseData.specimen.axes) {
      if (axis === "size" || axis === "density") {
        if (caseData.fixture.props[axis] !== undefined) continue;
        for (const value of enumValues(iface, axis)) {
          group.instances.push(
            instanceFor(caseData, `${caseData.specimen.caption} · ${value}`, {
              ...caseData.fixture.props,
              [axis]: value,
            }),
          );
        }
      }
    }
  }
  return groups;
}
