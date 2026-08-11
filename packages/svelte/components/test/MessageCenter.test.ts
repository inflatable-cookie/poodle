import { fireEvent, render, screen } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import MessageCenter from "../src/MessageCenter.svelte";

const items = [
  { id: "build", title: "Build complete", message: "Release candidate is ready.", read: false, tone: "success" as const },
  { id: "mention", title: "New mention", message: "Ada mentioned you in Studio.", read: true, meta: "Studio" },
];

describe("MessageCenter (svelte)", () => {
  it("exposes unread state on the trigger and renders the archive on open", async () => {
    render(MessageCenter, { props: { items } });

    const trigger = screen.getByRole("button", { name: "Notifications, 1 unread" });
    expect(trigger.getAttribute("aria-expanded")).toBe("false");

    await fireEvent.click(trigger);

    expect(screen.getByRole("dialog", { name: "Notifications" })).toBeTruthy();
    expect(screen.getByText("Build complete")).toBeTruthy();
  });

  it("emits read, remove, select, and mark-all requests without mutating items", async () => {
    const onReadChange = vi.fn();
    const onRemove = vi.fn();
    const onItemSelect = vi.fn();
    const onMarkAllRead = vi.fn();
    render(MessageCenter, {
      props: { items, defaultOpen: true, onReadChange, onRemove, onItemSelect, onMarkAllRead },
    });

    await fireEvent.click(screen.getByRole("button", { name: "Build complete" }));
    await fireEvent.click(screen.getByRole("button", { name: "Mark Build complete read" }));
    await fireEvent.click(screen.getByRole("button", { name: "Remove Build complete" }));
    await fireEvent.click(screen.getByRole("button", { name: "Mark all read" }));

    expect(onItemSelect).toHaveBeenCalledWith("build");
    expect(onReadChange).toHaveBeenCalledWith("build", true);
    expect(onRemove).toHaveBeenCalledWith("build");
    expect(onMarkAllRead).toHaveBeenCalledOnce();
    expect(items).toHaveLength(2);
  });
});
