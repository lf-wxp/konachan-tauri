import 'jest';
import { render, fireEvent } from '@testing-library/react';
import Download from './index';
import mockImage from '../../../test/mock/file';
import { wrapper } from '../../../test/util';
import { downloadItemsState } from '../../store';

const DownloadTest = wrapper(<Download />, { download: downloadItemsState });

describe('<Download />', () => {
  it('render correctly', () => {
    const { container } = render(<DownloadTest />);
    expect(container).toBeTruthy();
  });

  it('render right download list item length', () => {
    const { container } = render(<DownloadTest />);

    expect(container).toBeTruthy();
    expect(container.querySelectorAll('.bk-download__item').length).toBe(0);
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
    const { container } = render(<DownloadTest values={{ download: value }} />);
    fireEvent.click(container.querySelector('.bk-download__retry')!);
  });

  it('update the status and percent', () => {
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
    expect(container.querySelectorAll('.bk-download__icon').length).toBe(1);
  });
});
