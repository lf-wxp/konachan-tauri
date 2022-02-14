import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event'

import { ImageDetail } from '../model/image';
import { DownloadItem } from '../model/downloadItem';
import { TFunc1Void } from './type';

enum Action {
  GET_POST = 'get_post',
  DOWNLOAD_ITEM = 'download_image',
}

enum Event {
  PROGRESS = 'progress',
}

export enum ProgressAction {
  UPDATE = 'update',
  REMOVE = 'remove',
}

type Data = {
  data: {
    images: ImageDetail[];
    count: number;
  };
  code: number;
};

export type ProgressData = {
  payload: DownloadItem;
  event: string;
  id: number;
};


export const getPost = async (params: { page: number; tags: string; refresh: boolean }) => {
  try {
    const data = await invoke<Data>(Action.GET_POST, params);
    return data.data;
  } catch (error) {
    console.error(error);
  }
};

export const downloadItem = async (params: { url: string; preview: string }) => {
  try {
    await invoke<Data>(Action.DOWNLOAD_ITEM, params);
  } catch (error) {
    console.error(error);
  }
};

export const updateProgress = (source: DownloadItem[], action: ProgressAction, value: DownloadItem) => {
  if (action === ProgressAction.REMOVE) {
    return source.filter(item => item.url !== value.url);
  }
  return source.map((item) => {
    if (item.url !== value.url) return item;
    return {
      ...item,
      ...value
    }
  })
};

export const listenProgress = (callback: TFunc1Void<DownloadItem>) => {
  listen(Event.PROGRESS, (data: ProgressData) => {
    callback(data.payload);
  });
};
