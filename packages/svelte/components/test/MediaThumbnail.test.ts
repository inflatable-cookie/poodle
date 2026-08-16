import { render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import MediaThumbnail from "../src/MediaThumbnail.svelte";
import { asSnippet } from "./snippet";

describe("MediaThumbnail (svelte)", () => {
  it("projects kind, state, aspect ratio, presentation, and fit data attributes", () => {
    const { container } = render(MediaThumbnail, {
      props: { kind: "video", state: "ready", aspectRatio: "square", presentation: "compact", fit: "contain" },
    });
    const figure = container.querySelector(".poodle-media-thumbnail") as HTMLElement;
    expect(figure.tagName).toBe("FIGURE");
    expect(figure.dataset.kind).toBe("video");
    expect(figure.dataset.state).toBe("ready");
    expect(figure.dataset.aspectRatio).toBe("square");
    expect(figure.dataset.presentation).toBe("compact");
    expect(figure.dataset.fit).toBe("contain");
  });

  it("shows a play indicator for audio and video kinds", () => {
    const video = render(MediaThumbnail, { props: { kind: "video" } });
    expect(video.container.querySelector(".poodle-media-thumbnail__play")).not.toBeNull();
    const image = render(MediaThumbnail, { props: { kind: "image" } });
    expect(image.container.querySelector(".poodle-media-thumbnail__play")).toBeNull();
  });

  it("renders the state display with a default title and busy flag while loading", () => {
    const { container } = render(MediaThumbnail, { props: { state: "loading" } });
    const figure = container.querySelector(".poodle-media-thumbnail") as HTMLElement;
    expect(figure.getAttribute("aria-busy")).toBe("true");
    expect(container.querySelector(".poodle-media-thumbnail__state strong")?.textContent).toBe(
      "Loading preview",
    );
    expect(container.querySelector(".poodle-media-thumbnail__spinner")).not.toBeNull();
  });

  it("renders state-specific default titles", () => {
    const error = render(MediaThumbnail, { props: { state: "error" } });
    expect(error.container.querySelector(".poodle-media-thumbnail__state strong")?.textContent).toBe(
      "Preview unavailable",
    );
    const empty = render(MediaThumbnail, { props: { state: "empty" } });
    expect(empty.container.querySelector(".poodle-media-thumbnail__state strong")?.textContent).toBe(
      "No preview",
    );
  });

  it("prefers an explicit stateTitle and hides the message in compact mode", () => {
    const { container } = render(MediaThumbnail, {
      props: {
        state: "error",
        stateTitle: "Corrupted file",
        stateMessage: "Cannot render",
        presentation: "compact",
      },
    });
    expect(container.querySelector(".poodle-media-thumbnail__state strong")?.textContent).toBe(
      "Corrupted file",
    );
    expect(container.querySelector(".poodle-media-thumbnail__state p")).toBeNull();
  });

  it("overlays the badge and renders the caption in default presentation", () => {
    const { container } = render(MediaThumbnail, {
      props: { title: "Photo", meta: "2.4 MB", badge: "New" },
    });
    expect(container.querySelector(".poodle-media-thumbnail__badge")?.textContent).toBe("New");
    const caption = container.querySelector(".poodle-media-thumbnail__caption") as HTMLElement;
    expect(caption.querySelector("strong")?.textContent).toBe("Photo");
    expect(caption.querySelector("span")?.textContent).toBe("2.4 MB");
  });

  it("hides the caption in compact presentation", () => {
    const { container } = render(MediaThumbnail, {
      props: { title: "Photo", presentation: "compact" },
    });
    expect(container.querySelector(".poodle-media-thumbnail__caption")).toBeNull();
  });

  it("renders slot content instead of the placeholder and keeps the frame labelled", () => {
    const { container } = render(MediaThumbnail, {
      props: { ariaLabel: "Preview area", children: asSnippet(() => "<img alt='asset'>") },
    });
    expect(container.querySelector(".poodle-media-thumbnail__placeholder")).toBeNull();
    const figure = container.querySelector(".poodle-media-thumbnail") as HTMLElement;
    expect(figure.getAttribute("aria-label")).toBe("Preview area");
  });
});