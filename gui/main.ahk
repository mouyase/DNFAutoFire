#Requires AutoHotkey v2.0

class MainGui {
    static gui := ""  ; 添加静态变量

    static Create() {
        ; 创建主窗口
        this.gui := Gui("-MinimizeBox -MaximizeBox")
        this.gui.Title := "DNF Auto Fire"
        
        ; 添加GroupBox
        this.gui.Add("GroupBox", "x8 y8 w926 h276", "按键设置 - [ 绿色为启用连发 蓝色为关闭连发 ]")
        
        ; 创建所有按键区域
        this.CreateFunctionKeys()
        this.CreateNumberKeys()
        this.CreateMainKeys()
        this.CreateEditKeys()
        this.CreateArrowKeys()
        this.CreateNumpadKeys()
        this.CreateInfoArea()
        
        return this.gui
    }
    
    static CreateFunctionKeys() {
        ; 功能键行
        this.gui.SetFont("s12")
        this.gui.Add("Text", "vEsc x16 y30 w36 h36 +0x200 +Border +Center", "Esc")
        
        ; F1-F4
        x := 90
        for key in ["F1", "F2", "F3", "F4"] {
            this.gui.Add("Text", "v" key " x" x " y30 w36 h36 +0x200 +Border +Center", key)
            x += 40
        }
        
        ; F5-F8
        x += 20
        for key in ["F5", "F6", "F7", "F8"] {
            this.gui.Add("Text", "v" key " x" x " y30 w36 h36 +0x200 +Border +Center", key)
            x += 40
        }
        
        ; F9-F12
        x += 20
        for key in ["F9", "F10", "F11", "F12"] {
            this.gui.Add("Text", "v" key " x" x " y30 w36 h36 +0x200 +Border +Center", key)
            x += 40
        }
        
        ; 编辑键区域（小字体）
        this.gui.SetFont("s7")
        specialKeys := Map("PrtSc", 630, "ScrLk", 670, "Pause", 710)
        for key, pos in specialKeys {
            this.gui.Add("Text", "v" key " x" pos " y30 w36 h36 +0x200 +Border +Center", key)
        }
    }
    
    static CreateNumberKeys() {
        this.gui.SetFont("s12")
        this.gui.Add("Text", "vTilde x16 y80 w36 h36 +0x200 +Border +Center", "``")
        
        x := 56
        for key in ["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"] {
            this.gui.Add("Text", "v" key " x" x " y80 w36 h36 +0x200 +Border +Center", key)
            x += 40
        }
        
        symbolKeys := Map("Minus", 456, "Equal", 496)
        for key, pos in symbolKeys {
            this.gui.Add("Text", "v" key " x" pos " y80 w36 h36 +0x200 +Border +Center", key == "Minus" ? "-" : "=")
        }
        
        this.gui.SetFont("s9")
        this.gui.Add("Text", "vBackspace x536 y80 w70 h36 +0x200 +Border +Center", "Backspace")
    }
    
    static CreateMainKeys() {
        ; QWERTY行
        this.gui.SetFont("s12")
        this.gui.Add("Text", "vTab x16 y120 w54 h36 +0x200 +Border +Center", "Tab")
        x := 74
        for key in ["Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P"] {
            this.gui.Add("Text", "v" key " x" x " y120 w36 h36 +0x200 +Border +Center", key)
            x += 40
        }
        
        this.gui.Add("Text", "vLBracket x474 y120 w36 h36 +0x200 +Border +Center", "[")
        this.gui.Add("Text", "vRBracket x514 y120 w36 h36 +0x200 +Border +Center", "]")
        this.gui.Add("Text", "vBackslash x554 y120 w52 h36 +0x200 +Border +Center", "\")
        
        ; ASDF行
        this.gui.SetFont("s9")
        this.gui.Add("Text", "vCapsLock x16 y160 w64 h36 +0x200 +Border +Center", "CapsLock")
        x := 84
        this.gui.SetFont("s12")
        for key in ["A", "S", "D", "F", "G", "H", "J", "K", "L"] {
            this.gui.Add("Text", "v" key " x" x " y160 w36 h36 +0x200 +Border +Center", key)
            x += 40
        }
        this.gui.Add("Text", "vSemicolon x444 y160 w36 h36 +0x200 +Border +Center", ";")
        this.gui.Add("Text", "vQuote x484 y160 w36 h36 +0x200 +Border +Center", "'")
        this.gui.Add("Text", "vEnter x524 y160 w82 h36 +0x200 +Border +Center", "Enter")
        
        ; ZXCV行
        this.gui.Add("Text", "vShift x16 y200 w86 h36 +0x200 +Border +Center", "Shift")
        x := 106
        for key in ["Z", "X", "C", "V", "B", "N", "M"] {
            this.gui.Add("Text", "v" key " x" x " y200 w36 h36 +0x200 +Border +Center", key)
            x += 40
        }
        this.gui.Add("Text", "vComma x386 y200 w36 h36 +0x200 +Border +Center", ",")
        this.gui.Add("Text", "vPeriod x426 y200 w36 h36 +0x200 +Border +Center", ".")
        this.gui.Add("Text", "vSlash x466 y200 w36 h36 +0x200 +Border +Center", "/")
        this.gui.Add("Text", "vRShift x506 y200 w100 h36 +0x200 +Border +Center", "RShift")
        
        ; 空格行
        this.gui.Add("Text", "vCtrl x16 y240 w48 h36 +0x200 +Border +Center", "LCtrl")
        this.gui.Add("Text", "x68 y240 w48 h36 +0x200 +Border +Center +Disabled", "Win")
        this.gui.Add("Text", "vAlt x120 y240 w48 h36 +0x200 +Border +Center", "LAlt")
        this.gui.Add("Text", "vSpace x172 y240 w226 h36 +0x200 +Border +Center", "Space")
        this.gui.Add("Text", "vRAlt x402 y240 w48 h36 +0x200 +Border +Center", "RAlt")
        this.gui.Add("Text", "x454 y240 w48 h36 +0x200 +Border +Center +Disabled", "Fn")
        this.gui.Add("Text", "x506 y240 w48 h36 +0x200 +Border +Center +Disabled", "Menu")
        this.gui.Add("Text", "vRCtrl x558 y240 w48 h36 +0x200 +Border +Center", "RCtrl")
    }
    
    static CreateEditKeys() {
        ; 编辑键区域
        this.gui.SetFont("s9")
        editKeys := Map(
            "Ins", [630, 70], "Home", [670, 70], "PgUp", [710, 70],
            "Del", [630, 110], "End", [670, 110], "PgDn", [710, 110]
        )
        for key, pos in editKeys {
            this.gui.Add("Text", "v" key " x" pos[1] " y" pos[2] " w36 h36 +0x200 +Border +Center", key)
        }
    }
    
    static CreateArrowKeys() {
        ; 方向键区域
        this.gui.SetFont("s9")
        arrowKeys := Map(
            "Up", [670, 200],
            "Left", [630, 240], "Down", [670, 240], "Right", [710, 240]
        )
        for key, pos in arrowKeys {
            this.gui.Add("Text", "v" key " x" pos[1] " y" pos[2] " w36 h36 +0x200 +Border +Center", key)
        }
    }
    
    static CreateNumpadKeys() {
        ; 小键盘区域
        this.gui.SetFont("s8")
        this.gui.Add("Text", "vNumLk x770 y80 w36 h36 +0x200 +Border +Center", "NumLk")
        
        numpadOps := Map(
            "NumSlash", [810, 80], 
            "NumStar", [850, 80],
            "NumMinus", [890, 80]
        )
        for key, pos in numpadOps {
            this.gui.Add("Text", "v" key " x" pos[1] " y" pos[2] " w36 h36 +0x200 +Border +Center", key = "NumSlash" ? "/" : key = "NumStar" ? "*" : "-")
        }
        
        ; 加号键（特殊高度）
        this.gui.Add("Text", "vNumPlus x890 y120 w36 h76 +0x200 +Border +Center", "+")
        
        ; 数字键区域
        this.gui.SetFont("s9")
        numpadPos := [
            [7,770,120], [8,810,120], [9,850,120],
            [4,770,160], [5,810,160], [6,850,160],
            [1,770,200], [2,810,200], [3,850,200]
        ]
        for pos in numpadPos {
            this.gui.Add("Text", "vNum" pos[1] " x" pos[2] " y" pos[3] " w36 h36 +0x200 +Border +Center", pos[1])
        }
        
        ; 0和小数点
        this.gui.Add("Text", "vNum0 x770 y240 w76 h36 +0x200 +Border +Center", "0")
        this.gui.Add("Text", "vNumDot x850 y240 w36 h36 +0x200 +Border +Center", ".")
        
        ; Enter键（特殊高度和文本）
        this.gui.SetFont("s7")
        this.gui.Add("Text", "vNumEnter x890 y200 w36 h76 +0x200 +Border +Center", "Enter")
    }
    
    static CreateInfoArea() {
        this.gui.SetFont("s9")
        this.gui.Add("Text", "x770 y34", "当前版本: v1.0")
        this.gui.Add("Link", "x770 y54", '<a href="https://bbs.colg.cn/thread-8894989-1-1.html">Colg</a> <a href="https://github.com/mouyase/DNFAutoFire">Github</a>')
        this.gui.Add("Text", "x890 y30 w36 h36 +0x200 +Border +Center", "清空")
    }
}

; 创建并显示主窗口
MainGui.Create().Show()