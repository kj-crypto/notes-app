<script lang="ts">
  import { onMount, onDestroy, untrack } from 'svelte';
  import NoteItem from './NoteItem.svelte';

  const COLUMN_WIDTH = 320;
  const GAP = 10;

  let { filteredIds }: { filteredIds: number[] } = $props();
  let numColumns = $state(1);
  let containerHeight = $state(0);
  let containerWidth = $derived(numColumns * (COLUMN_WIDTH + GAP) - GAP);
  let layoutVector = $state<Record<number, { height: number; top: number; left: number }>>({});

  // grid
  let grid: number[][];
  let columnHeights: number[];

  function fetchItemHeight(id: number, height: number) {
    if (layoutVector[id].height > 0) {
      updateLayout(id, layoutVector[id].height, height);
    } else {
      incrementalLayoutUpdate(id, height);
    }
  }

  function incrementalLayoutUpdate(id: number, height: number) {
    const layout = { height, top: 0, left: 0 };
    const colNum = columnHeights.indexOf(Math.min(...columnHeights));
    grid[colNum].push(id);
    layout.top = columnHeights[colNum];
    layout.left = colNum * (COLUMN_WIDTH + GAP);
    columnHeights[colNum] += layout.height + GAP;

    containerHeight = Math.max(...columnHeights);
    layoutVector[id] = layout;
  }

  function updateLayout(id: number, oldHeight: number, newHeight: number) {
    const i = layoutVector[id].left / (COLUMN_WIDTH + GAP);
    const j = grid[i].findIndex((x) => x === id);
    if (j === -1) return;

    layoutVector[id].height = newHeight;

    for (let idx = j + 1; idx < grid[i].length; ++idx) {
      layoutVector[grid[i][idx]].top += newHeight - oldHeight;
    }
    columnHeights[i] += newHeight - oldHeight;

    while (true) {
      let shuffled = false;
      for (let c = 0; c < grid.length; ++c) {
        const topId = grid[c][grid[c].length - 1];
        const topLayout = layoutVector[topId];

        const tmpColumnHeights = [...columnHeights];
        tmpColumnHeights[c] -= topLayout.height + GAP;
        const newTopIndex = tmpColumnHeights.indexOf(Math.min(...tmpColumnHeights));

        if (newTopIndex !== c) {
          grid[c].pop();
          grid[newTopIndex].push(topId);
          topLayout.top = tmpColumnHeights[newTopIndex];
          topLayout.left = newTopIndex * (COLUMN_WIDTH + GAP);
          tmpColumnHeights[newTopIndex] += topLayout.height + GAP;
          columnHeights = tmpColumnHeights;

          shuffled = true;
        }
      }
      if (!shuffled) {
        break;
      }
    }
    containerHeight = Math.max(...columnHeights);
  }

  function computeFullLayout() {
    columnHeights = Array(numColumns).fill(0);
    grid = Array.from({ length: numColumns }, () => []);
    const tempLayoutVector = {} as Record<number, { height: number; top: number; left: number }>;
    for (const id of filteredIds) {
      if (!layoutVector[id] || layoutVector[id].height === 0) {
        tempLayoutVector[id] = { height: 0, top: 0, left: 0 };
        continue;
      }
      if (layoutVector[id].height > 0) {
        const currentHeight = layoutVector[id].height;
        const colNum = columnHeights.indexOf(Math.min(...columnHeights));
        grid[colNum].push(id);
        tempLayoutVector[id] = {
          height: currentHeight,
          top: columnHeights[colNum],
          left: colNum * (COLUMN_WIDTH + GAP),
        };
        columnHeights[colNum] += currentHeight + GAP;
      }
    }
    containerHeight = Math.max(...columnHeights, 0);
    layoutVector = tempLayoutVector;
  }

  $effect(() => {
    filteredIds;
    numColumns;
    untrack(() => {
      computeFullLayout();
    });
  });

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

<div class="notes-list-container">
  <div class="notes-masonry-grid" style="height: {containerHeight}px; width: {containerWidth}px; --col-width: {COLUMN_WIDTH}px">
    {#each Object.keys(layoutVector) as strId (strId)}
      {@const id = Number(strId)}
      {@const layout = layoutVector[id]}
      {#if layout}
        <div class="tile-absolute-wrapper" style="transform: translate3d({layout.left}px, {layout.top}px, 0);">
          <NoteItem {id} onHeightChange={fetchItemHeight} />
        </div>
      {/if}
    {/each}
  </div>
</div>

<style>
  .notes-list-container {
    display: flex;
    justify-content: center;
    box-sizing: border-box;
    width: 100%;
  }

  .notes-masonry-grid {
    position: relative;
    box-sizing: border-box;
    display: block;

    transition:
      height 0.25s ease,
      width 0.25s ease;
  }

  .tile-absolute-wrapper {
    position: absolute;
    top: 0;
    left: 0;
    width: var(--col-width);
    display: block;
    box-sizing: border-box;
    margin: 0 !important;
    padding: 0 !important;

    transition: transform 0.3s cubic-bezier(0.25, 1, 0.5, 1);
    will-change: transform;
    content-visibility: auto;
    contain-intrinsic-size: 0 140px;
  }
</style>
