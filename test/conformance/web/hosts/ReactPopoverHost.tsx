import { useMemo, useState } from "react";
import { Popover } from "../../../../packages/react/components/src/Popover";

interface FixtureShape {
  props: Record<string, unknown>;
  regions: Record<string, string>;
  host?: Record<string, unknown>;
}

/**
 * Conformance fixture host: mounts the real React Popover from the case
 * fixture and owns the controlled open state the same way a consumer host
 * does (spec 066 harness step 1). Region strings render as the trigger and
 * content snippets; `host.nested` composes a second Popover inside the
 * content (the nested dismiss-stack proof). No expected result is restated.
 */
export function ReactPopoverHost({
  fixture,
  onOpenChange,
}: {
  fixture: FixtureShape;
  onOpenChange: (open: boolean) => void;
}) {
  const [open, setOpen] = useState<boolean | null>((fixture.props.open as boolean | null) ?? null);
  const props = useMemo(() => ({ ...fixture.props, open }), [fixture.props, open]);

  const handleOpenChange = (next: boolean): void => {
    onOpenChange(next);
    setOpen((current) => (current === null ? current : next));
  };

  const nested = fixture.host?.nested as { trigger?: string; children?: string } | undefined;

  return (
    <Popover
      {...(props as never)}
      onOpenChange={handleOpenChange}
      trigger={<div dangerouslySetInnerHTML={{ __html: fixture.regions.trigger }} />}
    >
      <div dangerouslySetInnerHTML={{ __html: fixture.regions.children }} />
      {nested ? (
        <Popover
          defaultOpen
          onOpenChange={onOpenChange}
          trigger={<div dangerouslySetInnerHTML={{ __html: nested.trigger ?? "" }} />}
        >
          <div dangerouslySetInnerHTML={{ __html: nested.children ?? "" }} />
        </Popover>
      ) : null}
    </Popover>
  );
}
