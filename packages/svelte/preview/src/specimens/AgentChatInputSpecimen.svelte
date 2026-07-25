<script lang="ts">
  import {
    AgentChatInput,
    Button,
    ModelPicker,
    Icon,
    type AgentChatAttachment,
    type ModelCapabilityAxis,
    type ModelOption,
    type ModelSelection,
  } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

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
    {
      value: "atlas-mini",
      label: "Atlas Mini",
      description: "Fastest",
      icon: "zap",
      axes: ["effort"],
    },
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
    {
      key: "fast",
      label: "Fast mode",
      kind: "toggle",
      onLabel: "Fast",
      offLabel: "Normal",
    },
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

  let selection = $state<ModelSelection>({
    model: "atlas-pro",
    axes: { effort: "high", fast: false, context: "1m" },
  });

  let message = $state("");
  let busyMessage = $state("Summarise the release notes and open a PR");
  let sizeMessage = $state("");
  let densityMessage = $state("");
  let lastSubmitted = $state<string | null>(null);
  let stopCount = $state(0);

  let attachments = $state<AgentChatAttachment[]>([
    { id: "a1", label: "architecture.png", kind: "image", thumbnailUrl: diagramThumb },
    { id: "a2", label: "release-notes.md", kind: "document", icon: "file-text" },
  ]);

  function removeAttachment(id: string): void {
    attachments = attachments.filter((attachment) => attachment.id !== id);
  }
</script>

<SpecimenLayout>
  <SpecimenGroup label="Composer with model picker + context ring">
    <AgentChatInput
      bind:value={message}
      placeholder="Ask for follow-up changes or attach images"
      contextUsed={64_000}
      contextLimit={200_000}
      onSubmit={(next) => (lastSubmitted = next)}
    >
      {#snippet toolbar()}
        <ModelPicker {models} {axes} bind:value={selection} emphasis="subdued" />
        <Button variant="ghost" size="sm" leadingIcon="unlock" chevron>Full access</Button>
        <Button variant="ghost" size="sm" leadingIcon="package">Build</Button>
      {/snippet}
    </AgentChatInput>
    <p>Last submitted: {lastSubmitted ?? "—"}</p>
  </SpecimenGroup>

  <SpecimenGroup label="Busy (stop state) — Escape also stops">
    <AgentChatInput
      bind:value={busyMessage}
      status="busy"
      contextUsed={172_000}
      contextLimit={200_000}
      onStop={() => (stopCount += 1)}
    >
      {#snippet toolbar()}
        <ModelPicker {models} {axes} value={selection} emphasis="subdued" />
      {/snippet}
    </AgentChatInput>
    <p>Stop pressed {stopCount} time(s) — context ring is above the warn threshold</p>
  </SpecimenGroup>

  <SpecimenGroup label="Attachments (image tile + file chip) + footer bar">
    <AgentChatInput
      value="Fix the failing parity gate"
      {attachments}
      onRemoveAttachment={removeAttachment}
      contextUsed={22_000}
      contextLimit={200_000}
    >
      {#snippet toolbar()}
        <ModelPicker {models} {axes} value={selection} emphasis="subdued" />
      {/snippet}
      {#snippet footer()}
        <span class="poodle-agent-chat-specimen__footer-item">
          <Icon name="folder" size="xs" /> Current checkout
        </span>
        <span class="poodle-agent-chat-specimen__footer-spacer"></span>
        <span class="poodle-agent-chat-specimen__footer-item">
          <Icon name="git-branch" size="xs" /> main
        </span>
      {/snippet}
    </AgentChatInput>
  </SpecimenGroup>

  <SpecimenGroup label="Empty (submit disabled)">
    <AgentChatInput value="" />
  </SpecimenGroup>

  <SpecimenGroup label="allowEmptySubmit">
    <AgentChatInput value="" allowEmptySubmit />
  </SpecimenGroup>

  <SpecimenGroup label="No context ring, no dividers, Cmd/Ctrl+Enter only">
    <AgentChatInput value="Enter inserts a newline here" submitOnEnter={false} toolbarDividers={false}>
      {#snippet toolbar()}
        <ModelPicker {models} value={{ model: "atlas", axes: {} }} emphasis="subdued" />
      {/snippet}
    </AgentChatInput>
  </SpecimenGroup>

  <SpecimenGroup label="Grown editor (at the maxRows ceiling)">
    <AgentChatInput
      value={"Line one\nLine two\nLine three\nLine four\nLine five\nLine six"}
      maxRows={4}
    />
  </SpecimenGroup>

  <SpecimenGroup label="Read-only">
    <AgentChatInput value="This transcript entry cannot be edited" readOnly />
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <AgentChatInput value="Composer unavailable" disabled contextUsed={10_000} contextLimit={200_000} />
  </SpecimenGroup>

  {#snippet sizes(size)}
    <AgentChatInput bind:value={sizeMessage} {size} contextUsed={40_000} contextLimit={200_000}>
      {#snippet toolbar()}
        <ModelPicker {models} {axes} value={selection} {size} emphasis="subdued" />
      {/snippet}
    </AgentChatInput>
  {/snippet}

  {#snippet densities(density)}
    <AgentChatInput bind:value={densityMessage} {density} contextUsed={40_000} contextLimit={200_000}>
      {#snippet toolbar()}
        <ModelPicker {models} {axes} value={selection} {density} emphasis="subdued" />
      {/snippet}
    </AgentChatInput>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-agent-chat-specimen__footer-item {
    display: inline-flex;
    align-items: center;
    gap: 0.375rem;
  }

  .poodle-agent-chat-specimen__footer-spacer {
    flex: 1;
  }
</style>
