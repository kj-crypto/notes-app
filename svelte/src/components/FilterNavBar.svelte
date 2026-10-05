<script lang="ts">
  import TagsFilterModal from './TagsFilterModal.svelte';

  let {
    filterData = $bindable(),
    tags,
  }: {
    filterData: {
      contentFilter: string;
      typeFilter: 'both' | 'note' | 'link';
      selectedTags: string[];
    };
    tags: string[];
  } = $props();

  let showTagModal = $state(false);

  function removeTag(tag: string) {
    filterData.selectedTags = filterData.selectedTags.filter((t) => t !== tag);
  }

  const typeStates: { key: 'both' | 'note' | 'link'; label: string; icon: string }[] = [
    { key: 'both', label: 'Both', icon: '🌀' },
    { key: 'note', label: 'Note', icon: '📝' },
    { key: 'link', label: 'Link', icon: '🔗' },
  ];
</script>

<div class="filter-navbar">
  <div class="input-clear-wrapper">
    <input type="text" placeholder="Filter content..." bind:value={filterData.contentFilter} />
    {#if filterData.contentFilter}
      <button
        type="button"
        class="clear-btn"
        onclick={() => {
          filterData.contentFilter = '';
        }}
        aria-label="Clear"
        tabindex="-1">&times;</button
      >
    {/if}
  </div>
  <div class="row actions">
    <div class="three-state-group">
      {#each typeStates as state}
        <button
          class:selected={filterData.typeFilter === state.key}
          onclick={() => {
            filterData.typeFilter = state.key;
          }}
          aria-pressed={filterData.typeFilter === state.key}
          type="button"
        >
          <span class="icon">{state.icon}</span>
          {state.label}
        </button>
      {/each}
    </div>

    <button class="filter-tags-btn" onclick={() => (showTagModal = true)}> Filter Tags </button>

    <div class="selected-tags">
      {#each filterData.selectedTags as tag (tag)}
        <span class="tag">
          {tag}
          <button class="remove" onclick={() => removeTag(tag)}>&times;</button>
        </span>
      {/each}
    </div>
  </div>
</div>

<TagsFilterModal bind:showTagModal bind:selectedTags={filterData.selectedTags} {tags} />

<style>
  .input-clear-wrapper input[type='text'] {
    width: 100%;
    background: var(--input-bg, #fff);
    color: var(--input-fg, #18181b);
    border: 1.5px solid var(--input-border, #3b3b4f);
    border-radius: 5px;
    padding: 0.5em 2em 0.5em 0.7em;
    font-weight: 500;
    box-shadow: 0 1px 4px rgba(30, 41, 59, 0.07);
    transition:
      background 0.15s,
      color 0.15s,
      border-color 0.15s,
      box-shadow 0.15s;
  }
  .input-clear-wrapper input[type='text']:focus {
    outline: none;
    border-color: var(--accent, #6366f1);
    background: var(--input-bg-focus, #f4f4ff);
  }

  .input-clear-wrapper input[type='text']::placeholder {
    color: var(--input-placeholder, #888);
    opacity: 1;
  }

  .clear-btn {
    position: absolute;
    right: 0.6em;
    background: none;
    border: none;
    color: var(--input-clear, #444);
    font-size: 1.4em;
    font-weight: 700;
    cursor: pointer;
    padding: 0;
    display: flex;
    align-items: center;
    height: 100%;
    transition:
      color 0.15s,
      opacity 0.15s;
    box-shadow: none;
    opacity: 0.78;
    line-height: 1;
    user-select: none;
  }
  .clear-btn:hover,
  .clear-btn:focus {
    color: var(--accent, #6366f1);
    background: none;
    outline: none;
    opacity: 1;
  }

  /* DARK THEME OVERRIDES */
  :global(body.dark) .input-clear-wrapper input[type='text'] {
    background: var(--input-bg-dark, #232336);
    color: var(--input-fg-dark, #fafaff);
    border-color: var(--input-border-dark, #6b7280);
    box-shadow: 0 1px 8px rgba(30, 41, 59, 0.13);
  }
  :global(body.dark) .input-clear-wrapper input[type='text']:focus {
    background: var(--input-bg-dark-focus, #2a2a40);
  }
  :global(body.dark) .input-clear-wrapper input[type='text']::placeholder {
    color: var(--input-placeholder-dark, #aaa);
  }
  :global(body.dark) .clear-btn {
    color: var(--input-clear-dark, #e2e8f0);
  }
  :global(body.dark) .clear-btn:hover,
  :global(body.dark) .clear-btn:focus {
    color: var(--accent, #66aaff);
  }

  .filter-navbar {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    background: var(--card-bg);
    border-radius: 10px;
    padding: 1.1rem 1.2rem 1rem 1.2rem;
    box-shadow: var(--note-shadow-main);
    border: 1px solid var(--border-color);
    color: var(--foreground);
    transition:
      background 0.2s,
      color 0.2s;
  }

  .input-clear-wrapper {
    position: relative;
    display: flex;
    align-items: center;
    width: 100%;
  }

  .input-clear-wrapper input[type='text'] {
    width: 100%;
    background: var(--background);
    color: var(--foreground);
    border: 1px solid var(--border-color);
    border-radius: 5px;
    padding: 0.5em 2em 0.5em 0.7em;
    transition:
      background 0.15s,
      color 0.15s,
      border-color 0.15s;
  }

  .input-clear-wrapper input[type='text']::placeholder {
    color: var(--border-color);
    opacity: 1;
  }

  .clear-btn {
    position: absolute;
    right: 0.6em;
    background: none;
    border: none;
    color: var(--border-color);
    font-size: 1.2em;
    cursor: pointer;
    padding: 0;
    display: flex;
    align-items: center;
    height: 100%;
    transition:
      color 0.15s,
      opacity 0.15s;
    box-shadow: none;
    opacity: 0.6;
  }
  .clear-btn:hover,
  .clear-btn:focus {
    color: var(--foreground);
    background: none;
    outline: none;
    opacity: 1;
  }

  .row.actions {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 1em;
    flex-wrap: wrap;
  }

  .three-state-group {
    display: flex;
    gap: 0.5em;
  }

  .three-state-group button {
    background: var(--card-bg);
    color: var(--foreground);
    border: 1px solid var(--border-color);
    border-radius: 5px;
    padding: 0.3em 0.7em;
    cursor: pointer;
    transition:
      background 0.15s,
      color 0.15s,
      border-color 0.15s;
    font-size: 1em;
  }
  .three-state-group button.selected,
  .three-state-group button[aria-pressed='true'] {
    background: var(--accent);
    color: #fff;
    border-color: var(--accent);
  }

  .filter-tags-btn {
    background: var(--accent);
    color: #fff;
    border: 1px solid var(--accent);
    border-radius: 5px;
    padding: 0.3em 1em;
    cursor: pointer;
    transition:
      background 0.15s,
      color 0.15s,
      border-color 0.15s;
    font-size: 1em;
  }
  .filter-tags-btn:hover,
  .filter-tags-btn:focus {
    background: var(--note-border-hover);
    color: #fff;
    box-shadow: 0 0 0 2px var(--note-border-hover);
  }

  .selected-tags {
    display: flex;
    gap: 0.4em;
    flex-wrap: wrap;
  }

  .tag {
    background: var(--note-tag-bg);
    color: var(--note-tag-color);
    border-radius: 3px;
    padding: 1.5px 6px;
    font-size: 0.82em;
    border: 1px solid var(--note-tag-border);
    white-space: nowrap;
    display: flex;
    align-items: center;
    gap: 0.2em;
  }
  .tag .remove {
    background: none;
    border: none;
    color: var(--note-tag-color);
    font-size: 1.1em;
    cursor: pointer;
    padding: 0 2px;
    opacity: 0.7;
    transition:
      color 0.15s,
      opacity 0.15s;
  }
  .tag .remove:hover,
  .tag .remove:focus {
    color: var(--accent);
    opacity: 1;
  }
</style>
