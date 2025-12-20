// 104键键盘布局定义
// 参考 history 实现，完整定义所有按键的位置和尺寸

use crate::core::vk;

/// 按键定义结构
#[derive(Debug, Clone)]
pub struct KeyDef {
    pub vk: u16,             // 虚拟键码
    pub label: &'static str, // 按键标签
    pub x: i32,              // X 坐标
    pub y: i32,              // Y 坐标
    pub width: i32,          // 宽度
    pub height: i32,         // 高度
}

// 按键尺寸常量
const KEY_SIZE: i32 = 40; // 标准按键尺寸
const KEY_GAP: i32 = 4; // 按键间隙
const KEY_SPACING: i32 = KEY_SIZE + KEY_GAP; // 按键间距（44）

// 起始坐标
const START_X: i32 = 10;
const START_Y: i32 = 10;

/// 获取完整的104键键盘布局
pub fn get_keyboard_layout() -> Vec<KeyDef> {
    let mut keys = Vec::with_capacity(104);

    // ============ 第一排：功能键 F1-F12 ============
    let row1_y = START_Y;

    // ESC 键
    keys.push(KeyDef {
        vk: vk::VK_ESCAPE,
        label: "Esc",
        x: START_X,
        y: row1_y,
        width: KEY_SIZE,
        height: KEY_SIZE,
    });

    // F1-F4（与ESC有间隙）
    let f1_start_x = START_X + KEY_SPACING + 20;
    for (i, &vk) in [vk::VK_F1, vk::VK_F2, vk::VK_F3, vk::VK_F4]
        .iter()
        .enumerate()
    {
        keys.push(KeyDef {
            vk,
            label: match i {
                0 => "F1",
                1 => "F2",
                2 => "F3",
                3 => "F4",
                _ => unreachable!(),
            },
            x: f1_start_x + i as i32 * KEY_SPACING,
            y: row1_y,
            width: KEY_SIZE,
            height: KEY_SIZE,
        });
    }

    // F5-F8（与F4有间隙）
    let f5_start_x = f1_start_x + 4 * KEY_SPACING + 10;
    for (i, &vk) in [vk::VK_F5, vk::VK_F6, vk::VK_F7, vk::VK_F8]
        .iter()
        .enumerate()
    {
        keys.push(KeyDef {
            vk,
            label: match i {
                0 => "F5",
                1 => "F6",
                2 => "F7",
                3 => "F8",
                _ => unreachable!(),
            },
            x: f5_start_x + i as i32 * KEY_SPACING,
            y: row1_y,
            width: KEY_SIZE,
            height: KEY_SIZE,
        });
    }

    // F9-F12（与F8有间隙）
    let f9_start_x = f5_start_x + 4 * KEY_SPACING + 10;
    for (i, &vk) in [vk::VK_F9, vk::VK_F10, vk::VK_F11, vk::VK_F12]
        .iter()
        .enumerate()
    {
        keys.push(KeyDef {
            vk,
            label: match i {
                0 => "F9",
                1 => "F10",
                2 => "F11",
                3 => "F12",
                _ => unreachable!(),
            },
            x: f9_start_x + i as i32 * KEY_SPACING,
            y: row1_y,
            width: KEY_SIZE,
            height: KEY_SIZE,
        });
    }

    // ============ 第二排：数字键区 ============
    let row2_y = START_Y + KEY_SPACING + 10; // 与功能键有额外间隙

    // `~ 键
    keys.push(KeyDef {
        vk: vk::VK_OEM_3, // `~
        label: "`~",
        x: START_X,
        y: row2_y,
        width: KEY_SIZE,
        height: KEY_SIZE,
    });

    // 数字键 1-9, 0
    let numbers = [
        (vk::VK_1, "1"),
        (vk::VK_2, "2"),
        (vk::VK_3, "3"),
        (vk::VK_4, "4"),
        (vk::VK_5, "5"),
        (vk::VK_6, "6"),
        (vk::VK_7, "7"),
        (vk::VK_8, "8"),
        (vk::VK_9, "9"),
        (vk::VK_0, "0"),
    ];
    for (i, &(vk, label)) in numbers.iter().enumerate() {
        keys.push(KeyDef {
            vk,
            label,
            x: START_X + (i as i32 + 1) * KEY_SPACING,
            y: row2_y,
            width: KEY_SIZE,
            height: KEY_SIZE,
        });
    }

    // -_ 键
    keys.push(KeyDef {
        vk: vk::VK_OEM_MINUS,
        label: "-_",
        x: START_X + 11 * KEY_SPACING,
        y: row2_y,
        width: KEY_SIZE,
        height: KEY_SIZE,
    });

    // =+ 键
    keys.push(KeyDef {
        vk: vk::VK_OEM_PLUS,
        label: "=+",
        x: START_X + 12 * KEY_SPACING,
        y: row2_y,
        width: KEY_SIZE,
        height: KEY_SIZE,
    });

    // Backspace（宽度为2倍）
    keys.push(KeyDef {
        vk: vk::VK_BACK,
        label: "Backspace",
        x: START_X + 13 * KEY_SPACING,
        y: row2_y,
        width: KEY_SIZE * 2 + KEY_GAP,
        height: KEY_SIZE,
    });

    // ============ 第三排：Tab + 字母键 Q-P ============
    let row3_y = row2_y + KEY_SPACING;

    // Tab（宽度为1.5倍）
    keys.push(KeyDef {
        vk: vk::VK_TAB,
        label: "Tab",
        x: START_X,
        y: row3_y,
        width: KEY_SIZE + KEY_SIZE / 2,
        height: KEY_SIZE,
    });

    // Q-P 字母键
    let qwerty_row1 = [
        (vk::VK_Q, "Q"),
        (vk::VK_W, "W"),
        (vk::VK_E, "E"),
        (vk::VK_R, "R"),
        (vk::VK_T, "T"),
        (vk::VK_Y, "Y"),
        (vk::VK_U, "U"),
        (vk::VK_I, "I"),
        (vk::VK_O, "O"),
        (vk::VK_P, "P"),
    ];
    let tab_width = KEY_SIZE + KEY_SIZE / 2 + KEY_GAP;
    for (i, &(vk, label)) in qwerty_row1.iter().enumerate() {
        keys.push(KeyDef {
            vk,
            label,
            x: START_X + tab_width + i as i32 * KEY_SPACING,
            y: row3_y,
            width: KEY_SIZE,
            height: KEY_SIZE,
        });
    }

    // [{ 键
    keys.push(KeyDef {
        vk: vk::VK_OEM_4,
        label: "[{",
        x: START_X + tab_width + 10 * KEY_SPACING,
        y: row3_y,
        width: KEY_SIZE,
        height: KEY_SIZE,
    });

    // ]} 键
    keys.push(KeyDef {
        vk: vk::VK_OEM_6,
        label: "]}",
        x: START_X + tab_width + 11 * KEY_SPACING,
        y: row3_y,
        width: KEY_SIZE,
        height: KEY_SIZE,
    });

    // \| 键（宽度为1.5倍）
    keys.push(KeyDef {
        vk: vk::VK_OEM_5,
        label: "\\|",
        x: START_X + tab_width + 12 * KEY_SPACING,
        y: row3_y,
        width: KEY_SIZE + KEY_SIZE / 2,
        height: KEY_SIZE,
    });

    // ============ 第四排：Caps Lock + 字母键 A-L ============
    let row4_y = row3_y + KEY_SPACING;

    // Caps Lock（宽度为1.75倍）
    let caps_width = KEY_SIZE + KEY_SIZE * 3 / 4;
    keys.push(KeyDef {
        vk: vk::VK_CAPITAL,
        label: "Caps",
        x: START_X,
        y: row4_y,
        width: caps_width,
        height: KEY_SIZE,
    });

    // A-L 字母键
    let qwerty_row2 = [
        (vk::VK_A, "A"),
        (vk::VK_S, "S"),
        (vk::VK_D, "D"),
        (vk::VK_F, "F"),
        (vk::VK_G, "G"),
        (vk::VK_H, "H"),
        (vk::VK_J, "J"),
        (vk::VK_K, "K"),
        (vk::VK_L, "L"),
    ];
    for (i, &(vk, label)) in qwerty_row2.iter().enumerate() {
        keys.push(KeyDef {
            vk,
            label,
            x: START_X + caps_width + KEY_GAP + i as i32 * KEY_SPACING,
            y: row4_y,
            width: KEY_SIZE,
            height: KEY_SIZE,
        });
    }

    // ;: 键
    keys.push(KeyDef {
        vk: vk::VK_OEM_1,
        label: ";:",
        x: START_X + caps_width + KEY_GAP + 9 * KEY_SPACING,
        y: row4_y,
        width: KEY_SIZE,
        height: KEY_SIZE,
    });

    // '" 键
    keys.push(KeyDef {
        vk: vk::VK_OEM_7,
        label: "'\"",
        x: START_X + caps_width + KEY_GAP + 10 * KEY_SPACING,
        y: row4_y,
        width: KEY_SIZE,
        height: KEY_SIZE,
    });

    // Enter（宽度为2.25倍）
    keys.push(KeyDef {
        vk: vk::VK_RETURN,
        label: "Enter",
        x: START_X + caps_width + KEY_GAP + 11 * KEY_SPACING,
        y: row4_y,
        width: KEY_SIZE * 2 + KEY_SIZE / 4,
        height: KEY_SIZE,
    });

    // ============ 第五排：Shift + 字母键 Z-M ============
    let row5_y = row4_y + KEY_SPACING;

    // Left Shift（宽度为2.25倍）
    let lshift_width = KEY_SIZE * 2 + KEY_SIZE / 4;
    keys.push(KeyDef {
        vk: vk::VK_LSHIFT,
        label: "Shift",
        x: START_X,
        y: row5_y,
        width: lshift_width,
        height: KEY_SIZE,
    });

    // Z-M 字母键
    let qwerty_row3 = [
        (vk::VK_Z, "Z"),
        (vk::VK_X, "X"),
        (vk::VK_C, "C"),
        (vk::VK_V, "V"),
        (vk::VK_B, "B"),
        (vk::VK_N, "N"),
        (vk::VK_M, "M"),
    ];
    for (i, &(vk, label)) in qwerty_row3.iter().enumerate() {
        keys.push(KeyDef {
            vk,
            label,
            x: START_X + lshift_width + KEY_GAP + i as i32 * KEY_SPACING,
            y: row5_y,
            width: KEY_SIZE,
            height: KEY_SIZE,
        });
    }

    // ,< 键
    keys.push(KeyDef {
        vk: vk::VK_OEM_COMMA,
        label: ",<",
        x: START_X + lshift_width + KEY_GAP + 7 * KEY_SPACING,
        y: row5_y,
        width: KEY_SIZE,
        height: KEY_SIZE,
    });

    // .> 键
    keys.push(KeyDef {
        vk: vk::VK_OEM_PERIOD,
        label: ".>",
        x: START_X + lshift_width + KEY_GAP + 8 * KEY_SPACING,
        y: row5_y,
        width: KEY_SIZE,
        height: KEY_SIZE,
    });

    // /? 键
    keys.push(KeyDef {
        vk: vk::VK_OEM_2,
        label: "/?",
        x: START_X + lshift_width + KEY_GAP + 9 * KEY_SPACING,
        y: row5_y,
        width: KEY_SIZE,
        height: KEY_SIZE,
    });

    // Right Shift（宽度为2.75倍）
    keys.push(KeyDef {
        vk: vk::VK_RSHIFT,
        label: "Shift",
        x: START_X + lshift_width + KEY_GAP + 10 * KEY_SPACING,
        y: row5_y,
        width: KEY_SIZE * 2 + KEY_SIZE * 3 / 4,
        height: KEY_SIZE,
    });

    // ============ 第六排：Ctrl/Win/Alt + 空格 ============
    let row6_y = row5_y + KEY_SPACING;

    // Left Ctrl（宽度为1.25倍）
    let ctrl_width = KEY_SIZE + KEY_SIZE / 4;
    keys.push(KeyDef {
        vk: vk::VK_LCONTROL,
        label: "Ctrl",
        x: START_X,
        y: row6_y,
        width: ctrl_width,
        height: KEY_SIZE,
    });

    // Left Win
    keys.push(KeyDef {
        vk: vk::VK_LWIN,
        label: "Win",
        x: START_X + ctrl_width + KEY_GAP,
        y: row6_y,
        width: ctrl_width,
        height: KEY_SIZE,
    });

    // Left Alt
    keys.push(KeyDef {
        vk: vk::VK_LMENU,
        label: "Alt",
        x: START_X + (ctrl_width + KEY_GAP) * 2,
        y: row6_y,
        width: ctrl_width,
        height: KEY_SIZE,
    });

    // Space（宽度为6.25倍）
    let space_x = START_X + (ctrl_width + KEY_GAP) * 3;
    keys.push(KeyDef {
        vk: vk::VK_SPACE,
        label: "Space",
        x: space_x,
        y: row6_y,
        width: KEY_SIZE * 6 + KEY_SIZE / 4,
        height: KEY_SIZE,
    });

    // Right Alt
    let ralt_x = space_x + KEY_SIZE * 6 + KEY_SIZE / 4 + KEY_GAP;
    keys.push(KeyDef {
        vk: vk::VK_RMENU,
        label: "Alt",
        x: ralt_x,
        y: row6_y,
        width: ctrl_width,
        height: KEY_SIZE,
    });

    // Right Win
    keys.push(KeyDef {
        vk: vk::VK_RWIN,
        label: "Win",
        x: ralt_x + ctrl_width + KEY_GAP,
        y: row6_y,
        width: ctrl_width,
        height: KEY_SIZE,
    });

    // Menu
    keys.push(KeyDef {
        vk: vk::VK_APPS,
        label: "Menu",
        x: ralt_x + (ctrl_width + KEY_GAP) * 2,
        y: row6_y,
        width: ctrl_width,
        height: KEY_SIZE,
    });

    // Right Ctrl
    keys.push(KeyDef {
        vk: vk::VK_RCONTROL,
        label: "Ctrl",
        x: ralt_x + (ctrl_width + KEY_GAP) * 3,
        y: row6_y,
        width: ctrl_width,
        height: KEY_SIZE,
    });

    // ============ 编辑键区（Insert/Home/PgUp等）============
    let edit_x = START_X + 15 * KEY_SPACING + 20; // 与主键盘有间隙

    // 第一排编辑键：PrtSc, ScrLk, Pause
    keys.push(KeyDef {
        vk: vk::VK_SNAPSHOT,
        label: "PrtSc",
        x: edit_x,
        y: row1_y,
        width: KEY_SIZE,
        height: KEY_SIZE,
    });

    keys.push(KeyDef {
        vk: vk::VK_SCROLL,
        label: "ScrLk",
        x: edit_x + KEY_SPACING,
        y: row1_y,
        width: KEY_SIZE,
        height: KEY_SIZE,
    });

    keys.push(KeyDef {
        vk: vk::VK_PAUSE,
        label: "Pause",
        x: edit_x + KEY_SPACING * 2,
        y: row1_y,
        width: KEY_SIZE,
        height: KEY_SIZE,
    });

    // 第二排编辑键：Insert, Home, PgUp
    keys.push(KeyDef {
        vk: vk::VK_INSERT,
        label: "Ins",
        x: edit_x,
        y: row2_y,
        width: KEY_SIZE,
        height: KEY_SIZE,
    });

    keys.push(KeyDef {
        vk: vk::VK_HOME,
        label: "Home",
        x: edit_x + KEY_SPACING,
        y: row2_y,
        width: KEY_SIZE,
        height: KEY_SIZE,
    });

    keys.push(KeyDef {
        vk: vk::VK_PRIOR,
        label: "PgUp",
        x: edit_x + KEY_SPACING * 2,
        y: row2_y,
        width: KEY_SIZE,
        height: KEY_SIZE,
    });

    // 第三排编辑键：Delete, End, PgDn
    keys.push(KeyDef {
        vk: vk::VK_DELETE,
        label: "Del",
        x: edit_x,
        y: row3_y,
        width: KEY_SIZE,
        height: KEY_SIZE,
    });

    keys.push(KeyDef {
        vk: vk::VK_END,
        label: "End",
        x: edit_x + KEY_SPACING,
        y: row3_y,
        width: KEY_SIZE,
        height: KEY_SIZE,
    });

    keys.push(KeyDef {
        vk: vk::VK_NEXT,
        label: "PgDn",
        x: edit_x + KEY_SPACING * 2,
        y: row3_y,
        width: KEY_SIZE,
        height: KEY_SIZE,
    });

    // 第四排编辑键：方向键（上）
    keys.push(KeyDef {
        vk: vk::VK_UP,
        label: "↑",
        x: edit_x + KEY_SPACING,
        y: row5_y,
        width: KEY_SIZE,
        height: KEY_SIZE,
    });

    // 第五排编辑键：方向键（左下右）
    keys.push(KeyDef {
        vk: vk::VK_LEFT,
        label: "←",
        x: edit_x,
        y: row6_y,
        width: KEY_SIZE,
        height: KEY_SIZE,
    });

    keys.push(KeyDef {
        vk: vk::VK_DOWN,
        label: "↓",
        x: edit_x + KEY_SPACING,
        y: row6_y,
        width: KEY_SIZE,
        height: KEY_SIZE,
    });

    keys.push(KeyDef {
        vk: vk::VK_RIGHT,
        label: "→",
        x: edit_x + KEY_SPACING * 2,
        y: row6_y,
        width: KEY_SIZE,
        height: KEY_SIZE,
    });

    // ============ 小键盘区 ============
    let numpad_x = edit_x + KEY_SPACING * 4; // 与编辑键有间隙

    // 第一排小键盘：Num Lock, /, *, -
    keys.push(KeyDef {
        vk: vk::VK_NUMLOCK,
        label: "Num",
        x: numpad_x,
        y: row2_y,
        width: KEY_SIZE,
        height: KEY_SIZE,
    });

    keys.push(KeyDef {
        vk: vk::VK_DIVIDE,
        label: "/",
        x: numpad_x + KEY_SPACING,
        y: row2_y,
        width: KEY_SIZE,
        height: KEY_SIZE,
    });

    keys.push(KeyDef {
        vk: vk::VK_MULTIPLY,
        label: "*",
        x: numpad_x + KEY_SPACING * 2,
        y: row2_y,
        width: KEY_SIZE,
        height: KEY_SIZE,
    });

    keys.push(KeyDef {
        vk: vk::VK_SUBTRACT,
        label: "-",
        x: numpad_x + KEY_SPACING * 3,
        y: row2_y,
        width: KEY_SIZE,
        height: KEY_SIZE,
    });

    // 第二排小键盘：7, 8, 9, +（高度为2倍）
    keys.push(KeyDef {
        vk: vk::VK_NUMPAD7,
        label: "7",
        x: numpad_x,
        y: row3_y,
        width: KEY_SIZE,
        height: KEY_SIZE,
    });

    keys.push(KeyDef {
        vk: vk::VK_NUMPAD8,
        label: "8",
        x: numpad_x + KEY_SPACING,
        y: row3_y,
        width: KEY_SIZE,
        height: KEY_SIZE,
    });

    keys.push(KeyDef {
        vk: vk::VK_NUMPAD9,
        label: "9",
        x: numpad_x + KEY_SPACING * 2,
        y: row3_y,
        width: KEY_SIZE,
        height: KEY_SIZE,
    });

    keys.push(KeyDef {
        vk: vk::VK_ADD,
        label: "+",
        x: numpad_x + KEY_SPACING * 3,
        y: row3_y,
        width: KEY_SIZE,
        height: KEY_SIZE * 2 + KEY_GAP, // 高度为2倍
    });

    // 第三排小键盘：4, 5, 6
    keys.push(KeyDef {
        vk: vk::VK_NUMPAD4,
        label: "4",
        x: numpad_x,
        y: row4_y,
        width: KEY_SIZE,
        height: KEY_SIZE,
    });

    keys.push(KeyDef {
        vk: vk::VK_NUMPAD5,
        label: "5",
        x: numpad_x + KEY_SPACING,
        y: row4_y,
        width: KEY_SIZE,
        height: KEY_SIZE,
    });

    keys.push(KeyDef {
        vk: vk::VK_NUMPAD6,
        label: "6",
        x: numpad_x + KEY_SPACING * 2,
        y: row4_y,
        width: KEY_SIZE,
        height: KEY_SIZE,
    });

    // 第四排小键盘：1, 2, 3, Enter（高度为2倍）
    keys.push(KeyDef {
        vk: vk::VK_NUMPAD1,
        label: "1",
        x: numpad_x,
        y: row5_y,
        width: KEY_SIZE,
        height: KEY_SIZE,
    });

    keys.push(KeyDef {
        vk: vk::VK_NUMPAD2,
        label: "2",
        x: numpad_x + KEY_SPACING,
        y: row5_y,
        width: KEY_SIZE,
        height: KEY_SIZE,
    });

    keys.push(KeyDef {
        vk: vk::VK_NUMPAD3,
        label: "3",
        x: numpad_x + KEY_SPACING * 2,
        y: row5_y,
        width: KEY_SIZE,
        height: KEY_SIZE,
    });

    // 小键盘 Enter（注意与主键盘Enter使用不同VK码）
    keys.push(KeyDef {
        vk: 0x0D, // VK_RETURN，小键盘Enter与主键盘Enter VK码相同
        label: "Ent",
        x: numpad_x + KEY_SPACING * 3,
        y: row5_y,
        width: KEY_SIZE,
        height: KEY_SIZE * 2 + KEY_GAP, // 高度为2倍
    });

    // 第五排小键盘：0（宽度为2倍）, .
    keys.push(KeyDef {
        vk: vk::VK_NUMPAD0,
        label: "0",
        x: numpad_x,
        y: row6_y,
        width: KEY_SIZE * 2 + KEY_GAP, // 宽度为2倍
        height: KEY_SIZE,
    });

    keys.push(KeyDef {
        vk: vk::VK_DECIMAL,
        label: ".",
        x: numpad_x + KEY_SPACING * 2,
        y: row6_y,
        width: KEY_SIZE,
        height: KEY_SIZE,
    });

    keys
}

/// 获取键盘区域的总宽度和高度（用于计算窗口尺寸）
pub fn get_keyboard_bounds() -> (i32, i32) {
    let keys = get_keyboard_layout();

    let max_x = keys.iter().map(|k| k.x + k.width).max().unwrap_or(0);

    let max_y = keys.iter().map(|k| k.y + k.height).max().unwrap_or(0);

    (max_x + 10, max_y + 10) // 添加右下边距
}
