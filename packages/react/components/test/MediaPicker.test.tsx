import { fireEvent, render, waitFor } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { MediaPicker } from "../src/MediaPicker";
import type { MediaPickerItem } from "../src/types";

const items: MediaPickerItem[] = [
  { id: "m1", label: "Hero banner", thumbnailUrl: "/hero.png", kind: "image" },
  { id: "m2", label: "Intro clip", kind: "video" },
];

describe("MediaPicker (react)", () => {
  it("renders the browse grid with listbox semantics when open", () => {
    render(<MediaPicker open items={items} />);
    const grid = document.querySelector(".poodle-media-picker__grid") as HTMLElement;
    expect(grid.getAttribute("role")).toBe("listbox");
    expect(grid.getAttribute("aria-label")).toBe("Media items");
    const options = [...document.querySelectorAll('[role="option"]')];
    expect(options.length).toBe(2);
    expect(options[0].getAttribute("aria-selected")).toBe("false");
  });

  it("reports selection and requests the dialog close", () => {
    const onSelect = vi.fn();
    const onOpenChange = vi.fn();
    render(<MediaPicker open items={items} onSelect={onSelect} onOpenChange={onOpenChange} />);
    const first = document.querySelector('[role="option"]') as HTMLButtonElement;
    fireEvent.click(first);
    expect(onSelect).toHaveBeenCalledWith(items[0]);
    expect(onOpenChange).toHaveBeenCalledWith(false);
  });

  it("filters the browse grid by the search query", () => {
    render(<MediaPicker open items={items} />);
    const search = document.querySelector("#media-picker-search") as HTMLInputElement;
    fireEvent.input(search, { target: { value: "banner" } });
    const labels = [...document.querySelectorAll(".poodle-media-picker__label")].map(
      (el) => el.textContent,
    );
    expect(labels).toEqual(["Hero banner"]);
  });

  it("shows the empty message when no items match", async () => {
    render(<MediaPicker open items={items} emptyMessage="Nothing found" />);
    const search = document.querySelector("#media-picker-search") as HTMLInputElement;
    fireEvent.input(search, { target: { value: "zzz" } });
    await waitFor(() => {
      expect(document.querySelector(".poodle-media-picker__empty")).not.toBeNull();
    });
    expect(document.querySelector(".poodle-media-picker__empty")?.textContent).toContain(
      "Nothing found",
    );
  });

  it("switches to the upload tab and exposes the file upload surface", () => {
    render(<MediaPicker open items={items} />);
    const uploadTab = document.querySelector('[role="tab"][data-value="upload"]') as HTMLButtonElement;
    fireEvent.click(uploadTab);
    expect(document.querySelector(".poodle-media-picker__upload")).not.toBeNull();
  });
});