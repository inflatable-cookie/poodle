import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { MediaBrowsePanel } from "../src/MediaBrowsePanel";
import type { MediaPickerItem } from "../src/types";

const items: MediaPickerItem[] = [
  { id: "m1", label: "Hero banner", thumbnailUrl: "/hero.png", meta: "1920x1080" },
  { id: "m2", label: "Intro clip", kind: "video" },
];

describe("MediaBrowsePanel (react)", () => {
  it("shows the loading state while no items are present", () => {
    const { container } = render(<MediaBrowsePanel loading items={[]} />);
    expect(container.querySelector(".poodle-media-browse-panel__state")?.textContent).toContain(
      "Loading media...",
    );
  });

  it("shows the error callout when an error is set", () => {
    const { container } = render(<MediaBrowsePanel error="Failed to load media" />);
    expect(container.querySelector(".poodle-media-browse-panel__state")).toBeNull();
    expect(container.textContent).toContain("Failed to load media");
  });

  it("shows the empty message with no items", () => {
    const { container } = render(<MediaBrowsePanel items={[]} emptyMessage="No media found" />);
    expect(container.querySelector(".poodle-media-browse-panel__state")?.textContent).toContain(
      "No media found",
    );
  });

  it("renders the browse grid and reports selection", () => {
    const onSelect = vi.fn();
    const { container } = render(<MediaBrowsePanel items={items} onSelect={onSelect} />);
    const buttons = [...container.querySelectorAll(".poodle-media-browse-panel__item")];
    expect(buttons.length).toBe(2);
    expect(container.querySelector("img")?.getAttribute("alt")).toBe("Hero banner");
    fireEvent.click(buttons[1]);
    expect(onSelect).toHaveBeenCalledWith(items[1]);
  });

  it("shows the load-more action only when more items are available", () => {
    const onLoadMore = vi.fn();
    const withMore = render(
      <MediaBrowsePanel items={items} hasMore onLoadMore={onLoadMore} />,
    );
    const loadMore = [...withMore.container.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Load more"),
    ) as HTMLButtonElement;
    expect(loadMore).not.toBeNull();
    fireEvent.click(loadMore);
    expect(onLoadMore).toHaveBeenCalledTimes(1);

    const withoutMore = render(<MediaBrowsePanel items={items} hasMore={false} />);
    expect(
      [...withoutMore.container.querySelectorAll("button")].some((button) =>
        button.textContent?.includes("Load more"),
      ),
    ).toBe(false);
  });

  it("disables and relabels the load-more button while loading", () => {
    const { container } = render(<MediaBrowsePanel items={items} hasMore loading />);
    const loadMore = [...container.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Loading..."),
    ) as HTMLButtonElement;
    expect(loadMore).not.toBeNull();
    expect(loadMore.disabled).toBe(true);
  });
});