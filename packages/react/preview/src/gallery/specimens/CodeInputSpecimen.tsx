import { useState } from "react";
import { CodeInput } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

export function CodeInputSpecimen() {
  const [code, setCode] = useState("");
  const [completed, setCompleted] = useState(false);

  return (
    <SpecimenLayout
      sizes={(size) => (
        <CodeInput id={`size-${size}`} size={size} groups={[3, 3]} label={`Code at ${size}`} ariaLabel={`Code at ${size}`} />
      )}
      densities={(density) => (
        <CodeInput
          id={`density-${density}`}
          density={density}
          groups={[3, 3]}
          label={`Code at ${density}`}
          ariaLabel={`Code at ${density}`}
        />
      )}
    >
      <SpecimenGroup label="Default">
        <CodeInput
          id="code-default"
          value={code}
          label="Verification code"
          hint="Enter the 6-digit code from your authenticator app."
          groups={[3, 3]}
          onValueChange={(value) => {
            setCode(value);
            setCompleted(false);
          }}
          onComplete={() => {
            setCompleted(true);
          }}
        />
        {completed && (
          <p>
            Completed value: <strong>{code}</strong>
          </p>
        )}
      </SpecimenGroup>

      <SpecimenGroup label="Masked">
        <CodeInput id="code-masked" label="PIN code" hint="Digits are hidden for security." length={4} mask />
      </SpecimenGroup>

      <SpecimenGroup label="Multiple groups">
        <CodeInput
          id="code-grouped"
          label="Grouped recovery key"
          defaultValue="AB12CD34"
          length={12}
          groups={[4, 4, 4]}
          numbersOnly={false}
          autoComplete="off"
        />
      </SpecimenGroup>

      <SpecimenGroup label="Alphanumeric">
        <CodeInput
          id="code-alphanumeric"
          label="Recovery code"
          hint="Supports letters and numbers when the consumer opts in."
          defaultValue="AB12"
          length={6}
          numbersOnly={false}
          autoComplete="off"
        />
      </SpecimenGroup>

      <SpecimenGroup label="With error">
        <CodeInput
          id="code-error"
          value="12"
          label="Email code"
          error="That verification code is invalid."
          groups={[3, 3]}
        />
      </SpecimenGroup>

      <SpecimenGroup label="Disabled">
        <CodeInput id="code-disabled" defaultValue="1234" length={4} label="Disabled code" disabled />
      </SpecimenGroup>
    </SpecimenLayout>
  );
}
