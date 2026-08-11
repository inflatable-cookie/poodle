<script lang="ts">
  import { HistoryCenter, type HistoryBranch, type HistoryEntry } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let linearEntries = $state<HistoryEntry[]>([
    { id: "mix-1", label: "Committed mix 1", position: "past" },
    { id: "arrange", label: "Arranged intro", position: "past" },
    { id: "draft", label: "Current draft", position: "current" },
  ]);

  let forkedEntries = $state<HistoryEntry[]>([
    { id: "mix-1", label: "Committed mix 1", position: "past" },
    { id: "checkpoint", label: "Checkpoint: pre-fork", position: "past", checkpoint: true },
    { id: "fork", label: "Fork point", position: "past", branchCount: 2, groupId: "mix-session" },
    { id: "draft", label: "Current draft", position: "current" },
  ]);

  let branches = $state<HistoryBranch[]>([
    { id: "b-lead", name: "feature/lead", entryCount: 3, current: true },
    { id: "b-mix", name: "feature/mix-2", entryCount: 1 },
  ]);

  const rejectionEntries: HistoryEntry[] = [
    { id: "mix-1", label: "Committed mix 1", position: "past" },
    { id: "draft", label: "Current draft", position: "current" },
  ];
</script>

<SpecimenLayout bareVariants>
  {#snippet children()}
    <div class="poodle-history-center-specimen">
      <!-- Only the first group opens by default. The popover portals to the
           theme root, so several open at once stack on top of each other
           regardless of where their triggers sit. -->
      <SpecimenGroup label="linear">
        <div class="poodle-history-center-specimen__anchor">
          <HistoryCenter
            entries={linearEntries}
            totalEntries={3}
            defaultOpen
            canUndo
            onSelectEntry={(id) => console.log("select", id)}
          />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="fork">
        <div class="poodle-history-center-specimen__anchor">
          <HistoryCenter
            entries={forkedEntries}
            totalEntries={4}
            branches={branches}
            totalBranches={2}

            canUndo
            onSelectEntry={(id) => console.log("select", id)}
            onCheckout={(branchId, entryId) => console.log("checkout", branchId, entryId)}
            onRenameBranch={(branchId, name) =>
              branches = branches.map((branch) => branch.id === branchId ? { ...branch, name } : branch)}
          />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="rejection">
        <div class="poodle-history-center-specimen__anchor">
          <HistoryCenter
            entries={rejectionEntries}
            totalEntries={2}

            rejection="Branch name is already taken on the authority"
          />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="empty">
        <div class="poodle-history-center-specimen__anchor">
          <HistoryCenter entries={[]} totalEntries={0} />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="loading">
        <div class="poodle-history-center-specimen__anchor">
          <HistoryCenter entries={[]} totalEntries={0} status="loading" />
        </div>
      </SpecimenGroup>
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-history-center-specimen { display: grid; gap: 2rem; min-height: 40rem; }
  .poodle-history-center-specimen__anchor { display: flex; justify-content: flex-end; width: min(42rem, 100%); }
</style>
