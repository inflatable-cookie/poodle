import { cleanup, fireEvent, render } from "@testing-library/svelte";
import { afterEach, describe, expect, it } from "vitest";

import Harness from "./PickerInDialogHarness.svelte";

afterEach(cleanup);

const CASES = [
  {
    picker: "date" as const,
    trigger: ".poodle-date-picker__trigger",
    surface: ".poodle-date-picker__surface",
  },
  {
    picker: "date-range" as const,
    trigger: ".poodle-date-range-picker__trigger",
    surface: ".poodle-date-range-picker__surface",
  },
  {
    picker: "date-time" as const,
    trigger: ".poodle-date-time-picker__trigger",
    surface: ".poodle-date-time-picker__surface",
  },
  {
    picker: "date-time-range" as const,
    trigger: ".poodle-date-time-range-picker__trigger",
    surface: ".poodle-date-time-range-picker__surface",
  },
  {
    picker: "color" as const,
    trigger: ".poodle-color-picker__trigger",
    surface: ".poodle-color-picker__surface",
  },
];

describe("Picker inside Dialog (svelte)", () => {
  for (const { picker, trigger, surface } of CASES) {
    it(`${picker}: one Escape closes only the picker; second Escape closes the dialog`, async () => {
      render(Harness, { props: { picker } });

      expect(document.querySelector(".poodle-dialog")).not.toBeNull();
      const triggerButton = document.querySelector<HTMLButtonElement>(trigger)!;
      await fireEvent.click(triggerButton);
      expect(document.querySelector(surface)).not.toBeNull();

      await fireEvent.keyDown(document, { key: "Escape" });
      expect(document.querySelector(surface)).toBeNull();
      expect(document.querySelector(".poodle-dialog")).not.toBeNull();

      await fireEvent.keyDown(document, { key: "Escape" });
      expect(document.querySelector(".poodle-dialog")).toBeNull();
    });

    it(`${picker}: mousedown inside the portalled surface keeps both picker and dialog open`, async () => {
      render(Harness, { props: { picker } });

      const triggerButton = document.querySelector<HTMLButtonElement>(trigger)!;
      await fireEvent.click(triggerButton);
      const surfaceEl = document.querySelector<HTMLElement>(surface)!;
      expect(surfaceEl).not.toBeNull();

      await fireEvent.mouseDown(surfaceEl);
      expect(document.querySelector(surface)).not.toBeNull();
      expect(document.querySelector(".poodle-dialog")).not.toBeNull();
    });
  }
});
