import { fireEvent, render, waitFor } from "@testing-library/svelte";
import { writable } from "svelte/store";
import { afterEach, describe, expect, it, vi } from "vitest";

import ToastHost from "../src/ToastHost.svelte";
import type { ToastHostStore, ToastHostStoreItem } from "../src/types";

function makeStore(initial: ToastHostStoreItem[] = []) {
  const store = writable<ToastHostStoreItem[]>(initial);
  const hostStore: ToastHostStore = {
    toasts: store,
    dismiss: (id) =>
      store.update((current) => current.filter((toast) => toast.id !== id)),
  };
  return { store, hostStore };
}

describe("ToastHost (svelte)", () => {
  afterEach(() => {
    vi.useRealTimers();
  });

  it("renders nothing while the store is empty", () => {
    const { hostStore } = makeStore([]);
    const { container } = render(ToastHost, { props: { store: hostStore } });
    expect(container.querySelector(".poodle-toast-host")).toBeNull();
  });

  it("renders the stack at the configured placement when toasts exist", () => {
    const { hostStore } = makeStore([
      { id: "t1", title: "Saved", message: "Done.", tone: "success" },
    ]);
    const { container } = render(ToastHost, {
      props: { store: hostStore, placement: "top-start" },
    });
    const host = container.querySelector(".poodle-toast-host") as HTMLElement;
    expect(host.dataset.placement).toBe("top-start");
    expect(container.querySelector(".poodle-toast")).not.toBeNull();
  });

  it("normalises the legacy variant field to a tone", async () => {
    const { hostStore } = makeStore([
      { id: "t1", message: "Publishing failed.", variant: "error" },
    ]);
    const { container } = render(ToastHost, { props: { store: hostStore } });
    await waitFor(() => {
      expect(container.querySelector(".poodle-toast")).not.toBeNull();
    });
    const toast = container.querySelector(".poodle-toast") as HTMLElement;
    expect(toast.dataset.tone).toBe("danger");
    expect(toast.textContent).toContain("Publishing failed.");
  });

  it("auto-dismisses non-sticky toasts after the configured delay", async () => {
    vi.useFakeTimers();
    const { store, hostStore } = makeStore([
      { id: "t1", title: "Saved", message: "Done.", tone: "success" },
      { id: "t2", title: "Deploy failed", message: "Boom.", tone: "danger" },
    ]);
    render(ToastHost, { props: { store: hostStore, autoDismissMs: 100 } });

    await vi.advanceTimersByTimeAsync(150);

    await waitFor(() => {
      const current = getStoreSnapshot(store);
      expect(current.map((toast) => toast.id)).toEqual(["t2"]);
    });
  });

  it("keeps sticky toasts until explicitly dismissed", async () => {
    vi.useFakeTimers();
    const { store, hostStore } = makeStore([
      { id: "t1", title: "Sticky", message: "Sticky.", tone: "warning", sticky: true },
    ]);
    render(ToastHost, { props: { store: hostStore, autoDismissMs: 100 } });
    await vi.advanceTimersByTimeAsync(300);

    await waitFor(() => {
      const current = getStoreSnapshot(store);
      expect(current.map((toast) => toast.id)).toEqual(["t1"]);
    });
  });

  it("dismisses through the store and reports onDismiss", async () => {
    const onDismiss = vi.fn();
    const { hostStore } = makeStore([
      { id: "t1", title: "Saved", message: "Done.", tone: "success" },
    ]);
    const { container } = render(ToastHost, { props: { store: hostStore, onDismiss } });
    await waitFor(() => {
      expect(container.querySelector(".poodle-toast")).not.toBeNull();
    });

    const dismiss = container.querySelector(
      'button[aria-label="Dismiss Saved"]',
    ) as HTMLButtonElement;
    await fireEvent.click(dismiss);
    expect(onDismiss).toHaveBeenCalledWith("t1");
  });

  it("forwards the action callback from the toast action button", async () => {
    const onAction = vi.fn();
    const { hostStore } = makeStore([
      { id: "t1", title: "New version", message: "v2.1", actionLabel: "Update", tone: "info" },
    ]);
    const { container } = render(ToastHost, { props: { store: hostStore, onAction } });
    await waitFor(() => {
      expect(container.querySelector(".poodle-toast")).not.toBeNull();
    });

    const action = [...container.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Update"),
    ) as HTMLButtonElement;
    await fireEvent.click(action);
    expect(onAction).toHaveBeenCalledWith("t1");
  });

  it("clears a running clock when the same id becomes sticky", async () => {
    vi.useFakeTimers();
    const { store, hostStore } = makeStore([
      { id: "job", title: "Saving", tone: "info" },
    ]);
    render(ToastHost, { props: { store: hostStore, autoDismissMs: 100 } });
    store.set([{ id: "job", title: "Failed", tone: "danger" }]);
    await vi.advanceTimersByTimeAsync(300);
    expect(getStoreSnapshot(store).map((toast) => toast.id)).toEqual(["job"]);
  });

  it("starts the configured delay when sticky pending settles to success", async () => {
    vi.useFakeTimers();
    const { store, hostStore } = makeStore([
      { id: "job", title: "Publishing", sticky: true },
    ]);
    render(ToastHost, { props: { store: hostStore, autoDismissMs: 2500 } });
    store.set([{ id: "job", title: "Published", tone: "success" }]);
    await vi.advanceTimersByTimeAsync(2499);
    expect(getStoreSnapshot(store).map((toast) => toast.id)).toEqual(["job"]);
    await vi.advanceTimersByTimeAsync(2);
    expect(getStoreSnapshot(store).map((toast) => toast.id)).toEqual([]);
  });

  it("keeps one live row when the store repeats an id", async () => {
    const { hostStore } = makeStore([
      { id: "job", title: "First" },
      { id: "job", title: "Last", tone: "success" },
    ]);
    const { container } = render(ToastHost, { props: { store: hostStore } });
    await waitFor(() => {
      expect(container.querySelectorAll(".poodle-toast").length).toBe(1);
    });
    expect(container.querySelector(".poodle-toast")?.textContent).toContain("Last");
  });
});

function getStoreSnapshot(store: ReturnType<typeof writable<ToastHostStoreItem[]>>) {
  let snapshot: ToastHostStoreItem[] = [];
  store.subscribe((value) => {
    snapshot = value;
  })();
  return snapshot;
}