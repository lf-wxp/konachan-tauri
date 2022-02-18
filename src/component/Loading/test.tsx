import 'jest';
import { render } from '@testing-library/react';
import Loading from './index';
import { wrapper } from '../../../test/util';
import { loadingState } from '../../store';

const LoadingTest = wrapper(<Loading />, { loading: loadingState });

describe('<Loading />', () => {
  it('render correctly is loading', () => {
    const { container } = render(<LoadingTest />);
    expect(container).toBeTruthy();
  });

  it(`render correctly isn't loading`, () => {
    const { container } = render(<LoadingTest values={{ loading: false }} />);
    expect(container).toBeTruthy();
  });
});
