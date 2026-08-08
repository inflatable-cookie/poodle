import { useState, type CSSProperties } from "react";
import { Code, Eyebrow, Field, Surface, TokenInput } from "@inflatable-cookie/poodle-react";
import { SpecimenLayout } from "../SpecimenLayout";

const specimenStyle: CSSProperties = {
  display: "flex",
  flexDirection: "column",
  gap: "1.25rem",
};

const itemStyle: CSSProperties = {
  display: "flex",
  flexDirection: "column",
  gap: "0.375rem",
};

const controlStyle: CSSProperties = {
  width: "100%",
  maxWidth: "40rem",
};

const controlNarrowStyle: CSSProperties = {
  ...controlStyle,
  maxWidth: "18rem",
};

export function TokenInputSpecimen() {
  const [tags, setTags] = useState(["ifrs", "tax", "audit"]);
  const [workflowTags, setWorkflowTags] = useState(["draft", "review"]);
  const [longTags, setLongTags] = useState([
    "legacy-migration",
    "a-very-long-tag-that-should-wrap-inside-the-token-without-breaking-the-field",
  ]);

  return (
    <SpecimenLayout
      sizes={(size) => (
        <div style={controlStyle}>
          <TokenInput id={"size-" + size} values={["alpha", "beta"]} size={size} placeholder={size.toUpperCase()} />
        </div>
      )}
      densities={(density) => (
        <div style={controlStyle}>
          <TokenInput id={"density-" + density} values={["audit", "reporting"]} density={density} placeholder="Type a tag…" />
        </div>
      )}
    >
      <Surface tone="panel" border="subtle" padding="md">
        <div style={specimenStyle}>
          <div style={itemStyle}>
            <Eyebrow>Default</Eyebrow>
            <div style={controlStyle}>
              <Field id="blog-tags" label="Tags" description="Type a tag, then use comma, enter, tab, or blur to commit it.">
                <TokenInput id="blog-tags" name="tags" values={tags} onValuesChange={setTags} placeholder="Type a tag…" />
              </Field>
            </div>
            <Code source={JSON.stringify(tags)} inline={false} inlineVariant="plain" />
          </div>

          <div style={itemStyle}>
            <Eyebrow>Multiple separators</Eyebrow>
            <div style={controlStyle}>
              <Field id="workflow-tags" label="Workflow tags" description="Supports comma and semicolon separators.">
                <TokenInput
                  id="workflow-tags"
                  values={workflowTags}
                  onValuesChange={setWorkflowTags}
                  separators={[",", ";"]}
                  placeholder="draft; review; live"
                />
              </Field>
            </div>
          </div>

          <div style={itemStyle}>
            <Eyebrow>Narrow and long values</Eyebrow>
            <div style={controlNarrowStyle}>
              <Field id="long-tags" label="Migration labels" description="Long tokens wrap inside the field instead of overflowing.">
                <TokenInput id="long-tags" values={longTags} onValuesChange={setLongTags} placeholder="Type a label..." />
              </Field>
            </div>
          </div>

          <div style={itemStyle}>
            <Eyebrow>Read only</Eyebrow>
            <div style={controlStyle}>
              <Field id="readonly-tags" label="Locked tags">
                <TokenInput id="readonly-tags" values={["premium", "acca", "2027"]} readOnly />
              </Field>
            </div>
          </div>

          <div style={itemStyle}>
            <Eyebrow>Disabled</Eyebrow>
            <div style={controlStyle}>
              <Field id="disabled-tags" label="Disabled tags">
                <TokenInput id="disabled-tags" values={["finance", "ethics"]} disabled />
              </Field>
            </div>
          </div>
        </div>
      </Surface>
    </SpecimenLayout>
  );
}
