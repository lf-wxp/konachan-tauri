import React, { useMemo } from 'react';
import { useRecoilState } from 'recoil';

import { TFuncVoid } from '../../utils/type';
import {
  securityState,
  refreshToggleState,
  loadingState,
  modeState,
  colorSetState,
} from '../../store';

import './style.pcss';

export default React.memo(() => {
  const [security, setSecurity] = useRecoilState(securityState);
  const [, setRefresh] = useRecoilState(refreshToggleState);
  const [mode, setMode] = useRecoilState(modeState);
  const [loading] = useRecoilState(loadingState);
  const [{ mute }] = useRecoilState(colorSetState);
  const handleSecurityClick: TFuncVoid = (): void => {
    setSecurity((s) => !s);
  };

  const handleRefreshClick: TFuncVoid = (): void => {
    setRefresh((s) => !s);
  };

  const handleModeClick: TFuncVoid = (): void => {
    if (mode === 'xml') {
      setMode('json');
      return;
    }
    setMode('xml');
  };

  const animationClass = useMemo(() => (mute ? 'animation' : ''), [mute]);

  return (
    <section className="bk-setting">
      <article
        className={`bk-setting__security ${
          security ? 'active' : ''
        } ${animationClass}`}
      >
        <label className="bk-setting__toggle" onClick={handleSecurityClick}>
          <span className={`bk-setting__fake ${animationClass}`} />
        </label>
      </article>
      <article
        className={`bk-mode ${mode === 'json' ? 'json' : ''}`}
        onClick={handleModeClick}
      >
        <span className={`bk-mode__toggle ${animationClass}`} />
      </article>
      <article
        className={`bk-setting__refresh ${
          loading ? 'active' : ''
        } ${animationClass}`}
        onClick={handleRefreshClick}
      >
        <div />
        <div />
        <div />
        <div />
        <div />
      </article>
    </section>
  );
});
