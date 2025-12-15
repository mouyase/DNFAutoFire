//! 动作定义模块
//!
//! 定义按键动作和触发规则

#![allow(dead_code)]

use serde::{Deserialize, Serialize};

/// 单个按键动作
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum KeyAction {
    /// 按下按键（普通模式，聊天框可识别）
    Press { vk: u16 },

    /// 按下按键（游戏专用模式，聊天框不识别）
    GamePress { vk: u16 },

    /// 按住按键（按下不释放）
    Hold { vk: u16 },

    /// 释放按键
    Release { vk: u16 },

    /// 等待指定时间（毫秒）
    Delay { ms: u64 },
}

impl KeyAction {
    /// 创建普通按键按下动作
    pub fn press(vk: u16) -> Self {
        Self::Press { vk }
    }

    /// 创建游戏专用按键动作
    pub fn game_press(vk: u16) -> Self {
        Self::GamePress { vk }
    }

    /// 创建按住动作
    pub fn hold(vk: u16) -> Self {
        Self::Hold { vk }
    }

    /// 创建释放动作
    pub fn release(vk: u16) -> Self {
        Self::Release { vk }
    }

    /// 创建延迟动作
    pub fn delay(ms: u64) -> Self {
        Self::Delay { ms }
    }
}

/// 触发规则
///
/// 定义当特定按键被按下时，应该执行什么动作序列
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriggerRule {
    /// 规则 ID（用于标识和更新）
    pub id: String,

    /// 触发条件：当这些按键被物理按下时触发
    pub trigger_keys: Vec<u16>,

    /// 执行的动作序列
    pub actions: Vec<KeyAction>,

    /// 是否循环执行动作序列
    pub repeat: bool,

    /// 循环间隔（毫秒），仅当 repeat = true 时有效
    /// 如果为 None，则动作序列执行完毕后立即开始下一轮
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_ms: Option<u64>,

    /// 是否启用此规则
    #[serde(default = "default_true")]
    pub enabled: bool,
}

fn default_true() -> bool {
    true
}

impl TriggerRule {
    /// 创建简单的连发规则
    ///
    /// 按住按键时，以指定间隔重复按下该按键
    pub fn simple_repeat(vk: u16, interval_ms: u64) -> Self {
        Self {
            id: format!("simple-{:04X}", vk),
            trigger_keys: vec![vk],
            actions: vec![KeyAction::game_press(vk)],
            repeat: true,
            interval_ms: Some(interval_ms),
            enabled: true,
        }
    }

    /// 创建延迟连发规则
    ///
    /// 按住按键时，先按一次，等待指定时间后再开始连发
    pub fn delayed_repeat(vk: u16, initial_delay_ms: u64, interval_ms: u64) -> Self {
        Self {
            id: format!("delayed-{:04X}", vk),
            trigger_keys: vec![vk],
            actions: vec![
                KeyAction::game_press(vk),
                KeyAction::delay(initial_delay_ms),
            ],
            repeat: true,
            interval_ms: Some(interval_ms),
            enabled: true,
        }
    }

    /// 创建组合连发规则
    ///
    /// 按住一个键时，交替按下多个键
    pub fn combo_repeat(trigger_vk: u16, combo_vks: Vec<u16>, interval_ms: u64) -> Self {
        let actions: Vec<KeyAction> = combo_vks
            .iter()
            .flat_map(|&vk| vec![KeyAction::game_press(vk), KeyAction::delay(interval_ms)])
            .collect();

        Self {
            id: format!("combo-{:04X}", trigger_vk),
            trigger_keys: vec![trigger_vk],
            actions,
            repeat: true,
            interval_ms: None, // 动作序列中已包含延迟
            enabled: true,
        }
    }

    /// 创建自定义规则
    pub fn custom(id: &str, trigger_keys: Vec<u16>, actions: Vec<KeyAction>, repeat: bool) -> Self {
        Self {
            id: id.to_string(),
            trigger_keys,
            actions,
            repeat,
            interval_ms: None,
            enabled: true,
        }
    }
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_repeat_rule() {
        let rule = TriggerRule::simple_repeat(0x4A, 33); // J 键，33ms 间隔

        assert_eq!(rule.trigger_keys, vec![0x4A]);
        assert_eq!(rule.actions.len(), 1);
        assert!(matches!(rule.actions[0], KeyAction::GamePress { vk: 0x4A }));
        assert!(rule.repeat);
        assert_eq!(rule.interval_ms, Some(33));
    }

    #[test]
    fn test_delayed_repeat_rule() {
        let rule = TriggerRule::delayed_repeat(0x4A, 150, 33);

        assert_eq!(rule.actions.len(), 2);
        assert!(matches!(rule.actions[0], KeyAction::GamePress { vk: 0x4A }));
        assert!(matches!(rule.actions[1], KeyAction::Delay { ms: 150 }));
    }

    #[test]
    fn test_combo_repeat_rule() {
        let rule = TriggerRule::combo_repeat(0x4A, vec![0x4A, 0x4B], 50); // J 触发，J K 交替

        assert_eq!(rule.trigger_keys, vec![0x4A]);
        assert_eq!(rule.actions.len(), 4); // J, delay, K, delay
        assert!(matches!(rule.actions[0], KeyAction::GamePress { vk: 0x4A }));
        assert!(matches!(rule.actions[1], KeyAction::Delay { ms: 50 }));
        assert!(matches!(rule.actions[2], KeyAction::GamePress { vk: 0x4B }));
        assert!(matches!(rule.actions[3], KeyAction::Delay { ms: 50 }));
    }

    #[test]
    fn test_rule_serialization() {
        let rule = TriggerRule::simple_repeat(0x4A, 33);
        let json = serde_json::to_string(&rule).unwrap();
        let parsed: TriggerRule = serde_json::from_str(&json).unwrap();

        assert_eq!(rule, parsed);
    }

    #[test]
    fn test_action_serialization() {
        let action = KeyAction::Press { vk: 0x4A };
        let json = serde_json::to_string(&action).unwrap();
        let parsed: KeyAction = serde_json::from_str(&json).unwrap();

        assert_eq!(action, parsed);
    }
}
