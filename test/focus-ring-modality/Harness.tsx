import { TokenInput } from "../../packages/react/components/src/TokenInput";

export function Harness() {
  return (
    <section data-framework="react">
      <button type="button" data-before>
        before react
      </button>
      <TokenInput ariaLabel="React tags" placeholder="Add a tag" />
    </section>
  );
}
