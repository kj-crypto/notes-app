import { fetchOgMeta } from './tauriInvokes';
import type { OgMeta } from './tauriInvokes';

const ogMetaCache = new Map<string, OgMeta | null>();
const pendingPromises = new Map<string, Promise<OgMeta | null>>();

export async function getOgMeta(url: string): Promise<OgMeta | null> {
  if (ogMetaCache.has(url) && ogMetaCache.get(url) !== null) {
    return Promise.resolve(ogMetaCache.get(url)!);
  }
  if (pendingPromises.has(url)) {
    return pendingPromises.get(url)!
  }

  const promise = async () => {
    try {
      const meta = await fetchOgMeta(url);
      ogMetaCache.set(url, meta);
      return meta;
    }
    catch (error) {
      console.error('Failed to fetch OG meta:', error);
      return null;
    }
    finally {
      pendingPromises.delete(url);
    }
  }
  const runningPromise = promise();
  pendingPromises.set(url, runningPromise);
  return runningPromise;
}
