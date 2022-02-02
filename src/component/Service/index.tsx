import React, { useEffect } from 'react';
import { useRecoilState } from 'recoil';
import { invoke } from '@tauri-apps/api/tauri';

import {
	imagesState,
	totalState,
	loadingState,
	tagsState,
	pageState,
	refreshToggleState,
} from '../../store';

import { ImageDetail } from '../../model/image';

export default React.memo(() => {
	const [, setImages] = useRecoilState(imagesState);
	const [, setTotal] = useRecoilState(totalState);
	const [, setLoading] = useRecoilState(loadingState);
	const [tags] = useRecoilState(tagsState);
	const [page] = useRecoilState(pageState);
	const [refresh] = useRecoilState(refreshToggleState);

	useEffect(() => {
		setLoading(true);
		invoke<{ images: ImageDetail[]; count: number }>('get_post', { page, tags, refresh })
			.then((data) => {
				setImages(data.images);
				setTotal(data.count);
			})
			.finally(() => {
				setLoading(false);
			});
	}, [refresh, tags, page]);
	return null;
});
