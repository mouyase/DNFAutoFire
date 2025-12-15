//! 动作执行器模块
//!
//! 负责解析和执行动作序列

#![allow(dead_code)]

use super::action::{KeyAction, TriggerRule};
use crate::core::traits::{KeyboardDriver, WindowDetector};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// 执行结果
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionResult {
    /// 执行成功
    Success,
    /// 被中断（如按键释放）
    Interrupted,
    /// 窗口不活动
    WindowInactive,
}

/// 动作执行器
///
/// 使用泛型实现依赖注入，可以在测试中使用 Mock 实现
#[derive(Debug)]
pub struct ActionExecutor<K: KeyboardDriver, W: WindowDetector> {
    keyboard: Arc<K>,
    window: Arc<W>,
}

impl<K: KeyboardDriver, W: WindowDetector> ActionExecutor<K, W> {
    /// 创建执行器
    pub fn new(keyboard: Arc<K>, window: Arc<W>) -> Self {
        Self { keyboard, window }
    }

    /// 执行单个动作
    pub fn execute_action(&self, action: &KeyAction) {
        match action {
            KeyAction::Press { vk } => {
                let sc = self.keyboard.vk_to_scan_code(*vk);
                self.keyboard.send_key_down(*vk, sc);
                self.keyboard.send_key_up(*vk, sc);
            }
            KeyAction::GamePress { vk } => {
                let sc = self.keyboard.vk_to_scan_code(*vk);
                self.keyboard.send_game_key_down(sc);
                self.keyboard.send_game_key_up(sc);
            }
            KeyAction::Hold { vk } => {
                let sc = self.keyboard.vk_to_scan_code(*vk);
                self.keyboard.send_key_down(*vk, sc);
            }
            KeyAction::Release { vk } => {
                let sc = self.keyboard.vk_to_scan_code(*vk);
                self.keyboard.send_key_up(*vk, sc);
            }
            KeyAction::Delay { ms } => {
                thread::sleep(Duration::from_millis(*ms));
            }
        }
    }

    /// 执行动作序列
    ///
    /// # Arguments
    /// * `actions` - 要执行的动作序列
    /// * `should_continue` - 检查是否应该继续执行的函数
    ///
    /// # Returns
    /// 执行结果
    pub fn execute_sequence<F>(
        &self,
        actions: &[KeyAction],
        mut should_continue: F,
    ) -> ExecutionResult
    where
        F: FnMut() -> bool,
    {
        for action in actions {
            // 检查是否应该继续
            if !should_continue() {
                return ExecutionResult::Interrupted;
            }

            // 检查窗口是否活动
            if !self.window.is_target_active() {
                return ExecutionResult::WindowInactive;
            }

            self.execute_action(action);
        }

        ExecutionResult::Success
    }

    /// 执行触发规则（单次）
    pub fn execute_rule_once<F>(&self, rule: &TriggerRule, should_continue: F) -> ExecutionResult
    where
        F: FnMut() -> bool,
    {
        self.execute_sequence(&rule.actions, should_continue)
    }

    /// 执行触发规则（循环）
    ///
    /// 当 rule.repeat = true 时，会持续循环执行直到 should_continue 返回 false
    pub fn execute_rule_loop<F>(
        &self,
        rule: &TriggerRule,
        mut should_continue: F,
    ) -> ExecutionResult
    where
        F: FnMut() -> bool,
    {
        if !rule.repeat {
            return self.execute_rule_once(rule, should_continue);
        }

        loop {
            if !should_continue() {
                return ExecutionResult::Interrupted;
            }

            let result = self.execute_sequence(&rule.actions, &mut should_continue);

            match result {
                ExecutionResult::Success => {
                    // 如果有循环间隔，等待
                    if let Some(interval) = rule.interval_ms {
                        thread::sleep(Duration::from_millis(interval));
                    }
                }
                _ => return result,
            }
        }
    }

    /// 获取键盘驱动引用
    pub fn keyboard(&self) -> &K {
        &self.keyboard
    }

    /// 获取窗口检测器引用
    pub fn window(&self) -> &W {
        &self.window
    }
}

/// 执行上下文
///
/// 用于在执行过程中传递状态
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    /// 当前是否应该继续执行
    pub should_run: bool,
    /// 当前按下的触发键
    pub active_keys: Vec<u16>,
}

impl Default for ExecutionContext {
    fn default() -> Self {
        Self::new()
    }
}

impl ExecutionContext {
    pub fn new() -> Self {
        Self {
            should_run: true,
            active_keys: Vec::new(),
        }
    }

    /// 检查指定按键是否处于按下状态
    pub fn is_key_active(&self, vk: u16) -> bool {
        self.active_keys.contains(&vk)
    }

    /// 添加按下的按键
    pub fn key_down(&mut self, vk: u16) {
        if !self.active_keys.contains(&vk) {
            self.active_keys.push(vk);
        }
    }

    /// 移除按下的按键
    pub fn key_up(&mut self, vk: u16) {
        self.active_keys.retain(|&k| k != vk);
    }
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::keyboard::mock::{MockKeyboardDriver, RecordedKeyEvent};
    use crate::core::keyboard::vk;
    use crate::core::window::mock::{AlwaysActiveDetector, NeverActiveDetector};

    fn create_test_executor() -> ActionExecutor<MockKeyboardDriver, AlwaysActiveDetector> {
        ActionExecutor::new(
            Arc::new(MockKeyboardDriver::new()),
            Arc::new(AlwaysActiveDetector),
        )
    }

    #[test]
    fn test_execute_press_action() {
        let keyboard = Arc::new(MockKeyboardDriver::new());
        let executor = ActionExecutor::new(keyboard.clone(), Arc::new(AlwaysActiveDetector));

        executor.execute_action(&KeyAction::Press { vk: vk::VK_J });

        let events = keyboard.get_events();
        assert_eq!(events.len(), 2);
        assert!(matches!(
            events[0],
            RecordedKeyEvent::KeyDown {
                vk: vk::VK_J,
                scan_code: _
            }
        ));
        assert!(matches!(
            events[1],
            RecordedKeyEvent::KeyUp {
                vk: vk::VK_J,
                scan_code: _
            }
        ));
    }

    #[test]
    fn test_execute_game_press_action() {
        let keyboard = Arc::new(MockKeyboardDriver::new());
        let executor = ActionExecutor::new(keyboard.clone(), Arc::new(AlwaysActiveDetector));

        executor.execute_action(&KeyAction::GamePress { vk: vk::VK_J });

        let events = keyboard.get_events();
        assert_eq!(events.len(), 2);
        assert!(matches!(
            events[0],
            RecordedKeyEvent::GameKeyDown { scan_code: _ }
        ));
        assert!(matches!(
            events[1],
            RecordedKeyEvent::GameKeyUp { scan_code: _ }
        ));
    }

    #[test]
    fn test_execute_sequence_success() {
        let keyboard = Arc::new(MockKeyboardDriver::new());
        let executor = ActionExecutor::new(keyboard.clone(), Arc::new(AlwaysActiveDetector));

        let actions = vec![
            KeyAction::Press { vk: vk::VK_J },
            KeyAction::Press { vk: vk::VK_K },
        ];

        let result = executor.execute_sequence(&actions, || true);

        assert_eq!(result, ExecutionResult::Success);
        assert_eq!(keyboard.get_events().len(), 4); // 2 个按键，每个 2 个事件
    }

    #[test]
    fn test_execute_sequence_interrupted() {
        let keyboard = Arc::new(MockKeyboardDriver::new());
        let executor = ActionExecutor::new(keyboard.clone(), Arc::new(AlwaysActiveDetector));

        let actions = vec![
            KeyAction::Press { vk: vk::VK_J },
            KeyAction::Press { vk: vk::VK_K },
        ];

        // 第一次返回 true，第二次返回 false
        let mut count = 0;
        let result = executor.execute_sequence(&actions, || {
            count += 1;
            count <= 1
        });

        assert_eq!(result, ExecutionResult::Interrupted);
        assert_eq!(keyboard.get_events().len(), 2); // 只执行了第一个按键
    }

    #[test]
    fn test_execute_sequence_window_inactive() {
        let keyboard = Arc::new(MockKeyboardDriver::new());
        let executor = ActionExecutor::new(keyboard.clone(), Arc::new(NeverActiveDetector));

        let actions = vec![KeyAction::Press { vk: vk::VK_J }];

        let result = executor.execute_sequence(&actions, || true);

        assert_eq!(result, ExecutionResult::WindowInactive);
        assert_eq!(keyboard.get_events().len(), 0); // 没有执行任何按键
    }

    #[test]
    fn test_execute_simple_rule() {
        let keyboard = Arc::new(MockKeyboardDriver::new());
        let executor = ActionExecutor::new(keyboard.clone(), Arc::new(AlwaysActiveDetector));

        let rule = TriggerRule::simple_repeat(vk::VK_J, 33);

        let result = executor.execute_rule_once(&rule, || true);

        assert_eq!(result, ExecutionResult::Success);
        assert_eq!(keyboard.get_events().len(), 2);
    }

    #[test]
    fn test_execution_context() {
        let mut ctx = ExecutionContext::new();

        assert!(!ctx.is_key_active(vk::VK_J));

        ctx.key_down(vk::VK_J);
        assert!(ctx.is_key_active(vk::VK_J));

        ctx.key_up(vk::VK_J);
        assert!(!ctx.is_key_active(vk::VK_J));
    }
}
