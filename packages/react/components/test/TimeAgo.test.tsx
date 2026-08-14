import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { TimeAgo } from "../src/TimeAgo";

describe("TimeAgo (react)", () => {
  it("can place the future direction after the duration", () => {
    const datetime = Date.now() + 5 * 60_000 + 30_000;
    const { container } = render(
      <TimeAgo datetime={datetime} live={false} futureFormat="from-now" />,
    );

    expect(container.querySelector("time")?.textContent).toBe("5m from now");
  });

  it("keeps the standalone future form as the default", () => {
    const datetime = Date.now() + 5 * 60_000 + 30_000;
    const { container } = render(<TimeAgo datetime={datetime} live={false} />);

    expect(container.querySelector("time")?.textContent).toBe("in 5m");
  });

  it("can keep a deadline phrase grammatical on either side of the timestamp", () => {
    const future = render(
      <TimeAgo
        datetime={Date.now() + 5 * 60_000 + 30_000}
        live={false}
        futurePrefix="ends"
        pastPrefix="ended"
      />,
    );
    expect(future.container.querySelector("time")?.textContent).toBe("ends in 5m");

    const past = render(
      <TimeAgo
        datetime={Date.now() - 5 * 60_000 - 30_000}
        live={false}
        futurePrefix="ends"
        pastPrefix="ended"
      />,
    );
    expect(past.container.querySelector("time")?.textContent).toBe("ended 5m ago");
  });
});
