import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { defaultLucideIconSet } from "@inflatable-cookie/poodle-core/icons";
import {
  Button,
  Icon,
  IconProvider,
} from "@inflatable-cookie/poodle-react";

describe("packed @inflatable-cookie/poodle-react", () => {
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
