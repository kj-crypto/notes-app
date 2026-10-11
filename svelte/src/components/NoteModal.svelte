<script lang="ts">
  import { appData, upsertData, type Data } from '$lib/tauriInvokes';
  import { toasts } from '$lib/toastStore';
  import YoutubePlaylistSelector from './YoutubePlaylistSelector.svelte';

  export interface NoteModalProps {
    id?: number | null;
    type?: 'link' | 'note';
    onClose: () => void;
  }

  let { id = null, type = 'link', onClose }: NoteModalProps = $props();

  const initialData = id !== null ? appData.get(id) : null;
  type = initialData ? initialData.type : type;
  let content = $state(initialData ? initialData.data : '');
  let tags = $state(initialData ? initialData.tags.join(', ') : '');

  let isYouTubeLink = $derived(type === 'link' && (content.includes('youtube.com') || content.includes('youtu.be')));
  let playlistName = $state('');
  let shouldStripYoutubeLink = $state(true);

  function parseTags(raw: string): string[] {
    return Array.from(
      new Set(
        raw
          .split(',')
          .map((t) => t.trim())
          .filter(Boolean)
          .map((t) => t.toLowerCase()),
      ),
    );
  }

  async function submit() {
    if (!content) return;
    const payload: { id: number | null; data: Data } = {
      id,
      data: {
        type: type as 'link' | 'note',
        data: content,
        tags: parseTags(tags),
      },
    };
    const response = await upsertData({ ...payload, playlistName: isYouTubeLink ? playlistName : null });
    if (response.status === 'success') {
      toasts.show(`${type === 'link' ? 'Link' : 'Note'} saved!`, 'success');
    } else {
      toasts.show(`${type === 'link' ? 'Link' : 'Note'} failed to save. ${response.message}`, 'error');
    }
    onClose();
  }
</script>

<div class="modal-backdrop" onclick={onClose}></div>
<div class="modal" onclick={(e) => e.stopPropagation()}>
  <h2>{id === null ? 'Add ' : 'Edit '} {type === 'link' ? 'Link' : 'Note'}</h2>
  <div class="modal-body-form">
    {#if type === 'link'}
      <input type="url" class="modal-field-input" placeholder="Paste link..." bind:value={content} />
    {:else}
      <textarea class="modal-field-input" placeholder="Write note..." bind:value={content}></textarea>
    {/if}
    <input type="text" class="modal-field-input" placeholder="Tags (comma separated)" bind:value={tags} />
    {#if isYouTubeLink}
      <YoutubePlaylistSelector bind:name={playlistName} bind:stripLink={shouldStripYoutubeLink} inputClass="modal-field-input" />
    {/if}
  </div>
  <div class="modal-actions">
    <button onclick={submit}>Save</button>
    <button onclick={onClose}>Cancel</button>
  </div>
</div>

<style>
  h2 {
    margin: 0 0 0.5rem 0;
    font-size: 1.3rem;
    font-weight: 600;
    color: var(--modal-header-color, #232946);
    letter-spacing: 0.01em;
  }

  .modal-body-form {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    margin-bottom: 1.25rem;
  }

  .modal :global(.modal-field-input) {
    width: 100%;
    padding: 0.65rem 0.9rem;
    border-radius: 6px;
    border: 1.5px solid var(--modal-input-border, #e0e6ed);
    font-size: 1rem;
    background: var(--modal-input-bg, #f7f7fa);
    color: var(--modal-input-color, #232946);
    transition: border-color 0.18s;
    box-sizing: border-box;
  }

  .modal :global(.modal-field-input):focus {
    border-color: var(--modal-input-focus, #6c63ff);
    outline: none;
  }

  .modal textarea.modal-field-input {
    min-height: 78px;
    resize: vertical;
  }
</style>
