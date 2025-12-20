// 配置管理模块 - 支持多配置的JSON持久化

#[allow(dead_code)]
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

/// 单个配置文件
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    #[serde(rename = "enabledKeys")]
    pub enabled_keys: Vec<u16>,
}

#[allow(dead_code)]
impl Profile {
    /// 创建新配置
    pub fn new(name: String) -> Self {
        Self {
            name,
            enabled_keys: Vec::new(),
        }
    }

    /// 从启用的按键集合创建配置
    pub fn from_keys(name: String, keys: &HashSet<u16>) -> Self {
        let mut enabled_keys: Vec<u16> = keys.iter().copied().collect();
        enabled_keys.sort_unstable(); // 保证顺序一致性
        Self { name, enabled_keys }
    }

    /// 获取启用的按键集合
    pub fn get_enabled_keys(&self) -> HashSet<u16> {
        self.enabled_keys.iter().copied().collect()
    }
}

/// 配置管理器
#[allow(dead_code)]
pub struct ConfigManager {
    config_dir: PathBuf,
    current_profile: Option<String>,
}

#[allow(dead_code)]
impl ConfigManager {
    /// 创建配置管理器
    pub fn new() -> Self {
        let config_dir = Self::get_config_dir();

        // 确保配置目录存在
        if let Err(e) = fs::create_dir_all(&config_dir) {
            eprintln!("⚠️  创建配置目录失败: {}", e);
        }

        Self {
            config_dir,
            current_profile: None,
        }
    }

    /// 获取配置目录路径
    fn get_config_dir() -> PathBuf {
        // 使用当前目录下的 configs/ 文件夹
        let mut path = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .unwrap_or_else(|| PathBuf::from("."));

        path.push("configs");
        path
    }

    /// 获取配置文件路径
    fn get_profile_path(&self, name: &str) -> PathBuf {
        let filename = format!("{}.json", name);
        self.config_dir.join(filename)
    }

    /// 列出所有配置
    pub fn list_profiles(&self) -> Vec<String> {
        let mut profiles = Vec::new();

        if let Ok(entries) = fs::read_dir(&self.config_dir) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".json") {
                        let profile_name = name.trim_end_matches(".json").to_string();
                        profiles.push(profile_name);
                    }
                }
            }
        }

        profiles.sort();

        // 如果没有配置，创建默认配置
        if profiles.is_empty() {
            if let Err(e) = self.save_profile(&Profile::new("默认配置".to_string())) {
                eprintln!("⚠️  创建默认配置失败: {}", e);
            } else {
                profiles.push("默认配置".to_string());
            }
        }

        profiles
    }

    /// 加载配置
    pub fn load_profile(&self, name: &str) -> Result<Profile, String> {
        let path = self.get_profile_path(name);

        let content = fs::read_to_string(&path).map_err(|e| format!("读取配置文件失败: {}", e))?;

        let profile: Profile =
            serde_json::from_str(&content).map_err(|e| format!("解析配置文件失败: {}", e))?;

        Ok(profile)
    }

    /// 保存配置
    pub fn save_profile(&self, profile: &Profile) -> Result<(), String> {
        let path = self.get_profile_path(&profile.name);

        let content =
            serde_json::to_string_pretty(profile).map_err(|e| format!("序列化配置失败: {}", e))?;

        fs::write(&path, content).map_err(|e| format!("写入配置文件失败: {}", e))?;

        Ok(())
    }

    /// 删除配置
    pub fn delete_profile(&self, name: &str) -> Result<(), String> {
        let path = self.get_profile_path(name);

        if !path.exists() {
            return Err(format!("配置文件不存在: {}", name));
        }

        fs::remove_file(&path).map_err(|e| format!("删除配置文件失败: {}", e))?;

        Ok(())
    }

    /// 重命名配置
    pub fn rename_profile(&self, old_name: &str, new_name: &str) -> Result<(), String> {
        // 加载旧配置
        let mut profile = self.load_profile(old_name)?;

        // 更新名称
        profile.name = new_name.to_string();

        // 保存新配置
        self.save_profile(&profile)?;

        // 删除旧配置
        self.delete_profile(old_name)?;

        Ok(())
    }

    /// 克隆配置
    pub fn clone_profile(&self, source_name: &str, new_name: &str) -> Result<(), String> {
        // 加载源配置
        let mut profile = self.load_profile(source_name)?;

        // 更新名称
        profile.name = new_name.to_string();

        // 保存新配置
        self.save_profile(&profile)?;

        Ok(())
    }

    /// 检查配置是否存在
    pub fn profile_exists(&self, name: &str) -> bool {
        self.get_profile_path(name).exists()
    }

    /// 获取当前配置名称
    pub fn current_profile(&self) -> Option<&str> {
        self.current_profile.as_deref()
    }

    /// 设置当前配置
    pub fn set_current_profile(&mut self, name: String) {
        self.current_profile = Some(name);
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_creation() {
        let profile = Profile::new("测试配置".to_string());
        assert_eq!(profile.name, "测试配置");
        assert!(profile.enabled_keys.is_empty());
    }

    #[test]
    fn test_profile_from_keys() {
        let mut keys = HashSet::new();
        keys.insert(0x4A); // J
        keys.insert(0x4C); // L

        let profile = Profile::from_keys("测试".to_string(), &keys);
        assert_eq!(profile.enabled_keys.len(), 2);
        assert!(profile.enabled_keys.contains(&0x4A));
        assert!(profile.enabled_keys.contains(&0x4C));
    }
}
