import { useState } from "react";
import { DurationInput, NumberInput, TextInput, TokenInput } from "@poodle/react";
import { registerSpecimen, Row, SpecimenSection } from "../harness";

function TextInputSpecimen() {
  const [v, setV] = useState("");
  const [slug, setSlug] = useState("");
  return (
    <SpecimenSection title="TextInput">
      <TextInput value={v} onValueChange={setV} placeholder="Type here" ariaLabel="Basic" />
      <span data-testid="text-value">value: {v}</span>
      <TextInput type="search" defaultValue="query" ariaLabel="Search" />
      <TextInput
        type="slug"
        source="Hello World Again"
        value={slug}
        onValueChange={setSlug}
        ariaLabel="Slug"
      />
      <span data-testid="slug-value">slug: {slug}</span>
      <TextInput type="multiline" rows={3} defaultValue="Multi\nline" ariaLabel="Notes" maxLength={100} showCharCount />
      <TextInput
        ariaLabel="Validated"
        validate={(value) => ({ valid: value.length >= 3, message: value.length >= 3 ? "" : "Too short" })}
        validationDebounce={50}
        placeholder="min 3 chars"
      />
    </SpecimenSection>
  );
}
registerSpecimen({ slug: "text-input", title: "TextInput", render: () => <TextInputSpecimen /> });

function NumberInputSpecimen() {
  const [n, setN] = useState<number | string | null>(5);
  return (
    <SpecimenSection title="NumberInput">
      <NumberInput value={n} onValueChange={setN} min={0} max={10} step={1} showSteppers ariaLabel="Quantity" />
      <span data-testid="number-value">value: {String(n)}</span>
      <NumberInput defaultValue={2.5} precision={1} step={0.5} prefix="$" suffix="/kg" ariaLabel="Price" />
    </SpecimenSection>
  );
}
registerSpecimen({ slug: "number-input", title: "NumberInput", render: () => <NumberInputSpecimen /> });

function DurationSpecimen() {
  const [total, setTotal] = useState(0);
  return (
    <SpecimenSection title="DurationInput">
      <DurationInput defaultHours={1} defaultMinutes={30} onChange={(d) => setTotal(d.totalSeconds)} />
      <span data-testid="duration-value">total: {total}</span>
    </SpecimenSection>
  );
}
registerSpecimen({ slug: "duration-input", title: "DurationInput", render: () => <DurationSpecimen /> });

function TokenInputSpecimen() {
  const [tokens, setTokens] = useState<string[]>(["alpha", "beta"]);
  return (
    <SpecimenSection title="TokenInput">
      <TokenInput values={tokens} onValuesChange={setTokens} placeholder="Add tags" ariaLabel="Tags" />
      <span data-testid="token-value">tokens: {tokens.join(",")}</span>
    </SpecimenSection>
  );
}
registerSpecimen({ slug: "token-input", title: "TokenInput", render: () => <TokenInputSpecimen /> });
