//! 配置管理模块

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// 应用程序配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// 当前选中的预设名称
    pub current_preset: String,
    /// 所有预设
    pub presets: HashMap<String, Preset>,
    /// 是否自动启动连发
    pub auto_start: bool,
    /// 窗口位置 X
    pub window_x: Option<f32>,
    /// 窗口位置 Y
    pub window_y: Option<f32>,
}

/// 预设配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preset {
    /// 预设名称
    pub name: String,
    /// 启用连发的按键（虚拟键码列表）
    pub keys: Vec<u16>,
}

impl Default for AppConfig {
    fn default() -> Self {
        let mut presets = HashMap::new();
        presets.insert("默认".to_string(), Preset {
            name: "默认".to_string(),
            keys: vec![],
        });

        Self {
            current_preset: "默认".to_string(),
            presets,
            auto_start: false,
            window_x: None,
            window_y: None,
        }
    }
}

impl AppConfig {
    /// 获取配置文件路径
    pub fn config_path() -> PathBuf {
        let exe_path = std::env::current_exe().unwrap_or_default();
        let exe_dir = exe_path.parent().unwrap_or(std::path::Path::new("."));
        exe_dir.join("config.json")
    }

    /// 从文件加载配置
    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            match fs::read_to_string(&path) {
                Ok(content) => {
                    match serde_json::from_str(&content) {
                        Ok(config) => return config,
                        Err(e) => eprintln!("解析配置文件失败: {}", e),
                    }
                }
                Err(e) => eprintln!("读取配置文件失败: {}", e),
            }
        }
        Self::default()
    }

    /// 保存配置到文件
    pub fn save(&self) -> Result<(), String> {
        let path = Self::config_path();
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| format!("序列化配置失败: {}", e))?;
        fs::write(&path, content)
            .map_err(|e| format!("写入配置文件失败: {}", e))?;
        Ok(())
    }

    /// 获取当前预设
    pub fn current_preset(&self) -> Option<&Preset> {
        self.presets.get(&self.current_preset)
    }

    /// 获取当前预设的按键列表
    pub fn current_keys(&self) -> Vec<u16> {
        self.current_preset()
            .map(|p| p.keys.clone())
            .unwrap_or_default()
    }

    /// 更新当前预设的按键列表
    pub fn update_current_keys(&mut self, keys: Vec<u16>) {
        if let Some(preset) = self.presets.get_mut(&self.current_preset) {
            preset.keys = keys;
        }
    }

    /// 创建新预设
    pub fn create_preset(&mut self, name: String) {
        if !self.presets.contains_key(&name) {
            self.presets.insert(name.clone(), Preset {
                name,
                keys: vec![],
            });
        }
    }

    /// 删除预设
    pub fn delete_preset(&mut self, name: &str) {
        if name != "默认" && self.presets.len() > 1 {
            self.presets.remove(name);
            if self.current_preset == name {
                self.current_preset = self.presets.keys().next().cloned().unwrap_or_default();
            }
        }
    }

    /// 切换到指定预设
    pub fn switch_preset(&mut self, name: &str) {
        if self.presets.contains_key(name) {
            self.current_preset = name.to_string();
        }
    }

    /// 获取所有预设名称
    pub fn preset_names(&self) -> Vec<String> {
        self.presets.keys().cloned().collect()
    }
}
