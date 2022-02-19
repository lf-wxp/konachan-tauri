import 'jest';
import { render } from '@testing-library/react';
import DotLine from './index';
import { wrapper } from '../../../test/util';

jest.useFakeTimers();

describe('<DotLine />', () => {
  it('render correctly', () => {
    const DotLineTest = wrapper(<DotLine width={'100px'} height={'100px'} />);
    const { container } = render(<DotLineTest />);
    jest.runOnlyPendingTimers();
    expect(container).toBeTruthy();
  });
});
