import { render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import Avatar from "../src/Avatar.svelte";

describe("Avatar (svelte)", () => {
  it("renders initials trimmed to three uppercase characters when no src", () => {
    const { container } = render(Avatar, { props: { initials: "tomasz" } });
    expect(container.querySelector(".poodle-avatar__initials")?.textContent).toBe("TOM");
  });

  it("projects role=img with an accessible label for the initials fallback", () => {
    const { container } = render(Avatar, { props: { initials: "JD", ariaLabel: "Jane Doe" } });
    const root = container.querySelector(".poodle-avatar");
    expect(root?.getAttribute("role")).toBe("img");
    expect(root?.getAttribute("aria-label")).toBe("Jane Doe");
  });

  it("renders an image with alt when src is set", () => {
    const { container } = render(Avatar, { props: { src: "/me.png", alt: "Me" } });
    const img = container.querySelector(".poodle-avatar img") as HTMLImageElement | null;
    expect(img?.getAttribute("src")).toBe("/me.png");
    expect(img?.getAttribute("alt")).toBe("Me");
    expect(container.querySelector(".poodle-avatar")?.getAttribute("role")).toBeNull();
  });

  it("hides the avatar from assistive tech in decorative mode", () => {
    const { container } = render(Avatar, {
      props: { src: "/me.png", decorative: true },
    });
    const root = container.querySelector(".poodle-avatar");
    expect(root?.getAttribute("aria-hidden")).toBe("true");
    expect((container.querySelector(".poodle-avatar img") as HTMLImageElement).alt).toBe("");
  });
});
