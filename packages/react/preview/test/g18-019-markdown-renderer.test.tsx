import { readFileSync } from "node:fs";
import { join } from "node:path";
import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { allComponents } from "../src/gallery/registry";
import { specimenMap } from "../src/gallery/specimen-map";
import { MarkdownRendererSpecimen } from "../src/gallery/specimens/MarkdownRendererSpecimen";
import { ComponentsSection } from "../src/gallery/ComponentsSection";

const PREVIEW_ROOT = join(import.meta.dirname, "..");
const SLUG = "markdown-renderer";

describe("g18.019 MarkdownRenderer preview specimen (react)", () => {
  it("registers the standalone renderer as a web-only catalogue entry", () => {
    const entry = allComponents.find((component) => component.slug === SLUG);
    expect(entry).toBeDefined();
    expect(entry?.displayName).toBe("MarkdownRenderer");
    expect(entry?.hasSpecimen).toBe(true);
    expect(specimenMap[SLUG]).toBeDefined();
  });

  it("mounts the public markdown subpath export", () => {
    const source = readFileSync(
      join(PREVIEW_ROOT, "src/gallery/specimens/MarkdownRendererSpecimen.tsx"),
      "utf8",
    );
    expect(source).toContain('from "@inflatable-cookie/poodle-react/markdown"');
    expect(source).not.toContain("markdown-content");
    expect(source).not.toContain("sanitizeMarkdownHtml");
  });

  it("renders safe, trusted, custom, and empty postures distinctly", () => {
    const { container } = render(<MarkdownRendererSpecimen />);
    const renderers = [...container.querySelectorAll(".poodle-md-renderer")];
    expect(renderers.length).toBeGreaterThanOrEqual(4);

    const safe = container.querySelector(".poodle-md-renderer__content") as HTMLElement;
    expect(safe.querySelector("h1")?.textContent).toBe("Release notes");

    const trusted = container.querySelector("[data-trusted-document]") as HTMLElement;
    expect(trusted).not.toBeNull();
    const warning = container.querySelector('[data-policy-label="trusted"]') as HTMLElement;
    expect(warning?.textContent).toContain('htmlPolicy="trusted"');

    expect(container.querySelector("[data-custom-parser]")).toBeNull();
    const customHeading = [...container.querySelectorAll("h2")].find(
      (heading) => heading.textContent === "Custom parser section",
    );
    expect(customHeading).toBeDefined();
    const customStrong = customHeading?.parentElement?.querySelector("strong")?.textContent;
    expect(customStrong).toBe("renderHtml");
    expect(container.textContent).not.toContain("Nothing to preview");
    expect(container.querySelector("textarea")).toBeNull();
  });

  it("resolves the direct catalogue route and search result", () => {
    render(<ComponentsSection search="MarkdownRenderer" />);
    expect(
      document.querySelector('[data-catalogue-result="markdown-renderer"]')?.getAttribute("href"),
    ).toBe("#components/markdown-renderer");
    expect(screen.getAllByText("MarkdownRenderer").length).toBeGreaterThan(0);
  });
});
