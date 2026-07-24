import { defineConfig } from 'vite';

// 双入口：主窗口 + 托盘弹出窗
export default defineConfig({
  build: {
    rollupOptions: {
      input: {
        main: 'index.html',
        popover: 'popover.html',
      },
    },
  },
});
