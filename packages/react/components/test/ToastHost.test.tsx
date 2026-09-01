import { act, fireEvent, render, waitFor } from "@testing-library/react";
import { writable } from "svelte/store";
import { afterEach, describe, expect, it, vi } from "vitest";

import { ToastHost } from "../src/ToastHost";
import type { ToastHostStore, ToastHostStoreItem } from "../src/types";

function makeStore(initial: ToastHostStoreItem[] = []) {
  const store = writable(initial);
  const hostStore: ToastHostStore = {
    toasts: store,
    dismiss: (id) => store.update((current) => current.filter((toast) => toast.id !== id)),
  };
  return { store, hostStore };
}

function getStoreSnapshot(store: ReturnType<typeof writable<ToastHostStoreItem[]>>) {
  let snapshot: ToastHostStoreItem[] = [];
  store.subscribe((value) => {
    snapshot = value;
  })();
  return snapshot;
}

describe("ToastHost (react)", () => {
  afterEach(() => {
    vi.useRealTimers();
  });

  it("renders nothing while the store is empty", () => {
    const { hostStore } = makeStore([]);
    const { container } = render(<ToastHost store={hostStore} />);
    expect(container.querySelector(".poodle-toast-host")).toBeNull();
  });

  it("renders the stack at the configured placement when toasts exist", () => {
    const { hostStore } = makeStore([
      { id: "t1", title: "Saved", message: "Done.", tone: "success" },
    ]);
    const { container } = render(<ToastHost store={hostStore} placement="top-start" />);
    const host = container.querySelector(".poodle-toast-host") as HTMLElement;
    expect(host.dataset.placement).toBe("top-start");
    expect(container.querySelector(".poodle-toast")).not.toBeNull();
  });

  it("normalises the legacy variant field to a tone", async () => {
    const { hostStore } = makeStore([
      { id: "t1", message: "Publishing failed.", variant: "error" },
    ]);
    const { container } = render(<ToastHost store={hostStore} />);
    await waitFor(() => {
      expect(container.querySelector(".poodle-toast")).not.toBeNull();
    });
    const toast = container.querySelector(".poodle-toast") as HTMLElement;
    expect(toast.dataset.tone).toBe("danger");
    expect(toast.textContent).toContain("Publishing failed.");
  });

  it("auto-dismisses non-sticky toasts after the configured delay", async () => {
    const { store, hostStore } = makeStore([
      { id: "t1", title: "Saved", tone: "success" },
      { id: "t2", title: "Deploy failed", tone: "danger" },
    ]);
    render(<ToastHost store={hostStore} autoDismissMs={20} />);

    await waitFor(
      () => {
        const current = getStoreSnapshot(store);
        expect(current.map((toast) => toast.id)).toEqual(["t2"]);
      },
      { timeout: 2000 },
    );
  });

  it("keeps sticky toasts until explicitly dismissed", async () => {
    const { store, hostStore } = makeStore([
      { id: "t1", title: "Sticky", tone: "warning", sticky: true },
    ]);
    render(<ToastHost store={hostStore} autoDismissMs={20} />);
    await new Promise((resolve) => setTimeout(resolve, 100));

    const current = getStoreSnapshot(store);
    expect(current.map((toast) => toast.id)).toEqual(["t1"]);
  });

  it("dismisses through the store and reports onDismiss", async () => {
    const onDismiss = vi.fn();
    const { hostStore } = makeStore([{ id: "t1", title: "Saved", tone: "success" }]);
    const { container } = render(<ToastHost store={hostStore} onDismiss={onDismiss} />);
    await waitFor(() => {
      expect(container.querySelector(".poodle-toast")).not.toBeNull();
    });

    const dismiss = container.querySelector(
      'button[aria-label="Dismiss Saved"]',
    ) as HTMLButtonElement;
    fireEvent.click(dismiss);
    expect(onDismiss).toHaveBeenCalledWith("t1");
  });

  it("forwards the action callback from the toast action button", async () => {
    const onAction = vi.fn();
    const { hostStore } = makeStore([
      { id: "t1", title: "New version", actionLabel: "Update", tone: "info" },
    ]);
    const { container } = render(<ToastHost store={hostStore} onAction={onAction} />);
    await waitFor(() => {
      expect(container.querySelector(".poodle-toast")).not.toBeNull();
    });

    const action = [...container.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Update"),
    ) as HTMLButtonElement;
    fireEvent.click(action);
    expect(onAction).toHaveBeenCalledWith("t1");
  });

  it("clears a running clock when the same id becomes sticky", async () => {
    const { store, hostStore } = makeStore([{ id: "job", title: "Saving", tone: "info" }]);
    render(<ToastHost store={hostStore} autoDismissMs={20} />);
    act(() => {
      store.set([{ id: "job", title: "Failed", tone: "danger" }]);
    });
    await act(async () => {
      await new Promise((resolve) => setTimeout(resolve, 80));
    });
    expect(getStoreSnapshot(store).map((toast) => toast.id)).toEqual(["job"]);
  });

  it("starts the configured delay when sticky pending settles to success", async () => {
    vi.useFakeTimers();
    const { store, hostStore } = makeStore([{ id: "job", title: "Publishing", sticky: true }]);
    render(<ToastHost store={hostStore} autoDismissMs={2500} />);
    act(() => {
      store.set([{ id: "job", title: "Published", tone: "success" }]);
    });
    await act(async () => {
      await vi.advanceTimersByTimeAsync(2499);
    });
    expect(getStoreSnapshot(store).map((toast) => toast.id)).toEqual(["job"]);
    await act(async () => {
      await vi.advanceTimersByTimeAsync(2);
    });
    expect(getStoreSnapshot(store).map((toast) => toast.id)).toEqual([]);
  });

  it("keeps one live row when the store repeats an id", async () => {
    const { hostStore } = makeStore([
      { id: "job", title: "First" },
      { id: "job", title: "Last", tone: "success" },
    ]);
    const { container } = render(<ToastHost store={hostStore} />);
    await waitFor(() => {
      expect(container.querySelectorAll(".poodle-toast").length).toBe(1);
    });
    expect(container.querySelector(".poodle-toast")?.textContent).toContain("Last");
  });
});