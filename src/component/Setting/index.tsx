import React from 'react';
import { useRecoilState } from 'recoil';

import { TFuncVoid } from '../../utils/type';
import { securityState, refreshToggleState, loadingState, modeState } from '../../store';

import './style.pcss';

export default React.memo(() => {
  const [security, setSecurity] = useRecoilState(securityState);
  const [, setRefresh] = useRecoilState(refreshToggleState);
  const [mode, setMode] = useRecoilState(modeState);
  const [loading] = useRecoilState(loadingState);
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

  return (
    <section className="bk-setting">
      <article className={`bk-setting__security ${security ? 'active' : ''}`}>
        <label className="bk-setting__toggle" onClick={handleSecurityClick}>
          <span className="bk-setting__fake" />
        </label>
      </article>
      <article className={`bk-mode ${mode === 'json' ? 'json' : ''}`} onClick={handleModeClick}>
        <span className="bk-mode__toggle" />
      </article>
      <article
        className={`bk-setting__refresh ${loading ? 'active' : ''}`}
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
