import { WebviewWindow } from '@tauri-apps/api/window'

let recDot: WebviewWindow | undefined;

export default {
  /* 创建右下角记录/播放 图标，默认隐藏*/
  create: function (url = "src/views/rec-dot/rec-dot.html") {
    recDot = new WebviewWindow('recDot', {
      url,
      decorations: false,
      transparent: true,
      fileDropEnabled: true,
      skipTaskbar: false,
      visible: false,
      alwaysOnTop: true,
      x: screen.width - 200,
      y: screen.height - 200
    })
  },

  /* 手动显示/隐藏  */
  display: function (i: number) {
    if (i < 0) recDot?.hide() 
    else recDot?.show()
    recDot?.emit("change", {i})
  }
}





