<script lang="ts">
  import ConfirmModal from './ConfirmModal.svelte';
  import NoteModal from './NoteModal.svelte';
  import { deleteData } from '$lib/tauriInvokes';
  import { toasts } from '$lib/toastStore';
  import { getOgMeta } from '$lib/ogMetaFetch';
  import type { OgMeta } from '$lib/tauriInvokes';
  import { openUrl } from '$lib/tauriInvokes';

  let {
    id,
    data,
    onChange,
    rootEl = $bindable(),
  }: {
    id: number;
    data: { type: string; data: string; tags: string[] };
    onChange: () => void;
    rootEl?: HTMLElement;
  } = $props();
  let showConfirm = $state(false);
  let showEditModal = $state(false);
  let ogMeta: OgMeta = $state({});

  const onDelete: () => void = async () => {
    showConfirm = false;
    const response = await deleteData(id);
    if (response.status === 'success') {
      toasts.show('Item deleted', 'success');
      onChange();
    } else {
      toasts.show(`Failed to delete item: ${response.message}`, 'error');
    }
  };

  const fetchOgMeta = async () => {
    if (data.type !== 'link') return;
    ogMeta = (await getOgMeta(data.data)) || {};
  };

  $effect(() => {
    fetchOgMeta();
  });
</script>

<ConfirmModal bind:open={showConfirm} message="Are you sure you want to delete this item?" onConfirm={onDelete} onCancel={() => (showConfirm = false)} />
{#if showEditModal}
  <NoteModal bind:open={showEditModal} type={data.type} {id} content={data.data} tags={data.tags.join(', ')} onSubmit={onChange} />
{/if}

{#snippet itemActions()}
  <div class="item-actions">
    <button title="Edit" onclick={() => (showEditModal = true)}>✏️</button>
    <button title="Delete" onclick={() => (showConfirm = true)}>🗑️</button>
  </div>
{/snippet}

{#snippet itemTags()}
  <div class="item-tags">
    {#each data.tags as tag}
      <div class="item-tag">{tag}</div>
    {/each}
  </div>
{/snippet}

<div bind:this={rootEl} class="item-container">
  {#if data.type === 'link' && (ogMeta.title || ogMeta.description || ogMeta.image)}
    <div class="item-header">
      {#if ogMeta.title}
        <div class="item-title">{ogMeta.title}</div>
      {/if}
      {@render itemActions()}
    </div>
    {#if ogMeta.description}
      <div class="item-desc">{ogMeta.description}</div>
    {/if}
    <div class="og-bottom">
      {#if ogMeta.image}
        <div class="og-img-container">
          <a
            href="#top"
            onclick={() => {
              openUrl(data.data, 'firefox', true);
            }}
          >
            <img src={ogMeta.image} alt="preview" />
          </a>
        </div>
      {/if}
      {@render itemTags()}
    </div>
  {:else if data.type === 'link'}
    <div class="link-fallback-row">
      <a
        class="link"
        href="#top"
        onclick={() => {
          openUrl(data.data, 'firefox', true);
        }}>{data.data}</a
      >
      {@render itemActions()}
    </div>
    {@render itemTags()}
  {:else if data.type === 'note'}
    <div class="item-header">
      <span class="item-title">Note</span>
      {@render itemActions()}
    </div>
    <div class="item-desc">{data.data}</div>
    {@render itemTags()}
  {/if}
</div>

<style>
  .item-container {
    display: flex;
    flex-direction: column;
    gap: 0.12rem;
    padding: 0.33rem 0.5rem;
    border-radius: 6px;
    background: var(--note-bg);
    box-shadow: var(--note-shadow-main);
    border: 1px solid var(--note-border);
    width: 300px;
    margin: 0;
    transition:
      box-shadow 0.15s,
      border-color 0.15s,
      background 0.15s;
  }
  .item-container:hover {
    box-shadow: var(--note-shadow-hover);
    border-color: var(--note-border-hover);
  }

  .item-header {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 2px;
    gap: 0.5rem;
    flex-wrap: nowrap;
  }

  .item-title {
    flex: 1 1 auto;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    line-clamp: 2;
    font-size: 1rem;
    font-weight: bold;
    white-space: normal;
    color: var(--note-title);
  }

  .item-actions {
    display: flex;
    flex-direction: row;
    gap: 0.3rem;
    flex-shrink: 0;
  }

  .item-desc {
    font-size: 0.92em;
    color: var(--note-desc);
    margin-top: 2px;
    margin-bottom: 2px;
    line-height: 1.4;
    white-space: pre-wrap;
  }

  .og-bottom {
    display: flex;
    gap: 12px;
    align-items: flex-start;
  }

  .og-img-container img {
    max-width: 100%;
    height: 70px;
    width: auto;
    border-radius: 4px;
    background: #eee;
    display: block;
    margin: 0 auto 8px auto;
  }

  .link {
    color: var(--note-link);
    font-size: 0.95em;
    text-decoration: underline;
    word-break: break-all;
  }

  .item-tags {
    display: flex;
    flex-direction: row;
    gap: 4px;
    flex-wrap: wrap;
    margin-top: 0.6em;
  }

  .item-tag {
    background: var(--note-tag-bg);
    color: var(--note-tag-color);
    border-radius: 3px;
    padding: 1.5px 6px;
    font-size: 0.82em;
    border: 1px solid var(--note-tag-border);
    white-space: nowrap;
    margin-top: 0;
  }

  .link-fallback-row {
    display: flex;
    align-items: center;
    gap: 0.5em;
  }
</style>
