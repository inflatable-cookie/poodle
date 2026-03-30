# Auth UI And Workflow Recipes

Reusable auth-screen composition rules for Poodle-based Svelte apps.

## Purpose

Use this guide when contracts tell you what the auth-facing Poodle surfaces do,
but you need a stable answer for how to build real login, password-reset, and
account-password screens without recreating wrapper churn.

## Default Posture

- use Poodle for auth page framing, fields, actions, tabs, and feedback
- use Poodle `TotpInput` directly for one-time-code entry
- use Poodle `PasswordRequirements` directly when password-policy loading is
  already caller-owned
- keep auth transport, token refresh, and policy loading in host code or in a
  retained host workflow layer

## Auth Page Shell

Keep auth-page framing app-local. The stable generic composition is just a
local layout over `Card`.

```svelte
<script lang="ts">
  import { Card } from "@poodle/svelte-primitives";
</script>

<div class="auth-shell">
  <Card variant="elevated">
    <h1>Sign in</h1>
    <!-- host-owned auth flow -->
  </Card>
</div>
```

## Login Screen

Compose the visible screen in Poodle even when the flow logic lives elsewhere.

```svelte
<script lang="ts">
  import { Button, Callout, Field, TextInput } from "@poodle/svelte-primitives";

  let email = "";
  let password = "";
  let error: string | null = null;
</script>

<form class="auth-form">
  {#if error}
    <Callout tone="danger" message={error} />
  {/if}

  <Field id="login-email" label="Email" required>
    <TextInput id="login-email" type="email" bind:value={email} autocomplete="username" />
  </Field>

  <Field id="login-password" label="Password" required>
    <TextInput id="login-password" type="password" bind:value={password} autocomplete="current-password" />
  </Field>

  <Button type="submit" variant="primary">Sign in</Button>
</form>
```

## One-Time Code Entry

Use `TotpInput` directly. Do not add a second app-level or system-level OTP
wrapper unless the flow owns non-generic orchestration around it.

```svelte
<script lang="ts">
  import { Field, TotpInput } from "@poodle/svelte-primitives";

  let code = "";
</script>

<Field id="login-totp" label="Verification code" hint="Enter the 6-digit code from your authenticator app." required>
  <TotpInput id="login-totp" bind:value={code} />
</Field>
```

## Password Policy Checklist

Use Poodle `PasswordRequirements` when the caller already owns the policy
contract.

```svelte
<script lang="ts">
  import { PasswordRequirements } from "@poodle/svelte-primitives";

  let password = "";

  const requirements = {
    minLength: 12,
    requireMixedCase: true,
    requireDigit: true,
    requireSpecial: true
  };
</script>

<PasswordRequirements {password} {requirements} />
```

## What Stays Out

- token/session orchestration
- login-step state machines
- password-policy fetching and fallback defaults
- route redirects after completion
- app vocabulary and branding

Those remain host-owned unless a real shared workflow shell still earns its
place outside Poodle.

## Decision

- keep auth UI composition Poodle-first
- keep auth layout app-local
- only retain non-generic auth orchestration outside Poodle

## Related Contracts

- [Card](../contracts/foundation/card.md)
- [Field](../contracts/foundation/field.md)
- [TextInput](../contracts/foundation/text-input.md)
- [TotpInput](../contracts/foundation/totp-input.md)
- [PasswordRequirements](../contracts/foundation/password-requirements.md)

## Next Task

Add the next auth recipe only when a reusable host-side workflow seam is
proven across multiple apps, instead of moving retained app orchestration into
Poodle by convenience.
