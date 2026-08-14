import "@inflatable-cookie/poodle-core/styles/licence.css";

import { useCallback, useMemo, useRef, useState, type FormEvent } from "react";

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

import { Button } from "./Button";
import { Field } from "./Field";
import { FileUpload } from "./FileUpload";
import { Tabs } from "./Tabs";
import { TextInput } from "./TextInput";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize } from "./types";

let nextActivationId = 0;

export interface LicenceActivationProps {
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
  onActivate?: (detail: { credential: LicenceCredential; label: string | null }) => void;
}

export function LicenceActivation({
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
  onActivate,
}: LicenceActivationProps) {
  const uiPresentation = useUiPresentation();
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, "control");
  const resolvedDensity = density ?? uiPresentation.density;

  const instanceId = useMemo(() => `poodle-licence-activation-${nextActivationId++}`, []);
  const keyFieldId = `${instanceId}-key`;
  const labelFieldId = `${instanceId}-label`;
  const routeMessageId = `${instanceId}-route-message`;

  const [route, setRoute] = useState<LicenceActivationRoute>(defaultRoute);
  const [keyDraft, setKeyDraft] = useState("");
  const [machineLabel, setMachineLabel] = useState("");
  const [keyMessage, setKeyMessage] = useState<string | null>(null);
  /* The route panel's own message: a failed account flow or an unreadable
     file. Never the credential, and never anything the provider returned. */
  const [routeMessage, setRouteMessage] = useState<string | null>(null);
  const [accountBusy, setAccountBusy] = useState(false);
  const fileContentsRef = useRef<string | null>(null);
  const panelRef = useRef<HTMLDivElement | null>(null);

  /* All three routes are peers, so all three are tabs, always visible, never
     behind an overflow menu. `disabled` greys them; it does not remove them. */
  const routeItems = LICENCE_ROUTES.map((entry) => ({
    value: entry.value,
    label: entry.label,
    disabled,
  }));
  const submitBlocked = disabled || pending || accountBusy;

  const focusFirstControl = useCallback(() => {
    if (!panelRef.current) return;
    getFocusableElements(panelRef.current)[0]?.focus();
  }, []);

  function handleRouteChange(next: string): void {
    setRoute(next as LicenceActivationRoute);
    setKeyMessage(null);
    setRouteMessage(null);
    // The route's own first field, not the form's. Landing a customer on the
    // machine-label box after they chose "Licence file" is landing them in the
    // wrong place.
    queueMicrotask(focusFirstControl);
  }

  function handleFiles(files: File[]): void {
    const file = files[0];
    if (!file) return;
    setRouteMessage(null);
    const reader = new FileReader();
    reader.onload = () => {
      const read = typeof reader.result === "string" ? reader.result : null;
      // A data URL carries a `data:...;base64,` prefix the authority will not
      // accept. Core strips it once, for both renderers.
      fileContentsRef.current = read === null ? null : licenceFileContentsBase64(read);
      if (fileContentsRef.current === null) setRouteMessage(LICENCE_FILE_UNREADABLE_MESSAGE);
    };
    reader.onerror = () => {
      fileContentsRef.current = null;
      setRouteMessage(LICENCE_FILE_UNREADABLE_MESSAGE);
    };
    reader.readAsDataURL(file);
  }

  function handleFileRemoved(): void {
    fileContentsRef.current = null;
    setRouteMessage(null);
  }

  function emit(token: string | null): void {
    const resolution = resolveLicenceSubmit(
      {
        route,
        key: keyDraft,
        token,
        fileContentsBase64: fileContentsRef.current,
        label: machineLabel,
      },
      keyFormat,
    );
    if (resolution.outcome === "emit") {
      setKeyMessage(null);
      setRouteMessage(null);
      onActivate?.({ credential: resolution.credential, label: resolution.label });
      return;
    }
    // A cancelled account flow says nothing at all — the customer already knows
    // they backed out, and an error would read as a fault they caused.
    if (resolution.outcome === "quiet") return;
    if (route === "key") {
      setKeyMessage(resolution.message);
      queueMicrotask(focusFirstControl);
      return;
    }
    setRouteMessage(resolution.message);
    queueMicrotask(focusFirstControl);
  }

  async function submit(): Promise<void> {
    if (submitBlocked) return;
    if (route !== "accountToken") {
      emit(null);
      return;
    }
    // The host owns the account journey; Poodle only asks for its result.
    setAccountBusy(true);
    setRouteMessage(null);
    try {
      emit(await accountTokenProvider.acquire());
    } catch {
      setRouteMessage(LICENCE_ACCOUNT_FAILED_MESSAGE);
    } finally {
      setAccountBusy(false);
    }
  }

  function handleSubmit(event: FormEvent<HTMLFormElement>): void {
    event.preventDefault();
    void submit();
  }

  return (
    <form
      className="poodle-licence-activation"
      aria-busy={pending}
      data-route={route}
      data-pending={pending}
      data-size={resolvedSize}
      data-density={resolvedDensity}
      onSubmit={handleSubmit}
    >
      <h3 className="poodle-licence-activation__title">{title}</h3>

      <div className="poodle-licence-activation__routes">
        <Tabs
          items={routeItems}
          value={route}
          ariaLabel={title}
          size={resolvedSize}
          density={resolvedDensity}
          onValueChange={handleRouteChange}
        >
          {(active) => (
            <div
              ref={panelRef}
              className="poodle-licence-activation__route"
              data-route={active}
              aria-describedby={
                routeMessage && active !== "key" ? routeMessageId : undefined
              }
            >
              {active === "key" ? (
                <Field
                  id={keyFieldId}
                  label="Licence key"
                  error={keyMessage}
                  validationState={keyMessage ? "invalid" : "none"}
                  size={resolvedSize}
                  density={resolvedDensity}
                  control={(fieldProps) => (
                    /* The raw text goes to the injected parser untouched: lower
                       case, dashes, spaces and I/L/O confusions are its job, and
                       normalising here would judge the key twice. */
                    <TextInput
                      id={keyFieldId}
                      value={keyDraft}
                      disabled={disabled}
                      describedBy={fieldProps.describedBy}
                      validationState={fieldProps.validationState}
                      onValueChange={setKeyDraft}
                    />
                  )}
                />
              ) : active === "accountToken" ? (
                <>
                  <p className="poodle-licence-activation__explanation">
                    Continue with your account to authorise this machine. There is nothing to type
                    here.
                  </p>
                  <span className="poodle-licence-activation__account-action">
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
                </>
              ) : (
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
              )}

              {routeMessage && active !== "key" ? (
                <p
                  className="poodle-licence-activation__explanation"
                  id={routeMessageId}
                  role="status"
                >
                  {routeMessage}
                </p>
              ) : null}
            </div>
          )}
        </Tabs>
      </div>

      {/* The machine label and the submit belong to the activation, not to a
          route: naming this machine means the same thing whichever credential
          carries it. */}
      <div className="poodle-licence-activation__shared">
        <Field
          id={labelFieldId}
          label={machineLabelLabel}
          size={resolvedSize}
          density={resolvedDensity}
          control={(fieldProps) => (
            <TextInput
              id={labelFieldId}
              value={machineLabel}
              disabled={disabled}
              describedBy={fieldProps.describedBy}
              onValueChange={setMachineLabel}
            />
          )}
        />
      </div>

      <div className="poodle-licence-activation__actions">
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
  );
}
