import { writable } from 'svelte/store';

export type ToastType = 'info' | 'success' | 'error' | 'warning';

export interface Toast {
  id: number;
  message: string;
  type: ToastType;
  duration: number;
}

function createToastStore() {
  const { subscribe, update } = writable<Toast[]>([]);
  let id = 0;

  function show(message: string, type: ToastType = 'info', duration = 3000) {
    const toast: Toast = { id: id++, message, type, duration };
    update((toasts) => [toast, ...toasts]);
    setTimeout(() => {
      update((toasts) => toasts.filter((t) => t.id !== toast.id));
    }, duration);
  }

  return { subscribe, show };
}

export const toasts = createToastStore();
