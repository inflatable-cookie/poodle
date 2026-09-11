import { CodeEditor } from "../../packages/react/components/src/CodeEditor";

export function Harness() {
  return (
    <section data-framework="react">
      <button type="button" data-before>
        before react
      </button>
      <CodeEditor value="const answer = 42;" ariaLabel="React code editor" />
    </section>
  );
}
