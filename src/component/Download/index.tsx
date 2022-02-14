import React, { useContext, useEffect } from 'react';
import {
  IoIosCheckmarkCircle,
  IoMdInformationCircleOutline,
  IoIosRefresh
} from 'react-icons/io';
import PerfectScrollbar from 'react-perfect-scrollbar';
import { useRecoilValue } from 'recoil';
import { downloadItemsState  } from '../../store';
import { DownloadItem, DownloadStatus } from '../../model/downloadItem';
import Progress from '../Progress';

import 'react-perfect-scrollbar/dist/css/styles.css';
import './style.pcss';

export default React.memo(() => {
  const downloadItems = useRecoilValue<DownloadItem[]>(downloadItemsState)

  const downloadRetry = (url: string): void => {};

  return (
    <section className='bk-download'>
      <PerfectScrollbar>
        <div className='bk-download__box'>
          {downloadItems.map((item: DownloadItem, key: number) => (
            <div className='bk-download__item' key={key}>
              {item.status ===  DownloadStatus.SUCCESS && (
                <span className='bk-download__icon'>
                  <IoIosCheckmarkCircle />
                </span>
              )}
              {item.status === DownloadStatus.FAIL && (
                <div className='bk-download__catch'>
                  <span className='bk-download__error'>
                    <IoMdInformationCircleOutline />
                  </span>
                  <span
                    className='bk-download__retry'
                    onClick={(): void => downloadRetry(item.url)}
                  >
                    <IoIosRefresh />
                  </span>
                </div>
              )}
              <img src={item.preview} alt='preview' />
              <Progress
                percent={item.percent}
                error={item.status === DownloadStatus.FAIL}
              />
            </div>
          ))}
        </div>
      </PerfectScrollbar>
    </section>
  );
});
