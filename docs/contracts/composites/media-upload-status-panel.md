# MediaUploadStatusPanel

Status: seed contract
Updated: 2026-03-25

## 1. Purpose

- Component name: `MediaUploadStatusPanel`
- Layer: `composites`
- Summary: a status surface for duplicate-check, upload-progress, completion, and error postures in media upload workflows
- In scope: status copy, progress bar, duplicate prompt, success/error actions
- Out of scope: file selection, upload execution, duplicate detection, media selection ownership

## 2. Props And Inputs

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `uploadStep` | `MediaUploadWorkflowStep` | `"checking"` | no | Current workflow posture |
| `duplicateLabel` | `string \| null` | `null` | no | Label for the duplicate media item |
| `uploadProgress` | `number` | `0` | no | Percentage shown during upload |
| `uploadError` | `string \| null` | `null` | no | Error message for the error state |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |

### Types

```ts
type MediaUploadWorkflowStep =
  | "checking"
  | "duplicate"
  | "uploading"
  | "finalising"
  | "complete"
  | "error";
```

## 3. States

| State | Trigger | Visual Effect |
|-------|---------|---------------|
| checking | `uploadStep === "checking"` | Spinner and duplicate-check copy |
| duplicate | `uploadStep === "duplicate"` | Existing-file warning and two actions |
| uploading | `uploadStep === "uploading"` | Progress bar and percent |
| finalising | `uploadStep === "finalising"` | Spinner and finalising copy |
| complete | `uploadStep === "complete"` | Success copy and follow-up actions |
| error | `uploadStep === "error"` | Error copy and retry action |

## 4. Events

| Event | When It Fires | Payload |
|-------|---------------|---------|
| `uploadAnyway` | User chooses to upload a duplicate as new | `void` |
| `selectDuplicate` | User chooses the existing duplicate item | `void` |
| `clearUpload` | User clears the current upload state | `void` |
| `selectUploaded` | User chooses the newly uploaded media | `void` |

## 5. Accessibility

- Uses real buttons for each action
- Progress state keeps textual progress alongside the bar
- Error and success states remain readable without color alone

## 6. Adoption Notes

Use `MediaUploadStatusPanel` when the host owns the actual media upload
workflow but needs a reusable UI for the duplicate-check and upload-status
postures. Do not treat it as a full upload controller.

