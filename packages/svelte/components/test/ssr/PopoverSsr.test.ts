import { render } from "svelte/server";
import { describe, expect, it } from "vitest";

import Harness from "./PopoverSsrHarness.svelte";

/**
 * g15.041 server-render evidence. Runs under the `svelte-components-ssr`
 * project, which compiles every `.svelte` import with `generate: "server"` —
 * `render` from `svelte/server` cannot drive client-compiled components.
 *
 * The assertions read the raw server HTML (attribute order is template order
 * but never relied on): the whole point is that the trigger/surface
 * relationship is present in the first byte of HTML, with no post-mount
 * repair. The `anchored` action is client-only — Svelte skips `use:` under
 * SSR — so the surface renders inline inside the root rather than portalled;
 * its part attributes (id, role, state) are unaffected.
 */

/** The opening tag of the first element carrying `marker`, from the HTML. */
function openingTag(html: string, marker: string): string {
  const at = html.indexOf(marker);
  if (at < 0) throw new Error(`marker ${marker} not found in server HTML:\n${html}`);
  const start = html.lastIndexOf("<", at);
  const end = html.indexOf(">", at);
  return html.slice(start, end + 1);
}

function idOf(tag: string): string {
  const match = tag.match(/id="([^"]+)"/);
  if (!match) throw new Error(`no id attribute on: ${tag}`);
  return match[1];
}

describe("Popover SSR — interactive trigger semantics (g15.041)", () => {
  it("closed: wrapper is roleless and untabbable; the real control carries the closed disclosure state", () => {
    const { body } = render(Harness, { props: { triggerIsInteractive: true } });

    const wrapper = openingTag(body, 'data-part="trigger"');
    expect(wrapper).not.toContain("role=");
    expect(wrapper).not.toContain("tabindex");
    expect(wrapper).not.toContain("aria-expanded");
    expect(wrapper).not.toContain("aria-controls");

    const control = openingTag(body, "poodle-button");
    expect(control).toContain('aria-expanded="false"');
    expect(control).not.toContain("aria-controls");
    expect(control).not.toContain("disabled");

    expect(body).not.toContain('data-part="surface"');
  });

  it("defaultOpen: the real control's aria-controls equals the rendered surface id in the same HTML", () => {
    const { body } = render(Harness, { props: { triggerIsInteractive: true, defaultOpen: true } });

    const surface = openingTag(body, 'data-part="surface"');
    expect(surface).toContain('role="dialog"');
    const surfaceId = idOf(surface);

    const control = openingTag(body, "poodle-button");
    expect(control).toContain('aria-expanded="true"');
    expect(control).toContain(`aria-controls="${surfaceId}"`);
  });

  it("disabled: the disabled state reaches the real control and the surface stays unmounted", () => {
    const { body } = render(Harness, { props: { triggerIsInteractive: true, disabled: true } });

    const control = openingTag(body, "poodle-button");
    expect(control).toContain("disabled");
    expect(control).toContain('aria-expanded="false"');

    const wrapper = openingTag(body, 'data-part="trigger"');
    expect(wrapper).toContain('data-disabled="true"');
    expect(body).not.toContain('data-part="surface"');
  });
});

describe("Popover SSR — default trigger mode", () => {
  it("closed: the wrapper keeps role, tab stop, and closed disclosure ARIA", () => {
    const { body } = render(Harness, { props: {} });

    const wrapper = openingTag(body, 'data-part="trigger"');
    expect(wrapper).toContain('role="button"');
    expect(wrapper).toContain('tabindex="0"');
    expect(wrapper).toContain('aria-expanded="false"');
    expect(wrapper).not.toContain("aria-controls");

    expect(body).not.toContain('data-part="surface"');
  });

  it("defaultOpen: the wrapper's aria-controls equals the rendered surface id", () => {
    const { body } = render(Harness, { props: { defaultOpen: true } });

    const surfaceId = idOf(openingTag(body, 'data-part="surface"'));

    const wrapper = openingTag(body, 'data-part="trigger"');
    expect(wrapper).toContain('role="button"');
    expect(wrapper).toContain('aria-expanded="true"');
    expect(wrapper).toContain(`aria-controls="${surfaceId}"`);
  });
});
