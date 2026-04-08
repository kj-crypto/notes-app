<script lang="ts">
  let { open = $bindable(false), message = 'Are you sure?', onConfirm, onCancel }: { open: boolean; message: string; onConfirm: () => void; onCancel: () => void } = $props();
</script>

{#if open}
  <div class="modal-backdrop"></div>
  <div class="modal">
    <p>{message}</p>
    <div class="modal-actions">
      <button onclick={onConfirm}>Yes</button>
      <button onclick={onCancel}>No</button>
    </div>
  </div>
{/if}

<style>
  /* Styles are imported from modal.css */
  .modal {
    max-width: 340px;
    min-width: 0;
    width: auto;
    padding: 1.5rem 1.2rem 1.2rem 1.2rem;
    box-sizing: border-box;
  }
  p {
    margin: 0 0 1rem 0;
    font-size: 1.08rem;
    color: var(--foreground, #232946);
    text-align: center;
    font-weight: 500;
    letter-spacing: 0.01em;
  }
</style>
