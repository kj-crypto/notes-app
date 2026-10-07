import { mount, unmount } from 'svelte';
import NoteModal from '$components/NoteModal.svelte';

let currentModalInstance: any = null;

export function openNoteModal(id: number | null, type?: 'link' | 'note') {
  if (currentModalInstance) {
    unmount(currentModalInstance);
  }
  const target = document.body;

  currentModalInstance = mount(NoteModal, {
    target,
    props: {
      id: id ? id : null,
      type: type ? type : 'note',
      onClose: () => {
        if (currentModalInstance) {
          unmount(currentModalInstance);
          currentModalInstance = null;
        }
      },
    },
  });
}
