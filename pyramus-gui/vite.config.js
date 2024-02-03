import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import alias from '@rollup/plugin-alias'
import { resolve } from 'path'
import eslint from 'vite-plugin-eslint'
import svgLoader from 'vite-svg-loader'

const projectRootDir = resolve(__dirname)

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    alias({
      entries: [
        {
          find: '@',
          replacement: resolve(projectRootDir, 'src'),
        },
      ],
    }),
    svgLoader({
      svgoConfig: {
        plugins: [
          {
            name: 'preset-default',
            params: {
              overrides: {
                removeViewBox: false,
              },
            },
          },
        ],
      },
    }),
    eslint(),
  ],

  server: {
    port: 8080,
    strictPort: true,
  },
  envPrefix: ['VITE_', 'TAURI_'],
  build: {
    commonjsOptions: {
      esmExternals: true,
    },
  },
})
