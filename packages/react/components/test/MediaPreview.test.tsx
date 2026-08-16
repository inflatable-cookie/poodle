import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { MediaPreview } from "../src/MediaPreview";

describe("MediaPreview (react)", () => {
  it("renders the title, eyebrow, and description in the header", () => {
    const { container } = render(
      <MediaPreview title="Hero banner" eyebrow="Image" description="Main landing banner." />,
    );
    const heading = container.querySelector(".poodle-media-preview h3") as HTMLElement;
    expect(heading.textContent).toBe("Hero banner");
    expect(container.querySelector(".poodle-media-preview__eyebrow")?.textContent).toBe("Image");
    expect(container.querySelector(".poodle-media-preview__description")?.textContent).toContain(
      "Main landing banner.",
    );
  });

  it("renders the metadata list with the preview metadata label", () => {
    const { container } = render(<MediaPreview title="Hero" meta={["1920 x 1080", "245 KB"]} />);
    const list = container.querySelector(".poodle-media-preview__meta") as HTMLElement;
    expect(list.getAttribute("aria-label")).toBe("preview metadata");
    expect(list.textContent).toContain("1920 x 1080");
    expect(list.textContent).toContain("245 KB");
  });

  it("prepends thumbnailMeta to the metadata list", () => {
    const { container } = render(
      <MediaPreview title="Hero" thumbnailMeta="PNG" meta={["245 KB"]} />,
    );
    const list = container.querySelector(".poodle-media-preview__meta") as HTMLElement;
    const items = [...list.querySelectorAll("li")].map((li) => li.textContent);
    expect(items).toEqual(["PNG", "245 KB"]);
  });

  it("renders the caption and passes the error posture through to the thumbnail", () => {
    const { container } = render(
      <MediaPreview
        title="Corrupted file"
        caption="A caption"
        state="error"
        stateTitle="Preview unavailable"
        stateMessage="Cannot render"
      />,
    );
    expect(container.querySelector(".poodle-media-preview__caption")?.textContent).toContain(
      "A caption",
    );
    const thumbnail = container.querySelector(".poodle-media-thumbnail") as HTMLElement;
    expect(thumbnail.dataset.state).toBe("error");
    expect(thumbnail.textContent).toContain("Preview unavailable");
  });

  it("overlays the badge on the thumbnail frame", () => {
    const { container } = render(<MediaPreview title="Hero" badge="New" />);
    const badge = container.querySelector(".poodle-media-thumbnail__badge") as HTMLElement;
    expect(badge.textContent).toBe("New");
  });

  it("renders custom body content", () => {
    const { container } = render(
      <MediaPreview title="Hero">
        <p>body</p>
      </MediaPreview>,
    );
    expect(container.querySelector(".poodle-media-preview__body")).not.toBeNull();
  });
});