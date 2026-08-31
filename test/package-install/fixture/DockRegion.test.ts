import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import "@inflatable-cookie/poodle-core/styles/licence.css";
import "@inflatable-cookie/poodle-core/styles/model-connection.css";
import {
  DockRegion,
  LicenceActivation,
  LicenceSeats,
  LicenceStatus,
  ModelCatalogueEditor,
  ModelConnectionCard,
  ModelConnectionPicker,
  ModelConnectionSetup,
} from "@inflatable-cookie/poodle-svelte";
import type { LicenceKeyFormat, LicenceSeat } from "@inflatable-cookie/poodle-core";
import type {
  CrossWindowDragSourceBridge,
  PanelTabItem,
} from "@inflatable-cookie/poodle-svelte";

const items: PanelTabItem[] = [
  { value: "explorer", label: "Explorer" },
  { value: "inspector", label: "Inspector" },
];

const licenceKeyFormat: LicenceKeyFormat = {
  parse: (input) => ({ ok: true, key: input, grouped: input }),
  isProbablyATypo: () => false,
};

const seats: readonly LicenceSeat[] = [
  { machineId: "packed-seat", label: "Studio", thisMachine: true },
];

const connectionOptions = [
  {
    id: "openai-responses",
    providerLabel: "OpenAI",
    routeLabel: "Responses API",
    description: "Hosted route",
    group: "Hosted",
    keywords: ["openai"],
    badges: [],
    availability: "available" as const,
    availabilityLabel: "Available",
    isDisabled: false,
  },
];

describe("packed @inflatable-cookie/poodle-svelte", () => {
  it("resolves the licence stylesheet and mounts every licence export", () => {
    const status = render(LicenceStatus, {
      props: {
        usability: { state: "active" },
        trustBasis: { kind: "offlineSignature" },
        useUntil: null,
        updateUntil: null,
        usable: true,
        attention: "none",
      },
    });
    const activation = render(LicenceActivation, {
      props: {
        mode: "key",
        keyFormat: licenceKeyFormat,
        machineLabel: "Studio Mac",
      },
    });
    const seatList = render(LicenceSeats, { props: { seats } });

    expect(status.getByRole("heading", { name: "Licence active" })).toBeTruthy();
    expect(activation.getByRole("heading", { name: "Activate licence" })).toBeTruthy();
    expect(seatList.getByRole("heading", { name: "Activated machines" })).toBeTruthy();
    status.unmount();
    activation.unmount();
    seatList.unmount();
  });

  it("resolves the model-connection stylesheet and mounts every model-connection export", () => {
    const picker = render(ModelConnectionPicker, {
      props: { options: connectionOptions },
    });
    const setup = render(ModelConnectionSetup, {
      props: { options: connectionOptions, defaultValue: "openai-responses" },
    });
    const card = render(ModelConnectionCard, {
      props: {
        id: "conn-1",
        title: "OpenAI · Work",
        providerLabel: "OpenAI",
        readiness: "ready",
        readinessLabel: "Ready",
      },
    });
    const catalogue = render(ModelCatalogueEditor, {
      props: {
        items: [
          {
            id: "model-alpha",
            label: "Frontier Alpha",
            providerLabel: "OpenAI",
            description: null,
            badges: [],
            visible: true,
            isDisabled: false,
          },
        ],
      },
    });

    expect(
      picker.container.querySelector(".poodle-model-connection-picker"),
    ).toBeTruthy();
    expect(setup.getByRole("button", { name: "Continue" })).toBeTruthy();
    expect(card.getByRole("switch", { name: /Enable OpenAI/i })).toBeTruthy();
    expect(
      catalogue.container.querySelector(".poodle-model-catalogue-editor"),
    ).toBeTruthy();
    picker.unmount();
    setup.unmount();
    card.unmount();
    catalogue.unmount();
  });

  it("mounts the public cross-window seam and keeps local reorder", async () => {
    const onReorder = vi.fn();
    const prepare = vi.fn();

    // The public seam is one semantic bridge now, not three DOM-event
    // callbacks. Packing has to keep both the prop and its type.
    const crossWindowDragSource: CrossWindowDragSourceBridge = {
      capabilities: { pointer: true, touch: false, keyboardTargetPicker: false },
      prepare: (request) => {
        prepare(request.sourceId);
        return Promise.resolve(null);
      },
      start: () => () => {},
      cancel: () => {},
    };

    const { getAllByRole, getByRole } = render(DockRegion, {
      props: {
        items,
        value: "explorer",
        ariaLabel: "Consumer panels",
        onReorder,
        crossWindowDragSource,
      },
    });
    const [firstTab] = getAllByRole("tab");

    expect(getByRole("region", { name: "Consumer panels" })).toBeTruthy();

    // No host has armed a receipt, so nothing advertises a native drag and
    // nothing reaches `DataTransfer`.
    const dataTransfer = new DataTransfer();
    await fireEvent.dragStart(firstTab, { dataTransfer });
    expect([...dataTransfer.types]).toEqual([]);
    expect(firstTab.getAttribute("draggable")).toBe("false");

    // Local reorder still runs, through the shared substrate's keyboard route.
    firstTab.focus();
    await fireEvent.keyDown(firstTab, { key: "ArrowRight", altKey: true });
    expect(onReorder).toHaveBeenCalledWith(["inspector", "explorer"]);
  });
});
