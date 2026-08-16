import { render } from "@testing-library/svelte";
import { beforeEach, describe, expect, it } from "vitest";

import EmbedPreview from "../src/EmbedPreview.svelte";

const youtube = { provider: "youtube", id: "dQw4w9WgXcQ" };
const vimeo = { provider: "vimeo", id: "76979871", embedType: "video" as const };

// happy-dom loads iframe pages for real; the embed cases would emit fetch
// NetworkError/AbortError noise. The fetch interceptor answers every iframe
// request with an empty 204, keeping the suites hermetic while the URL and
// sandbox assertions still run.
beforeEach(() => {
  const settings = (
    window as unknown as { happyDOM?: { settings?: { fetch?: { interceptor?: unknown } } } }
  ).happyDOM?.settings;
  if (settings?.fetch) {
    settings.fetch.interceptor = {
      beforeAsyncRequest: async () => new Response("", { status: 204 }),
    };
  }
});

describe("EmbedPreview (svelte)", () => {
  it("shows the loading state before any other state", () => {
    const { container } = render(EmbedPreview, { props: { loading: true, parsed: youtube } });
    expect(container.querySelector(".poodle-embed-preview__loading")).not.toBeNull();
    expect(container.querySelector(".poodle-embed-preview__loading-text")?.textContent).toContain(
      "Loading preview...",
    );
    expect(container.querySelector(".poodle-embed-preview__iframe")).toBeNull();
  });

  it("shows the error state over a parsed embed", () => {
    const { container } = render(EmbedPreview, {
      props: { error: "Failed to load embed", parsed: youtube },
    });
    expect(container.querySelector(".poodle-embed-preview__error")).not.toBeNull();
    expect(container.querySelector(".poodle-embed-preview__error")?.textContent).toContain(
      "Failed to load embed",
    );
  });

  it("shows the empty state when nothing is parsed or trusted", () => {
    const { container } = render(EmbedPreview, { props: { emptyMessage: "Paste a URL" } });
    expect(container.querySelector(".poodle-embed-preview__empty")).not.toBeNull();
    expect(container.querySelector(".poodle-embed-preview__empty")?.textContent).toContain(
      "Paste a URL",
    );
  });

  it("renders a sandboxed privacy-enhanced iframe for YouTube", () => {
    const { container } = render(EmbedPreview, { props: { parsed: youtube } });
    const iframe = container.querySelector(".poodle-embed-preview__iframe") as HTMLIFrameElement;
    expect(iframe.getAttribute("src")).toBe("https://www.youtube-nocookie.com/embed/dQw4w9WgXcQ");
    expect(iframe.getAttribute("title")).toBe("youtube embed");
    expect(iframe.getAttribute("sandbox")).toBe("allow-scripts allow-same-origin allow-popups");
    expect(iframe.getAttribute("loading")).toBe("lazy");
    expect(iframe.hasAttribute("allowfullscreen")).toBe(true);
  });

  it("renders the Vimeo player URL", () => {
    const { container } = render(EmbedPreview, { props: { parsed: vimeo } });
    const iframe = container.querySelector(".poodle-embed-preview__iframe") as HTMLIFrameElement;
    expect(iframe.getAttribute("src")).toBe("https://player.vimeo.com/video/76979871");
  });

  it("renders raw embed HTML when no provider URL is derivable", () => {
    const parsed = {
      provider: "generic",
      id: '<iframe src="https://player.example.com/x"></iframe>',
      originalEmbed: '<iframe src="https://player.example.com/x"></iframe>',
    };
    const { container } = render(EmbedPreview, { props: { parsed } });
    const containerEl = container.querySelector(".poodle-embed-preview__container") as HTMLElement;
    expect(containerEl.querySelector("iframe")).not.toBeNull();
  });

  it("renders caller-trusted HTML in the container", () => {
    const { container } = render(EmbedPreview, {
      props: { trustedHtml: '<video src="https://example.com/clip.mp4"></video>' },
    });
    const containerEl = container.querySelector(".poodle-embed-preview__container") as HTMLElement;
    expect(containerEl.querySelector("video")).not.toBeNull();
  });

  it("falls back to the parsed identity when no URL or embed code is derivable", () => {
    // A generic embed with no originalUrl and no embed code cannot produce an
    // iframe or raw render; the fallback region carries the identity.
    const { container } = render(EmbedPreview, {
      props: { parsed: { provider: "generic", id: "https://example.com/asset" } },
    });
    const fallback = container.querySelector(".poodle-embed-preview__fallback") as HTMLElement;
    expect(fallback).not.toBeNull();
    expect(fallback.textContent).toContain("https://example.com/asset");
  });

  it("records the fixed aspect ratio on the container", () => {
    const { container } = render(EmbedPreview, {
      props: { parsed: youtube, aspectRatio: 4 / 3 },
    });
    const containerEl = container.querySelector(".poodle-embed-preview__container") as HTMLElement;
    expect(containerEl.dataset.fixedAspect).toBe("true");
    expect(containerEl.style.aspectRatio).toMatch(/^1\.3333333333333333/);
  });
});