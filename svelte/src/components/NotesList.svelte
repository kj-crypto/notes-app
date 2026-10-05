<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import NoteItem from './NoteItem.svelte';

  const COLUMN_WIDTH = 320;
  const GAP = 10;

  let { filteredIds }: { filteredIds: number[] } = $props();
  let numColumns = $state(1);
  let heights = $state<Record<number, number>>({});

  function watchHeight(node: HTMLElement, id: number) {
    const observer = new ResizeObserver((entries) => {
      for (let entry of entries) {
        const newHeight = entry.borderBoxSize?.[0]?.blockSize ?? node.offsetHeight;
        if (heights[id] !== newHeight) {
          heights[id] = newHeight;
        }
      }
    });
    observer.observe(node);
    return {
      destroy() {
        observer.disconnect();
        delete heights[id];
      }
    };
  }

  let columns = $derived.by(() => {
    const cols = Array.from({ length: numColumns }, () => []) as number[][]
    const columnHeights = Array(numColumns).fill(0);
    for (const id of filteredIds) {
      const height = heights[id] ?? 140;
      const minColumnIndex = columnHeights.indexOf(Math.min(...columnHeights));
      cols[minColumnIndex].push(id);
      columnHeights[minColumnIndex] += height + GAP;
    }
    return cols;
  })

  function computeNumColumns() {
    if (typeof window === 'undefined') return 1;
    const maxTotalWidth = 3440 - 2 * GAP;
    const width = Math.min(window.innerWidth, maxTotalWidth);
    const n = Math.floor((width - GAP) / (COLUMN_WIDTH + GAP));
    return Math.max(1, n);
  }

  function updateDimensions() {
    numColumns = computeNumColumns();
  }

  onMount(() => {
    updateDimensions();
    window.addEventListener('resize', updateDimensions);
  });

  onDestroy(() => {
    window.removeEventListener('resize', updateDimensions);
  });
</script>

<div class="notes-masonry" style="--grid-gap: {GAP}px; --col-width: {COLUMN_WIDTH}px">
  {#each columns as column}
    <div class="notes-column">
      {#each column as id (id)}
        <div class="tile-wrapper" use:watchHeight={id}>
          <NoteItem {id} />
        </div>
      {/each}
    </div>
  {/each}
</div>

<style>
  .notes-masonry {
    display: flex;
    gap: var(--grid-gap);
    justify-content: center;
    align-items: flex-start;
    width: 100%;

    padding: 20px;
    box-sizing: border-box;
  }
  .notes-column {
    width: var(--col-width);
    display: flex;
    flex-direction: column;
    gap: var(--grid-gap);
    flex-shrink: 0;
    position: relative;

    padding: 0 !important;
    margin: 0 !important;
    align-items: center;
  }

  .tile-wrapper {
    width: var(--col-width);
    display: block;
    box-sizing: border-box;
    margin: 0 !important;
    padding: 0 !important;
  }
</style>
