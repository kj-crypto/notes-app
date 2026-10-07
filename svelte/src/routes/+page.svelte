<script lang="ts">
  import ToastContainer from '$components/ToastContainer.svelte';
  import NotesList from '$components/NotesList.svelte';
  import { getData, appData } from '$lib/tauriInvokes';
  import FilterNavBar from '$components/FilterNavBar.svelte';
  import ThemeToggle from '$components/ThemeToggle.svelte';
  import { onMount } from 'svelte';
  import { openNoteModal } from '$lib/modalService';

  let filterData: {
    contentFilter: string;
    typeFilter: 'both' | 'note' | 'link';
    selectedTags: string[];
  } = $state({
    contentFilter: '',
    typeFilter: 'both',
    selectedTags: [],
  });

  onMount(() => {
    getData();
  })

  let tags = $derived(
    Array.from(new Set(Array.from(appData.values()).flatMap((item) => item.tags)))
  );

  let filteredIds = $derived.by(() => {
    const entries = Array.from(appData.entries());

    return entries
      .filter(([_, item]) => {
        if (filterData.selectedTags.length > 0 && !item.tags.some(t => filterData.selectedTags.includes(t))) return false;
        if (filterData.typeFilter !== 'both' && item.type !== filterData.typeFilter) return false;
        if (filterData.contentFilter.trim() !== '' && !item.data.toLowerCase().includes(filterData.contentFilter.toLowerCase())) return false;
        return true;
      })
      .map(([id, _]) => id);
  });
</script>

<div class="page-root">
  <div class="navbar">
    <FilterNavBar bind:filterData {tags} />
    <div class="navbar-actions">
      <button
        onclick={() => {
          openNoteModal(null, 'link');
        }}>Add Link</button
      >
      <button
        onclick={() => {
          openNoteModal(null, 'note');
        }}>Add Note</button
      >
      <ThemeToggle />
    </div>
  </div>

  <div class="noteslist-scroll">
    <NotesList {filteredIds} />
  </div>
</div>

<ToastContainer />

<style>
  :global(html),
  :global(body) {
    overflow: hidden;
    background: var(--background, #f4f4f8);
    color: var(--foreground, #232946);
  }

  .page-root {
    height: 100vh;
    min-height: 600px;
    min-width: 400px;
    display: flex;
    flex-direction: column;
    background: var(--background, #f4f4f8);
    color: var(--foreground, #232946);
  }

  .navbar {
    position: sticky;
    top: 0;
    z-index: 10;
    background: var(--card-bg, #fff);
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    margin-bottom: 25px;
    width: 80%;
    max-width: 1400px;
    margin-left: auto;
    margin-right: auto;
    padding: 16px 24px;
    box-sizing: border-box;
    box-shadow: var(--note-shadow-main, 0 4px 20px rgba(0, 0, 0, 0.04));
    border-radius: 0 0 12px 12px;
    border: 1px solid var(--border-color, #e0e7ef);
    border-top: none;
  }

  .navbar-actions {
    display: flex;
    justify-content: flex-start;
    gap: 10px;
    margin-top: 0.2em;
  }

  .navbar-actions button {
    padding: 8px 16px;
    border: none;
    border-radius: 6px;
    background: var(--accent, #6366f1);
    color: #fff;
    font-weight: 500;
    font-size: 1rem;
    cursor: pointer;
    transition:
      background 0.15s,
      box-shadow 0.15s;
    box-shadow: 0 1px 3px var(--accent-shadow, rgba(99, 102, 241, 0.08));
    margin-right: 8px;
  }

  .navbar-actions button:last-child {
    margin-right: 0;
  }

  .navbar-actions button:hover,
  .navbar-actions button:focus {
    background: var(--note-border-hover, #5548c8);
    outline: none;
    box-shadow: 0 2px 8px var(--accent-shadow-hover, rgba(99, 102, 241, 0.13));
  }

  .noteslist-scroll {
    flex: 1 1 auto;
    overflow-y: auto;
    width: 100%;
    margin-left: auto;
    margin-right: auto;
    padding-bottom: 2em;
  }

  /* DARK THEME OVERRIDES */
  :global(body.dark) {
    background: var(--background-dark, #181826);
    color: var(--foreground-dark, #fafaff);
  }
  :global(body.dark) .page-root {
    background: var(--background-dark, #181826);
    color: var(--foreground-dark, #fafaff);
  }
  :global(body.dark) .navbar {
    background: var(--card-bg-dark, #232336);
    color: var(--foreground-dark, #fafaff);
    border-color: var(--border-color-dark, #363a4f);
    box-shadow: 0 4px 24px rgba(15, 15, 25, 0.3);
  }
  :global(body.dark) .navbar-actions button {
    background: var(--accent, #66aaff);
    color: #232336;
    box-shadow: 0 1px 3px rgba(102, 170, 255, 0.1);
  }
  :global(body.dark) .navbar-actions button:hover,
  :global(body.dark) .navbar-actions button:focus {
    background: var(--note-border-hover, #3377bb);
    color: #fff;
    box-shadow: 0 2px 8px rgba(102, 170, 255, 0.18);
  }
</style>
