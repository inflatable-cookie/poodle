import { fireEvent, render, screen } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import MessageCenter from "../src/MessageCenter.svelte";
import type { MessageCenterItem } from "../src/types";

const items = [
  { id: "build", title: "Build complete", message: "Release candidate is ready.", read: false, tone: "success" as const },
  { id: "mention", title: "New mention", message: "Ada mentioned you in Studio.", read: true, meta: "Studio" },
];

describe("MessageCenter (svelte)", () => {
  it("exposes unread state on the trigger and renders the archive on open", async () => {
    render(MessageCenter, { props: { items } });

    const trigger = screen.getByRole("button", { name: "Notifications, 1 unread" });
    expect(trigger.getAttribute("aria-expanded")).toBe("false");
    expect(trigger.getAttribute("aria-controls")).toBeNull();

    await fireEvent.click(trigger);

    // The trigger/surface relationship lands on the real control (the
    // IconButton), not on any decorative wrapper.
    const dialog = screen.getByRole("dialog", { name: "Notifications" });
    expect(dialog).toBeTruthy();
    expect(trigger.getAttribute("aria-expanded")).toBe("true");
    expect(trigger.getAttribute("aria-controls")).toBe(dialog.id);
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

  describe("mixed activity feed", () => {
    const mixed: MessageCenterItem[] = [
      {
        id: "job-render",
        title: "Mix preview",
        message: "Rendering…",
        meta: "Render queue",
        read: true,
        tone: "pending",
        progress: { value: 60 },
        selectable: false,
        removable: false,
        readControl: false,
      },
      {
        id: "job-upload",
        title: "Uploading stems",
        meta: "Cloud sync",
        read: true,
        progress: { value: null, indeterminate: true },
        selectable: false,
        removable: false,
        readControl: false,
      },
      { id: "build", title: "Build complete", message: "Release candidate is ready.", read: false, tone: "success" },
    ];

    it("renders determinate and indeterminate live rows without interaction controls", async () => {
      render(MessageCenter, {
        props: { items: mixed, defaultOpen: true, onItemSelect: vi.fn(), onReadChange: vi.fn(), onRemove: vi.fn() },
      });

      const determinate = screen.getByRole("progressbar", { name: "Mix preview progress" });
      expect(determinate.getAttribute("aria-valuenow")).toBe("60");
      expect(determinate.getAttribute("aria-valuemax")).toBe("100");

      const indeterminate = screen.getByRole("progressbar", { name: "Uploading stems progress" });
      expect(indeterminate.getAttribute("aria-valuenow")).toBeNull();

      expect(screen.queryByRole("button", { name: "Mix preview" })).toBeNull();
      expect(screen.queryByRole("button", { name: "Uploading stems" })).toBeNull();
      expect(screen.queryByRole("button", { name: "Remove Mix preview" })).toBeNull();
      expect(screen.queryByRole("button", { name: "Mark Mix preview unread" })).toBeNull();

      expect(screen.getByRole("button", { name: "Build complete" })).toBeTruthy();
      expect(screen.getByRole("button", { name: "Remove Build complete" })).toBeTruthy();
      expect(screen.getByRole("button", { name: "Mark Build complete read" })).toBeTruthy();
    });

    it("keeps live rows out of the unread count", async () => {
      render(MessageCenter, { props: { items: mixed } });

      expect(screen.getByRole("button", { name: "Notifications, 1 unread" })).toBeTruthy();
    });

    it("honours per-item removability independently of the global callback", async () => {
      const onRemove = vi.fn();
      const partly: MessageCenterItem[] = [
        { ...items[0], removable: false },
        { ...items[1] },
      ];
      render(MessageCenter, { props: { items: partly, defaultOpen: true, onRemove } });

      expect(screen.queryByRole("button", { name: "Remove Build complete" })).toBeNull();
      expect(screen.getByRole("button", { name: "Remove New mention" })).toBeTruthy();

      await fireEvent.click(screen.getByRole("button", { name: "Remove New mention" }));
      expect(onRemove).toHaveBeenCalledWith("mention");
    });

    it("renders host progress updates in place without local authority", async () => {
      const host = () =>
        render(MessageCenter, {
          props: {
            items: [{
              ...mixed[0],
              progress: { value: mixed[0].progress!.value! },
            }],
            defaultOpen: true,
          },
        });
      const first = host();
      expect(screen.getByRole("progressbar", { name: "Mix preview progress" }).getAttribute("aria-valuenow")).toBe("60");

      await first.rerender({
        items: [{
          ...mixed[0],
          progress: { value: 80 },
        }],
        defaultOpen: true,
      });
      expect(screen.getByRole("progressbar", { name: "Mix preview progress" }).getAttribute("aria-valuenow")).toBe("80");
    });
  });
});
