import { useState } from "react";
import { AgentChatInput, AgentPlan, type AgentPlanStatus } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const planMarkdown = [
  "## Proposed plan",
  "",
  "1. Add the `AgentPlan` surface to the composer",
  "2. Wire the decision callbacks through the host",
  "3. Append the settled record to the transcript",
  "",
  "Then run the gates.",
].join("\n");

export function AgentPlanSpecimen() {
  // The arrangement that matters: the plan inside the composer, where the
  // editor is the revise channel — feedback goes as an ordinary message.
  const [composerValue, setComposerValue] = useState("");
  const [decision, setDecision] = useState<AgentPlanStatus>("pending");
  const [lastMessage, setLastMessage] = useState("");

  return (
    <SpecimenLayout
      sizes={(size) => <AgentPlan plan={planMarkdown} size={size} />}
      densities={(density) => <AgentPlan plan={planMarkdown} density={density} />}
    >
      <SpecimenGroup
        label="Hosted by the composer"
        description="A proposed plan is input, so it mounts in the composer region — not the transcript. The turn is already complete; sending a message is the revise channel."
      >
        <AgentChatInput
          value={composerValue}
          onValueChange={setComposerValue}
          status="reviewing-plan"
          plan={
            <AgentPlan
              plan={planMarkdown}
              status={decision}
              onAccept={() => setDecision("accepted")}
              onRevise={() => setDecision("revised")}
              onDismiss={() => setDecision("dismissed")}
            />
          }
          onSubmit={(value) => {
            setLastMessage(value);
            setComposerValue("");
          }}
        />
        <p style={{ color: "var(--poodle-color-text-secondary)", fontSize: "0.8125rem" }}>
          {decision !== "pending"
            ? `decided: ${decision}`
            : lastMessage
              ? `revision sent: ${lastMessage}`
              : "no decision yet"}
        </p>
      </SpecimenGroup>

      <SpecimenGroup
        label="Pending"
        description="Controls render only while the plan waits on the operator."
      >
        <AgentPlan plan={planMarkdown} />
      </SpecimenGroup>

      <SpecimenGroup
        label="Settled"
        description="A settled status swaps the controls for the badge — the transitional render before the host swaps in the record."
      >
        <AgentPlan plan={planMarkdown} status="accepted" />
      </SpecimenGroup>

      <SpecimenGroup
        label="Not dismissible"
        description="Dismiss is a first-class decision for a plan, so it renders by default; a host can still withhold it."
      >
        <AgentPlan plan={planMarkdown} dismissible={false} />
      </SpecimenGroup>
    </SpecimenLayout>
  );
}
