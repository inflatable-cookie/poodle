import { render } from "@testing-library/react";
import type { ReactElement } from "react";
import { describe, expect, it } from "vitest";

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
} from "../src";

// Data-driven anatomy smoke: every listed component mounts, emits a poodle-*
// class (proof the Spec/token wiring resolved), and logs no console.error
// (guarded globally in test/vitest.setup.ts). Mirrors the Svelte smoke set so
// the two implementations are held to the same anatomy floor.
const cases: Array<[string, ReactElement]> = [
  ["Button", <Button>Go</Button>],
  ["Pill", <Pill>New</Pill>],
  ["Icon", <Icon name="info" />],
  ["Spinner", <Spinner />],
  ["Avatar", <Avatar name="Ada Lovelace" />],
  ["Callout", <Callout>Heads up</Callout>],
  ["Code", <Code>npm i</Code>],
  ["Checkbox", <Checkbox />],
  ["Switch", <Switch />],
  ["Radio", <Radio value="a" />],
  ["TextLink", <TextLink href="#">link</TextLink>],
  ["Meter", <Meter value={50} />],
  ["MetricTile", <MetricTile label="Streams" value="1.2k" />],
  ["StatusIndicator", <StatusIndicator />],
  ["Skeleton", <Skeleton />],
  ["Field", <Field id="f1" label="Name" />],
  ["EmptyState", <EmptyState title="Nothing here" />],
];

describe("react component smoke", () => {
  for (const [name, element] of cases) {
    it(`${name} mounts and emits a poodle- class`, () => {
      const { container } = render(element);
      expect(
        container.querySelector('[class*="poodle-"]'),
        `${name}: no poodle- classed element rendered`,
      ).not.toBeNull();
    });
  }
});
