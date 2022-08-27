import ReactDOM from 'react-dom/client';
import { ToastContainer, Slide } from 'react-toastify';
import { RecoilRoot } from 'recoil';

import App from './App';

import 'react-toastify/dist/ReactToastify.css';

const container = document.getElementById('root')!;
const root = ReactDOM.createRoot(container);

root.render(
  <RecoilRoot>
    <App />
    <ToastContainer transition={Slide}/>
  </RecoilRoot>
);
