import "@inflatable-cookie/poodle-core/styles/licence.css";

import {
  useCallback,
  useEffect,
  useMemo,
  useRef,
  useState,
  type FormEvent,
  type ReactNode,
} from "react";

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

import { Button } from "./Button";
import { CodeInput } from "./CodeInput";
import { EditableLabel } from "./EditableLabel";
import { Field } from "./Field";
import { FileUpload } from "./FileUpload";
import { TextInput } from "./TextInput";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize } from "./types";

let nextActivationId = 0;

interface LicenceActivationCommonProps {
  mode: LicenceActivationMode;
  pending?: boolean;
  disabled?: boolean;
  title?: string;
  machineLabel?: string | null;
  activateLabel?: string | null;
  size?: ControlSize | null;
  density?: ControlDensity | null;
  onActivate?: (detail: { credential: LicenceCredential; label: string | null }) => void;
}

export type LicenceActivationProps = LicenceActivationCommonProps &
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
        accountContent?: (disabled: boolean) => ReactNode;
        keyCodeInput?: never;
        keyFormat?: never;
        fileAccept?: string | null;
      }
  );

export function LicenceActivation(props: LicenceActivationProps) {
  const {
    mode,
    pending = false,
    disabled = false,
    title = "Activate licence",
    machineLabel = undefined,
    activateLabel = null,
    size = null,
    density = null,
    onActivate,
  } = props;

  const uiPresentation = useUiPresentation();
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, "control");
  const resolvedDensity = density ?? uiPresentation.density;

  const instanceId = useMemo(() => `poodle-licence-activation-${nextActivationId++}`, []);
  const keyFieldId = `${instanceId}-key`;
  const routeMessageId = `${instanceId}-route-message`;

  const [accountRoute, setAccountRoute] = useState<"accountToken" | "licenceFile">("accountToken");
  const [keyDraft, setKeyDraft] = useState("");
  const [machineLabelDraft, setMachineLabelDraft] = useState(machineLabel ?? "");
  const [keyMessage, setKeyMessage] = useState<string | null>(null);
  const [routeMessage, setRouteMessage] = useState<string | null>(null);
  const [accountBusy, setAccountBusy] = useState(false);
  const fileContentsRef = useRef<string | null>(null);
  const fileReaderRef = useRef<FileReader | null>(null);
  const fileReadGenerationRef = useRef(0);
  const formRef = useRef<HTMLFormElement | null>(null);
  const routeRef = useRef<HTMLDivElement | null>(null);

  const route: LicenceActivationRoute = mode === "key" ? "key" : accountRoute;
  const interactionDisabled = disabled || accountBusy;
  const submitBlocked = disabled || pending || accountBusy;

  const clearFileRead = useCallback(() => {
    fileReadGenerationRef.current += 1;
    const reader = fileReaderRef.current;
    if (reader && reader.readyState === reader.LOADING) reader.abort();
    fileReaderRef.current = null;
    fileContentsRef.current = null;
  }, []);

  useEffect(() => clearFileRead, [clearFileRead]);

  useEffect(() => {
    clearFileRead();
    setAccountRoute("accountToken");
    setKeyMessage(null);
    setRouteMessage(null);
  }, [mode, clearFileRead]);

  useEffect(() => {
    setMachineLabelDraft(machineLabel ?? "");
  }, [machineLabel]);

  const focusRouteControl = useCallback(() => {
    getFocusableElements(routeRef.current)[0]?.focus();
  }, []);

  function switchAccountRoute(next: "accountToken" | "licenceFile"): void {
    if (interactionDisabled || mode !== "account" || accountRoute === next) return;
    if (accountRoute === "licenceFile") clearFileRead();
    setAccountRoute(next);
    setRouteMessage(null);
    queueMicrotask(() => {
      if (next === "licenceFile") focusRouteControl();
      else formRef.current?.querySelector<HTMLButtonElement>('button[type="submit"]')?.focus();
    });
  }

  function handleFiles(files: File[]): void {
    const file = files[0];
    if (!file) return;
    clearFileRead();
    setRouteMessage(null);
    const reader = new FileReader();
    const generation = fileReadGenerationRef.current;
    fileReaderRef.current = reader;
    reader.onload = () => {
      if (generation !== fileReadGenerationRef.current) return;
      fileReaderRef.current = null;
      const read = typeof reader.result === "string" ? reader.result : null;
      fileContentsRef.current = read === null ? null : licenceFileContentsBase64(read);
      setRouteMessage(
        fileContentsRef.current === null ? LICENCE_FILE_UNREADABLE_MESSAGE : null,
      );
    };
    reader.onerror = () => {
      if (generation !== fileReadGenerationRef.current) return;
      fileReaderRef.current = null;
      fileContentsRef.current = null;
      setRouteMessage(LICENCE_FILE_UNREADABLE_MESSAGE);
    };
    reader.readAsDataURL(file);
  }

  function handleFileRemoved(): void {
    clearFileRead();
    setRouteMessage(null);
  }

  function handleKeyChange(value: string): void {
    setKeyDraft(value);
    setKeyMessage(null);
  }

  function emit(draft: LicenceSubmitDraft): void {
    const keyFormat = mode === "key" ? props.keyFormat : null;
    const resolution = resolveLicenceSubmit(draft, keyFormat);
    if (resolution.outcome === "emit") {
      setKeyMessage(null);
      setRouteMessage(null);
      onActivate?.({ credential: resolution.credential, label: resolution.label });
      return;
    }
    if (resolution.outcome === "quiet") return;
    if (draft.route === "key") {
      setKeyMessage(resolution.message);
      queueMicrotask(focusRouteControl);
      return;
    }
    setRouteMessage(resolution.message);
    queueMicrotask(focusRouteControl);
  }

  async function submit(): Promise<void> {
    if (submitBlocked) return;
    if (route !== "accountToken") {
      emit({
        route,
        key: keyDraft,
        token: null,
        fileContentsBase64: fileContentsRef.current,
        label: machineLabelDraft,
      });
      return;
    }

    const submittedLabel = machineLabelDraft;
    setAccountBusy(true);
    setRouteMessage(null);
    try {
      emit({
        route: "accountToken",
        key: "",
        token: await props.accountTokenProvider.acquire(),
        fileContentsBase64: null,
        label: submittedLabel,
      });
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

  const submitLabel = activateLabel ?? (route === "accountToken" ? "Continue with account" : "Activate");

  return (
    <form
      ref={formRef}
      className="poodle-licence-activation"
      aria-busy={pending || accountBusy}
      data-mode={mode}
      data-route={route}
      data-pending={pending || accountBusy}
      data-size={resolvedSize}
      data-density={resolvedDensity}
      onSubmit={handleSubmit}
    >
      <div className="poodle-licence-activation__header">
        <h3 className="poodle-licence-activation__title">{title}</h3>
        {mode === "account" ? (
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
            {route === "accountToken" ? "Activate offline" : "Use account activation"}
          </Button>
        ) : null}
      </div>

      <div ref={routeRef} className="poodle-licence-activation__route" data-route={route}>
        {route === "key" && props.keyCodeInput ? (
          <CodeInput
            id={keyFieldId}
            name="licenceKey"
            value={keyDraft}
            label="Licence key"
            error={keyMessage}
            disabled={interactionDisabled}
            length={props.keyCodeInput.length}
            groups={props.keyCodeInput.groups}
            separator={props.keyCodeInput.separator}
            numbersOnly={false}
            autoComplete="off"
            size={resolvedSize}
            density={resolvedDensity}
            validate={(value) => ({ valid: props.keyFormat.parse(value).ok })}
            onValueChange={handleKeyChange}
          />
        ) : route === "key" ? (
          <Field
            id={keyFieldId}
            label="Licence key"
            error={keyMessage}
            validationState={keyMessage ? "invalid" : "none"}
            size={resolvedSize}
            density={resolvedDensity}
            control={(fieldProps) => (
              <TextInput
                id={keyFieldId}
                value={keyDraft}
                disabled={interactionDisabled}
                describedBy={fieldProps.describedBy}
                validationState={fieldProps.validationState}
                onValueChange={handleKeyChange}
              />
            )}
          />
        ) : route === "accountToken" ? (
          props.accountContent ? (
            <div className="poodle-licence-activation__account-content">
              {props.accountContent(interactionDisabled)}
            </div>
          ) : (
            <p className="poodle-licence-activation__explanation">
              Continue with your account to authorise this machine.
            </p>
          )
        ) : (
          <FileUpload
            accept={props.fileAccept}
            multiple={false}
            showPreview={false}
            disabled={interactionDisabled}
            describedBy={routeMessage ? routeMessageId : null}
            size={resolvedSize}
            density={resolvedDensity}
            onUpload={handleFiles}
            onRemove={handleFileRemoved}
          />
        )}

        {routeMessage && route !== "key" ? (
          <p className="poodle-licence-activation__explanation" id={routeMessageId} role="status">
            {routeMessage}
          </p>
        ) : null}
      </div>

      <div className="poodle-licence-activation__actions">
        {machineLabel !== undefined ? (
          <div className="poodle-licence-activation__machine">
            <span className="poodle-licence-activation__machine-name">Machine name</span>
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
              onCommit={({ value }) => setMachineLabelDraft(value)}
            />
          </div>
        ) : null}
        <Button
          type="submit"
          variant="primary"
          size={resolvedSize}
          density={resolvedDensity}
          disabled={submitBlocked}
          loading={pending || accountBusy}
        >
          {submitLabel}
        </Button>
      </div>
    </form>
  );
}
