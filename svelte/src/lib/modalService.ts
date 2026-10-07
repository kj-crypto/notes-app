import { mount, unmount } from 'svelte';
import NoteModal from '$components/NoteModal.svelte';
import ConfirmModal from '$components/ConfirmModal.svelte';

let currentModalInstance: any = null;

export function openNoteModal(id: number | null, type?: 'link' | 'note') {
  if (currentModalInstance) {
    unmount(currentModalInstance);
  }
  const target = document.body;

  currentModalInstance = mount(NoteModal, {
    target,
    props: {
      id: id ?? null,
      type: type ?? 'link',
      onClose: () => {
        if (currentModalInstance) {
          unmount(currentModalInstance);
          currentModalInstance = null;
        }
      },
    },
  });
}

export function openConfirmModal(message: string, onConfirm: () => void) {
  if (currentModalInstance) {
    unmount(currentModalInstance);
  }
  const target = document.body;
  const onClose = () => {
    if (currentModalInstance) {
      unmount(currentModalInstance);
      currentModalInstance = null;
    }
  };

  currentModalInstance = mount(ConfirmModal, {
    target,
    props: {
      message,
      onConfirm: () => {
        onConfirm();
        onClose();
      },
      onCancel: onClose,
    },
  });
}
