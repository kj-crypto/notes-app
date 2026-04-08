<script lang="ts">
  import { onMount } from 'svelte';

  let dark = false;

  function setDark(d: boolean) {
    dark = d;
    document.body.classList.toggle('dark', dark);
    localStorage.setItem('theme', dark ? 'dark' : 'light');
  }

  function toggle() {
    setDark(!dark);
  }

  onMount(() => {
    const saved = localStorage.getItem('theme');
    if (saved) {
      setDark(saved === 'dark');
    } else {
      setDark(window.matchMedia('(prefers-color-scheme: dark)').matches);
    }
  });
</script>

<button class="theme-toggle-btn" onclick={toggle} aria-label="Toggle dark mode">
  {dark ? '🌙 Dark' : '☀️ Light'}
</button>

<style>
  .theme-toggle-btn {
    padding: 0.5rem 1.1rem;
    border-radius: 5px;
    border: none;
    font-size: 1rem;
    font-weight: 500;
    cursor: pointer;
    background: var(--accent, #6c63ff);
    color: #fff;
    box-shadow: 0 1px 2px rgba(24, 28, 38, 0.04);
    transition:
      background 0.18s,
      box-shadow 0.18s,
      color 0.18s;
    display: flex;
    align-items: center;
    gap: 0.5em;
  }

  .theme-toggle-btn:hover {
    background: #5548c8;
  }

  :global(body.dark) .theme-toggle-btn {
    background: var(--accent, #66aaff);
    color: #222;
  }

  :global(body.dark) .theme-toggle-btn:hover {
    background: #3377bb;
    color: #fff;
  }
</style>
