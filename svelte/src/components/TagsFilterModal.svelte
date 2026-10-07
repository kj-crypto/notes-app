<script lang="ts">
  let { showTagModal = $bindable(false), selectedTags = $bindable([]), tags }: { showTagModal: boolean; selectedTags: string[]; tags: string[] } = $props();

  let filter = $state('');

  // Tags not yet chosen
  let availableTags = $derived(tags.filter((tag) => !selectedTags.includes(tag) && tag.toLowerCase().includes(filter.trim().toLowerCase())));

  function choose(tag: string) {
    if (!selectedTags.includes(tag)) selectedTags = [...selectedTags, tag];
  }
  function unchoose(tag: string) {
    selectedTags = selectedTags.filter((t) => t !== tag);
  }
  function close() {
    filter = '';
    showTagModal = false;
  }
  function clear() {
    filter = '';
    selectedTags = [];
  }
</script>

{#if showTagModal}
  <div class="modal-backdrop">
    <div class="modal modal-tags">
      <div class="modal-content">
        <h3>Select tags</h3>
        <div class="modal-fields">
          <!-- Chosen tags box -->
          <div class="chosen-tags">
            {#if selectedTags.length === 0}
              <span class="placeholder">No tags chosen</span>
            {/if}
            {#each selectedTags as tag}
              <span class="tag chosen" onclick={() => unchoose(tag)}>{tag}</span>
            {/each}
          </div>
          <!-- Filter input -->
          <input class="tag-filter-input" placeholder="Filter tags..." bind:value={filter} />
          <!-- Available tags box -->
          <div class="available-tags">
            {#if availableTags.length === 0}
              <span class="placeholder">No tags found</span>
            {/if}
            {#each availableTags as tag}
              <span class="tag" onclick={() => choose(tag)}>{tag}</span>
            {/each}
          </div>
        </div>
        <div class="modal-actions">
          <button onclick={clear}>Clear</button>
          <button onclick={close}>Close</button>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  /* Universal box-sizing for layout consistency */
  *,
  *:before,
  *:after {
    box-sizing: border-box;
  }

  .modal-content {
    padding: 0;
    width: 100%;
    max-width: none;
    margin: 0;
    display: flex;
    flex-direction: column;
  }

  .modal-fields {
    width: 100%;
    margin: 0;
    padding: 0 0 1em 0;
    display: flex;
    flex-direction: column;
    gap: 0.7em;
  }

  .tag-filter-input {
    width: 100%;
    max-width: 100%;
    min-width: 0;
    box-sizing: border-box;
    margin: 0;
    padding: 6px;
    font-size: 1em;
    border-radius: 4px;
    border: 1px solid #bbb;
    display: block;
  }

  .chosen-tags,
  .available-tags {
    width: 100%;
    display: flex;
    flex-wrap: wrap;
    gap: 0.3em;
    min-height: 32px;
    background: var(--modal-chosen-bg, #f9f9f9);
    border-radius: 6px;
    padding: 5px 6px;
    border: 1px solid var(--modal-chosen-border, #eee);
    align-items: flex-start;
  }
  .chosen-tags {
    background: var(--modal-chosen-bg, #f3f7ff);
    border: 1px solid var(--modal-chosen-border, #b3d0ff);
    margin-bottom: 0.2em;
  }
  .available-tags {
    background: var(--modal-available-bg, #fff);
    border: 1px solid var(--modal-available-border, #eee);
    max-height: 120px;
    overflow-y: auto;
  }
  .tag {
    display: inline-flex;
    align-items: center;
    background: var(--modal-tag-bg, #e0e0e0);
    border-radius: 12px;
    padding: 2px 10px 2px 8px;
    font-size: 0.97em;
    margin: 1px 2px;
    cursor: pointer;
    user-select: none;
    position: relative;
    min-height: 22px;
    line-height: 1.2;
    transition: background 0.15s;
    color: var(--modal-tag-color, #222);
    border: 1px solid transparent;
  }
  .tag:hover {
    background: var(--modal-tag-hover-bg, #b3d0ff);
  }
  .tag.chosen {
    background: var(--modal-tag-chosen-bg, #90caf9);
    color: var(--modal-tag-chosen-color, #222);
    border: 1px solid var(--modal-tag-chosen-border, #2196f3);
  }
  .placeholder {
    color: var(--modal-placeholder-color, #aaa);
    font-style: italic;
  }
</style>
