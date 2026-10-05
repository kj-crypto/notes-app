<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import NoteItem from './NoteItem.svelte';

  let { filteredIds }: { filteredIds: number[] } = $props();

  $effect(() => {
    console.log("++ NotesList, ids changed ", filteredIds);
  })

  let heights = $state<Record<number, number>>({});

    const n = Math.floor((width - 2 * gap) / (columnWidth + gap));
    return Math.min(Object.keys(items).length, Math.max(1, n));
  }

  $effect(() => {
    console.log("++ NotesList, heights changed ", $state.snapshot(heights));
  })

  let numColumns = 2;
  let columns = $derived.by(() => {
    const cols = Array.from({ length: numColumns }, () => []) as number[][]
    for (const id of filteredIds) {
      const colIndex = id % numColumns;
      cols[colIndex].push(id);
    }
    return cols;
  });
</script>

<div class="notes-masonry debug-masonry">
  {#each columns as column, colIdx}
    <div class="notes-column debug-column" style="width: {COLUMN_WIDTH}px">
      <span class="debug-col-label">Column {colIdx + 1}</span>

      {#each column as id (id)}
        <div class="tile-wrapper debug-tile">
          <NoteItem {id} />
        </div>
      {/each}
    </div>
  {/each}
</div>

<style>
  .notes-masonry {
    display: flex;
    gap: 10px;
    justify-content: center;
    padding: 10px;
  }
  .notes-column {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }
</style>
