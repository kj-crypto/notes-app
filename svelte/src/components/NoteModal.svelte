<script lang="ts">
  import { appData, upsertData, type Data } from '$lib/tauriInvokes';
  import { toasts } from '$lib/toastStore';

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
    const response = await upsertData(payload);
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
  {#if type === 'link'}
    <input type="url" placeholder="Paste link..." bind:value={content} />
  {:else}
    <textarea placeholder="Write note..." bind:value={content}></textarea>
  {/if}
  <input type="text" placeholder="Tags (comma separated)" bind:value={tags} />
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

  .modal input[type='url'],
  .modal input[type='text'],
  .modal textarea {
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

  .modal input:focus,
  .modal textarea:focus {
    border-color: var(--modal-input-focus, #6c63ff);
    outline: none;
  }

  .modal textarea {
    min-height: 78px;
    resize: vertical;
  }
</style>
