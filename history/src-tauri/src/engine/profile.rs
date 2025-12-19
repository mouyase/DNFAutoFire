//! 配置档案模块
//!
//! 管理用户的配置档案，支持保存、加载、切换

#![allow(dead_code)]

use super::action::TriggerRule;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// 用户配置档案
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    /// 唯一 ID
    pub id: String,

    /// 显示名称
    pub name: String,

    /// 触发规则列表
    pub rules: Vec<TriggerRule>,

    /// 创建时间（Unix 时间戳）
    pub created_at: u64,

    /// 最后修改时间（Unix 时间戳）
    pub updated_at: u64,
}

impl Profile {
    /// 创建新的空配置
    pub fn new(name: &str) -> Self {
        let now = current_timestamp();
        Self {
            id: generate_id(),
            name: name.to_string(),
            rules: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// 创建带规则的配置
    pub fn with_rules(name: &str, rules: Vec<TriggerRule>) -> Self {
        let now = current_timestamp();
        Self {
            id: generate_id(),
            name: name.to_string(),
            rules,
            created_at: now,
            updated_at: now,
        }
    }

    /// 创建默认配置（简单连发模式）
    pub fn default_profile() -> Self {
        Self::new("默认配置")
    }

    /// 添加规则
    pub fn add_rule(&mut self, rule: TriggerRule) {
        self.rules.push(rule);
        self.updated_at = current_timestamp();
    }

    /// 移除规则
    pub fn remove_rule(&mut self, rule_id: &str) -> bool {
        let original_len = self.rules.len();
        self.rules.retain(|r| r.id != rule_id);
        if self.rules.len() != original_len {
            self.updated_at = current_timestamp();
            true
        } else {
            false
        }
    }

    /// 启用/禁用规则
    pub fn set_rule_enabled(&mut self, rule_id: &str, enabled: bool) -> bool {
        if let Some(rule) = self.rules.iter_mut().find(|r| r.id == rule_id) {
            rule.enabled = enabled;
            self.updated_at = current_timestamp();
            true
        } else {
            false
        }
    }

    /// 获取所有已启用的规则
    pub fn enabled_rules(&self) -> Vec<&TriggerRule> {
        self.rules.iter().filter(|r| r.enabled).collect()
    }

    /// 获取所有触发按键（去重）
    pub fn all_trigger_keys(&self) -> Vec<u16> {
        let mut keys: Vec<u16> = self
            .rules
            .iter()
            .filter(|r| r.enabled)
            .flat_map(|r| r.trigger_keys.clone())
            .collect();
        keys.sort();
        keys.dedup();
        keys
    }

    /// 根据触发按键查找匹配的规则
    pub fn find_rules_for_key(&self, vk: u16) -> Vec<&TriggerRule> {
        self.rules
            .iter()
            .filter(|r| r.enabled && r.trigger_keys.contains(&vk))
            .collect()
    }

    /// 重命名配置
    pub fn rename(&mut self, new_name: &str) {
        self.name = new_name.to_string();
        self.updated_at = current_timestamp();
    }

    /// 克隆配置（生成新 ID）
    pub fn duplicate(&self, new_name: &str) -> Self {
        let now = current_timestamp();
        Self {
            id: generate_id(),
            name: new_name.to_string(),
            rules: self.rules.clone(),
            created_at: now,
            updated_at: now,
        }
    }
}

/// 配置管理器
///
/// 管理多个配置档案，支持切换当前活动配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileManager {
    /// 所有配置
    profiles: Vec<Profile>,

    /// 当前活动配置的 ID
    active_profile_id: Option<String>,
}

impl Default for ProfileManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ProfileManager {
    /// 创建新的配置管理器
    pub fn new() -> Self {
        let default_profile = Profile::default_profile();
        let active_id = default_profile.id.clone();
        Self {
            profiles: vec![default_profile],
            active_profile_id: Some(active_id),
        }
    }

    /// 获取所有配置
    pub fn profiles(&self) -> &[Profile] {
        &self.profiles
    }

    /// 获取当前活动配置
    pub fn active_profile(&self) -> Option<&Profile> {
        self.active_profile_id
            .as_ref()
            .and_then(|id| self.profiles.iter().find(|p| &p.id == id))
    }

    /// 获取当前活动配置（可变）
    pub fn active_profile_mut(&mut self) -> Option<&mut Profile> {
        let id = self.active_profile_id.clone();
        id.and_then(move |id| self.profiles.iter_mut().find(|p| p.id == id))
    }

    /// 切换活动配置
    pub fn set_active(&mut self, profile_id: &str) -> bool {
        if self.profiles.iter().any(|p| p.id == profile_id) {
            self.active_profile_id = Some(profile_id.to_string());
            true
        } else {
            false
        }
    }

    /// 添加新配置
    pub fn add_profile(&mut self, profile: Profile) -> String {
        let id = profile.id.clone();
        self.profiles.push(profile);
        id
    }

    /// 创建新配置并添加
    pub fn create_profile(&mut self, name: &str) -> String {
        let profile = Profile::new(name);
        self.add_profile(profile)
    }

    /// 删除配置
    pub fn remove_profile(&mut self, profile_id: &str) -> bool {
        // 不能删除最后一个配置
        if self.profiles.len() <= 1 {
            return false;
        }

        let original_len = self.profiles.len();
        self.profiles.retain(|p| p.id != profile_id);

        if self.profiles.len() != original_len {
            // 如果删除的是当前活动配置，切换到第一个
            if self.active_profile_id.as_deref() == Some(profile_id) {
                self.active_profile_id = self.profiles.first().map(|p| p.id.clone());
            }
            true
        } else {
            false
        }
    }

    /// 获取指定配置
    pub fn get_profile(&self, profile_id: &str) -> Option<&Profile> {
        self.profiles.iter().find(|p| p.id == profile_id)
    }

    /// 获取指定配置（可变）
    pub fn get_profile_mut(&mut self, profile_id: &str) -> Option<&mut Profile> {
        self.profiles.iter_mut().find(|p| p.id == profile_id)
    }

    /// 复制配置
    pub fn duplicate_profile(&mut self, profile_id: &str, new_name: &str) -> Option<String> {
        let profile = self.get_profile(profile_id)?.duplicate(new_name);
        Some(self.add_profile(profile))
    }
}

// ============================================================================
// 辅助函数
// ============================================================================

/// 生成唯一 ID
fn generate_id() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);

    let timestamp = current_timestamp();
    let counter = COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("profile-{}-{}", timestamp, counter)
}

/// 获取当前时间戳
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::action::TriggerRule;

    #[test]
    fn test_profile_creation() {
        let profile = Profile::new("测试配置");
        assert_eq!(profile.name, "测试配置");
        assert!(profile.rules.is_empty());
        assert!(profile.created_at > 0);
    }

    #[test]
    fn test_profile_add_remove_rule() {
        let mut profile = Profile::new("测试");
        let rule = TriggerRule::simple_repeat(0x4A, 33);
        let rule_id = rule.id.clone();

        profile.add_rule(rule);
        assert_eq!(profile.rules.len(), 1);

        let removed = profile.remove_rule(&rule_id);
        assert!(removed);
        assert!(profile.rules.is_empty());
    }

    #[test]
    fn test_profile_find_rules_for_key() {
        let mut profile = Profile::new("测试");
        profile.add_rule(TriggerRule::simple_repeat(0x4A, 33));
        profile.add_rule(TriggerRule::simple_repeat(0x4B, 33));

        let rules = profile.find_rules_for_key(0x4A);
        assert_eq!(rules.len(), 1);
        assert!(rules[0].trigger_keys.contains(&0x4A));
    }

    #[test]
    fn test_profile_manager_creation() {
        let manager = ProfileManager::new();
        assert_eq!(manager.profiles().len(), 1);
        assert!(manager.active_profile().is_some());
    }

    #[test]
    fn test_profile_manager_add_remove() {
        let mut manager = ProfileManager::new();

        let id = manager.create_profile("新配置");
        assert_eq!(manager.profiles().len(), 2);

        let removed = manager.remove_profile(&id);
        assert!(removed);
        assert_eq!(manager.profiles().len(), 1);
    }

    #[test]
    fn test_profile_manager_cannot_remove_last() {
        let mut manager = ProfileManager::new();
        let id = manager.active_profile().unwrap().id.clone();

        let removed = manager.remove_profile(&id);
        assert!(!removed);
        assert_eq!(manager.profiles().len(), 1);
    }

    #[test]
    fn test_profile_manager_switch_active() {
        let mut manager = ProfileManager::new();
        let new_id = manager.create_profile("新配置");

        let switched = manager.set_active(&new_id);
        assert!(switched);
        assert_eq!(manager.active_profile().unwrap().name, "新配置");
    }

    #[test]
    fn test_profile_serialization() {
        let mut profile = Profile::new("测试");
        profile.add_rule(TriggerRule::simple_repeat(0x4A, 33));

        let json = serde_json::to_string(&profile).unwrap();
        let parsed: Profile = serde_json::from_str(&json).unwrap();

        assert_eq!(profile.name, parsed.name);
        assert_eq!(profile.rules.len(), parsed.rules.len());
    }
}
