import { cleanup as cleanupReact, render as renderReact } from "@testing-library/react";
import { cleanup as cleanupSvelte, render as renderSvelte } from "@testing-library/svelte";
import type { ReactElement } from "react";
import { createRawSnippet } from "svelte";
import { describe, expect, it } from "vitest";

// Text children for Svelte components that take a Snippet, so both frameworks
// render equivalent content (e.g. Button's label wrapper is gated on children).
const text = (value: string) =>
  createRawSnippet(() => ({ render: () => `<span>${value}</span>` }));

import SvAvatar from "../../packages/svelte/components/src/Avatar.svelte";
import SvButton from "../../packages/svelte/components/src/Button.svelte";
import SvCallout from "../../packages/svelte/components/src/Callout.svelte";
import SvCheckbox from "../../packages/svelte/components/src/Checkbox.svelte";
import SvCode from "../../packages/svelte/components/src/Code.svelte";
import SvEmptyState from "../../packages/svelte/components/src/EmptyState.svelte";
import SvField from "../../packages/svelte/components/src/Field.svelte";
import SvIcon from "../../packages/svelte/components/src/Icon.svelte";
import SvMeter from "../../packages/svelte/components/src/Meter.svelte";
import SvMetricTile from "../../packages/svelte/components/src/MetricTile.svelte";
import SvPill from "../../packages/svelte/components/src/Pill.svelte";
import SvRadio from "../../packages/svelte/components/src/Radio.svelte";
import SvSkeleton from "../../packages/svelte/components/src/Skeleton.svelte";
import SvSpinner from "../../packages/svelte/components/src/Spinner.svelte";
import SvStatusIndicator from "../../packages/svelte/components/src/StatusIndicator.svelte";
import SvSwitch from "../../packages/svelte/components/src/Switch.svelte";
import SvTextLink from "../../packages/svelte/components/src/TextLink.svelte";
import {
  Avatar,
  Button,
  Callout,
  Checkbox,
  Code,
  EmptyState,
  Field,
  Icon,
  Meter,
  MetricTile,
  Pill,
  Radio,
  Skeleton,
  Spinner,
  StatusIndicator,
  Switch,
  TextLink,
} from "../../packages/react/components/src";

// Classes that differ by framework idiom, not component anatomy: Svelte context
// providers render a wrapper element; React context emits no DOM node.
const IGNORE = new Set(["poodle-ui-presentation-provider"]);

// Known, accepted anatomy divergences per component: case name -> allowed
// divergent classes (either side). The gate fails on any divergence NOT listed
// here; closing a gap means deleting its entry. Svelte is the parity authority
// (see contracts) — any entry here is a React shell to reconcile. Currently
// empty: all covered components emit identical anatomy given matched content.
const KNOWN_DIVERGENCE: Record<string, string[]> = {};

function anatomy(container: HTMLElement): string[] {
  const set = new Set<string>();
  for (const el of container.querySelectorAll("*")) {
    for (const c of el.classList) {
      if (c.startsWith("poodle-") && !IGNORE.has(c)) set.add(c);
    }
  }
  return [...set].sort();
}

interface Case {
  name: string;
  svelte: unknown;
  props: Record<string, unknown>;
  react: ReactElement;
}

// Matched props so both implementations render equivalent anatomy.
const cases: Case[] = [
  { name: "Button", svelte: SvButton, props: { children: text("Go") }, react: <Button>Go</Button> },
  { name: "Pill", svelte: SvPill, props: { children: text("New") }, react: <Pill>New</Pill> },
  { name: "Icon", svelte: SvIcon, props: { name: "info" }, react: <Icon name="info" /> },
  { name: "Spinner", svelte: SvSpinner, props: {}, react: <Spinner /> },
  { name: "Avatar", svelte: SvAvatar, props: { name: "Ada Lovelace" }, react: <Avatar name="Ada Lovelace" /> },
  { name: "Callout", svelte: SvCallout, props: { children: text("Heads up") }, react: <Callout>Heads up</Callout> },
  { name: "Code", svelte: SvCode, props: { children: text("npm i") }, react: <Code>npm i</Code> },
  { name: "Checkbox", svelte: SvCheckbox, props: {}, react: <Checkbox /> },
  { name: "Switch", svelte: SvSwitch, props: {}, react: <Switch /> },
  { name: "Radio", svelte: SvRadio, props: { value: "a" }, react: <Radio value="a" /> },
  { name: "TextLink", svelte: SvTextLink, props: { href: "#", children: text("link") }, react: <TextLink href="#">link</TextLink> },
  { name: "Meter", svelte: SvMeter, props: { value: 50 }, react: <Meter value={50} /> },
  { name: "MetricTile", svelte: SvMetricTile, props: { label: "Streams", value: "1.2k" }, react: <MetricTile label="Streams" value="1.2k" /> },
  { name: "StatusIndicator", svelte: SvStatusIndicator, props: {}, react: <StatusIndicator /> },
  { name: "Skeleton", svelte: SvSkeleton, props: {}, react: <Skeleton /> },
  { name: "Field", svelte: SvField, props: { id: "f1", label: "Name" }, react: <Field id="f1" label="Name" /> },
  { name: "EmptyState", svelte: SvEmptyState, props: { title: "None" }, react: <EmptyState title="None" /> },
];

describe("svelte <-> react anatomy parity", () => {
  for (const c of cases) {
    it(`${c.name} emits matching poodle- anatomy classes`, () => {
      const sv = renderSvelte(c.svelte as never, { props: c.props }).container;
      const svClasses = anatomy(sv);
      cleanupSvelte();
      const re = renderReact(c.react).container;
      const reClasses = anatomy(re);
      cleanupReact();

      const allowed = new Set(KNOWN_DIVERGENCE[c.name] ?? []);
      const svelteOnly = svClasses.filter((x) => !reClasses.includes(x) && !allowed.has(x));
      const reactOnly = reClasses.filter((x) => !svClasses.includes(x) && !allowed.has(x));
      expect({ svelteOnly, reactOnly }).toEqual({ svelteOnly: [], reactOnly: [] });
    });
  }
});
