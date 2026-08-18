import { useState, type CSSProperties } from "react";
import { Code, Field, TokenInput } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

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
            <SpecimenGroup label="Default">
        <div style={controlStyle}>
                      <Field id="blog-tags" label="Tags" description="Type a tag, then use comma, enter, tab, or blur to commit it.">
                        <TokenInput id="blog-tags" name="tags" values={tags} onValuesChange={setTags} placeholder="Type a tag…" />
                      </Field>
                    </div>
                    <Code source={JSON.stringify(tags)} inline={false} inlineVariant="plain" />
      </SpecimenGroup>

                <SpecimenGroup label="Multiple separators">
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
      </SpecimenGroup>

                <SpecimenGroup label="Narrow and long values">
        <div style={controlNarrowStyle}>
                      <Field id="long-tags" label="Migration labels" description="Long tokens wrap inside the field instead of overflowing.">
                        <TokenInput id="long-tags" values={longTags} onValuesChange={setLongTags} placeholder="Type a label..." />
                      </Field>
                    </div>
      </SpecimenGroup>

                <SpecimenGroup label="Read only">
        <div style={controlStyle}>
                      <Field id="readonly-tags" label="Locked tags">
                        <TokenInput id="readonly-tags" values={["premium", "acca", "2027"]} readOnly />
                      </Field>
                    </div>
      </SpecimenGroup>

      <SpecimenGroup label="Disabled">
        <div style={controlStyle}>
          <Field id="disabled-tags" label="Disabled tags">
            <TokenInput id="disabled-tags" values={["finance", "ethics"]} disabled />
          </Field>
        </div>
      </SpecimenGroup>
    </SpecimenLayout>
  );
}
