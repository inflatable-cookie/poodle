# Value Track: Form Validation Patterns

Status: complete (findings documented)
Created: 2026-03-11
Updated: 2026-03-11
Priority: high (informs g01.008, g02.001)

## Purpose

Research form validation, field states, and error handling patterns to inform:
- Form primitives (g01.008)
- Field wrapper components (g02.001)
- Validation state management
- Error display patterns

---

## Key Findings

### Field States

**Common Validation States:**

| State | Description | Use Case |
|-------|-------------|----------|
| `pristine` | Not touched by user | Initial state, no validation shown |
| `dirty` | Value changed from initial | Track modifications |
| `touched` | Field lost focus | Show validation after interaction |
| `untouched` | Never lost focus | Delay validation feedback |
| `valid` | Passes validation | Show success state |
| `invalid` | Fails validation | Show error state |
| `pending` | Async validation in progress | Show loading state |
| `disabled` | Non-interactive | Gray out, skip validation |

**State Transitions:**
```
Initial: pristine + untouched
↓
User types: dirty
↓
User blurs: touched → validate → valid/invalid
```

### Validation Timing Strategies

**1. On Submit Only**
- Validate when form submitted
- Pros: Less intrusive
- Cons: Errors found late

**2. On Blur (Recommended)**
- Validate when field loses focus
- Pros: Immediate feedback, not annoying
- Cons: May show error before finished typing

**3. On Change (with debounce)**
- Validate after user stops typing
- Pros: Immediate feedback
- Cons: Can be noisy

**4. Mixed Approach (Best Practice)**
- On blur for most fields
- On change for fields with specific format (email, phone)
- Always on submit

### Validation Types

**Synchronous:**
- Required fields
- Min/max length
- Pattern matching (regex)
- Type checking (number, email)

**Asynchronous:**
- Username availability
- Email uniqueness
- Server-side business rules

### Error Display Patterns

**Inline Errors:**
```
[Label]
[Input]
[Error message - below input]
```

**Tooltip/Popover Errors:**
- Show on focus if invalid
- Icon indicator next to input

**Summary Errors:**
- List at top of form
- Links to invalid fields

### Error Message Guidelines

**Good Error Messages:**
- Clear what went wrong
- Explain how to fix it
- Specific to the field
- Non-technical language

**Examples:**
| Bad | Good |
|-----|------|
| "Invalid input" | "Email must contain @ symbol" |
| "Error 404" | "This username is already taken" |
| "Validation failed" | "Password must be at least 8 characters" |

### Field Wrapper Pattern

**Structure:**
```
[Field.Root]
├── [Field.Label]
├── [Field.Description] (optional)
├── [Field.Input] (slot for Input, Select, etc.)
├── [Field.Error] (shows when invalid)
└── [Field.Success] (optional, shows when valid)
```

**Props:**
```typescript
interface FieldProps {
  name: string;
  label?: string;
  description?: string;
  required?: boolean;
  error?: string;
  success?: string;
  touched?: boolean;
  disabled?: boolean;
}
```

### ARIA for Validation

**Required Attributes:**
- `aria-required="true"` - Required field
- `aria-invalid="true"` - Invalid state
- `aria-describedby` - Links to error message

**Example:**
```html
<label for="email">Email</label>
<input 
  id="email"
  aria-required="true"
  aria-invalid="true"
  aria-describedby="email-error"
/>
<span id="email-error" role="alert">
  Please enter a valid email address
</span>
```

### Form-Level Validation

**Validation Schema:**
- JSON Schema
- Zod (TypeScript)
- Yup
- Joi

**Error Aggregation:**
```typescript
interface FormErrors {
  [fieldName: string]: string[];
}

interface FormState {
  values: Record<string, any>;
  errors: FormErrors;
  touched: Record<string, boolean>;
  dirty: Record<string, boolean>;
  isValid: boolean;
  isSubmitting: boolean;
}
```

### Common Validation Rules

**String Fields:**
- `required` - Not empty
- `minLength` - Minimum character count
- `maxLength` - Maximum character count
- `pattern` - Regex match
- `email` - Valid email format

**Number Fields:**
- `min` - Minimum value
- `max` - Maximum value
- `integer` - Whole numbers only

**Date Fields:**
- `minDate` - Earliest date
- `maxDate` - Latest date

**Custom Rules:**
- Cross-field validation (confirm password)
- Async validation (check availability)

---

## Recommendations for Poodle

### Form Component Suite

```
Layer 2 - Forms
├── Field
│   ├── Field.Label
│   ├── Field.Description
│   ├── Field.Error
│   └── Field.Success
├── Form
│   ├── Form.Root
│   ├── Form.Field (combines Field + input)
│   ├── Form.Submit
│   └── Form.Reset
├── FormRow
├── FormGroup
└── ValidationMessage
```

### Validation State Management

**Svelte (using formsnap or similar):**
```svelte
<script>
  import { Form } from '@inflatable-cookie/poodle-svelte';
  
  const schema = z.object({
    email: z.string().email(),
    password: z.string().min(8)
  });
</script>

<Form.Root {schema}>
  <Form.Field name="email">
    <Form.Label>Email</Form.Label>
    <Form.Input type="email" />
    <Form.Error />
  </Form.Field>
</Form.Root>
```

**GPUI:**
- Manual state management
- Validation on input/change/blur
- Error state in component

### Field Token Usage

| Element | Token |
|---------|-------|
| Label | `semantic.color.text.primary` |
| Description | `semantic.color.text.secondary` |
| Error text | `semantic.color.text.error` |
| Error border | `semantic.color.border.error` |
| Success text | `semantic.color.text.success` |
| Required indicator | `semantic.color.text.error` |

### Validation Timing Policy

**Default:** Validate on blur
**Exceptions:**
- Email: validate on change with debounce
- Confirm password: validate on change
- Search: validate on submit only

### Error Message Standards

- Be specific about the error
- Suggest how to fix it
- Use field name in message
- Keep under 100 characters

---

## Related

- Milestone: [g01.008](../../roadmaps/g01/008-action-and-text-entry-primitives.md)
- Milestone: [g02.001](../../roadmaps/g02/001-forms-and-validation-system-depth.md)
- Formsnap (Svelte): Form primitive library

---

## Next Task

Create Field and Form component contracts with validation specifications.
