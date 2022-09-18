import 'jest';
import { render } from '@testing-library/react';
import Background from './index';
import { wrapper } from '../../../test/util';

describe.only('<Background />', () => {
  it('render correctly', () => {
    const BackgroundTest = wrapper(<Background />);
    const { container } = render(<BackgroundTest />);
    expect(container).toBeTruthy();
    expect(container.querySelector('img')).toBeTruthy();
  });
});
