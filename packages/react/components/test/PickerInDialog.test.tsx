import { cleanup, fireEvent, render } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";

import { ColorPicker } from "../src/ColorPicker";
import { DatePicker } from "../src/DatePicker";
import { DateRangePicker } from "../src/DateRangePicker";
import { DateTimePicker } from "../src/DateTimePicker";
import { DateTimeRangePicker } from "../src/DateTimeRangePicker";
import { Dialog } from "../src/Dialog";

afterEach(cleanup);

const CASES = [
  {
    name: "date",
    trigger: ".poodle-date-picker__trigger",
    surface: ".poodle-date-picker__surface",
    renderPicker: () => <DatePicker defaultValue="2026-03-14" />,
  },
  {
    name: "date-range",
    trigger: ".poodle-date-range-picker__trigger",
    surface: ".poodle-date-range-picker__surface",
    renderPicker: () => <DateRangePicker defaultValue={{ start: "2026-03-14", end: "2026-03-18" }} />,
  },
  {
    name: "date-time",
    trigger: ".poodle-date-time-picker__trigger",
    surface: ".poodle-date-time-picker__surface",
    renderPicker: () => <DateTimePicker defaultValue={{ date: "2026-03-14", time: "10:00" }} />,
  },
  {
    name: "date-time-range",
    trigger: ".poodle-date-time-range-picker__trigger",
    surface: ".poodle-date-time-range-picker__surface",
    renderPicker: () => (
      <DateTimeRangePicker
        defaultValue={{
          start: { date: "2026-03-14", time: "10:00" },
          end: { date: "2026-03-18", time: "14:00" },
        }}
      />
    ),
  },
  {
    name: "color",
    trigger: ".poodle-color-picker__trigger",
    surface: ".poodle-color-picker__surface",
    renderPicker: () => <ColorPicker value="#6366f1" />,
  },
];

describe("Picker inside Dialog (react)", () => {
  for (const { name, trigger, surface, renderPicker } of CASES) {
    it(`${name}: one Escape closes only the picker; second Escape closes the dialog`, () => {
      render(
        <div data-poodle-theme-root style={{ zIndex: 0 }}>
          <Dialog defaultOpen title="Schedule" dismissOnOutsideInteract>
            {renderPicker()}
          </Dialog>
        </div>,
      );

      expect(document.querySelector(".poodle-dialog")).not.toBeNull();
      const triggerButton = document.querySelector<HTMLButtonElement>(trigger)!;
      fireEvent.click(triggerButton);
      expect(document.querySelector(surface)).not.toBeNull();

      fireEvent.keyDown(document, { key: "Escape" });
      expect(document.querySelector(surface)).toBeNull();
      expect(document.querySelector(".poodle-dialog")).not.toBeNull();

      fireEvent.keyDown(document, { key: "Escape" });
      expect(document.querySelector(".poodle-dialog")).toBeNull();
    });

    it(`${name}: mousedown inside the portalled surface keeps both picker and dialog open`, () => {
      render(
        <div data-poodle-theme-root style={{ zIndex: 0 }}>
          <Dialog defaultOpen title="Schedule" dismissOnOutsideInteract>
            {renderPicker()}
          </Dialog>
        </div>,
      );

      const triggerButton = document.querySelector<HTMLButtonElement>(trigger)!;
      fireEvent.click(triggerButton);
      const surfaceEl = document.querySelector<HTMLElement>(surface)!;
      expect(surfaceEl).not.toBeNull();

      fireEvent.mouseDown(surfaceEl);
      expect(document.querySelector(surface)).not.toBeNull();
      expect(document.querySelector(".poodle-dialog")).not.toBeNull();
    });
  }
});
