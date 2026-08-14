import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { defaultLucideIconSet } from "@inflatable-cookie/poodle-core/icons";
import "@inflatable-cookie/poodle-core/styles/licence.css";
import type { LicenceKeyFormat, LicenceSeat } from "@inflatable-cookie/poodle-core";
import {
  Button,
  Icon,
  IconProvider,
  LicenceActivation,
  LicenceSeats,
  LicenceStatus,
} from "@inflatable-cookie/poodle-react";

const licenceKeyFormat: LicenceKeyFormat = {
  parse: (input) => ({ ok: true, key: input, grouped: input }),
  isProbablyATypo: () => false,
};

const seats: readonly LicenceSeat[] = [
  { machineId: "packed-seat", label: "Studio", thisMachine: true },
];

describe("packed @inflatable-cookie/poodle-react", () => {
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
});
