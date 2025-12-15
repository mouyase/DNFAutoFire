//! 核心能力 Trait 定义
//!
//! 定义平台无关的抽象接口，使核心逻辑可以在任何平台上测试

#![allow(dead_code)]

use std::fmt::Debug;

/// 按键事件类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyEventType {
    /// 按键按下
    KeyDown,
    /// 按键释放
    KeyUp,
}

/// 按键事件
#[derive(Debug, Clone)]
pub struct KeyEvent {
    /// 虚拟键码
    pub vk: u16,
    /// 扫描码
    pub scan_code: u16,
    /// 事件类型
    pub event_type: KeyEventType,
    /// 是否是注入的按键（非物理按键）
    pub is_injected: bool,
}

/// 键盘驱动能力
///
/// 提供发送键盘输入的能力
pub trait KeyboardDriver: Send + Sync + Debug {
    /// 虚拟键码转扫描码
    fn vk_to_scan_code(&self, vk: u16) -> u16;

    /// 发送按键按下（普通模式，聊天框可识别）
    fn send_key_down(&self, vk: u16, scan_code: u16);

    /// 发送按键释放（普通模式）
    fn send_key_up(&self, vk: u16, scan_code: u16);

    /// 发送完整按键（按下+释放）
    fn send_key_press(&self, vk: u16) {
        let sc = self.vk_to_scan_code(vk);
        self.send_key_down(vk, sc);
        self.send_key_up(vk, sc);
    }

    /// 发送游戏专用按键按下（仅游戏识别，聊天框不识别）
    fn send_game_key_down(&self, scan_code: u16);

    /// 发送游戏专用按键释放
    fn send_game_key_up(&self, scan_code: u16);

    /// 检测按键是否被物理按下
    fn is_key_pressed(&self, vk: u16) -> bool;
}

/// 窗口检测能力
///
/// 提供检测目标窗口是否活动的能力
pub trait WindowDetector: Send + Sync + Debug {
    /// 检测目标窗口是否处于活动状态
    fn is_target_active(&self) -> bool;

    /// 获取当前前台窗口的类名（调试用）
    fn get_foreground_class_name(&self) -> String;
}

/// 钩子事件处理结果
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HookResult {
    /// 继续传递事件
    PassThrough,
    /// 阻止事件传递
    Block,
}

/// 按键钩子回调类型
pub type KeyHookCallback = Box<dyn Fn(KeyEvent) -> HookResult + Send + Sync>;

/// 键盘钩子能力
///
/// 提供监听和拦截键盘输入的能力
pub trait KeyboardHook: Send + Sync + Debug {
    /// 安装钩子并开始监听
    ///
    /// # Arguments
    /// * `callback` - 按键事件回调函数
    ///
    /// # Returns
    /// * `Ok(())` - 安装成功
    /// * `Err(String)` - 安装失败的错误信息
    fn install(&mut self, callback: KeyHookCallback) -> Result<(), String>;

    /// 卸载钩子
    fn uninstall(&mut self) -> Result<(), String>;

    /// 检查钩子是否已安装
    fn is_installed(&self) -> bool;
}

/// 平台能力集合
///
/// 将所有平台相关能力打包在一起，方便依赖注入
pub trait PlatformCapabilities: Send + Sync + Debug {
    type Keyboard: KeyboardDriver;
    type Window: WindowDetector;
    type Hook: KeyboardHook;

    fn keyboard(&self) -> &Self::Keyboard;
    fn window(&self) -> &Self::Window;
    fn hook(&mut self) -> &mut Self::Hook;
}
