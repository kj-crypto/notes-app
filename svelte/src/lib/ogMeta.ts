import { fetchOgMeta } from './tauriInvokes';
import type { OgMeta } from './tauriInvokes';
import { linksState } from './appStates.svelte';

export const BrokenLinkInstance = Symbol('BrokenLink');
export type BrokenLink = typeof BrokenLinkInstance;

const ogMetaCache = new Map<string, OgMeta | null | BrokenLink>();
const pendingPromises = new Map<string, Promise<OgMeta | null | BrokenLink>>();

export async function getOgMeta(url: string, cached = true): Promise<OgMeta | null | BrokenLink> {
  if (cached && ogMetaCache.has(url)) {
    return ogMetaCache.get(url)!;
  }
  if (pendingPromises.has(url)) {
    return pendingPromises.get(url)!;
  }

  const promise = async () => {
    try {
      const meta = await fetchOgMeta(url);
      ogMetaCache.set(url, meta);
      return meta;
    } catch (error) {
      console.error('Failed to fetch OG meta:', error);
      if (String(error).toLowerCase().includes('broken link')) {
        ogMetaCache.set(url, BrokenLinkInstance);
        linksState.isAnyBrokenLinks = true;
        return BrokenLinkInstance;
      }
      ogMetaCache.set(url, null);
      linksState.isAnyMeteToRefresh = true;
      return null;
    } finally {
      pendingPromises.delete(url);
    }
  };
  const runningPromise = promise();
  pendingPromises.set(url, runningPromise);
  return runningPromise;
}
