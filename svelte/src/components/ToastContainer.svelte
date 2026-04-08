<script lang="ts">
  import { toasts } from '$lib/toastStore';
  import Toast from './Toast.svelte';
</script>

<div class="toast-container">
  {#each $toasts as toast (toast.id)}
    <Toast {...toast} show={true} />
  {/each}
</div>

<style>
  .toast-container {
    position: fixed;
    top: 40px;
    right: 40px;
    display: flex;
    flex-direction: column;
    gap: 12px;
    z-index: 1000;
    align-items: flex-end;
  }
</style>
