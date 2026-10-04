import { invoke } from '@tauri-apps/api/core';

export type OgMeta = {
  title?: string;
  description?: string;
  image?: string;
  url?: string;
}

export async function fetchOgMeta(url: string): Promise<OgMeta> {
  return await invoke('fetch_og_meta', { url });
};

export async function getData(): Promise<Record<number, Data>> {
  return await invoke('get_data');
}

export async function deleteData(id: number): Promise<ApiResponse> {
  return await invoke('delete_data', { id });
}

export async function upsertData(payload: { id: number | null, data: Data }): Promise<ApiResponse> {
  return await invoke('upsert_data', { id: payload.id, data: payload.data });
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
