# LogList — stream timestamps render as milliseconds only

Date: 2026-07-27
From: Jetstream thread (editor rehost)
Component: `packages/svelte/components/src/LogList.svelte`

## Symptom

A `variant="stream"` `LogList` shows `454`, `658`, `658` in the timestamp
column instead of a time. Every entry looks like a three-digit number, and two
entries logged in the same millisecond are indistinguishable from two entries
logged an hour apart.

Reproduced with valid `Date` objects — this is not bad input. Jetstream's
editor console hits it on every line.

## Cause

`formatStreamTimestamp`, line 155:

```ts
return date.toLocaleTimeString("en-US", { hour12: false, fractionalSecondDigits: 3 });
```

`Intl.DateTimeFormat` treats the options bag as the complete list of fields to
render. Passing `fractionalSecondDigits` without `hour`/`minute`/`second`
*replaces* the default h:m:s set rather than adding to it, so the output is the
fraction alone. `hour12` is not a field, so it does not keep the hour.

## Fix

Name the fields:

```ts
return date.toLocaleTimeString("en-US", {
  hour12: false,
  hour: "2-digit",
  minute: "2-digit",
  second: "2-digit",
  fractionalSecondDigits: 3,
});
```

Gives `14:22:07.454`.

## Worth a test

The current behaviour passes any test that only checks the function returns a
string, which is presumably why it survived. Asserting the shape — that the
output contains at least two `:` separators — would have caught it.
