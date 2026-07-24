import { render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import Avatar from "../src/Avatar.svelte";
import Pill from "../src/Pill.svelte";
import Button from "../src/Button.svelte";
import Callout from "../src/Callout.svelte";
import Checkbox from "../src/Checkbox.svelte";
import Code from "../src/Code.svelte";
import EmptyState from "../src/EmptyState.svelte";
import Field from "../src/Field.svelte";
import Icon from "../src/Icon.svelte";
import Meter from "../src/Meter.svelte";
import MetricTile from "../src/MetricTile.svelte";
import Radio from "../src/Radio.svelte";
import Skeleton from "../src/Skeleton.svelte";
import Spinner from "../src/Spinner.svelte";
import StatusIndicator from "../src/StatusIndicator.svelte";
import Switch from "../src/Switch.svelte";
import TextLink from "../src/TextLink.svelte";

// Data-driven anatomy smoke: every listed component mounts, emits a poodle-*
// class (proof the Spec/token wiring resolved), and logs no console.error
// (guarded globally in test/vitest.setup.ts).
const cases: Array<[string, unknown, Record<string, unknown>]> = [
  ["Button", Button, {}],
  ["Pill", Pill, {}],
  ["Icon", Icon, { name: "info" }],
  ["Spinner", Spinner, {}],
  ["Avatar", Avatar, { name: "Ada Lovelace" }],
  ["Callout", Callout, {}],
  ["Code", Code, {}],
  ["Checkbox", Checkbox, {}],
  ["Switch", Switch, {}],
  ["Radio", Radio, { value: "a" }],
  ["TextLink", TextLink, { href: "#" }],
  ["Meter", Meter, { value: 50 }],
  ["MetricTile", MetricTile, { label: "Streams", value: "1.2k" }],
  ["StatusIndicator", StatusIndicator, {}],
  ["Skeleton", Skeleton, {}],
  ["Field", Field, { id: "f1", label: "Name" }],
  ["EmptyState", EmptyState, { title: "Nothing here" }],
];

describe("svelte component smoke", () => {
  for (const [name, Comp, props] of cases) {
    it(`${name} mounts and emits a poodle- class`, () => {
      const { container } = render(Comp as never, { props });
      expect(
        container.querySelector('[class*="poodle-"]'),
        `${name}: no poodle- classed element rendered`,
      ).not.toBeNull();
    });
  }
});
