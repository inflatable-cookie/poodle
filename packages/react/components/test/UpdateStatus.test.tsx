import { act, fireEvent, render, screen, within } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { UpdateStatus } from "../src/UpdateStatus";

const ready = { kind: "ready" } as const;

const offer = {
  state: "offer",
  version: "1.4.0",
  reason: "staged",
  notes: null,
} as const;

describe("UpdateStatus (react)", () => {
  it("renders the offer with version, notes, and both actions", () => {
    render(
      <UpdateStatus
        status={ready}
        availability={{ ...offer, notes: "Bug fixes and improvements." }}
      />,
    );

    expect(screen.getByText("Version 1.4.0 is available")).toBeTruthy();
    expect(screen.getByText("Bug fixes and improvements.")).toBeTruthy();
    expect(screen.getByRole("button", { name: "Install and restart" })).toBeTruthy();
    expect(screen.getByRole("button", { name: "Later" })).toBeTruthy();
  });

  it("renders up-to-date calmly, with no actions", () => {
    render(
      <UpdateStatus
        status={ready}
        availability={{ state: "upToDate" }}
        installedVersion="1.3.0"
        channel="production"
      />,
    );

    expect(screen.getByText("You're up to date")).toBeTruthy();
    expect(screen.getByText("Version 1.3.0 · production channel")).toBeTruthy();
    expect(screen.queryAllByRole("button")).toHaveLength(0);
  });

  it("renders ahead-of-channel with both versions, distinct from up-to-date", () => {
    render(
      <UpdateStatus
        status={ready}
        availability={{ state: "aheadOfChannel", installed: "1.3.0-nightly.4", channel: "1.2.9" }}
        aheadOfChannel={{ installed: "1.3.0-nightly.4", channel: "1.2.9" }}
      />,
    );

    expect(screen.getByText("You're ahead of your channel")).toBeTruthy();
    expect(screen.getByText("Installed 1.3.0-nightly.4 · channel 1.2.9")).toBeTruthy();
    expect(screen.queryByText("You're up to date")).toBeNull();
  });

  it("renders withheld-by-rollout as a staged-not-yet state", () => {
    render(<UpdateStatus status={ready} availability={{ state: "withheldByRollout", version: "2.0.0" }} />);

    expect(screen.getByText("Version 2.0.0 exists")).toBeTruthy();
    expect(screen.getByText("Not staged to you yet.")).toBeTruthy();
  });

  it("renders managed-elsewhere with the version and the manager", () => {
    render(
      <UpdateStatus
        status={ready}
        availability={{ state: "managedElsewhere", version: "1.4.0", manager: "homebrewCask" }}
      />,
    );

    expect(screen.getByText("Version 1.4.0 is available")).toBeTruthy();
    expect(screen.getByText("Managed by Homebrew.")).toBeTruthy();
  });

  it("renders a deferral with its reason and neutral (non-error) styling", () => {
    render(
      <UpdateStatus
        status={ready}
        availability={offer}
        deferral={{ version: "1.4.0", cause: { cause: "workInFlight", detail: "A transfer is running." } }}
      />,
    );

    const notice = screen.getByRole("status");
    expect(notice.textContent).toContain("Install is on hold: A transfer is running.");
    expect(notice.getAttribute("data-tone")).toBe("neutral");
  });

  it("renders an indeterminate bar when the download fraction is null", () => {
    render(<UpdateStatus status={ready} progress={{ state: "downloading", fraction: null }} />);

    const bar = screen.getByRole("progressbar");
    expect(bar.getAttribute("data-indeterminate")).toBe("true");
    expect(bar.getAttribute("aria-valuenow")).toBeNull();
  });

  it("renders a zero bar when the download fraction is zero", () => {
    render(<UpdateStatus status={ready} progress={{ state: "downloading", fraction: 0 }} />);

    const bar = screen.getByRole("progressbar");
    expect(bar.getAttribute("data-indeterminate")).toBe("false");
    expect(bar.getAttribute("aria-valuenow")).toBe("0");
  });

  it("confirms before emitting install", async () => {
    const onInstall = vi.fn();
    render(<UpdateStatus status={ready} availability={offer} onInstall={onInstall} />);

    fireEvent.click(screen.getAllByRole("button", { name: "Install and restart" })[0]);
    expect(onInstall).not.toHaveBeenCalled();

    const dialog = screen.getByRole("alertdialog");
    // AlertDialog awaits onConfirm before it closes, so the click leaves state
    // updates in flight past fireEvent's own act() window.
    await act(async () => {
      fireEvent.click(within(dialog).getByRole("button", { name: "Install and restart" }));
    });
    expect(onInstall).toHaveBeenCalledOnce();
  });

  it("emits install directly when confirmation is disabled", () => {
    const onInstall = vi.fn();
    render(<UpdateStatus status={ready} availability={offer} confirmInstall={false} onInstall={onInstall} />);

    fireEvent.click(screen.getByRole("button", { name: "Install and restart" }));
    expect(onInstall).toHaveBeenCalledOnce();
  });

  it("emits defer without confirming", () => {
    const onDefer = vi.fn();
    render(<UpdateStatus status={ready} availability={offer} onDefer={onDefer} />);

    fireEvent.click(screen.getByRole("button", { name: "Later" }));
    expect(onDefer).toHaveBeenCalledOnce();
  });

  it("offers no retry for a signature rejection", () => {
    render(<UpdateStatus status={ready} lastRejection="signatureRejected" />);

    expect(screen.getByRole("status").textContent).toContain("signature check");
    expect(screen.queryByRole("button", { name: "Try again" })).toBeNull();
  });

  it("offers a retry for a reachable rejection", () => {
    render(<UpdateStatus status={ready} lastRejection="unreachable" />);

    expect(screen.getByRole("button", { name: "Try again" })).toBeTruthy();
  });
});
