<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/licence.css";
  import {
    LICENCE_ACCOUNT_FAILED_MESSAGE,
    LICENCE_FILE_UNREADABLE_MESSAGE,
    getFocusableElements,
    licenceFileContentsBase64,
    resolveLicenceSubmit,
    type LicenceAccountTokenProvider,
    type LicenceActivationMode,
    type LicenceActivationRoute,
    type LicenceCredential,
    type LicenceKeyCodeInputOptions,
    type LicenceKeyFormat,
    type LicenceSubmitDraft,
  } from "@inflatable-cookie/poodle-core";
  import { onDestroy, type Snippet } from "svelte";

  import { default as Button } from "./Button.svelte";
  import { default as CodeInput } from "./CodeInput.svelte";
  import { default as EditableLabel } from "./EditableLabel.svelte";
  import { default as Field } from "./Field.svelte";
  import { default as FileUpload } from "./FileUpload.svelte";
  import { default as TextInput } from "./TextInput.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize } from "./types";

  let nextActivationId = 0;

  interface CommonProps {
    mode: LicenceActivationMode;
    pending?: boolean;
    disabled?: boolean;
    title?: string;
    machineLabel?: string | null;
    activateLabel?: string | null;
    size?: ControlSize | null;
    density?: ControlDensity | null;
    onActivate?:
      | ((detail: { credential: LicenceCredential; label: string | null }) => void)
      | undefined;
  }

  type Props = CommonProps &
    (
      | {
          mode: "key";
          keyFormat: LicenceKeyFormat;
          keyCodeInput?: LicenceKeyCodeInputOptions | null;
          accountContent?: never;
          accountTokenProvider?: never;
          fileAccept?: never;
        }
      | {
          mode: "account";
          accountTokenProvider: LicenceAccountTokenProvider;
          accountContent?: Snippet<[boolean]>;
          keyCodeInput?: never;
          keyFormat?: never;
          fileAccept?: string | null;
        }
    );

  let {
    mode,
    keyFormat = undefined,
    keyCodeInput = undefined,
    accountTokenProvider = undefined,
    accountContent = undefined,
    pending = false,
    disabled = false,
    title = "Activate licence",
    machineLabel = undefined,
    activateLabel = null,
    fileAccept = null,
    size = null,
    density = null,
    onActivate = undefined,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, "control"));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);

  const instanceId = `poodle-licence-activation-${nextActivationId++}`;
  const keyFieldId = `${instanceId}-key`;
  const routeMessageId = `${instanceId}-route-message`;

  let accountRoute = $state<"accountToken" | "licenceFile">("accountToken");
  let keyDraft = $state("");
  let machineLabelDraft = $state("");
  let keyMessage = $state<string | null>(null);
  let routeMessage = $state<string | null>(null);
  let fileContentsBase64 = $state<string | null>(null);
  let fileReader: FileReader | null = null;
  let fileReadGeneration = 0;
  let accountBusy = $state(false);
  let formElement = $state<HTMLFormElement | null>(null);
  let routeElement = $state<HTMLDivElement | null>(null);

  const route = $derived<LicenceActivationRoute>(mode === "key" ? "key" : accountRoute);
  const interactionDisabled = $derived(disabled || accountBusy);
  const submitBlocked = $derived(disabled || pending || accountBusy);
  const submitLabel = $derived(
    activateLabel ?? (route === "accountToken" ? "Continue with account" : "Activate"),
  );

  function clearFileRead(): void {
    fileReadGeneration += 1;
    if (fileReader && fileReader.readyState === fileReader.LOADING) fileReader.abort();
    fileReader = null;
    fileContentsBase64 = null;
  }

  onDestroy(clearFileRead);

  $effect(() => {
    mode;
    clearFileRead();
    accountRoute = "accountToken";
    keyMessage = null;
    routeMessage = null;
  });

  $effect(() => {
    machineLabelDraft = machineLabel ?? "";
  });

  function focusRouteControl(): void {
    getFocusableElements(routeElement)[0]?.focus();
  }

  function switchAccountRoute(next: "accountToken" | "licenceFile"): void {
    if (interactionDisabled || mode !== "account" || accountRoute === next) return;
    if (accountRoute === "licenceFile") clearFileRead();
    accountRoute = next;
    routeMessage = null;
    queueMicrotask(() => {
      if (next === "licenceFile") focusRouteControl();
      else formElement?.querySelector<HTMLButtonElement>('button[type="submit"]')?.focus();
    });
  }

  function handleFiles(files: File[]): void {
    const file = files[0];
    if (!file) return;
    clearFileRead();
    routeMessage = null;
    const reader = new FileReader();
    const generation = fileReadGeneration;
    fileReader = reader;
    reader.onload = () => {
      if (generation !== fileReadGeneration) return;
      fileReader = null;
      const read = typeof reader.result === "string" ? reader.result : null;
      fileContentsBase64 = read === null ? null : licenceFileContentsBase64(read);
      routeMessage =
        fileContentsBase64 === null ? LICENCE_FILE_UNREADABLE_MESSAGE : null;
    };
    reader.onerror = () => {
      if (generation !== fileReadGeneration) return;
      fileReader = null;
      fileContentsBase64 = null;
      routeMessage = LICENCE_FILE_UNREADABLE_MESSAGE;
    };
    reader.readAsDataURL(file);
  }

  function handleFileRemoved(): void {
    clearFileRead();
    routeMessage = null;
  }

  function emit(draft: LicenceSubmitDraft): void {
    const resolution = resolveLicenceSubmit(draft, mode === "key" ? keyFormat ?? null : null);
    if (resolution.outcome === "emit") {
      keyMessage = null;
      routeMessage = null;
      onActivate?.({ credential: resolution.credential, label: resolution.label });
      return;
    }
    if (resolution.outcome === "quiet") return;
    if (draft.route === "key") {
      keyMessage = resolution.message;
      queueMicrotask(focusRouteControl);
      return;
    }
    routeMessage = resolution.message;
    queueMicrotask(focusRouteControl);
  }

  async function submit(): Promise<void> {
    if (submitBlocked) return;
    if (route !== "accountToken") {
      emit({ route, key: keyDraft, token: null, fileContentsBase64, label: machineLabelDraft });
      return;
    }
    if (!accountTokenProvider) return;

    const submittedLabel = machineLabelDraft;
    accountBusy = true;
    routeMessage = null;
    try {
      emit({
        route: "accountToken",
        key: "",
        token: await accountTokenProvider.acquire(),
        fileContentsBase64: null,
        label: submittedLabel,
      });
    } catch {
      routeMessage = LICENCE_ACCOUNT_FAILED_MESSAGE;
    } finally {
      accountBusy = false;
    }
  }

  function handleSubmit(event: SubmitEvent): void {
    event.preventDefault();
    void submit();
  }
</script>

<form
  bind:this={formElement}
  class="poodle-licence-activation"
  aria-busy={pending || accountBusy}
  data-mode={mode}
  data-route={route}
  data-pending={pending || accountBusy}
  data-size={resolvedSize}
  data-density={resolvedDensity}
  onsubmit={handleSubmit}
>
  <div class="poodle-licence-activation__header">
    <h3 class="poodle-licence-activation__title">{title}</h3>
    {#if mode === "account"}
      <Button
        className="poodle-licence-activation__route-switch"
        type="button"
        variant="ghost"
        fit="content"
        size="xs"
        density={resolvedDensity}
        leadingIcon={route === "accountToken" ? "cloud-off" : "user"}
        disabled={interactionDisabled}
        onClick={() => switchAccountRoute(route === "accountToken" ? "licenceFile" : "accountToken")}
      >
        {#snippet children()}
          {route === "accountToken" ? "Activate offline" : "Use account activation"}
        {/snippet}
      </Button>
    {/if}
  </div>

  <div bind:this={routeElement} class="poodle-licence-activation__route" data-route={route}>
    {#if route === "key" && keyCodeInput}
      <CodeInput
        id={keyFieldId}
        name="licenceKey"
        value={keyDraft}
        label="Licence key"
        error={keyMessage}
        disabled={interactionDisabled}
        length={keyCodeInput.length}
        groups={keyCodeInput.groups}
        numbersOnly={false}
        autocomplete="off"
        size={resolvedSize}
        density={resolvedDensity}
        onValueChange={(value) => (keyDraft = value)}
      />
    {:else if route === "key"}
      <Field
        id={keyFieldId}
        label="Licence key"
        error={keyMessage}
        validationState={keyMessage ? "invalid" : "none"}
        size={resolvedSize}
        density={resolvedDensity}
      >
        {#snippet control(fieldProps)}
          <TextInput
            id={keyFieldId}
            value={keyDraft}
            disabled={interactionDisabled}
            describedBy={fieldProps.describedBy}
            validationState={fieldProps.validationState}
            onValueChange={(value) => (keyDraft = value)}
          />
        {/snippet}
      </Field>
    {:else if route === "accountToken"}
      {#if accountContent}
        <div class="poodle-licence-activation__account-content">
          {@render accountContent(interactionDisabled)}
        </div>
      {:else}
        <p class="poodle-licence-activation__explanation">
          Continue with your account to authorise this machine.
        </p>
      {/if}
    {:else}
      <FileUpload
        accept={fileAccept}
        multiple={false}
        showPreview={false}
        disabled={interactionDisabled}
        describedBy={routeMessage ? routeMessageId : null}
        size={resolvedSize}
        density={resolvedDensity}
        onUpload={handleFiles}
        onRemove={handleFileRemoved}
      />
    {/if}

    {#if routeMessage && route !== "key"}
      <p class="poodle-licence-activation__explanation" id={routeMessageId} role="status">
        {routeMessage}
      </p>
    {/if}
  </div>

  <div class="poodle-licence-activation__actions">
    {#if machineLabel !== undefined}
      <div class="poodle-licence-activation__machine">
        <span class="poodle-licence-activation__machine-name">Machine name</span>
        <EditableLabel
          value={machineLabelDraft}
          ariaLabel="Edit machine name"
          disabled={interactionDisabled}
          activationMode="enterOrSpace"
          emptyText="unnamed machine"
          placeholder="unnamed machine"
          showEditIcon
          size={resolvedSize}
          density={resolvedDensity}
          onCommit={({ value }) => (machineLabelDraft = value)}
        />
      </div>
    {/if}
    <Button
      type="submit"
      variant="primary"
      size={resolvedSize}
      density={resolvedDensity}
      disabled={submitBlocked}
      loading={pending || accountBusy}
    >
      {#snippet children()}{submitLabel}{/snippet}
    </Button>
  </div>
</form>
