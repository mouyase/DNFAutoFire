//! GUI 模块 - 使用 egui 实现虚拟键盘界面

mod keyboard_layout;

use eframe::egui::{self, FontData, FontDefinitions, FontFamily};
use std::sync::Arc;
use parking_lot::Mutex;

use crate::config::AppConfig;
use crate::core::autofire::AutoFireEngine;
use crate::core::keyboard::vk;

pub use keyboard_layout::KEYBOARD_LAYOUT;

/// 设置中文字体
fn setup_chinese_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    // 尝试加载 Windows 系统中文字体
    let font_paths = [
        "C:\\Windows\\Fonts\\msyh.ttc",      // 微软雅黑
        "C:\\Windows\\Fonts\\simhei.ttf",    // 黑体
        "C:\\Windows\\Fonts\\simsun.ttc",    // 宋体
    ];

    let mut font_loaded = false;
    for path in &font_paths {
        if let Ok(font_data) = std::fs::read(path) {
            fonts.font_data.insert(
                "chinese".to_owned(),
                FontData::from_owned(font_data).into(),
            );

            // 将中文字体设为首选
            fonts.families.entry(FontFamily::Proportional)
                .or_default()
                .insert(0, "chinese".to_owned());
            fonts.families.entry(FontFamily::Monospace)
                .or_default()
                .insert(0, "chinese".to_owned());

            font_loaded = true;
            break;
        }
    }

    if font_loaded {
        ctx.set_fonts(fonts);
    }
}

/// 主应用程序
pub struct App {
    /// 连发引擎
    engine: Arc<Mutex<AutoFireEngine>>,
    /// 配置
    config: AppConfig,
    /// 新预设名称输入
    new_preset_name: String,
    /// 状态消息
    status_message: String,
    /// 当前前台窗口类名（调试用）
    debug_window_class: String,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // 加载中文字体
        setup_chinese_fonts(&cc.egui_ctx);

        let config = AppConfig::load();
        let engine = Arc::new(Mutex::new(AutoFireEngine::new()));

        // 加载保存的按键配置
        {
            let mut eng = engine.lock();
            eng.set_keys(config.current_keys());
        }

        Self {
            engine,
            config,
            new_preset_name: String::new(),
            status_message: "就绪".to_string(),
            debug_window_class: String::new(),
        }
    }

    /// 切换按键状态
    fn toggle_key(&mut self, vk_code: u16) {
        let enabled = {
            let engine = self.engine.lock();
            engine.toggle_key(vk_code)
        };

        // 更新配置
        let enabled_keys = {
            let engine = self.engine.lock();
            engine.get_enabled_keys()
        };
        self.config.update_current_keys(enabled_keys);

        let key_name = vk::to_name(vk_code);
        self.status_message = if enabled {
            format!("已启用 {} 连发", key_name)
        } else {
            format!("已禁用 {} 连发", key_name)
        };
    }

    /// 检查按键是否启用
    fn is_key_enabled(&self, vk_code: u16) -> bool {
        let engine = self.engine.lock();
        engine.is_key_enabled(vk_code)
    }

    /// 启动/停止连发
    fn toggle_autofire(&mut self) -> bool {
        let mut engine = self.engine.lock();
        let running = engine.toggle();
        self.status_message = if running {
            "连发已启动".to_string()
        } else {
            "连发已停止".to_string()
        };
        running
    }

    /// 保存配置
    fn save_config(&mut self) {
        match self.config.save() {
            Ok(_) => self.status_message = "配置已保存".to_string(),
            Err(e) => self.status_message = format!("保存失败: {}", e),
        }
    }

    /// 渲染键盘按键
    fn render_key(&mut self, ui: &mut egui::Ui, key: &keyboard_layout::KeyInfo) {
        let enabled = self.is_key_enabled(key.vk);
        let color = if enabled {
            egui::Color32::from_rgb(220, 60, 60)  // 红色 - 已启用
        } else {
            egui::Color32::from_rgb(60, 60, 80)   // 深灰 - 未启用
        };

        let text_color = egui::Color32::WHITE;
        let size = egui::vec2(key.width, 32.0);

        let button = egui::Button::new(
            egui::RichText::new(&key.label)
                .color(text_color)
                .size(12.0)
        )
        .fill(color)
        .min_size(size);

        if ui.add(button).clicked() {
            self.toggle_key(key.vk);
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 更新调试信息
        self.debug_window_class = crate::core::window::get_foreground_class_name();

        // 请求持续重绘（用于实时状态更新）
        ctx.request_repaint();

        // 顶部面板 - 控制按钮
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let is_running = {
                    let engine = self.engine.lock();
                    engine.is_running()
                };

                let button_text = if is_running { "停止连发" } else { "启动连发" };
                let button_color = if is_running {
                    egui::Color32::from_rgb(220, 60, 60)
                } else {
                    egui::Color32::from_rgb(60, 160, 60)
                };

                if ui.add(
                    egui::Button::new(egui::RichText::new(button_text).size(16.0))
                        .fill(button_color)
                        .min_size(egui::vec2(100.0, 30.0))
                ).clicked() {
                    self.toggle_autofire();
                }

                ui.separator();

                // 预设选择
                ui.label("预设:");
                let current = self.config.current_preset.clone();
                egui::ComboBox::from_id_salt("preset_selector")
                    .selected_text(&current)
                    .show_ui(ui, |ui| {
                        for name in self.config.preset_names() {
                            if ui.selectable_label(name == current, &name).clicked() {
                                self.config.switch_preset(&name);
                                let keys = self.config.current_keys();
                                let engine = self.engine.lock();
                                engine.set_keys(keys);
                                self.status_message = format!("已切换到预设: {}", name);
                            }
                        }
                    });

                ui.separator();

                if ui.button("保存").clicked() {
                    self.save_config();
                }
            });
        });

        // 底部面板 - 状态栏
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(&self.status_message);
                ui.separator();
                ui.label(format!("窗口: {}", &self.debug_window_class));
            });
        });

        // 中央面板 - 虚拟键盘
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("虚拟键盘 - 点击按键切换连发状态");
            ui.separator();

            // 渲染键盘布局
            for row in KEYBOARD_LAYOUT.iter() {
                ui.horizontal(|ui| {
                    for key in row.iter() {
                        self.render_key(ui, key);
                    }
                });
            }

            ui.separator();

            // 预设管理
            ui.collapsing("预设管理", |ui| {
                ui.horizontal(|ui| {
                    ui.label("新预设名称:");
                    ui.text_edit_singleline(&mut self.new_preset_name);
                    if ui.button("创建").clicked() && !self.new_preset_name.is_empty() {
                        self.config.create_preset(self.new_preset_name.clone());
                        self.status_message = format!("已创建预设: {}", self.new_preset_name);
                        self.new_preset_name.clear();
                    }
                });

                ui.horizontal(|ui| {
                    if ui.button("删除当前预设").clicked() {
                        let name = self.config.current_preset.clone();
                        if name != "默认" {
                            self.config.delete_preset(&name);
                            self.status_message = format!("已删除预设: {}", name);
                            let keys = self.config.current_keys();
                            let engine = self.engine.lock();
                            engine.set_keys(keys);
                        } else {
                            self.status_message = "无法删除默认预设".to_string();
                        }
                    }
                });
            });

            // 使用说明
            ui.collapsing("使用说明", |ui| {
                ui.label("1. 点击按键可以切换该按键的连发状态（红色=启用）");
                ui.label("2. 点击「启动连发」开始连发");
                ui.label("3. 连发仅在 DNF 游戏窗口激活时有效");
                ui.label("4. 按住已启用连发的按键，程序会自动高速重复发送该按键");
            });
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // 停止连发引擎
        {
            let mut engine = self.engine.lock();
            engine.stop();
        }
        // 保存配置
        let _ = self.config.save();
    }
}
