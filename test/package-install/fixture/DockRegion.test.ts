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
  DockExternalDragSource,
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

  it("mounts the public drag seam and keeps local reorder", async () => {
    const onReorder = vi.fn();
    const end = vi.fn();
    const externalDragSource: DockExternalDragSource = {
      prepare: () => ({
        start: ({ dataTransfer }) => {
          dataTransfer.setData(
            "application/x-consumer-panel",
            "prepared-panel",
          );
        },
        end,
      }),
    };
    const { getAllByRole, getByRole } = render(DockRegion, {
      props: {
        items,
        value: "explorer",
        ariaLabel: "Consumer panels",
        onReorder,
        externalDragSource,
      },
    });
    const [firstTab, secondTab] = getAllByRole("tab");
    const dataTransfer = new DataTransfer();

    expect(getByRole("region", { name: "Consumer panels" })).toBeTruthy();
    await fireEvent.pointerDown(firstTab, { button: 0 });
    await fireEvent.dragStart(firstTab, { dataTransfer });
    await fireEvent.dragOver(secondTab, { dataTransfer });
    await fireEvent.drop(secondTab, { dataTransfer });
    await fireEvent.dragEnd(firstTab, { dataTransfer });

    expect(dataTransfer.getData("application/x-consumer-panel")).toBe(
      "prepared-panel",
    );
    expect(onReorder).toHaveBeenCalledWith(["inspector", "explorer"]);
    expect(end).toHaveBeenCalledOnce();
  });
});
