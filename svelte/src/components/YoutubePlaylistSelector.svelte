<script lang="ts">
  import { appData } from '$lib/tauriInvokes';
  import { tick } from 'svelte';

  const SUGGESTIONS_LIMIT = 5;

  let { playlists = $bindable([]), stripLink = $bindable(true), inputClass = '' }: { playlists: string[]; stripLink: boolean; inputClass: string } = $props();
  let isDropdownOpen = $state(false);
  let activeIndex = $state(-1); // Tracks currently keyboard-highlighted option
  let listElement = $state<HTMLUListElement | null>(null); // DOM reference for auto-scrolling
  let inputValue = $state('');
  let inputElement = $state<HTMLInputElement | null>(null);
  let currentPlaylists = $derived(Array.from(appData.playlists.keys()));

  $effect(() => {
    if (playlists.length > 0 && !inputValue) {
      inputValue = playlists.join(', ') + ', ';
    }
  });

  let currentSearchSegment = $derived.by(() => {
    const segments = inputValue.split(',');
    return segments[segments.length - 1].trim().toLowerCase();
  });

  let filteredPlaylists = $derived.by(() => {
    const searchString = currentSearchSegment;
    if (!searchString) return [];

    return currentPlaylists
      .filter((playlist) => {
        const isAlreadySelected = playlists.some((p) => p.toLowerCase() === playlist.toLowerCase());
        const matchesSearch = playlist.toLowerCase().includes(searchString);
        return matchesSearch && !isAlreadySelected;
      })
      .sort((a, b) => {
        const aStarts = a.toLowerCase().startsWith(searchString);
        const bStarts = b.toLowerCase().startsWith(searchString);
        if (aStarts && !bStarts) return -1;
        if (!aStarts && bStarts) return 1;
        return a.localeCompare(b);
      });
  });

  // Automatically adjust scroll position to keep the highlighted item in view
  $effect(() => {
    if (activeIndex >= 0 && listElement) {
      const activeChild = listElement.children[activeIndex] as HTMLElement;
      if (activeChild) {
        activeChild.scrollIntoView({ block: 'nearest' });
      }
    }
  });

  // Reset keyboard index whenever the user filters the list
  $effect(() => {
    if (filteredPlaylists.length) {
      activeIndex = -1;
    }
  });

  // Autocomplete Handler for mouse clicks and keyboard tab overrides
  async function autocomplete(selectedName: string) {
    const segments = inputValue.split(',').map((s) => s.trim());
    segments.pop();

    const cleanSegments = segments.filter(Boolean);
    playlists = [...cleanSegments, selectedName];
    inputValue = playlists.join(', ') + ', ';

    isDropdownOpen = false;
    activeIndex = -1;

    await tick();
    if (inputElement) {
      inputElement.focus();

      const endPosition = inputValue.length;
      inputElement.setSelectionRange(endPosition, endPosition);
      // Scroll to the maximum horizontal position
      inputElement.scrollLeft = inputElement.scrollWidth + 1000;
    }
  }

  // Intercept keyboard controls when suggestions are active
  function handleKeyDown(e: KeyboardEvent) {
    const totalItems = filteredPlaylists.length;
    if (totalItems === 0) return;

    if (e.key === 'ArrowDown') {
      e.preventDefault();
      isDropdownOpen = true;
      activeIndex = (activeIndex + 1) % totalItems;
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      isDropdownOpen = true;
      activeIndex = (activeIndex - 1 + totalItems) % totalItems;
    } else if (e.key === 'Enter') {
      if (isDropdownOpen && activeIndex >= 0 && activeIndex < totalItems) {
        e.preventDefault();
        autocomplete(filteredPlaylists[activeIndex]);
      }
    } else if (e.key === 'Tab') {
      if (isDropdownOpen) {
        e.preventDefault();
        const targetIndex = activeIndex >= 0 ? activeIndex : 0;
        autocomplete(filteredPlaylists[targetIndex]);
      }
    } else if (e.key === 'Escape') {
      isDropdownOpen = false;
    }
  }
</script>

<div class="single-row-container">
  <div class="playlist-selector-container">
    <input
      bind:this={inputElement}
      type="text"
      class={inputClass}
      placeholder="Youtube playlist names"
      value={inputValue}
      oninput={(e) => {
        inputValue = (e.target as HTMLInputElement).value;
        isDropdownOpen = true;

        const segments = inputValue.split(',').map((p) => p.trim());
        if (!inputValue.endsWith(',')) {
          segments.pop();
        }
        playlists = segments.filter((p) => p.length > 0);
      }}
      onkeydown={handleKeyDown}
      onfocus={() => (isDropdownOpen = true)}
      onblur={(e) => {
        const currentTarget = e.currentTarget as HTMLElement;
        if (!currentTarget.parentElement?.contains(e.relatedTarget as Node)) {
          isDropdownOpen = false;
        }
      }}
      autocomplete="off"
    />

    {#if isDropdownOpen && filteredPlaylists.length > 0}
      <!-- bind:this secures reference for scroll position updates -->
      <ul class="suggestions-dropdown" style="--suggestions-limit: {SUGGESTIONS_LIMIT}" bind:this={listElement} role="listbox">
        {#each filteredPlaylists as playlist, index}
          <li class:active={index === activeIndex}>
            <button type="button" onmousedown={() => autocomplete(playlist)} tabindex="-1">
              {playlist}
            </button>
          </li>
        {/each}
      </ul>
    {/if}
  </div>

  <label class="compact-checkbox-label">
    <input type="checkbox" bind:checked={stripLink} />
    <span>Strip link</span>
  </label>
</div>

<style>
  .single-row-container {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    width: 100%;
  }

  .playlist-selector-container {
    position: relative;
    flex: 1; /* Input expanding to take all available space */
  }

  .playlist-selector-container input {
    background: transparent;
    border: 1.5px solid transparent;
  }

  .compact-checkbox-label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.9rem;
    color: var(--modal-input-color, #232946);
    cursor: pointer;
    user-select: none;
    white-space: nowrap; /* Prevents text breaking into two lines */
  }

  .compact-checkbox-label input[type='checkbox'] {
    appearance: none; /* Strips the default native browser checkbox styling completely */
    -webkit-appearance: none;
    width: 18px;
    height: 18px;
    border: 1.5px solid var(--modal-input-border, #e0e6ed);
    border-radius: 4px;
    background: var(--modal-input-bg, #f7f7fa);
    cursor: pointer;
    margin: 0;
    display: grid;
    place-content: center;
    transition:
      background 0.15s,
      border-color 0.15s;
  }

  .compact-checkbox-label input[type='checkbox']:checked {
    background: var(--modal-input-focus, #6c63ff);
    border-color: var(--modal-input-focus, #6c63ff);
  }

  /* Pure CSS white checkmark indicator inside the box */
  .compact-checkbox-label input[type='checkbox']::before {
    content: '';
    width: 10px;
    height: 6px;
    border-left: 2px solid #ffffff;
    border-bottom: 2px solid #ffffff;
    transform: rotate(-45deg) translate(1px, -1px);
    scale: 0;
    transition:
      transform 0.1s ease-in-out,
      scale 0.1s ease-in-out;
  }

  .compact-checkbox-label input[type='checkbox']:checked::before {
    scale: 1;
  }

  /* Smooth focus ring indicator matching your app design guidelines */
  .compact-checkbox-label input[type='checkbox']:focus-visible {
    outline: 2px solid var(--modal-input-focus, #6c63ff);
    outline-offset: 2px;
  }

  .suggestions-dropdown {
    --dropdown-item-height: 36px;
    position: absolute;
    top: 100%;
    left: 0;
    width: 100%;
    z-index: 999;
    margin: 4px 0 0 0;
    padding: 0;
    list-style: none;
    background: var(--modal-bg, #ffffff);
    border: 1.5px solid var(--modal-input-border, #ccc);
    border-radius: 6px;
    box-shadow: var(--modal-shadow);

    max-height: calc(var(--suggestions-limit, 5) * var(--dropdown-item-height));
    overflow-y: auto;
    box-sizing: border-box;
  }

  .suggestions-dropdown li {
    width: 100%;
    height: var(--dropdown-item-height);
  }

  .suggestions-dropdown button {
    width: 100%;
    text-align: left;
    background: none;
    border: none;
    height: 100%;
    padding: 0 12px;
    /* Lock structural typography baseline directly to our item metric height */
    line-height: var(--dropdown-item-height);
    cursor: pointer;
    font-size: 0.95rem;
    color: var(--modal-input-color, #333);
    transition: background 0.1s;
    box-sizing: border-box;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis; /* Truncate extremely long playlist titles cleanly */
  }

  .suggestions-dropdown li.active button,
  .suggestions-dropdown button:hover {
    background-color: var(--modal-action-hover-bg, #f0f0f0);
    outline: none;
  }
</style>
