import { describe, expect, test } from "bun:test";

import {
  filterModelConnectionOptions,
  groupModelConnectionOptions,
  hiddenModelCatalogueItems,
  modelCatalogueFocusAfterHide,
  modelCatalogueReorderAnnouncement,
  modelCatalogueVisibilityAnnouncement,
  modelConnectionAvailabilityTone,
  modelConnectionOptionSelectable,
  modelConnectionPickerResultAnnouncement,
  modelConnectionPickerStateCopy,
  modelConnectionReadinessTone,
  modelConnectionSetupCanContinue,
  modelConnectionSetupCanSubmit,
  modelConnectionSetupTransition,
  MODEL_CATALOGUE_FIXTURES,
  MODEL_CONNECTION_PICKER_FIXTURES,
  requestModelCatalogueOrder,
  requestModelCatalogueVisibility,
  resolveModelConnectionPickerShellState,
  shownModelCatalogueItems,
  type ModelConnectionOption,
  type ModelConnectionSetupContext,
} from "../src/model-connection";

const options = MODEL_CONNECTION_PICKER_FIXTURES;

function setup(
  overrides: Partial<ModelConnectionSetupContext> = {},
): ModelConnectionSetupContext {
  return {
    stage: "choose",
    value: null,
    query: "",
    options,
    canSubmit: false,
    isPending: false,
    ...overrides,
  };
}

describe("filterModelConnectionOptions", () => {
  test("retains source order and matches provider, route, description, group, keywords", () => {
    const filtered = filterModelConnectionOptions(options, "LOCAL");
    expect(filtered.map((option) => option.id)).toEqual([
      "codex-app",
      "ollama-local",
      "lmstudio-local",
    ]);
  });

  test("empty query returns the full source order", () => {
    expect(filterModelConnectionOptions(options, "  ").map((o) => o.id)).toEqual(
      options.map((o) => o.id),
    );
  });

  test("case-folded keywords hit without reshuffling groups", () => {
    const filtered = filterModelConnectionOptions(options, "responses");
    expect(filtered.map((option) => option.id)).toEqual(["openai-responses"]);
    expect(groupModelConnectionOptions(filtered).map((group) => group.group)).toEqual([
      "Hosted",
    ]);
  });
});

describe("groupModelConnectionOptions", () => {
  test("preserves first-seen group order", () => {
    expect(groupModelConnectionOptions(options).map((group) => group.group)).toEqual([
      "Hosted",
      "Installed",
      "Local runtime",
    ]);
  });
});

describe("modelConnectionOptionSelectable", () => {
  test("only available non-disabled options are selectable", () => {
    const byId = Object.fromEntries(options.map((option) => [option.id, option])) as Record<
      string,
      ModelConnectionOption
    >;
    expect(modelConnectionOptionSelectable(byId["openai-responses"])).toBe(true);
    expect(modelConnectionOptionSelectable(byId["codex-app"])).toBe(false);
    expect(modelConnectionOptionSelectable(byId["lmstudio-local"])).toBe(false);
    expect(modelConnectionOptionSelectable(byId["vendor-legacy"])).toBe(false);
  });
});

describe("resolveModelConnectionPickerShellState", () => {
  test("maps catalogue postures and derived empty/no-results", () => {
    expect(resolveModelConnectionPickerShellState("loading", 3, 0, "")).toBe("loading");
    expect(resolveModelConnectionPickerShellState("error", 3, 0, "")).toBe("error");
    expect(resolveModelConnectionPickerShellState("ready", 0, 0, "")).toBe("empty");
    expect(resolveModelConnectionPickerShellState("empty", 4, 4, "")).toBe("empty");
    expect(resolveModelConnectionPickerShellState("ready", 4, 0, "zzz")).toBe("no-results");
    expect(resolveModelConnectionPickerShellState("noResults", 4, 0, "")).toBe("no-results");
    expect(resolveModelConnectionPickerShellState("ready", 4, 2, "open")).toBe("ready");
  });
});

describe("modelConnectionPickerResultAnnouncement", () => {
  test("names match counts", () => {
    expect(modelConnectionPickerResultAnnouncement(0, "zzz")).toContain("No connections");
    expect(modelConnectionPickerResultAnnouncement(1, "a")).toContain("1 connection matches");
    expect(modelConnectionPickerResultAnnouncement(3, "")).toBe("3 connections");
  });
});

describe("modelConnectionPickerStateCopy", () => {
  test("names every non-ready posture", () => {
    expect(modelConnectionPickerStateCopy("loading", "").title).toBe("Loading connections");
    expect(modelConnectionPickerStateCopy("error", "").title).toBe(
      "Could not load connections",
    );
    expect(modelConnectionPickerStateCopy("empty", "").title).toBe(
      "No connections available",
    );
    expect(modelConnectionPickerStateCopy("no-results", "local")).toEqual({
      title: "No matching connections",
      message: "No connections match “local”.",
    });
  });
});

describe("modelConnectionSetupTransition", () => {
  test("continue requires a selectable exact id", () => {
    expect(modelConnectionSetupCanContinue(setup({ value: null }))).toBe(false);
    expect(modelConnectionSetupCanContinue(setup({ value: "codex-app" }))).toBe(false);
    expect(modelConnectionSetupCanContinue(setup({ value: "openai-responses" }))).toBe(true);

    const blocked = modelConnectionSetupTransition(setup({ value: null }), { type: "CONTINUE" });
    expect(blocked.effects).toEqual([]);

    const ok = modelConnectionSetupTransition(setup({ value: "openai-responses" }), {
      type: "CONTINUE",
    });
    expect(ok.context.stage).toBe("configure");
    expect(ok.effects).toEqual([{ type: "emitStageChange", stage: "configure" }]);
  });

  test("submit requires configure, canSubmit, and exact id", () => {
    expect(
      modelConnectionSetupCanSubmit(
        setup({ stage: "configure", value: "openai-responses", canSubmit: false }),
      ),
    ).toBe(false);

    const denied = modelConnectionSetupTransition(
      setup({ stage: "configure", value: "openai-responses", canSubmit: false }),
      { type: "SUBMIT" },
    );
    expect(denied.effects).toEqual([]);

    const accepted = modelConnectionSetupTransition(
      setup({ stage: "configure", value: "openai-responses", canSubmit: true }),
      { type: "SUBMIT" },
    );
    expect(accepted.effects).toEqual([{ type: "emitSubmit", id: "openai-responses" }]);
  });

  test("pending guards workflow events", () => {
    const pending = setup({
      stage: "configure",
      value: "openai-responses",
      canSubmit: true,
      isPending: true,
    });
    expect(modelConnectionSetupTransition(pending, { type: "SUBMIT" }).effects).toEqual([]);
    expect(modelConnectionSetupTransition(pending, { type: "BACK" }).effects).toEqual([]);
    expect(modelConnectionSetupTransition(pending, { type: "CANCEL" }).effects).toEqual([]);
    expect(
      modelConnectionSetupTransition(pending, { type: "SELECT", id: "anthropic-messages" })
        .effects,
    ).toEqual([]);
  });

  test("back returns to choose without clearing selection", () => {
    const result = modelConnectionSetupTransition(
      setup({ stage: "configure", value: "openai-responses" }),
      { type: "BACK" },
    );
    expect(result.context).toMatchObject({ stage: "choose", value: "openai-responses" });
    expect(result.effects).toEqual([{ type: "emitStageChange", stage: "choose" }]);
  });

  test("select emits exact opaque ids only for selectable options", () => {
    const accepted = modelConnectionSetupTransition(setup(), {
      type: "SELECT",
      id: "openai-responses",
    });
    expect(accepted.effects).toEqual([{ type: "emitValueChange", id: "openai-responses" }]);

    const rejected = modelConnectionSetupTransition(setup(), {
      type: "SELECT",
      id: "vendor-legacy",
    });
    expect(rejected.effects).toEqual([]);
  });
});

describe("catalogue order and visibility", () => {
  test("shown and hidden partitions ignore hidden order meaning", () => {
    expect(shownModelCatalogueItems(MODEL_CATALOGUE_FIXTURES).map((item) => item.id)).toEqual([
      "model-alpha",
      "model-beta",
      "model-gamma",
      "model-dup-a",
    ]);
    expect(hiddenModelCatalogueItems(MODEL_CATALOGUE_FIXTURES).map((item) => item.id)).toEqual([
      "model-dup-b",
      "model-hidden",
    ]);
  });

  test("requestModelCatalogueOrder emits the complete shown-id order", () => {
    const shown = shownModelCatalogueItems(MODEL_CATALOGUE_FIXTURES).map((item) => item.id);
    expect(requestModelCatalogueOrder(shown, 0, 2)).toEqual([
      "model-beta",
      "model-gamma",
      "model-alpha",
      "model-dup-a",
    ]);
    expect(requestModelCatalogueOrder(shown, 0, 0)).toBeNull();
  });

  test("visibility requests only carry id and visible", () => {
    expect(requestModelCatalogueVisibility("model-alpha", false)).toEqual({
      id: "model-alpha",
      visible: false,
    });
  });

  test("focus after hide follows next, previous, or hidden section", () => {
    expect(modelCatalogueFocusAfterHide(["a", "b", "c"], "b")).toEqual({
      kind: "shown",
      id: "c",
    });
    expect(modelCatalogueFocusAfterHide(["a"], "a")).toEqual({ kind: "hidden-section" });
  });

  test("announcements name the model and outcome", () => {
    expect(modelCatalogueReorderAnnouncement("Frontier Alpha", 2, 4)).toBe(
      "Moved Frontier Alpha to position 2 of 4.",
    );
    expect(modelCatalogueVisibilityAnnouncement("Archive Delta", false)).toBe(
      "Hid Archive Delta.",
    );
    expect(modelCatalogueVisibilityAnnouncement("Archive Delta", true)).toBe(
      "Restored Archive Delta.",
    );
  });
});

describe("status tones", () => {
  test("map availability and readiness without collapsing dimensions", () => {
    expect(modelConnectionAvailabilityTone("available")).toBe("success");
    expect(modelConnectionAvailabilityTone("unsupported")).toBe("neutral");
    expect(modelConnectionReadinessTone("attention")).toBe("warning");
    expect(modelConnectionReadinessTone("error")).toBe("danger");
  });
});
