import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { Field } from "../src/Field";

function controlRender(api: {
  describedBy: string | null;
  descriptionId: string | null;
  errorId: string | null;
  messageId: string | null;
  validationState: string;
}) {
  return (
    <input
      id="email"
      className="harness-control"
      aria-describedby={api.describedBy ?? undefined}
      data-description-id={api.descriptionId ?? undefined}
      data-error-id={api.errorId ?? undefined}
      data-message-id={api.messageId ?? undefined}
      data-validation-state={api.validationState}
    />
  );
}

describe("Field (react)", () => {
  it("wires the label to the control and describes it with description + active error ids", () => {
    const { container } = render(
      <Field
        id="email"
        label="Email address"
        description="Shown publicly."
        error="Already taken."
        validationState="invalid"
        control={controlRender}
      />,
    );

    const label = container.querySelector<HTMLLabelElement>(".poodle-field__label");
    expect(label?.getAttribute("for")).toBe("email");

    const control = container.querySelector<HTMLInputElement>(".harness-control");
    expect(control?.getAttribute("aria-describedby")).toBe("email-description email-error");
    expect(control?.getAttribute("data-description-id")).toBe("email-description");
    expect(control?.getAttribute("data-error-id")).toBe("email-error");
    expect(control?.getAttribute("data-message-id")).toBe("email-error");
    expect(control?.getAttribute("data-validation-state")).toBe("invalid");
  });

  it("renders exactly the active validation message, error winning over pending", () => {
    const invalid = render(
      <Field id="name" label="Name" error="Too short." pendingMessage="Checking..." validationState="invalid" />,
    );
    const message = invalid.container.querySelector<HTMLParagraphElement>(".poodle-field__message");
    expect(message?.classList.contains("poodle-field__message--error")).toBe(true);
    expect(message?.classList.contains("poodle-field__message--pending")).toBe(false);
    expect(message?.textContent).toContain("Too short.");
    expect(message?.getAttribute("aria-live")).toBe("polite");

    const pending = render(
      <Field id="name" label="Name" pendingMessage="Checking..." validationState="pending" />,
    );
    const pendingMessage = pending.container.querySelector<HTMLParagraphElement>(".poodle-field__message");
    expect(pendingMessage?.classList.contains("poodle-field__message--pending")).toBe(true);
    expect(pendingMessage?.textContent).toContain("Checking...");

    const valid = render(<Field id="name" label="Name" error="Too short." validationState="valid" />);
    expect(valid.container.querySelector(".poodle-field__message")).toBeNull();
  });

  it("opens the info popover from the icon instead of rendering description inline", () => {
    const { container } = render(<Field id="title" label="Title" description="How the title appears." />);

    expect(document.querySelector(".poodle-field__info-content")).toBeNull();

    const infoIcon = container.querySelector<HTMLElement>(".poodle-field__info-icon");
    expect(infoIcon?.getAttribute("aria-label")).toBe("More information");

    const trigger = container.querySelector<HTMLElement>(".poodle-popover__trigger");
    expect(trigger?.getAttribute("aria-expanded")).toBe("false");
    fireEvent.click(trigger as HTMLElement);

    const surface = document.querySelector<HTMLElement>(".poodle-popover__surface");
    expect(surface).not.toBeNull();
    expect(surface?.getAttribute("role")).toBe("dialog");
    expect(surface.getAttribute("aria-label")).toBe("Field description");
    expect(surface.querySelector(".poodle-field__info-content")?.textContent).toBe("How the title appears.");

    const srDescription = container.querySelector<HTMLElement>(".poodle-field__sr-description");
    expect(srDescription?.getAttribute("id")).toBe("title-description");
    expect(srDescription?.textContent).toBe("How the title appears.");
  });

  it("lets description win over the deprecated hint alias in the popover", () => {
    const { container } = render(
      <Field id="title" label="Title" description="New text." hint="Old text." />,
    );

    fireEvent.click(container.querySelector(".poodle-popover__trigger") as HTMLElement);
    expect(document.querySelector(".poodle-field__info-content")?.textContent).toBe("New text.");
  });

  it("shows the required marker when required and the optional marker only when offered", () => {
    const required = render(<Field id="email" label="Email address" required />);
    expect(required.container.querySelector(".poodle-field__required")).not.toBeNull();
    expect(required.container.querySelector(".poodle-field__optional")).toBeNull();

    const optional = render(<Field id="phone" label="Phone number" optionalLabel="Optional" />);
    expect(optional.container.querySelector(".poodle-field__required")).toBeNull();
    const optionalMarker = optional.container.querySelector<HTMLElement>(".poodle-field__optional");
    expect(optionalMarker?.textContent).toBe("Optional");

    const plain = render(<Field id="phone" label="Phone number" />);
    expect(plain.container.querySelector(".poodle-field__optional")).toBeNull();
  });
});
