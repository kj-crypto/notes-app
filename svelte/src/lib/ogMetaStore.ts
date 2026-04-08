import { writable } from 'svelte/store';
import { fetchOgMeta } from './tauriInvokes';
import type { OgMeta } from './tauriInvokes';

type OgMetaCache = Record<string, OgMeta | null>;

function createOgMetaStore() {
  const { subscribe, update } = writable<OgMetaCache>({});

  async function fetch(url: string): Promise<OgMeta | null> {
    let meta: OgMeta | null = null;
    update(cache => {
      if (cache[url]) {
        meta = cache[url];
        return cache;
      }
      return cache;
    });
    if (meta) return meta;
    meta = await fetchOgMeta(url);
    update(cache => ({ ...cache, [url]: meta }));
    return meta;
  }

  return {
    subscribe,
    fetch
  };
}

export const ogMetaStore = createOgMetaStore();
