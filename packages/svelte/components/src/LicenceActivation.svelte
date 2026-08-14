<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/licence.css";
  import {
    LICENCE_ACCOUNT_FAILED_MESSAGE,
    LICENCE_FILE_UNREADABLE_MESSAGE,
    LICENCE_ROUTES,
    getFocusableElements,
    licenceFileContentsBase64,
    resolveLicenceSubmit,
    type LicenceAccountTokenProvider,
    type LicenceActivationRoute,
    type LicenceCredential,
    type LicenceKeyFormat,
  } from "@inflatable-cookie/poodle-core";
  import { untrack } from "svelte";

  import { default as Button } from "./Button.svelte";
  import { default as Field } from "./Field.svelte";
  import { default as FileUpload } from "./FileUpload.svelte";
  import { default as Tabs } from "./Tabs.svelte";
  import { default as TextInput } from "./TextInput.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize } from "./types";

  let nextActivationId = 0;

  interface Props {
    keyFormat: LicenceKeyFormat;
    accountTokenProvider: LicenceAccountTokenProvider;
    defaultRoute?: LicenceActivationRoute;
    pending?: boolean;
    disabled?: boolean;
    title?: string;
    machineLabelLabel?: string;
    activateLabel?: string;
    fileAccept?: string | null;
    size?: ControlSize | null;
    density?: ControlDensity | null;
    onActivate?:
      | ((detail: { credential: LicenceCredential; label: string | null }) => void)
      | undefined;
  }

  let {
    keyFormat,
    accountTokenProvider,
    defaultRoute = "key",
    pending = false,
    disabled = false,
    title = "Activate licence",
    machineLabelLabel = "Name this machine (optional)",
    activateLabel = "Activate",
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
  const labelFieldId = `${instanceId}-label`;
  const routeMessageId = `${instanceId}-route-message`;

  // Initial selection only, by contract: a later `defaultRoute` change must not
  // move a customer off the route they chose. `untrack` says so explicitly.
  let route = $state<LicenceActivationRoute>(untrack(() => defaultRoute));
  let keyDraft = $state("");
  let machineLabel = $state("");
  let keyMessage = $state<string | null>(null);
  /* The route panel's own message: a failed account flow or an unreadable
     file. Never the credential, and never anything the provider returned. */
  let routeMessage = $state<string | null>(null);
  let fileContentsBase64 = $state<string | null>(null);
  let accountBusy = $state(false);
  let panelElement = $state<HTMLDivElement | null>(null);

  /* All three routes are peers, so all three are tabs, always visible, never
     behind an overflow menu. `disabled` greys them; it does not remove them. */
  const routeItems = $derived(
    LICENCE_ROUTES.map((entry) => ({ value: entry.value, label: entry.label, disabled })),
  );
  const submitBlocked = $derived(disabled || pending || accountBusy);

  function focusFirstControl(): void {
    if (!panelElement) return;
    getFocusableElements(panelElement)[0]?.focus();
  }

  function handleRouteChange(next: string): void {
    route = next as LicenceActivationRoute;
    keyMessage = null;
    routeMessage = null;
    // The route's own first field, not the form's. Landing a customer on the
    // machine-label box after they chose "Licence file" is landing them in the
    // wrong place.
    queueMicrotask(focusFirstControl);
  }

  function handleFiles(files: File[]): void {
    const file = files[0];
    if (!file) return;
    routeMessage = null;
    const reader = new FileReader();
    reader.onload = () => {
      const read = typeof reader.result === "string" ? reader.result : null;
      // A data URL carries a `data:...;base64,` prefix the authority will not
      // accept. Core strips it once, for both renderers.
      fileContentsBase64 = read === null ? null : licenceFileContentsBase64(read);
      if (fileContentsBase64 === null) routeMessage = LICENCE_FILE_UNREADABLE_MESSAGE;
    };
    reader.onerror = () => {
      fileContentsBase64 = null;
      routeMessage = LICENCE_FILE_UNREADABLE_MESSAGE;
    };
    reader.readAsDataURL(file);
  }

  function handleFileRemoved(): void {
    fileContentsBase64 = null;
    routeMessage = null;
  }

  function emit(token: string | null): void {
    const resolution = resolveLicenceSubmit(
      { route, key: keyDraft, token, fileContentsBase64, label: machineLabel },
      keyFormat,
    );
    if (resolution.outcome === "emit") {
      keyMessage = null;
      routeMessage = null;
      onActivate?.({ credential: resolution.credential, label: resolution.label });
      return;
    }
    // A cancelled account flow says nothing at all — the customer already knows
    // they backed out, and an error would read as a fault they caused.
    if (resolution.outcome === "quiet") return;
    if (route === "key") {
      keyMessage = resolution.message;
      queueMicrotask(focusFirstControl);
      return;
    }
    routeMessage = resolution.message;
    queueMicrotask(focusFirstControl);
  }

  async function submit(): Promise<void> {
    if (submitBlocked) return;
    if (route !== "accountToken") {
      emit(null);
      return;
    }
    // The host owns the account journey; Poodle only asks for its result.
    accountBusy = true;
    routeMessage = null;
    try {
      emit(await accountTokenProvider.acquire());
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
  class="poodle-licence-activation"
  aria-busy={pending}
  data-route={route}
  data-pending={pending}
  data-size={resolvedSize}
  data-density={resolvedDensity}
  onsubmit={handleSubmit}
>
  <h3 class="poodle-licence-activation__title">{title}</h3>

  <div class="poodle-licence-activation__routes">
    <Tabs
      items={routeItems}
      value={route}
      ariaLabel={title}
      size={resolvedSize}
      density={resolvedDensity}
      onValueChange={handleRouteChange}
    >
      {#snippet children(active)}
        <div
          bind:this={panelElement}
          class="poodle-licence-activation__route"
          data-route={active}
          aria-describedby={routeMessage && active !== "key" ? routeMessageId : undefined}
        >
          {#if active === "key"}
            <Field
              id={keyFieldId}
              label="Licence key"
              error={keyMessage}
              validationState={keyMessage ? "invalid" : "none"}
              size={resolvedSize}
              density={resolvedDensity}
            >
              {#snippet control(fieldProps)}
                <!-- The raw text goes to the injected parser untouched: lower
                     case, dashes, spaces and I/L/O confusions are its job, and
                     normalising here would judge the key twice. -->
                <TextInput
                  id={keyFieldId}
                  value={keyDraft}
                  disabled={disabled}
                  describedBy={fieldProps.describedBy}
                  validationState={fieldProps.validationState}
                  onValueChange={(value) => (keyDraft = value)}
                />
              {/snippet}
            </Field>
          {:else if active === "accountToken"}
            <p class="poodle-licence-activation__explanation">
              Continue with your account to authorise this machine. There is nothing to type here.
            </p>
            <span class="poodle-licence-activation__account-action">
              <Button
                variant="secondary"
                size={resolvedSize}
                density={resolvedDensity}
                disabled={submitBlocked}
                loading={accountBusy}
                onClick={() => void submit()}
              >
                Continue with account
              </Button>
            </span>
          {:else}
            <FileUpload
              accept={fileAccept}
              multiple={false}
              showPreview={false}
              disabled={disabled}
              size={resolvedSize}
              density={resolvedDensity}
              onUpload={handleFiles}
              onRemove={handleFileRemoved}
            />
          {/if}

          {#if routeMessage && active !== "key"}
            <p class="poodle-licence-activation__explanation" id={routeMessageId} role="status">
              {routeMessage}
            </p>
          {/if}
        </div>
      {/snippet}
    </Tabs>
  </div>

  <!-- The machine label and the submit belong to the activation, not to a
       route: naming this machine means the same thing whichever credential
       carries it. -->
  <div class="poodle-licence-activation__shared">
    <Field
      id={labelFieldId}
      label={machineLabelLabel}
      size={resolvedSize}
      density={resolvedDensity}
    >
      {#snippet control(fieldProps)}
        <TextInput
          id={labelFieldId}
          value={machineLabel}
          disabled={disabled}
          describedBy={fieldProps.describedBy}
          onValueChange={(value) => (machineLabel = value)}
        />
      {/snippet}
    </Field>
  </div>

  <div class="poodle-licence-activation__actions">
    <Button
      type="submit"
      variant="primary"
      size={resolvedSize}
      density={resolvedDensity}
      disabled={submitBlocked}
      loading={pending}
    >
      {activateLabel}
    </Button>
  </div>
</form>
