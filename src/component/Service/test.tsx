import 'jest';
import { render } from '@testing-library/react';
import Service from './index';
import { wrapper } from '../../../test/util';

const ServiceTest = wrapper(<Service />);

describe('<Serivce />', () => {
  it('render correctly', () => {
    const { container } = render(<ServiceTest />);
    expect(container).toBeTruthy();
  });
});
