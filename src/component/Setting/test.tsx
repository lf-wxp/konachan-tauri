import 'jest';
import { render, act, fireEvent } from '@testing-library/react';
import Setting from './index';
import { wrapper } from '../../../test/util';
import { securityState, pageState, loadingState } from '../../store';

const SettingTest = wrapper(<Setting />, {
  security: securityState,
  loading: loadingState,
  page: pageState,
});

describe('<Setting />', () => {
  it('render correctly', () => {
    const { container } = render(
      <SettingTest values={{ page: 1, security: true, loading: false }} />
    );
    const toggle = container.querySelector('.bk-setting__toggle');
    const refresh = container.querySelector('.bk-setting__refresh');
    act(() => {
      fireEvent.click(toggle!);
      fireEvent.click(refresh!);
    });
    expect(container).toBeTruthy();
    expect(
      container
        .querySelector('.bk-setting__security')
        ?.classList.contains('active')
    ).toBeFalsy();
  });
});
