import { MarkdownEditor } from "../../packages/react/components/src/MarkdownEditor";
import { LONG_MARKDOWN, SHORT_MARKDOWN } from "./long-markdown";

export function Harness() {
  return (
    <section data-framework="react">
      <div data-case="constrained-preview" style={{ height: "16rem", display: "flex", flexDirection: "column" }}>
        <MarkdownEditor mode="preview" value={LONG_MARKDOWN} />
      </div>
      <p data-sibling="constrained-preview">react sibling after constrained preview</p>

      <div data-case="constrained-split" style={{ height: "16rem", display: "flex", flexDirection: "column" }}>
        <MarkdownEditor mode="split" value={"short source\n\n" + LONG_MARKDOWN} minHeight="8rem" />
      </div>
      <p data-sibling="constrained-split">react sibling after constrained split</p>

      <div data-case="natural-short">
        <MarkdownEditor mode="preview" value={SHORT_MARKDOWN} />
      </div>
    </section>
  );
}
