import { render } from "svelte/server";
import { describe, expect, it } from "vitest";

import ToastStack from "../../src/ToastStack.svelte";

describe("ToastStack SSR — authored initial items", () => {
  it("renders preloaded items as settled rows in the first server HTML", () => {
    const { body } = render(ToastStack, {
      props: {
        items: [
          { id: "save", title: "Saved", message: "The document is ready.", tone: "success" },
          { id: "sync", title: "Syncing" },
        ],
      },
    });

    expect(body).toContain("Saved");
    expect(body).toContain("The document is ready.");
    expect(body).toContain("Syncing");
    expect(body).toContain('data-motion="settled"');
    expect(body).not.toContain('data-motion="enter"');
  });
});
