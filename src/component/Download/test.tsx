import 'jest';
import { render, fireEvent } from '@testing-library/react';
import { mockIPC } from '@tauri-apps/api/mocks';
import { randomFillSync } from 'crypto';
import Download from './index';
import mockImage from '../../../test/mock/file';
import { wrapper } from '../../../test/util';
import { downloadItemsState } from '../../store';
import { Action } from '../../utils/action';

const DownloadTest = wrapper(<Download />, { download: downloadItemsState });

describe('<Download />', () => {
  beforeAll(() => {
    //@ts-ignore
    window.crypto = {
    //@ts-ignore
      getRandomValues: function (buffer) {
      //@ts-ignore
        return randomFillSync(buffer);
      },
    };
    mockIPC((cmd: string, args: Record<string, any>) => {
      if(cmd === Action.DOWNLOAD_ITEM) {
         return (args.a as number) + (args.b as number)
      }
    });
  });
	it('render correctly', () => {
		const { container } = render(<DownloadTest />);
		expect(container).toBeTruthy();
	});

	it('render right download list item length', () => {
		const { container } = render(<DownloadTest />);

		expect(container).toBeTruthy();
		expect(container.querySelectorAll('.bk-download__item').length).toBe(1);
	});

	it('retry download', () => {
		const value = [
			{
				url: mockImage,
				sample: 100,
				percent: 0,
				status: 'error',
			},
		];
    //@ts-ignore
    const spy = jest.spyOn(window, '__TAURI_IPC__')
		const { container } = render(<DownloadTest values={{ download: value }} />);
		fireEvent.click(container.querySelector('.bk-download__retry')!);
		expect(spy).toBeCalledTimes(1);
	});

	it.skip('update the status and percent', () => {
		const value = {
			download: [
				{
					url: mockImage,
					sample: 100,
					percent: 0,
					status: 'init',
				},
			],
		};
		const { container } = render(<DownloadTest values={{ download: value }} />);
		// act(() => {
		// 	ipcRenderer.send(EventDownload.STATUS, {
		// 		progress: 100,
		// 		status: 'success',
		// 		url: mockImage,
		// 	});
		// });
		expect(container.querySelectorAll('.bk-download__icon').length).toBe(1);
	});
});
