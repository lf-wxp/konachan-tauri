import 'jest';
import { render } from '@testing-library/react';
import Progress from './index';

describe('<Progress />', () => {
  it('render correctly', () => {
    const { container } = render(<Progress percent={1} />);
    expect(container).toBeTruthy();
  });
});
