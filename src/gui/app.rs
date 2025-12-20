// NWG 主窗口应用程序
// TODO: 完善 NWG 事件处理机制（按钮点击、下拉框选择等）

#[allow(dead_code)]
use native_windows_gui as nwg;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use super::config::{ConfigManager, Profile};
use super::keyboard_layout::{get_keyboard_bounds, get_keyboard_layout};
use crate::core::AutoFireEngine;

/// 应用程序状态
#[allow(dead_code)]
pub struct AppState {
    /// 配置管理器
    config_manager: ConfigManager,

    /// 当前启用的按键集合
    enabled_keys: HashSet<u16>,

    /// 连发引擎
    engine: Option<AutoFireEngine>,

    /// 引擎是否运行中
    is_running: bool,

    /// 是否有未保存的更改
    has_unsaved_changes: bool,
}

#[allow(dead_code)]
impl AppState {
    pub fn new() -> Self {
        Self {
            config_manager: ConfigManager::new(),
            enabled_keys: HashSet::new(),
            engine: None,
            is_running: false,
            has_unsaved_changes: false,
        }
    }

    /// 切换按键状态
    pub fn toggle_key(&mut self, vk: u16) {
        if self.enabled_keys.contains(&vk) {
            self.enabled_keys.remove(&vk);
        } else {
            self.enabled_keys.insert(vk);
        }
        self.has_unsaved_changes = true;
    }

    /// 检查按键是否启用
    pub fn is_key_enabled(&self, vk: u16) -> bool {
        self.enabled_keys.contains(&vk)
    }

    /// 加载配置
    pub fn load_profile(&mut self, name: &str) -> Result<(), String> {
        let profile = self.config_manager.load_profile(name)?;
        self.enabled_keys = profile.get_enabled_keys();
        self.config_manager.set_current_profile(name.to_string());
        self.has_unsaved_changes = false;
        Ok(())
    }

    /// 保存当前配置
    pub fn save_current_profile(&mut self) -> Result<(), String> {
        if let Some(name) = self.config_manager.current_profile() {
            let profile = Profile::from_keys(name.to_string(), &self.enabled_keys);
            self.config_manager.save_profile(&profile)?;
            self.has_unsaved_changes = false;
            Ok(())
        } else {
            Err("未选择配置".to_string())
        }
    }

    /// 启动连发引擎
    pub fn start_engine(&mut self) -> Result<(), String> {
        if self.is_running {
            return Err("引擎已在运行中".to_string());
        }

        let keys: Vec<u16> = self.enabled_keys.iter().copied().collect();
        if keys.is_empty() {
            return Err("请至少选择一个按键".to_string());
        }

        let mut engine = AutoFireEngine::new();
        engine.set_keys(keys);
        engine.start();

        self.engine = Some(engine);
        self.is_running = true;
        Ok(())
    }

    /// 停止连发引擎
    pub fn stop_engine(&mut self) {
        if let Some(mut engine) = self.engine.take() {
            engine.stop();
        }
        self.is_running = false;
    }
}

/// 主应用程序
#[allow(dead_code)]
pub struct App {
    /// NWG 窗口
    window: nwg::Window,

    /// 所有按键按钮的映射（VK -> Button）
    key_buttons: Vec<(u16, nwg::Button)>,

    /// 配置下拉列表
    profile_combo: nwg::ComboBox<String>,

    /// 保存按钮
    save_button: nwg::Button,

    /// 重置按钮
    reset_button: nwg::Button,

    /// 新建按钮
    new_button: nwg::Button,

    /// 克隆按钮
    clone_button: nwg::Button,

    /// 重命名按钮
    rename_button: nwg::Button,

    /// 删除按钮
    delete_button: nwg::Button,

    /// 启动/停止按钮
    toggle_button: nwg::Button,

    /// 状态标签
    status_label: nwg::Label,

    /// 应用程序状态
    state: Rc<RefCell<AppState>>,
}

#[allow(dead_code)]
impl App {
    /// 构建UI
    pub fn build_ui(state: Rc<RefCell<AppState>>) -> Result<Self, nwg::NwgError> {
        // 创建主窗口
        let mut window = Default::default();
        let (kbd_width, kbd_height) = get_keyboard_bounds();

        // 窗口高度 = 键盘高度 + 控制面板高度 + 边距
        let control_panel_height = 200;
        let window_width = kbd_width.max(910);
        let window_height = kbd_height + control_panel_height;

        nwg::Window::builder()
            .size((window_width, window_height))
            .position((100, 100))
            .title("DNF AutoFire - 连发工具")
            .build(&mut window)?;

        // 创建键盘按钮
        let mut key_buttons = Vec::new();
        let layout = get_keyboard_layout();

        for key_def in layout {
            let mut btn = Default::default();
            nwg::Button::builder()
                .text(key_def.label)
                .position((key_def.x, key_def.y))
                .size((key_def.width, key_def.height))
                .parent(&window)
                .build(&mut btn)?;

            key_buttons.push((key_def.vk, btn));
        }

        // 控制面板起始 Y 坐标
        let panel_y = kbd_height + 10;
        let panel_x = 10;

        // ============ 配置管理面板 ============
        // 配置选择标签
        let mut profile_label = Default::default();
        nwg::Label::builder()
            .text("配置:")
            .position((panel_x, panel_y))
            .size((60, 25))
            .parent(&window)
            .build(&mut profile_label)?;

        // 配置下拉列表
        let mut profile_combo = Default::default();
        nwg::ComboBox::builder()
            .position((panel_x + 60, panel_y))
            .size((200, 25))
            .parent(&window)
            .build(&mut profile_combo)?;

        // 保存按钮
        let mut save_button = Default::default();
        nwg::Button::builder()
            .text("保存")
            .position((panel_x + 270, panel_y))
            .size((70, 30))
            .parent(&window)
            .build(&mut save_button)?;

        // 重置按钮
        let mut reset_button = Default::default();
        nwg::Button::builder()
            .text("重置")
            .position((panel_x + 350, panel_y))
            .size((70, 30))
            .parent(&window)
            .build(&mut reset_button)?;

        // 第二排按钮
        let row2_y = panel_y + 40;

        // 新建按钮
        let mut new_button = Default::default();
        nwg::Button::builder()
            .text("新建配置")
            .position((panel_x + 60, row2_y))
            .size((90, 30))
            .parent(&window)
            .build(&mut new_button)?;

        // 克隆按钮
        let mut clone_button = Default::default();
        nwg::Button::builder()
            .text("克隆配置")
            .position((panel_x + 160, row2_y))
            .size((90, 30))
            .parent(&window)
            .build(&mut clone_button)?;

        // 重命名按钮
        let mut rename_button = Default::default();
        nwg::Button::builder()
            .text("重命名")
            .position((panel_x + 260, row2_y))
            .size((80, 30))
            .parent(&window)
            .build(&mut rename_button)?;

        // 删除按钮
        let mut delete_button = Default::default();
        nwg::Button::builder()
            .text("删除配置")
            .position((panel_x + 350, row2_y))
            .size((90, 30))
            .parent(&window)
            .build(&mut delete_button)?;

        // ============ 控制面板 ============
        let row3_y = panel_y + 90;

        // 启动/停止按钮（大按钮）
        let mut toggle_button = Default::default();
        nwg::Button::builder()
            .text("启动连发")
            .position((panel_x + 60, row3_y))
            .size((200, 60))
            .parent(&window)
            .build(&mut toggle_button)?;

        // 状态标签
        let mut status_label = Default::default();
        nwg::Label::builder()
            .text("状态: 未运行")
            .position((panel_x + 280, row3_y))
            .size((300, 25))
            .parent(&window)
            .build(&mut status_label)?;

        let app = Self {
            window,
            key_buttons,
            profile_combo,
            save_button,
            reset_button,
            new_button,
            clone_button,
            rename_button,
            delete_button,
            toggle_button,
            status_label,
            state,
        };

        Ok(app)
    }

    /// 初始化数据（加载配置列表等）
    pub fn init(&self) {
        self.refresh_profile_list();

        // 加载第一个配置
        if self.profile_combo.len() > 0 {
            self.profile_combo.set_selection(Some(0));
            self.on_profile_changed();
        }
    }

    /// 刷新配置列表
    fn refresh_profile_list(&self) {
        let state = self.state.borrow();
        let profiles = state.config_manager.list_profiles();

        self.profile_combo.set_collection(profiles);
    }

    /// 处理配置切换
    fn on_profile_changed(&self) {
        if let Some(idx) = self.profile_combo.selection() {
            if let Some(name) = self.profile_combo.collection().get(idx) {
                let mut state = self.state.borrow_mut();

                if let Err(e) = state.load_profile(name) {
                    nwg::modal_error_message(
                        &self.window,
                        "加载配置失败",
                        &format!("无法加载配置: {}", e),
                    );
                } else {
                    drop(state);
                    self.update_key_buttons();
                    self.update_status();
                }
            }
        }
    }

    /// 处理保存按钮点击
    fn on_save(&self) {
        let mut state = self.state.borrow_mut();

        if let Err(e) = state.save_current_profile() {
            drop(state);
            nwg::modal_error_message(&self.window, "保存失败", &format!("无法保存配置: {}", e));
        } else {
            drop(state);
            self.update_status();
            nwg::modal_info_message(&self.window, "保存成功", "配置已保存");
        }
    }

    /// 处理重置按钮点击
    fn on_reset(&self) {
        self.on_profile_changed();
    }

    /// 处理新建配置按钮点击
    fn on_new_profile(&self) {
        if let Some(name) = self.prompt_text("新建配置", "请输入配置名称:", "") {
            if name.trim().is_empty() {
                nwg::modal_error_message(&self.window, "错误", "配置名称不能为空");
                return;
            }

            let state = self.state.borrow();
            if state.config_manager.profile_exists(&name) {
                drop(state);
                nwg::modal_error_message(&self.window, "错误", "配置名称已存在");
                return;
            }

            let profile = Profile::new(name.clone());
            if let Err(e) = state.config_manager.save_profile(&profile) {
                drop(state);
                nwg::modal_error_message(&self.window, "创建失败", &format!("无法创建配置: {}", e));
            } else {
                drop(state);
                self.refresh_profile_list();

                // 选中新配置
                let profiles = self.profile_combo.collection();
                if let Some(idx) = profiles.iter().position(|n| n == &name) {
                    self.profile_combo.set_selection(Some(idx));
                    self.on_profile_changed();
                }
            }
        }
    }

    /// 处理克隆配置按钮点击
    fn on_clone_profile(&self) {
        if let Some(idx) = self.profile_combo.selection() {
            if let Some(current_name) = self.profile_combo.collection().get(idx) {
                if let Some(new_name) = self.prompt_text(
                    "克隆配置",
                    "请输入新配置名称:",
                    &format!("{} - 副本", current_name),
                ) {
                    if new_name.trim().is_empty() {
                        nwg::modal_error_message(&self.window, "错误", "配置名称不能为空");
                        return;
                    }

                    let state = self.state.borrow();
                    if state.config_manager.profile_exists(&new_name) {
                        drop(state);
                        nwg::modal_error_message(&self.window, "错误", "配置名称已存在");
                        return;
                    }

                    if let Err(e) = state.config_manager.clone_profile(current_name, &new_name) {
                        drop(state);
                        nwg::modal_error_message(
                            &self.window,
                            "克隆失败",
                            &format!("无法克隆配置: {}", e),
                        );
                    } else {
                        drop(state);
                        self.refresh_profile_list();

                        // 选中新配置
                        let profiles = self.profile_combo.collection();
                        if let Some(idx) = profiles.iter().position(|n| n == &new_name) {
                            self.profile_combo.set_selection(Some(idx));
                            self.on_profile_changed();
                        }
                    }
                }
            }
        }
    }

    /// 处理重命名配置按钮点击
    fn on_rename_profile(&self) {
        if let Some(idx) = self.profile_combo.selection() {
            if let Some(current_name) = self.profile_combo.collection().get(idx) {
                if let Some(new_name) =
                    self.prompt_text("重命名配置", "请输入新名称:", current_name)
                {
                    if new_name.trim().is_empty() {
                        nwg::modal_error_message(&self.window, "错误", "配置名称不能为空");
                        return;
                    }

                    if new_name == *current_name {
                        return; // 名称未改变
                    }

                    let state = self.state.borrow();
                    if state.config_manager.profile_exists(&new_name) {
                        drop(state);
                        nwg::modal_error_message(&self.window, "错误", "配置名称已存在");
                        return;
                    }

                    if let Err(e) = state.config_manager.rename_profile(current_name, &new_name) {
                        drop(state);
                        nwg::modal_error_message(
                            &self.window,
                            "重命名失败",
                            &format!("无法重命名配置: {}", e),
                        );
                    } else {
                        drop(state);
                        self.refresh_profile_list();

                        // 选中重命名后的配置
                        let profiles = self.profile_combo.collection();
                        if let Some(idx) = profiles.iter().position(|n| n == &new_name) {
                            self.profile_combo.set_selection(Some(idx));
                            self.on_profile_changed();
                        }
                    }
                }
            }
        }
    }

    /// 处理删除配置按钮点击
    fn on_delete_profile(&self) {
        if let Some(idx) = self.profile_combo.selection() {
            if let Some(name) = self.profile_combo.collection().get(idx) {
                // 确认删除
                let result = nwg::modal_message(
                    &self.window,
                    &nwg::MessageParams {
                        title: "确认删除",
                        content: &format!("确定要删除配置 \"{}\" 吗？", name),
                        buttons: nwg::MessageButtons::YesNo,
                        icons: nwg::MessageIcons::Warning,
                    },
                );

                if result == nwg::MessageChoice::Yes {
                    let state = self.state.borrow();

                    if let Err(e) = state.config_manager.delete_profile(name) {
                        drop(state);
                        nwg::modal_error_message(
                            &self.window,
                            "删除失败",
                            &format!("无法删除配置: {}", e),
                        );
                    } else {
                        drop(state);
                        self.refresh_profile_list();

                        // 选中第一个配置
                        if self.profile_combo.len() > 0 {
                            self.profile_combo.set_selection(Some(0));
                            self.on_profile_changed();
                        }
                    }
                }
            }
        }
    }

    /// 处理启动/停止按钮点击
    fn on_toggle_engine(&self) {
        let mut state = self.state.borrow_mut();

        if state.is_running {
            // 停止引擎
            state.stop_engine();
            drop(state);

            self.toggle_button.set_text("启动连发");
            self.update_status();
        } else {
            // 启动引擎
            if let Err(e) = state.start_engine() {
                drop(state);
                nwg::modal_error_message(
                    &self.window,
                    "启动失败",
                    &format!("无法启动连发引擎: {}", e),
                );
            } else {
                drop(state);

                self.toggle_button.set_text("停止连发");
                self.update_status();
            }
        }
    }

    /// 处理按键按钮点击
    fn on_key_button_click(&self, vk: u16) {
        let mut state = self.state.borrow_mut();

        // 如果引擎正在运行，禁止修改
        if state.is_running {
            drop(state);
            nwg::modal_error_message(
                &self.window,
                "操作被拒绝",
                "请先停止连发引擎，再修改按键配置",
            );
            return;
        }

        state.toggle_key(vk);
        drop(state);

        self.update_key_buttons();
        self.update_status();
    }

    /// 更新所有按键按钮的显示状态
    fn update_key_buttons(&self) {
        let state = self.state.borrow();

        for (vk, button) in &self.key_buttons {
            if state.is_key_enabled(*vk) {
                button.set_text(&format!("{} ✓", button.text()));
            }
        }
    }

    /// 更新状态标签
    fn update_status(&self) {
        let state = self.state.borrow();

        let status_text = if state.is_running {
            format!("状态: 运行中 | 启用按键: {}", state.enabled_keys.len())
        } else {
            format!(
                "状态: 未运行 | 启用按键: {}{}",
                state.enabled_keys.len(),
                if state.has_unsaved_changes {
                    " (未保存)"
                } else {
                    ""
                }
            )
        };

        self.status_label.set_text(&status_text);
    }

    /// 弹出文本输入对话框（简化版，使用 Windows API）
    fn prompt_text(&self, title: &str, message: &str, default: &str) -> Option<String> {
        // NWG 不提供内置的文本输入对话框，这里使用简化实现
        // 实际项目中应该创建一个单独的对话框窗口
        // 为了简化，这里暂时返回固定值进行测试

        // TODO: 实现真正的文本输入对话框
        nwg::modal_info_message(&self.window, title, message);
        Some(default.to_string())
    }
}
