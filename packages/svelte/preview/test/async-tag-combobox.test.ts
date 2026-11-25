import { fireEvent, render, screen } from "@testing-library/svelte";
import { tick } from "svelte";
import type { ComponentProps } from "svelte";
import { describe, expect, it, vi } from "vitest";
import TagCombobox from "../src/recipes/async-tag-combobox/TagCombobox.svelte";

type Search = ComponentProps<typeof TagCombobox>["searchTags"];
const design = { id: "design", label: "Design" };
const research = { id: "research", label: "Research" };
const archive = { id: "archive", label: "Archive", disabled: true };

function mountCombobox(searchTags: Search) {
  const view = render(TagCombobox, { searchTags });
  const input = screen.getByRole("combobox", { name: "Tags" });
  input.focus();
  return { input, ...view };
}

function key(input: HTMLElement, key: string, modifiers: KeyboardEventInit = {}) {
  return fireEvent.keyDown(input, { key, ...modifiers });
}

function activeOption(input: HTMLElement) {
  const id = input.getAttribute("aria-activedescendant");
  return id ? document.getElementById(id) : null;
}

function pendingSearch() {
  let resolve: (tags: Awaited<ReturnType<Search>>) => void;
  let reject: (error: Error) => void;
  const promise = new Promise<Awaited<ReturnType<Search>>>((fulfill, fail) => {
    resolve = fulfill;
    reject = fail;
  });
  return { promise, resolve, reject };
}

describe("async tag combobox recipe", () => {
  it("keeps input focus while navigating and commits tags across disjoint searches", async () => {
    const { input } = mountCombobox(async (query) => query === "re" ? [research] : [design, archive, research]);

    await key(input, "ArrowUp");
    const listbox = await screen.findByRole("listbox", { name: "Tag suggestions" });
    expect(input.getAttribute("aria-controls")).toBe(listbox.id);
    expect(input.getAttribute("aria-expanded")).toBe("true");
    expect(activeOption(input)?.textContent).toBe("Research");

    await key(input, "ArrowDown");
    expect(activeOption(input)?.textContent).toBe("Design");
    await key(input, "ArrowDown");
    expect(activeOption(input)?.textContent).toBe("Research");
    await key(input, "ArrowUp");
    expect(activeOption(input)?.getAttribute("aria-selected")).toBe("true");
    expect(activeOption(input)?.textContent).toBe("Design");
    expect(screen.getByRole("option", { name: "Archive" }).getAttribute("aria-disabled")).toBe("true");
    expect(document.activeElement).toBe(input);

    await key(input, "Enter");
    expect(screen.queryByRole("listbox")).toBeNull();
    expect(input.getAttribute("aria-expanded")).toBe("false");
    expect(input.hasAttribute("aria-activedescendant")).toBe(false);
    expect(screen.getByText("Added Design. 1 tag selected.")).toBeTruthy();

    await fireEvent.input(input, { target: { value: "re" } });
    expect(await screen.findByRole("option", { name: "Research" })).toBeTruthy();
    expect(screen.getByText("1 suggestion.").getAttribute("role")).toBe("status");
    expect(activeOption(input)).toBeNull();
    expect(screen.getByRole("button", { name: "Remove Design" })).toBeTruthy();
    await key(input, "ArrowDown");
    await key(input, "Enter");
    expect(screen.getByRole("list", { name: "Selected tags" }).textContent).toContain("Design");
    expect(screen.getByText("Added Research. 2 tags selected.")).toBeTruthy();
    expect(document.activeElement).toBe(input);

    await fireEvent.click(input);
    expect(await screen.findByRole("option", { name: "Archive" })).toBeTruthy();
    expect(screen.queryByRole("option", { name: "Design" })).toBeNull();
    await key(input, "ArrowDown");
    expect(activeOption(input)).toBeNull();
    await key(input, "Backspace");
    expect(screen.queryByRole("button", { name: "Remove Research" })).toBeNull();
    expect(screen.getByText("Removed Research. 1 tag selected.")).toBeTruthy();
    const remove = screen.getByRole("button", { name: "Remove Design" });
    remove.focus();
    await fireEvent.click(remove);
    expect(screen.queryByRole("list", { name: "Selected tags" })).toBeNull();
    expect(document.activeElement).toBe(input);
  });

  it("preserves the input on option clicks and closes on Escape or focus leaving", async () => {
    const { input, container } = mountCombobox(async () => [design, archive]);
    await fireEvent.click(input);
    const option = await screen.findByRole("option", { name: "Design" });
    const listbox = screen.getByRole("listbox");
    expect(container.contains(listbox)).toBe(false);

    const disabled = screen.getByRole("option", { name: "Archive" });
    await fireEvent.click(disabled);
    expect(screen.queryByRole("list", { name: "Selected tags" })).toBeNull();
    expect(await fireEvent.mouseDown(option)).toBe(false);
    expect(screen.getByRole("listbox")).toBe(listbox);
    await fireEvent.click(option);
    expect(document.activeElement).toBe(input);
    expect(screen.getByRole("button", { name: "Remove Design" })).toBeTruthy();

    await fireEvent.input(input, { target: { value: "a" } });
    await screen.findByRole("listbox");
    await key(input, "Escape");
    expect(screen.queryByRole("listbox")).toBeNull();
    expect(input).toHaveProperty("value", "a");
    expect(document.activeElement).toBe(input);

    await key(input, "ArrowDown");
    await screen.findByRole("listbox");
    input.blur();
    await tick();
    expect(screen.queryByRole("listbox")).toBeNull();
  });

  it("drops stale successes and failures and cancels on outside dismissal and unmount", async () => {
    const older = pendingSearch();
    const old = pendingSearch();
    const latest = pendingSearch();
    const dismissed = pendingSearch();
    const unmounted = pendingSearch();
    const searchTags = vi.fn<Search>()
      .mockReturnValueOnce(older.promise)
      .mockReturnValueOnce(old.promise)
      .mockReturnValueOnce(latest.promise)
      .mockReturnValueOnce(dismissed.promise)
      .mockReturnValueOnce(unmounted.promise);
    const { input, unmount } = mountCombobox(searchTags);

    for (const value of ["a", "ab", "abc"]) {
      await fireEvent.input(input, { target: { value } });
    }
    expect(screen.getByText("Searching for tags…")).toBeTruthy();
    expect(searchTags.mock.calls[0]?.[1].aborted).toBe(true);
    expect(searchTags.mock.calls[1]?.[1].aborted).toBe(true);
    latest.resolve([research]);
    expect(await screen.findByRole("option", { name: "Research" })).toBeTruthy();
    older.resolve([design]);
    old.reject(new Error("Stale failure"));
    await tick();
    expect(screen.queryByRole("option", { name: "Design" })).toBeNull();
    expect(screen.getByRole("option", { name: "Research" })).toBeTruthy();
    expect(screen.queryByText(/Could not load tags/)).toBeNull();

    await fireEvent.input(input, { target: { value: "outside" } });
    await fireEvent.mouseDown(document.body);
    expect(searchTags.mock.calls[3]?.[1].aborted).toBe(true);
    dismissed.resolve([design]);
    await tick();
    expect(screen.queryByRole("listbox")).toBeNull();
    expect(screen.queryByText("Searching for tags…")).toBeNull();

    await fireEvent.input(input, { target: { value: "unmount" } });
    unmount();
    expect(searchTags.mock.calls[4]?.[1].aborted).toBe(true);
    unmounted.resolve([design]);
    await tick();
    expect(screen.queryByRole("listbox")).toBeNull();
  });

  it("announces failure and empty results and retries the same query", async () => {
    const searchTags = vi.fn<Search>()
      .mockRejectedValueOnce(new Error("Offline"))
      .mockRejectedValueOnce(new Error("Still offline"))
      .mockResolvedValue([]);
    const { input } = mountCombobox(searchTags);

    await fireEvent.input(input, { target: { value: "absent" } });
    const failure = await screen.findByText(/Could not load tags/);
    expect(failure.getAttribute("role")).toBe("status");
    expect(input.getAttribute("aria-expanded")).toBe("false");
    expect(screen.queryByRole("listbox")).toBeNull();

    await key(input, "Enter");
    const retry = await screen.findByRole("button", { name: "Try again" });
    retry.focus();
    await fireEvent.click(retry);
    expect(document.activeElement).toBe(input);
    expect(searchTags.mock.calls.map(([query]) => query)).toEqual(["absent", "absent", "absent"]);
    const empty = await screen.findByText("No matching tags.");
    expect(empty).toBe(failure);
    expect(input.hasAttribute("aria-controls")).toBe(false);
  });

  it("ignores composition keystrokes and searches the committed text once", async () => {
    const searchTags = vi.fn<Search>().mockResolvedValue([design]);
    const { input } = mountCombobox(searchTags);
    await fireEvent.compositionStart(input);
    await fireEvent.input(input, { target: { value: "d" } });
    await key(input, "ArrowDown");
    await key(input, "Enter", { isComposing: true });
    expect(searchTags).not.toHaveBeenCalled();

    await fireEvent.compositionEnd(input, { target: { value: "design" } });
    await fireEvent.input(input, { target: { value: "design" } });
    expect(await screen.findByRole("option", { name: "Design" })).toBeTruthy();
    expect(searchTags.mock.calls.map(([query]) => query)).toEqual(["design"]);

    await key(input, "ArrowDown");
    expect(activeOption(input)?.textContent).toBe("Design");
    await key(input, "ArrowLeft");
    expect(activeOption(input)).toBeNull();
    expect(document.activeElement).toBe(input);
  });
});
