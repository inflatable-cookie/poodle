import { useState, type CSSProperties } from "react";
import { EmbedInput, Field, resolveEmbedParseState } from "@inflatable-cookie/poodle-react";
import type { ParsedEmbed } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const tableStyle: CSSProperties = {
  width: "100%",
  borderCollapse: "collapse",
  fontSize: "var(--poodle-typography-label-size, 0.75rem)",
};

const cellStyle: CSSProperties = {
  padding: "0.375rem 0.625rem",
  borderBottom: "0.0625rem solid var(--poodle-color-border-subtle)",
  textAlign: "left",
};

const thStyle: CSSProperties = {
  ...cellStyle,
  color: "var(--poodle-color-text-secondary)",
  fontWeight: 600,
};

const codeStyle: CSSProperties = {
  padding: "0.0625rem 0.25rem",
  borderRadius: "0.1875rem",
  background: "var(--poodle-color-background-panel, #1a1a1a)",
  fontFamily: "var(--poodle-typography-mono-family, monospace)",
  fontSize: "0.6875rem",
};

const detailStyle: CSSProperties = { marginTop: "0.25rem" };
const errorStyle: CSSProperties = { color: "var(--poodle-color-text-danger, #ef4444)" };

const parsedStyle: CSSProperties = {
  margin: 0,
  padding: "0.5rem 0.75rem",
  borderRadius: "0.375rem",
  background: "var(--poodle-color-background-panel, #1a1a1a)",
  fontFamily: "var(--poodle-typography-mono-family, monospace)",
  fontSize: "0.75rem",
  whiteSpace: "pre-wrap",
};

const detectionSamples = [
  {
    label: "YouTube short link",
    input: "https://youtu.be/dQw4w9WgXcQ",
    providers: [] as string[],
  },
  {
    label: "Vimeo link",
    input: "https://vimeo.com/123456",
    providers: [] as string[],
  },
  {
    label: "Iframe embed",
    input: '<iframe src="https://example.com/embed/1" width="640" height="480"></iframe>',
    providers: [] as string[],
  },
  {
    label: "Restricted generic URL",
    input: "https://example.com/file.zip",
    providers: ["youtube", "vimeo"],
  },
].map((sample) => ({
  ...sample,
  result: resolveEmbedParseState(sample.input, sample.providers),
}));

export function EmbedInputSpecimen() {
  const [value, setValue] = useState("");
  const [parsed, setParsed] = useState<ParsedEmbed | null>(null);

  return (
    <SpecimenLayout
      sizes={(size) => (
        <EmbedInput id={"embed-size-" + size} size={size} placeholder="Paste a URL or embed code..." />
      )}
      densities={(density) => (
        <EmbedInput id={"embed-density-" + density} density={density} placeholder="Paste a URL or embed code..." />
      )}
    >
      <div className="poodle-specimen">
        <SpecimenGroup label="Supported providers">
          <table style={tableStyle}>
            <thead>
              <tr><th style={thStyle}>Provider</th><th style={thStyle}>Detected patterns</th></tr>
            </thead>
            <tbody>
              <tr>
                <td style={cellStyle}><code style={codeStyle}>youtube</code></td>
                <td style={cellStyle}>
                  <code style={codeStyle}>youtube.com/watch?v=</code>, <code style={codeStyle}>youtube.com/embed/</code>, <code style={codeStyle}>youtu.be/</code>
                </td>
              </tr>
              <tr>
                <td style={cellStyle}><code style={codeStyle}>vimeo</code></td>
                <td style={cellStyle}><code style={codeStyle}>vimeo.com/{"{id}"}</code></td>
              </tr>
              <tr>
                <td style={cellStyle}><code style={codeStyle}>generic</code></td>
                <td style={cellStyle}>Any valid URL, or <code style={codeStyle}>{"<iframe>"}</code> embed code</td>
              </tr>
            </tbody>
          </table>
        </SpecimenGroup>

        <SpecimenGroup label="Detection matrix">
          <table style={tableStyle}>
            <thead>
              <tr><th style={thStyle}>Input</th><th style={thStyle}>Resolved state</th></tr>
            </thead>
            <tbody>
              {detectionSamples.map((sample) => (
                <tr key={sample.label}>
                  <td style={cellStyle}>
                    <strong>{sample.label}</strong>
                    <div style={detailStyle}><code style={codeStyle}>{sample.input}</code></div>
                  </td>
                  <td style={cellStyle}>
                    {sample.result.error ? (
                      <span style={errorStyle}>{sample.result.error}</span>
                    ) : sample.result.parsed ? (
                      <code style={codeStyle}>{JSON.stringify(sample.result.parsed)}</code>
                    ) : (
                      <span>empty</span>
                    )}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </SpecimenGroup>

        <SpecimenGroup label="URL or embed code input">
          <EmbedInput
            value={value}
            onValueChange={setValue}
            onParse={(next) => setParsed(next)}
            placeholder="Paste a YouTube URL, Vimeo link, or embed code..."
          />
        </SpecimenGroup>

        <SpecimenGroup label="With Field wrapper">
          <Field label="Video embed" id="embed-input-video">
            <EmbedInput placeholder="https://youtube.com/watch?v=..." />
          </Field>
        </SpecimenGroup>

        <SpecimenGroup label="Restricted providers">
          <EmbedInput
            providers={["youtube", "vimeo"]}
            placeholder="Only YouTube and Vimeo allowed..."
          />
        </SpecimenGroup>

        {parsed ? (
          <SpecimenGroup label="Parsed result">
            <pre style={parsedStyle}>{JSON.stringify(parsed, null, 2)}</pre>
          </SpecimenGroup>
        ) : null}
      </div>
    </SpecimenLayout>
  );
}
