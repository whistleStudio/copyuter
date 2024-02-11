import { once } from '@tauri-apps/api/event';
import { WebviewWindow } from '@tauri-apps/api/window'

export default {
    /* 新创建，过时销毁 */
    display: function (url = "src/views/countdown/countdown.html") {
      const countdown = new WebviewWindow('countdown', {
        url,
        decorations: false,
        transparent: true,
        fileDropEnabled: true,
        skipTaskbar: false,
        alwaysOnTop: true,
        center: true,
        width: 200,
        height: 200
      })

      once("countdown_over", () => {
        countdown?.close()
      })
    }
  }