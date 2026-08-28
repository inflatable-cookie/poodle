/**
 * Drag-and-drop semantic kernel — the claims the shared vectors cannot state.
 *
 * The lifecycle, ordering, and inertia claims live in the cross-language
 * corpus (packages/contracts/headless/vectors/machines.json, `dragDrop`). What
 * is left here is TypeScript-side purity: the transition must not mutate the
 * caller's state, and the resolver's optional `priority` must default without
 * the vectors having to spell a zero into every candidate.
 */

import { describe, expect, test } from "bun:test";

import {
  dragSessionTransition,
  resolveDropTarget,
  type DragSession,
  type DragSessionContext,
  type DropIntent,
  type DropTargetCandidate,
} from "../src/drag-drop";

const subject = { kind: "track", id: "t1" } as const;

const intent: DropIntent = { targetId: "list", position: "before", operation: "move" };

function dragging(): { context: DragSessionContext; session: DragSession } {
  const session: DragSession = {
    sessionId: "s1",
    sourceId: "src-a",
    subject: { ...subject },
    operation: "move",
    allowedOperations: ["move", "copy"],
    intent: null,
  };

  return { context: { session }, session };
}

describe("dragSessionTransition purity", () => {
  test("an accepted intent leaves the caller's session untouched", () => {
    const { context, session } = dragging();

    const result = dragSessionTransition("dragging", context, {
      type: "TARGET_INTENT",
      sessionId: "s1",
      intent,
    });

    expect(session.intent).toBeNull();
    expect(context.session).toBe(session);
    expect(result.context.session?.intent).toEqual(intent);
    expect(result.context.session).not.toBe(session);
  });

  test("preparation copies the caller's allowed operations", () => {
    const allowedOperations: ("move" | "copy")[] = ["move", "copy"];

    const result = dragSessionTransition(
      "idle",
      { session: null },
      {
        type: "PREPARE",
        sessionId: "s1",
        sourceId: "src-a",
        subject,
        operation: "move",
        allowedOperations,
      },
    );

    expect(result.context.session?.allowedOperations).toEqual(allowedOperations);
    expect(result.context.session?.allowedOperations).not.toBe(allowedOperations);
  });

  test("an inert event returns the caller's own context", () => {
    const { context } = dragging();

    const result = dragSessionTransition("dragging", context, {
      type: "DROP_REQUESTED",
      sessionId: "s1",
    });

    expect(result.state).toBe("dragging");
    expect(result.context).toBe(context);
    expect(result.effects).toEqual([]);
  });
});

describe("resolveDropTarget", () => {
  function candidate(overrides: Partial<DropTargetCandidate> & { targetId: string }): DropTargetCandidate {
    return {
      depth: 0,
      order: 0,
      containsPoint: true,
      eligibility: {
        accepted: true,
        intent: { targetId: overrides.targetId, position: "inside", operation: "move" },
      },
      ...overrides,
    };
  }

  test("an omitted priority reads as zero rather than losing to an explicit zero", () => {
    const winner = resolveDropTarget([
      candidate({ targetId: "explicit", depth: 1, order: 0, priority: 0 }),
      candidate({ targetId: "implicit", depth: 1, order: 1 }),
    ]);

    expect(winner?.targetId).toBe("explicit");
  });

  test("a negative priority loses to an omitted one at equal depth", () => {
    const winner = resolveDropTarget([
      candidate({ targetId: "demoted", depth: 1, order: 0, priority: -1 }),
      candidate({ targetId: "default", depth: 1, order: 1 }),
    ]);

    expect(winner?.targetId).toBe("default");
  });

  test("candidate order in the input does not change the winner", () => {
    const candidates = [
      candidate({ targetId: "root", depth: 0, order: 0 }),
      candidate({ targetId: "row", depth: 2, order: 2 }),
      candidate({ targetId: "group", depth: 1, order: 1 }),
    ];

    expect(resolveDropTarget(candidates)?.targetId).toBe("row");
    expect(resolveDropTarget([...candidates].reverse())?.targetId).toBe("row");
  });

  test("the returned intent is the winning target's own eligibility intent", () => {
    const accepted: DropIntent = { targetId: "row", position: "after", operation: "copy" };

    const winner = resolveDropTarget([
      candidate({ targetId: "root", depth: 0, order: 0 }),
      candidate({ targetId: "row", depth: 1, order: 1, eligibility: { accepted: true, intent: accepted } }),
    ]);

    expect(winner).toEqual(accepted);
  });
});
