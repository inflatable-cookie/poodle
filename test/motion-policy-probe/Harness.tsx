import { useState } from "react";

import { Collapsible } from "../../packages/react/components/src/Collapsible";
import { IconButton } from "../../packages/react/components/src/IconButton";
import { MotionPolicyProvider } from "../../packages/react/components/src/motion-policy";
import { Tabs } from "../../packages/react/components/src/Tabs";
import { ToastStack } from "../../packages/react/components/src/ToastStack";

export function Harness() {
  const [toastItems, setToastItems] = useState([
    { id: "keep", title: "Kept" },
    { id: "danger", title: "Danger", tone: "danger" as const },
  ]);
  const [tab, setTab] = useState("a");

  return (
    <section data-framework="react">
      <div data-case="disclosure">
        <Collapsible title="Details" defaultOpen>
          Disclosure body that must keep height while closing.
        </Collapsible>
      </div>

      <div data-case="tabs" style={{ width: "24rem" }}>
        <Tabs
          items={[
            { value: "a", label: "Alpha" },
            { value: "b", label: "Beta longer label" },
          ]}
          value={tab}
          activeEdge="underline"
          fullWidth
          onValueChange={(value) => {
            if (value) setTab(value);
          }}
        />
      </div>

      <div data-case="toast">
        <button type="button" data-entered-from>
          react outside
        </button>
        <ToastStack items={toastItems} onDismiss={(id) => setToastItems((current) => current.filter((item) => item.id !== id))} />
      </div>

      <div data-case="icon-reduced">
        <MotionPolicyProvider policy="reduced">
          <IconButton icon="star" ariaLabel="Star" />
        </MotionPolicyProvider>
      </div>
    </section>
  );
}
