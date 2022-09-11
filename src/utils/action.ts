import { toast } from 'react-toastify';
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event'

import { ImageDetail } from '../model/image';
import { DownloadItem } from '../model/downloadItem';
import { TFunc1Void } from './type';

export enum Action {
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

const toastError = (msg: string) => {
  toast.error(msg, {
    position: 'top-center',
    theme: 'colored',
    autoClose: 1000,
    hideProgressBar: true,
    closeOnClick: false,
    pauseOnHover: false,
    draggable: false,
    progress: undefined,
  });
}

export const getPost = async (params: { page: number; tags: string; refresh: boolean, mode: 'xml' | 'json'}) => {
  try {
    const data = await invoke<Data>(Action.GET_POST, params);
    if (data.code !== 0) {
      toastError('获取图片失败，请重试');
    }
    return data.data;
  } catch {
    toastError('获取图片失败，请重试');
  }
};

export const downloadItem = (params: { url: string; preview: string }) => invoke<Data>(Action.DOWNLOAD_ITEM, params);

export const updateValue = (source: DownloadItem[], value: DownloadItem) => {
  return source.map((item) => {
    if (item.url !== value.url) return item;
    return {
      ...item,
      ...value
    }
  });
};

export const updateProgress = (source: DownloadItem[], action: ProgressAction, value: DownloadItem) => {
  if (action === ProgressAction.REMOVE) {
    return source.filter(item => item.url !== value.url);
  }
  return updateValue(source, value);
};

export const listenProgress = (callback: TFunc1Void<DownloadItem>) => {
  listen(Event.PROGRESS, (data: ProgressData) => {
    callback(data.payload);
  });
};
