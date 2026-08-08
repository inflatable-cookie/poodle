# Auth UI And Workflow Recipes

Reusable auth-screen composition rules for Poodle-based Svelte apps.

## Purpose

Use this guide when contracts tell you what the auth-facing Poodle surfaces do,
but you need a stable answer for how to build real login, password-reset, and
account-password screens without recreating wrapper churn.

## Default Posture

- use Poodle for auth page framing, fields, actions, tabs, and feedback
- use Poodle `CodeInput` directly for one-time-code entry
- use Poodle `PasswordRequirements` directly when password-policy loading is
  already caller-owned
- keep auth transport, token refresh, and policy loading in host code or in a
  retained host workflow layer

## Auth Page Shell

Keep auth-page framing app-local. The stable generic composition is just a
local layout over `Card`.

```svelte
<script lang="ts">
  import { Card } from "@inflatable-cookie/poodle-svelte";
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
  import { Button, Callout, Field, TextInput } from "@inflatable-cookie/poodle-svelte";

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

Use `CodeInput` directly. Do not add a second app-level or system-level OTP
wrapper unless the flow owns non-generic orchestration around it.

```svelte
<script lang="ts">
  import { Field, CodeInput } from "@inflatable-cookie/poodle-svelte";

  let code = "";
</script>

<Field id="login-totp" label="Verification code" hint="Enter the 6-digit code from your authenticator app." required>
  <CodeInput id="login-totp" bind:value={code} />
</Field>
```

## Password Policy Checklist

Use Poodle `PasswordRequirements` when the caller already owns the policy
contract.

```svelte
<script lang="ts">
  import { PasswordRequirements } from "@inflatable-cookie/poodle-svelte";

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

## Account Profile And Security Pages

For signed-in account pages, keep the workflow state in host code and compose
the visible surface directly in Poodle. The stable posture is:

- profile overview in `Card`
- edit actions in the header area of that card
- password change as a verify-first flow using `CodeInput`, `TextInput`,
  `FormActions`, and `Callout`
- passkeys as a compact list with inline rename/delete actions plus
  `AlertDialog` confirmation
- 2FA setup in a `Card` with QR/manual details and `CodeInput`

```svelte
<Card>
  <header class="account-header">
    <h2>Profile</h2>
    <Button variant="ghost">Edit profile</Button>
  </header>
  <!-- detail items or local overview content -->
</Card>
```

Do not introduce a second shared auth-shell wrapper for account security pages.
These are app-local screens built from Poodle primitives plus host-owned auth
hooks and API commands.

## What Stays Out

- token/session orchestration
- login-step state machines
- password-policy fetching and fallback defaults
- route redirects after completion
- app vocabulary and branding
- signed-in account profile/security orchestration

Those remain host-owned unless a real shared workflow shell still earns its
place outside Poodle.

## Decision

- keep auth UI composition Poodle-first
- keep auth layout app-local
- only retain non-generic auth orchestration outside Poodle

## Related Contracts

- [Card](../contracts/components/card.md)
- [Field](../contracts/components/field.md)
- [TextInput](../contracts/components/text-input.md)
- [CodeInput](../contracts/components/code-input.md)
- [PasswordRequirements](../contracts/components/password-requirements.md)

## Next Task

Add the next auth recipe only when a reusable host-side workflow seam is
proven across multiple apps, instead of moving retained app orchestration into
Poodle by convenience.
