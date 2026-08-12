import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { NavigationMenu } from "../src/NavigationMenu";
import type { NavigationMenuItem } from "../src/types";

const items: NavigationMenuItem[] = [
  { value: "overview", label: "Overview" },
  { value: "releases", label: "Releases" },
];

describe("NavigationMenu (react) dismissOnOutsideInteract", () => {
  const triggerOf = (container: HTMLElement, label: string) =>
    Array.from(container.querySelectorAll<HTMLButtonElement>(".poodle-navigation-menu__trigger")).find(
      (trigger) => trigger.textContent?.trim() === label,
    ) as HTMLButtonElement;

  const viewportOf = (container: HTMLElement) =>
    container.querySelector(".poodle-navigation-menu__viewport") as HTMLElement;

  it("closes the open panel on outside mousedown by default", async () => {
    const { container } = render(<NavigationMenu items={items} />);
    await fireEvent.click(triggerOf(container, "Overview"));
    expect(viewportOf(container)).not.toBeNull();

    await fireEvent.mouseDown(document.body);
    expect(viewportOf(container)).toBeNull();
  });

  it("keeps the panel open on outside mousedown when dismissOnOutsideInteract=false", async () => {
    const { container } = render(
      <NavigationMenu items={items} dismissOnOutsideInteract={false} />,
    );
    await fireEvent.click(triggerOf(container, "Overview"));
    expect(viewportOf(container)).not.toBeNull();

    await fireEvent.mouseDown(document.body);
    expect(viewportOf(container)).not.toBeNull();
  });
});
