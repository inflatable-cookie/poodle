import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { defaultLucideIconSet } from "@inflatable-cookie/poodle-core/icons";
import "@inflatable-cookie/poodle-core/styles/licence.css";
import "@inflatable-cookie/poodle-core/styles/model-connection.css";
import {
  addMonthsPreservingDay,
  type LicenceKeyFormat,
  type LicenceSeat,
} from "@inflatable-cookie/poodle-core";
import {
  Button,
  Icon,
  IconProvider,
  LicenceActivation,
  LicenceSeats,
  LicenceStatus,
  ModelCatalogueEditor,
  ModelConnectionCard,
  ModelConnectionPicker,
  ModelConnectionSetup,
} from "@inflatable-cookie/poodle-react";
import { AgentPlan, AgentPlanRecord } from "@inflatable-cookie/poodle-react/markdown";

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
    badges: [] as { label: string }[],
    availability: "available" as const,
    availabilityLabel: "Available",
    isDisabled: false,
  },
];

describe("packed @inflatable-cookie/poodle-react", () => {
  it("executes the additive core-root date API from the packed tarball", () => {
    const day = new Date(Date.UTC(2026, 2, 14));
    expect(addMonthsPreservingDay(day, 1).getUTCDate()).toBe(14);
    expect(addMonthsPreservingDay(day, 1).getUTCMonth()).toBe(3);
    expect(addMonthsPreservingDay(new Date(Date.UTC(2026, 0, 31)), 1).getUTCDate()).toBe(28);
  });
  it("resolves the licence stylesheet and mounts every licence export", () => {
    const view = render(
      <>
        <LicenceStatus
          usability={{ state: "active" }}
          trustBasis={{ kind: "offlineSignature" }}
          useUntil={null}
          updateUntil={null}
          usable
          attention="none"
        />
        <LicenceActivation
          mode="key"
          keyFormat={licenceKeyFormat}
          machineLabel="Studio Mac"
        />
        <LicenceSeats seats={seats} />
      </>,
    );

    expect(view.getByRole("heading", { name: "Licence active" })).toBeTruthy();
    expect(view.getByRole("heading", { name: "Activate licence" })).toBeTruthy();
    expect(view.getByRole("heading", { name: "Activated machines" })).toBeTruthy();
  });

  it("resolves the model-connection stylesheet and mounts every model-connection export", () => {
    const view = render(
      <>
        <ModelConnectionPicker options={connectionOptions} />
        <ModelConnectionSetup options={connectionOptions} defaultValue="openai-responses" />
        <ModelConnectionCard
          id="conn-1"
          title="OpenAI · Work"
          providerLabel="OpenAI"
          readiness="ready"
          readinessLabel="Ready"
        />
        <ModelCatalogueEditor
          items={[
            {
              id: "model-alpha",
              label: "Frontier Alpha",
              providerLabel: "OpenAI",
              description: null,
              badges: [],
              visible: true,
              isDisabled: false,
            },
          ]}
        />
      </>,
    );

    expect(view.container.querySelector(".poodle-model-connection-picker")).toBeTruthy();
    expect(view.getByRole("button", { name: "Continue" })).toBeTruthy();
    expect(view.getByRole("switch", { name: /Enable OpenAI/i })).toBeTruthy();
    expect(view.container.querySelector(".poodle-model-catalogue-editor")).toBeTruthy();
  });

  it("mounts public components with the scoped default icon set", () => {
    const onClick = vi.fn();
    const view = render(
      <IconProvider icons={defaultLucideIconSet}>
        <Button leadingIcon="check" onClick={onClick}>
          Save
        </Button>
        <Icon name="check" ariaLabel="Complete" />
      </IconProvider>,
    );

    const button = view.getByRole("button", { name: "Save" });
    fireEvent.click(button);

    expect(onClick).toHaveBeenCalledOnce();
    expect(button.querySelector("svg path")).not.toBeNull();
    expect(view.getByRole("img", { name: "Complete" })).toBeTruthy();
  });

  it("mounts the AgentPlan pair from the packed markdown entry", () => {
    const view = render(
      <>
        <AgentPlan plan="1. Add the surface\n2. Wire the callbacks" />
        <AgentPlanRecord plan="1. Add the surface\n2. Wire the callbacks" status="accepted" />
      </>,
    );

    expect(view.container.querySelector(".poodle-agent-plan")).toBeTruthy();
    expect(view.getByRole("button", { name: "Accept plan" })).toBeTruthy();
    expect(view.getByRole("button", { name: "Revise" })).toBeTruthy();
    expect(view.container.querySelector(".poodle-agent-plan-record")).toBeTruthy();
    expect(view.getByRole("button", { name: "Show plan" })).toBeTruthy();
  });
});
