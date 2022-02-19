import 'jest';
import { render } from '@testing-library/react';
import App from './App';
import { wrapper } from '../test/util';

describe.only('<App />', () => {
  it('render correctly', () => {
    const AppTest = wrapper(<App />)
    const { container } = render(<AppTest />);
    expect(container).toBeTruthy();
    expect(container.querySelector('.bk-aside')).toBeTruthy();
    expect(container.querySelector('.bk-section')).toBeTruthy();
  });
});
