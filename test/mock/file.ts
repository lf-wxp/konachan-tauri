import fs from 'fs';
import path from 'path';

const imgBase64 = `data:image/png;base64,${fs
  .readFileSync(path.resolve(__dirname, '../../src/image/loaderror.png'))
  .toString('base64')}`;

export default imgBase64;
