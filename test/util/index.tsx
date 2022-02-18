import React from 'react';
import { MutableSnapshot, RecoilRoot, RecoilState } from 'recoil';

import Service from '../../src/component/Service';

export const wrapper = (
  component: React.ReactElement,
  states?: Record<string, RecoilState<any>>,
) => {
  return ({ values }: { values?: Record<string, any> }) => { 
    const initializeState = ({ set }: MutableSnapshot) => {
      if (!states || !values) return;
      Object.entries(states).forEach(([key, state]) => {
        set(state, values[key]);
      });
    };
    return (
      <RecoilRoot initializeState={initializeState}>
        <Service />
        {component}
      </RecoilRoot>
    );
   };
};
