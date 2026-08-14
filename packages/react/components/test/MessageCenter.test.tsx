import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { MessageCenter } from "../src";
import type { MessageCenterItem } from "../src";

const items = [
  { id: "build", title: "Build complete", message: "Release candidate is ready.", read: false, tone: "success" as const },
  { id: "mention", title: "New mention", message: "Ada mentioned you in Studio.", read: true, meta: "Studio" },
];

describe("MessageCenter (react)", () => {
  it("exposes unread state on the trigger and renders the archive on open", () => {
    render(<MessageCenter items={items} />);

    const trigger = screen.getByRole("button", { name: "Notifications, 1 unread" });
    expect(trigger.getAttribute("aria-expanded")).toBe("false");

    fireEvent.click(trigger);

    expect(screen.getByRole("dialog", { name: "Notifications" })).toBeTruthy();
    expect(screen.getByText("Build complete")).toBeTruthy();
  });

  it("emits read, remove, select, and mark-all requests without mutating items", () => {
    const onReadChange = vi.fn();
    const onRemove = vi.fn();
    const onItemSelect = vi.fn();
    const onMarkAllRead = vi.fn();
    render(
      <MessageCenter
        items={items}
        defaultOpen
        onReadChange={onReadChange}
        onRemove={onRemove}
        onItemSelect={onItemSelect}
        onMarkAllRead={onMarkAllRead}
      />,
    );

    fireEvent.click(screen.getByRole("button", { name: "Build complete" }));
    fireEvent.click(screen.getByRole("button", { name: "Mark Build complete read" }));
    fireEvent.click(screen.getByRole("button", { name: "Remove Build complete" }));
    fireEvent.click(screen.getByRole("button", { name: "Mark all read" }));

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

    it("renders determinate and indeterminate live rows without interaction controls", () => {
      render(
        <MessageCenter
          items={mixed}
          defaultOpen
          onItemSelect={vi.fn()}
          onReadChange={vi.fn()}
          onRemove={vi.fn()}
        />,
      );

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

    it("keeps live rows out of the unread count", () => {
      render(<MessageCenter items={mixed} />);

      expect(screen.getByRole("button", { name: "Notifications, 1 unread" })).toBeTruthy();
    });

    it("honours per-item removability independently of the global callback", () => {
      const onRemove = vi.fn();
      const partly: MessageCenterItem[] = [
        { ...items[0], removable: false },
        { ...items[1] },
      ];
      render(<MessageCenter items={partly} defaultOpen onRemove={onRemove} />);

      expect(screen.queryByRole("button", { name: "Remove Build complete" })).toBeNull();
      expect(screen.getByRole("button", { name: "Remove New mention" })).toBeTruthy();

      fireEvent.click(screen.getByRole("button", { name: "Remove New mention" }));
      expect(onRemove).toHaveBeenCalledWith("mention");
    });

    it("renders host progress updates in place without local authority", () => {
      const { rerender } = render(
        <MessageCenter items={[{ ...mixed[0] }]} defaultOpen />,
      );
      expect(screen.getByRole("progressbar", { name: "Mix preview progress" }).getAttribute("aria-valuenow")).toBe("60");

      rerender(
        <MessageCenter items={[{ ...mixed[0], progress: { value: 80 } }]} defaultOpen />,
      );
      expect(screen.getByRole("progressbar", { name: "Mix preview progress" }).getAttribute("aria-valuenow")).toBe("80");
    });
  });
});
