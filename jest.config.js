// In the following statement, replace `./tsconfig` with the path to your `tsconfig` file
// which contains the path mapping (ie the `compilerOptions.paths` option):
module.exports = {
  preset: 'ts-jest',
  testEnvironment: 'jsdom',
  setupFiles: ['<rootDir>/test-setup.js', 'jest-canvas-mock'],
  globals: {
    'ts-jest': {
      tsconfig: 'tsconfig.json'
    }
  },
  transform: {
    '\\.[jt]sx?$': 'ts-jest',
  },
  collectCoverage: true,
  coverageDirectory: '<rootDir>/coverage',
  collectCoverageFrom: [
    './src/**/*.{ts,tsx}',
    '!./src/**/*.d.{ts,tsx}',
    '!./src/index.tsx',
    '!**/node_modules/**',
    '!**/test/**'
  ],
  moduleNameMapper: {
    '\\.(jpg|jpeg|png)$': '<rootDir>/test/mock/file.ts',
    '\\.p?css$': 'identity-obj-proxy',
    'node-vibrant': '<rootDir>/test/mock/vibrant.ts',
    mousetrap: '<rootDir>/test/mock/mousetrap.ts',
    '@tauri-apps/api/event': '<rootDir>/test/mock/tauri-event.ts',
    '@tauri-apps/api/tauri': '<rootDir>/test/mock/tauri-api.ts',
  },
  moduleFileExtensions: ['ts', 'tsx', 'js'],
  testMatch: ['<rootDir>/**/*.test.(ts|tsx)', '<rootDir>/**/test.(ts|tsx)'],
  transformIgnorePatterns: [ 'node_modules/(?!(@tauri-apps)/.*)', ],
};
