import { useEffect, useState } from "react";

import { TextInput } from "../../../../packages/react/components/src/TextInput";

/**
 * Conformance fixture host: mounts the real React TextInput and owns
 * the controlled value the same way a consumer host does.
 */

export function ReactTextInputHost(props: {
  fixture: { props: Record<string, unknown>; regions: Record<string, string> };
  onValueChange: (value: string) => void;
  onSubmit: (value: string) => void;
  onCancel: () => void;
  onClear: () => void;
}) {
  const initial = (props.fixture.props.value as string | null | undefined) ?? null;
  const [value, setValue] = useState<string | null>(initial);
  useEffect(() => {
    setValue(initial);
  }, [initial]);

  return (
    <TextInput
      {...(props.fixture.props as Record<string, never>)}
      value={value}
      leadingIcon={
        (props.fixture.regions.leading as string | undefined) ??
        (props.fixture.props.leadingIcon as string | null | undefined) ??
        null
      }
      trailingIcon={
        (props.fixture.regions.trailing as string | undefined) ??
        (props.fixture.props.trailingIcon as string | null | undefined) ??
        null
      }
      onValueChange={(next) => {
        props.onValueChange(next);
        setValue(next);
      }}
      onSubmit={props.onSubmit}
      onCancel={props.onCancel}
      onClear={props.onClear}
    />
  );
}
