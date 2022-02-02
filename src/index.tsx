import ReactDOM from 'react-dom';
import { RecoilRoot } from 'recoil';
import App from './App';

const container = document.getElementById('root')!;
const root = ReactDOM.createRoot(container);

root.render(
  <RecoilRoot>
    <App />
  </RecoilRoot>
);
