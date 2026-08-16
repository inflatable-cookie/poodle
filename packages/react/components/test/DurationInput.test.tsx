import { fireEvent, render } from "@testing-library/react";
import { useState } from "react";
import { describe, expect, it, vi } from "vitest";

import { DurationInput } from "../src/DurationInput";

function field(container: HTMLElement, label: string): HTMLInputElement {
  return container.querySelector(`[aria-label="${label}"]`) as HTMLInputElement;
}

interface Segments {
  hours: number;
  minutes: number;
  seconds: number;
}

function Host(props: Segments & { maxHours?: number; onCommit?: (next: Segments) => void }) {
  const [value, setValue] = useState<Segments>({
    hours: props.hours,
    minutes: props.minutes,
    seconds: props.seconds,
  });
  return (
    <DurationInput
      hours={value.hours}
      minutes={value.minutes}
      seconds={value.seconds}
      maxHours={props.maxHours}
      onChange={(next) => {
        setValue({ hours: next.hours, minutes: next.minutes, seconds: next.seconds });
        props.onCommit?.(next);
      }}
    />
  );
}

describe("DurationInput (react)", () => {
  it("carries seconds over to minutes and hours on ArrowUp and reports totals", () => {
    const onCommit = vi.fn();
    const { container } = render(<Host hours={0} minutes={59} seconds={59} onCommit={onCommit} />);

    fireEvent.keyDown(field(container, "Seconds"), { key: "ArrowUp" });

    expect(onCommit).toHaveBeenCalledWith({ hours: 1, minutes: 0, seconds: 0, totalSeconds: 3600 });
    expect(field(container, "Hours").value).toBe("01");
    expect(field(container, "Seconds").value).toBe("00");
  });

  it("borrows minutes and hours on ArrowDown", () => {
    const onCommit = vi.fn();
    const { container } = render(<Host hours={1} minutes={0} seconds={0} onCommit={onCommit} />);

    fireEvent.keyDown(field(container, "Seconds"), { key: "ArrowDown" });

    expect(onCommit).toHaveBeenCalledWith({ hours: 0, minutes: 59, seconds: 59, totalSeconds: 3599 });
    expect(field(container, "Hours").value).toBe("00");
    expect(field(container, "Minutes").value).toBe("59");
  });

  it("clamps direct numeric entry per segment and to maxHours", () => {
    const onCommit = vi.fn();
    const { container } = render(<Host hours={0} minutes={0} seconds={0} maxHours={12} onCommit={onCommit} />);
    const input = field(container, "Minutes");

    fireEvent.change(input, { target: { value: "75" } });
    expect(onCommit).toHaveBeenLastCalledWith({ hours: 0, minutes: 59, seconds: 0, totalSeconds: 3540 });

    fireEvent.change(field(container, "Hours"), { target: { value: "99" } });
    expect(onCommit).toHaveBeenLastCalledWith({ hours: 12, minutes: 59, seconds: 0, totalSeconds: 46740 });
  });

  it("swallows upward carry at the maxHours bound", () => {
    const onCommit = vi.fn();
    const { container } = render(<Host hours={99} minutes={59} seconds={0} onCommit={onCommit} />);

    fireEvent.keyDown(field(container, "Minutes"), { key: "ArrowUp" });

    expect(onCommit).toHaveBeenCalledWith({ hours: 99, minutes: 0, seconds: 0, totalSeconds: 356400 });
    expect(field(container, "Hours").value).toBe("99");
  });

  it("flags data-invalid when the total falls outside min/max bounds", () => {
    const over = render(<DurationInput hours={2} minutes={0} maxTotalSeconds={3600} />);
    expect(over.container.querySelector(".poodle-duration-input")?.getAttribute("data-invalid")).toBe("true");

    const under = render(<DurationInput hours={0} minutes={0} minTotalSeconds={60} />);
    expect(under.container.querySelector(".poodle-duration-input")?.getAttribute("data-invalid")).toBe("true");

    const valid = render(<DurationInput hours={1} minutes={0} />);
    expect(valid.container.querySelector(".poodle-duration-input")?.getAttribute("data-invalid")).toBe("false");
  });

  it("hides the seconds segment when showSeconds is false", () => {
    const { container } = render(<DurationInput hours={1} minutes={30} showSeconds={false} />);
    expect(container.querySelector('[aria-label="Seconds"]')).toBeNull();
    expect(container.querySelectorAll('[aria-label="Hours"]').length).toBe(1);
  });

  it("disables every field and ignores keyboard edits", () => {
    const onChange = vi.fn();
    const { container } = render(
      <DurationInput hours={1} minutes={30} disabled onChange={onChange} />,
    );

    expect(field(container, "Hours").disabled).toBe(true);
    expect(field(container, "Minutes").disabled).toBe(true);
    expect(field(container, "Seconds").disabled).toBe(true);

    fireEvent.keyDown(field(container, "Hours"), { key: "ArrowUp" });
    expect(onChange).not.toHaveBeenCalled();
  });
});
