import React, { useCallback, useRef } from 'react';
import { useRecoilState } from 'recoil';
import Vibrant from 'node-vibrant/dist/vibrant.worker.min';

import { colorSetState } from '../../store';
import { CSSVariable } from '../../utils/cssVariable';

import bg0 from '../../image/bg0.jpg';
import bg1 from '../../image/bg1.jpg';
import bg2 from '../../image/bg2.jpg';
import bg3 from '../../image/bg3.jpg';
import bg4 from '../../image/bg4.jpg';
import bg5 from '../../image/bg5.jpg';
import bg6 from '../../image/bg6.jpg';
import bg7 from '../../image/bg7.jpg';
import bg8 from '../../image/bg8.jpg';
import bg9 from '../../image/bg9.jpg';
import bg10 from '../../image/bg10.jpg';

import './style.pcss';

const bgs: string[] = [bg0, bg1, bg2, bg3, bg4, bg5, bg6, bg7, bg8, bg9, bg10];

const bgUri: string = bgs[Math.floor(Math.random() * 11)];

export default React.memo(() => {
	const [, setColor] = useRecoilState(colorSetState);
	const img = useRef(null as unknown as HTMLImageElement);
	const onLoad = useCallback(async () => {
		const palette = await Vibrant.from(img.current).getSwatches();
		const vibrantColor = palette.Vibrant?.hex;
		const mutedColor = palette.Muted?.hex;
		if (vibrantColor && mutedColor) {
			CSSVariable.setValue('--themeBaseColor', vibrantColor);
			CSSVariable.setValue('--themeMutedColor', mutedColor);
			setColor({
				mute: mutedColor,
				vibrant: vibrantColor,
			});
		}
	}, [setColor]);

	return (
		<figure className="bk-bg">
			<img
				ref={img}
				src={bgUri}
				className="bk-bg__image"
				alt="bg"
				onLoad={onLoad}
			/>
		</figure>
	);
});
