<script lang="ts">
  import { Region } from "@inflatable-cookie/poodle-svelte";
  import { SplitView } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
</script>

<div class="poodle-specimen">
  <!-- 1. Simple layout with Regions -->
  <SpecimenGroup label="Basic horizontal layout">
    <div class="poodle-specimen__frame">
      <SplitView orientation="horizontal">
        {#snippet primary()}
          <div class="poodle-specimen__fill">
            <Region label="Sidebar" color="blue" />
          </div>
        {/snippet}
        {#snippet secondary()}
          <div class="poodle-specimen__fill">
            <Region label="Main content" color="green" />
          </div>
        {/snippet}
      </SplitView>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Basic vertical layout">
    <div class="poodle-specimen__frame poodle-specimen__frame--tall">
      <SplitView orientation="vertical">
        {#snippet primary()}
          <div class="poodle-specimen__fill">
            <Region label="Editor" color="blue" />
          </div>
        {/snippet}
        {#snippet secondary()}
          <div class="poodle-specimen__fill">
            <Region label="Terminal" color="purple" />
          </div>
        {/snippet}
      </SplitView>
    </div>
  </SpecimenGroup>

  <!-- 2. Horizontal with collapse toggles -->
  <SpecimenGroup label="Horizontal with collapse toggles (drag to edge to collapse)">
    <div class="poodle-specimen__frame">
      <SplitView
        orientation="horizontal"
        showCollapsePrimary
        showCollapseSecondary
      >
        {#snippet primary()}
          <div class="poodle-specimen__fill">
            <Region label="Primary" color="blue" />
          </div>
        {/snippet}
        {#snippet secondary()}
          <div class="poodle-specimen__fill">
            <Region label="Secondary" color="green" />
          </div>
        {/snippet}
      </SplitView>
    </div>
  </SpecimenGroup>

  <!-- 3. Vertical with collapse toggles -->
  <SpecimenGroup label="Vertical with collapse toggles">
    <div class="poodle-specimen__frame poodle-specimen__frame--tall">
      <SplitView
        orientation="vertical"
        showCollapsePrimary
        showCollapseSecondary
      >
        {#snippet primary()}
          <div class="poodle-specimen__fill">
            <Region label="Top" color="blue" />
          </div>
        {/snippet}
        {#snippet secondary()}
          <div class="poodle-specimen__fill">
            <Region label="Bottom" color="purple" />
          </div>
        {/snippet}
      </SplitView>
    </div>
  </SpecimenGroup>

  <!-- 4. Hover-revealed toggles -->
  <SpecimenGroup label="Hover-revealed toggles (move the pointer onto the seam)">
    <div class="poodle-specimen__frame">
      <SplitView
        orientation="horizontal"
        toggleVisibility="hover"
        showCollapsePrimary
        showCollapseSecondary
      >
        {#snippet primary()}
          <div class="poodle-specimen__fill">
            <Region label="Primary" color="blue" />
          </div>
        {/snippet}
        {#snippet secondary()}
          <div class="poodle-specimen__fill">
            <Region label="Secondary" color="green" />
          </div>
        {/snippet}
      </SplitView>
    </div>
  </SpecimenGroup>

  <!-- 5. Nested splits (IDE-style layout) -->
  <SpecimenGroup label="Nested splits (IDE-style layout)">
    <div class="poodle-specimen__frame poodle-specimen__frame--tall">
      <SplitView
        orientation="horizontal"
        ratio={0.25}
        showCollapsePrimary
      >
        {#snippet primary()}
          <div class="poodle-specimen__fill">
            <Region label="Explorer" color="blue" />
          </div>
        {/snippet}
        {#snippet secondary()}
          <div class="poodle-specimen__fill">
            <SplitView
              orientation="vertical"
              ratio={0.65}
              showCollapseSecondary
            >
              {#snippet primary()}
                <div class="poodle-specimen__fill">
                  <Region label="Editor" color="green" />
                </div>
              {/snippet}
              {#snippet secondary()}
                <div class="poodle-specimen__fill">
                  <Region label="Terminal" color="purple" />
                </div>
              {/snippet}
            </SplitView>
          </div>
        {/snippet}
      </SplitView>
    </div>
  </SpecimenGroup>

  <!-- 6. Disabled -->
  <SpecimenGroup label="Disabled">
    <div class="poodle-specimen__frame">
      <SplitView
        orientation="horizontal"
        disabled
        showCollapsePrimary
        showCollapseSecondary
      >
        {#snippet primary()}
          <div class="poodle-specimen__fill">
            <Region label="Left" color="blue" />
          </div>
        {/snippet}
        {#snippet secondary()}
          <div class="poodle-specimen__fill">
            <Region label="Right" color="green" />
          </div>
        {/snippet}
      </SplitView>
    </div>
  </SpecimenGroup>
</div>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-specimen__frame {
    height: 10rem;
    border: 0.0625rem solid var(--poodle-color-border-subtle);
    border-radius: var(--poodle-radius-surface);
    overflow: visible;
  }

  .poodle-specimen__frame--tall {
    height: 16rem;
  }

  .poodle-specimen__fill {
    width: 100%;
    height: 100%;
  }

  .poodle-specimen__fill :global(.poodle-region) {
    height: 100%;
    min-height: 0;
  }
</style>
