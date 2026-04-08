<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import NoteItem from './NoteItem.svelte';

  let { items = $bindable(), onChange }: { items: Record<number, any>; onChange: () => void } = $props();

  let noteRefs: Record<number, HTMLElement> = $state({});
  let numColumns = $derived.by(() => computeNumColumns());

  function computeNumColumns() {
    const maxTotalWidth = 3440;
    const gap = 10;
    const width = Math.min(window.innerWidth, maxTotalWidth);

    const n = Math.floor((width - 2 * gap) / (columnWidth + gap));
    return Math.min(Object.keys(items).length, Math.max(1, n));
  }

  function updateNumColumns() {
    numColumns = computeNumColumns();
  }

  let columnWidth = $derived.by(() => {
    const firstRef: HTMLElement = noteRefs[Number(Object.keys(noteRefs)[0])];
    if (firstRef) {
      return firstRef.offsetWidth;
    }
    return 320;
  });

  onMount(() => {
    updateNumColumns();
    window.addEventListener('resize', updateNumColumns);
  });
  onDestroy(() => {
    window.removeEventListener('resize', updateNumColumns);
  });

  let columns = $derived.by(() => {
    const cols: Array<Array<[string, any]>> = Array.from({ length: numColumns }, () => []);
    const columnHeights = Array(numColumns).fill(0);

    for (const [id, data] of Object.entries(items)) {
      const ref = noteRefs[Number(id)];
      const height = ref ? ref.offsetHeight : 0;
      const minIndex = columnHeights.indexOf(Math.min(...columnHeights));
      cols[minIndex].push([id, data]);
      columnHeights[minIndex] += height;
    }
    return cols;
  });
</script>

<div class="notes-masonry">
  {#each columns as column}
    <div class="notes-column" style="width: {columnWidth}px">
      {#each column as [id, data] (id)}
        <NoteItem id={Number(id)} {data} {onChange} bind:rootEl={noteRefs[Number(id)]} />
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
