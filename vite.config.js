import { join } from 'path'
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueJsx from '@vitejs/plugin-vue-jsx'
import AutoImport from 'unplugin-auto-import/vite'
import Components from 'unplugin-vue-components/vite'
import { VantResolver } from '@vant/auto-import-resolver'

function resolve(dir) {
  return join(__dirname, '.', dir)
}

const host = process.env.TAURI_DEV_HOST
const root = resolve('src')

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  mode: process.env.NODE_ENV,
  resolve: {
    alias: {
      '@src': root,
      '@components': resolve('src/components')
    }
  },
  base: './',
  plugins: [
    vueJsx(),
    vue(),
    AutoImport({
      resolvers: [VantResolver()]
    }),
    Components({
      resolvers: [VantResolver()]
    })
  ],
  build: {
    outDir: resolve('dist'),
    emptyOutDir: true,
    target: 'esnext',
    minify: 'terser',
    terserOptions: {
      compress: {
        drop_console: true,
        drop_debugger: true
      }
    },
    cssCodeSplit: false,
    rollupOptions: {
      input: {
        index: resolve('index.html')
      }
    }
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: 'ws',
          host,
          port: 1421
        }
      : undefined,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ['**/src-tauri/**']
    }
  },
  // CSS 相关选项
  css: {
    preprocessorOptions: {
      scss: {
        additionalData: `@import "@src/styles/common.scss";` //引入全局变量
      }
    }
  },
  optimizeDeps: {}
}))
