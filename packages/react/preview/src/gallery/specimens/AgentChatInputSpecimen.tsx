import { useState } from "react";
import { resolveQuestionAnswer } from "@inflatable-cookie/poodle-core";
import {
  AgentChatInput,
  AgentQuestion,
  Button,
  Icon,
  ModelPicker,
  RefSelect,
  type AgentChatAttachment,
  type AgentPlanStatus,
  type AgentQuestionAnswer,
  type AgentQuestionItem,
  type ModelCapabilityAxis,
  type ModelOption,
  type ModelSelection,
  type RefOption,
} from "@inflatable-cookie/poodle-react";
import { AgentPlan } from "@inflatable-cookie/poodle-react/markdown";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

// A stand-in image so the preview stays offline; a real app points this at the
// uploaded file's object URL.
const diagramThumb =
  "data:image/svg+xml;utf8," +
  encodeURIComponent(
    `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
       <rect width="64" height="64" fill="#1f2a44"/>
       <circle cx="22" cy="24" r="9" fill="#f6c445"/>
       <path d="M4 56l18-20 12 13 10-11 16 18z" fill="#3f7d58"/>
     </svg>`,
  );

// Each model names the axis keys it exposes (see the ModelPicker specimen for
// the cross-provider version of this).
const models: ModelOption[] = [
  {
    value: "atlas-pro",
    label: "Atlas Pro",
    description: "Deepest reasoning",
    badge: "1M",
    icon: "sparkles",
    axes: ["effort", "fast", "context"],
  },
  {
    value: "atlas",
    label: "Atlas",
    description: "Balanced",
    icon: "sparkles",
    axes: ["effort", "fast"],
  },
  { value: "atlas-mini", label: "Atlas Mini", description: "Fastest", icon: "zap", axes: ["effort"] },
];

const axes: ModelCapabilityAxis[] = [
  {
    key: "effort",
    label: "Effort",
    kind: "select",
    options: [
      { value: "low", label: "Low" },
      { value: "medium", label: "Medium" },
      { value: "high", label: "High" },
    ],
    defaultValue: "high",
  },
  { key: "fast", label: "Fast mode", kind: "toggle", onLabel: "Fast", offLabel: "Normal" },
  {
    key: "context",
    label: "Context window",
    kind: "select",
    options: [
      { value: "200k", label: "200K" },
      { value: "1m", label: "1M" },
    ],
    defaultValue: "1m",
  },
];

const placement: AgentQuestionItem = {
  id: "placement",
  header: "Placement",
  prompt: "When the agent needs an answer mid-turn, where should the question surface appear?",
  options: [
    { value: "inline", label: "Inline in the transcript", description: "A block in the conversation." },
    { value: "composer", label: "Anchored above the composer", description: "Pinned over the input." },
    { value: "modal", label: "Modal dialog", description: "Blocks the app until answered." },
  ],
};

const planMarkdown = "1. Inspect the contract.\n2. Apply the bounded change.";

// The footer's ref switcher — the "main" in the reference is a control, not a
// label.
const refs: RefOption[] = [
  { value: "main", label: "main", kind: "branch", description: "a1b2c3d", group: "Branches" },
  { value: "agent-composer", label: "agent-composer", kind: "branch", group: "Branches" },
  { value: "v1.4.0", label: "v1.4.0", kind: "tag", group: "Tags" },
];

const footerItem = { display: "inline-flex", alignItems: "center", gap: "0.375rem" } as const;

export function AgentChatInputSpecimen() {
  const [selection, setSelection] = useState<ModelSelection>({
    model: "atlas-pro",
    axes: { effort: "high", fast: false, context: "1m" },
  });
  const [message, setMessage] = useState("Summarise the release notes and open a PR");
  const [busyMessage, setBusyMessage] = useState("Summarise the release notes and open a PR");
  const [sizeMessage, setSizeMessage] = useState("");
  const [densityMessage, setDensityMessage] = useState("");
  const [lastSubmitted, setLastSubmitted] = useState<string | null>(null);
  const [stopCount, setStopCount] = useState(0);
  const [currentRef, setCurrentRef] = useState("main");
  const [attachments, setAttachments] = useState<AgentChatAttachment[]>([
    { id: "a1", label: "architecture.png", kind: "image", thumbnailUrl: diagramThumb },
    { id: "a2", label: "release-notes.md", kind: "document", icon: "file-text" },
  ]);
  const [questionValue, setQuestionValue] = useState("");
  const [questionSelections, setQuestionSelections] = useState<string[]>([]);
  const [questionAnswer, setQuestionAnswer] = useState<AgentQuestionAnswer | null>(null);
  const [planValue, setPlanValue] = useState("");
  const [planDecision, setPlanDecision] = useState<AgentPlanStatus>("pending");

  return (
    <SpecimenLayout
      sizes={(size) => (
        <AgentChatInput
          value={sizeMessage}
          onValueChange={setSizeMessage}
          size={size}
          contextUsed={40_000}
          contextLimit={200_000}
          toolbar={<ModelPicker models={models} axes={axes} value={selection} size={size} emphasis="subdued" />}
        />
      )}
      densities={(density) => (
        <AgentChatInput
          value={densityMessage}
          onValueChange={setDensityMessage}
          density={density}
          contextUsed={40_000}
          contextLimit={200_000}
          toolbar={<ModelPicker models={models} axes={axes} value={selection} density={density} emphasis="subdued" />}
        />
      )}
    >
      <SpecimenGroup
        label="Default composer"
        description="Composing with a model picker, submit feedback, and context below the warning threshold."
      >
        <AgentChatInput
          value={message}
          onValueChange={setMessage}
          placeholder="Ask for follow-up changes or attach images"
          contextUsed={64_000}
          contextLimit={200_000}
          onSubmit={setLastSubmitted}
          toolbar={
            <>
              <ModelPicker
                models={models}
                axes={axes}
                value={selection}
                onChange={setSelection}
                emphasis="subdued"
              />
              <Button variant="ghost" size="sm" leadingIcon="unlock" chevron>
                Full access
              </Button>
              <Button variant="ghost" size="sm" leadingIcon="package">
                Build
              </Button>
            </>
          }
        />
        <p>Last submitted: {lastSubmitted ?? "—"}</p>
      </SpecimenGroup>

      <SpecimenGroup
        label="Questions and plans"
        description="The composer hosts a live question or a pending plan. The editor is the override or revise channel."
      >
        <AgentChatInput
          value={questionValue}
          status="questioning"
          questionCanSubmit={questionSelections.length > 0}
          question={
            <AgentQuestion
              questions={[placement]}
              selections={questionSelections}
              override={questionValue}
              onSelectionChange={setQuestionSelections}
              onSubmit={(answer) => {
                setQuestionAnswer(answer);
                setQuestionValue("");
                setQuestionSelections([]);
              }}
            />
          }
          onValueChange={setQuestionValue}
          onSubmit={() => {
            const answer = resolveQuestionAnswer(placement, questionSelections, questionValue);
            if (answer) {
              setQuestionAnswer(answer);
              setQuestionValue("");
              setQuestionSelections([]);
            }
          }}
        />
        <p>{questionAnswer ? `answered: ${questionAnswer.outcome}` : "no answer yet"}</p>
        <AgentChatInput
          value={planValue}
          onValueChange={setPlanValue}
          status="reviewing-plan"
          plan={
            <AgentPlan
              plan={planMarkdown}
              status={planDecision}
              onAccept={() => setPlanDecision("accepted")}
              onRevise={() => setPlanDecision("revised")}
              onDismiss={() => setPlanDecision("dismissed")}
            />
          }
        />
        <p>{planDecision !== "pending" ? `decided: ${planDecision}` : "no decision yet"}</p>
      </SpecimenGroup>

      <SpecimenGroup
        label="Busy and unavailable"
        description="Busy shows stop and a high-context ring. Read-only blocks editing; disabled blocks everything."
      >
        <AgentChatInput
          value={busyMessage}
          onValueChange={setBusyMessage}
          status="busy"
          contextUsed={172_000}
          contextLimit={200_000}
          onStop={() => setStopCount((count) => count + 1)}
          toolbar={<ModelPicker models={models} axes={axes} value={selection} emphasis="subdued" />}
        />
        <p>Stop pressed {stopCount} time(s) — context ring is above the warn threshold</p>
        <AgentChatInput value="This transcript entry cannot be edited" readOnly />
        <AgentChatInput
          value="Composer unavailable"
          disabled
          contextUsed={10_000}
          contextLimit={200_000}
        />
      </SpecimenGroup>

      <SpecimenGroup
        label="Attachments and footer"
        description="Images render as tiles, files as chips. Removal is live; the footer holds checkout context."
      >
        <AgentChatInput
          value="Fix the failing parity gate"
          attachments={attachments}
          onRemoveAttachment={(id) =>
            setAttachments((current) => current.filter((attachment) => attachment.id !== id))
          }
          contextUsed={22_000}
          contextLimit={200_000}
          toolbar={<ModelPicker models={models} axes={axes} value={selection} emphasis="subdued" />}
          footer={
            <>
              <span style={footerItem}>
                <Icon name="folder" size="xs" /> Current checkout
              </span>
              <span style={{ flex: 1 }} />
              <RefSelect
                refs={refs}
                value={currentRef}
                onChange={setCurrentRef}
                currentRef="main"
                emphasis="subdued"
                size="sm"
              />
            </>
          }
        />
      </SpecimenGroup>

      <SpecimenGroup
        label="Submission rules"
        description="Empty submit stays disabled unless allowEmptySubmit. Enter can insert a newline instead of sending."
      >
        <AgentChatInput value="" />
        <AgentChatInput value="" allowEmptySubmit />
        <AgentChatInput
          value="Enter inserts a newline here"
          submitOnEnter={false}
          toolbarDividers={false}
          toolbar={<ModelPicker models={models} value={{ model: "atlas", axes: {} }} emphasis="subdued" />}
        />
      </SpecimenGroup>

      <SpecimenGroup label="Editor growth" description="The editor stops growing at the maxRows ceiling.">
        <AgentChatInput
          value={"Line one\nLine two\nLine three\nLine four\nLine five\nLine six"}
          maxRows={4}
        />
      </SpecimenGroup>
    </SpecimenLayout>
  );
}
