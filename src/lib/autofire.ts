import { invoke } from "@tauri-apps/api/core";

export class AutoFire {
  // 定义按键码常量
  static readonly KEY_CODES = {
    J: 74,
    K: 75,
    L: 76,
    H: 72,
  };

  async start(keys: number[] = []) {
    try {
      await invoke('start_auto_fire', { 
        keyCodes: keys 
      });
      console.log('自动连发已启动');
    } catch (error) {
      console.error('启动自动连发失败:', error);
      throw error;
    }
  }

  async stop() {
    try {
      await invoke('stop_auto_fire');
      console.log('自动连发已停止');
    } catch (error) {
      console.error('停止自动连发失败:', error);
      throw error;
    }
  }
}
