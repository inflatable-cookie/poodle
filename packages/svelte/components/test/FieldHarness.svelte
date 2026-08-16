<script lang="ts">
  import Field from "../src/Field.svelte";
  import { createRawSnippet } from "svelte";
  import type { ValidationState } from "../src/types";

  interface ControlApi {
    describedBy: string | null;
    descriptionId: string | null;
    errorId: string | null;
    messageId: string | null;
    validationState: ValidationState;
  }

  interface Props {
    id: string;
    label: string;
    description?: string | null;
    hint?: string | null;
    error?: string | null;
    pendingMessage?: string | null;
    validationState?: ValidationState;
    required?: boolean;
    optionalLabel?: string | null;
    useControl?: boolean;
  }

  let {
    id,
    label,
    description = null,
    hint = null,
    error = null,
    pendingMessage = null,
    validationState = "none",
    required = false,
    optionalLabel = null,
    useControl = false,
  }: Props = $props();

  // Raw snippets keep the branded Snippet type through the conditional prop
  // ternary (plain `{#snippet}` declarations widen it away), and they
  // materialize real markup under the test DOM. Params arrive as getters.
  const controlSnippet = createRawSnippet<[ControlApi]>((getApi) => {
    const api = getApi();
    return {
      render: () =>
        `<input id="${id}" class="harness-control" aria-describedby="${api.describedBy ?? ""}" data-description-id="${api.descriptionId ?? ""}" data-error-id="${api.errorId ?? ""}" data-message-id="${api.messageId ?? ""}" data-validation-state="${api.validationState}" />`,
    };
  });
</script>

<Field
  {id}
  {label}
  {description}
  {hint}
  {error}
  {pendingMessage}
  {validationState}
  {required}
  {optionalLabel}
  control={useControl ? controlSnippet : undefined}
>
  <input id={id} class="harness-control" />
</Field>
