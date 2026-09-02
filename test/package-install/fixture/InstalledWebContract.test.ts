import { render as renderSvelte } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import { Button as SvelteButton, Select as SvelteSelect } from "@inflatable-cookie/poodle-svelte";
import SvelteButtonDirect from "@inflatable-cookie/poodle-svelte/Button.svelte";
import SvelteSelectDirect from "@inflatable-cookie/poodle-svelte/Select.svelte";
import {
  AgentMessage,
  AgentPlan,
  AgentPlanRecord,
  AgentTranscript,
  MarkdownEditor,
} from "@inflatable-cookie/poodle-svelte/markdown";

describe("installed Svelte web contract", () => {
  it("loads root/direct controls and every markdown entry from the archive", () => {
    const controls = [
      renderSvelte(SvelteButton),
      renderSvelte(SvelteButtonDirect),
      renderSvelte(SvelteSelect, { props: { options: [] } }),
      renderSvelte(SvelteSelectDirect, { props: { options: [] } }),
    ];
    expect(controls.every((view) => view.container.querySelector("button,select"))).toBe(true);

    const markdown = [
      renderSvelte(AgentMessage, { props: { markdown: "installed markdown" } }),
      renderSvelte(AgentPlan, { props: { plan: "1. Installed plan" } }),
      renderSvelte(AgentPlanRecord, { props: { plan: "1. Installed record", status: "accepted" } }),
      renderSvelte(AgentTranscript, { props: { items: [] } }),
      renderSvelte(MarkdownEditor),
    ];
    expect(markdown[0].container.textContent).toContain("installed markdown");
    expect(markdown[1].container.textContent).toContain("Installed plan");
    expect(markdown[2].container.textContent).toContain("Installed record");
    expect(markdown[3].container).toBeTruthy();
    expect(markdown[4].container.querySelector("textarea")).not.toBeNull();

    for (const view of [...controls, ...markdown]) view.unmount();
  });
});
