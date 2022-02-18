import 'jest';
import { render } from '@testing-library/react';
import Background from './index';

describe.only('<Background />', () => {
  it('render correctly', () => {
    const { container } = render(<Background />);
    expect(container).toBeTruthy();
    expect(container.querySelector('img')).toBeTruthy();
  });

  it('render image', () => {
    const { container } = render(<Background />);
    expect(container).toBeTruthy();
  });
});
