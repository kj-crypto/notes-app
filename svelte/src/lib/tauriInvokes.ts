import { invoke } from '@tauri-apps/api/core';
import { SvelteMap } from 'svelte/reactivity';

export const appData = new SvelteMap<number, Data>();

export type OgMeta = {
  title?: string;
  description?: string;
  image?: string;
  url?: string;
};

export async function fetchOgMeta(url: string): Promise<OgMeta> {
  return await invoke('fetch_og_meta', { url });
}

export async function getData(): Promise<Record<number, Data>> {
  try {
    appData.clear();
    const data = (await invoke('get_data')) as Record<number, Data>;
    for (const [key, value] of Object.entries(data)) {
      appData.set(Number(key), value);
    }
    return data;
  } catch (error) {
    console.error('Error fetching data:', error);
    return {};
  }
}

export async function deleteData(id: number): Promise<ApiResponse> {
  try {
    await invoke('delete_data', { id });
    appData.delete(id);
    return { status: 'success', message: 'Data deleted' };
  } catch (error) {
    return { status: 'error', message: error as string };
  }
}

export async function upsertData(payload: { id: number | null; data: Data }): Promise<ApiResponse> {
  try {
    const changedId = (await invoke('upsert_data', { id: payload.id, data: payload.data })) as number;
    appData.set(changedId, payload.data);
    return { status: 'success', message: payload.id ? 'Data updated' : 'Data created' };
  } catch (error) {
    return { status: 'error', message: error as string };
  }
}

export async function openUrl(url: string, browser?: string, incognito?: boolean): Promise<void> {
  return await invoke('open_url', { url, browser, incognito });
}

export type Data = {
  type: 'note' | 'link';
  data: string;
  tags: string[];
};

export type ApiResponse = {
  status: string;
  message: string;
};
