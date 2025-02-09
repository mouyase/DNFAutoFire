#Requires AutoHotkey v2.0

; 创建主窗口
MainGui := Gui("-MinimizeBox -MaximizeBox -Theme +OwnDialogs")
MainGui.Title := "DNF Auto Fire"

; 添加GroupBox
MainGui.Add("GroupBox", "x8 y8 w926 h276", "按键设置 - [ 绿色为启用连发 蓝色为关闭连发 ]")

; 设置默认字体
MainGui.SetFont("s12 cBlue")

; 创建所有按键 - 功能键行
MainGui.Add("Text", "vEsc x16 y30 w36 h36 +0x200 +0x400000 +Center", "Esc")
x := 90
for key in ["F1", "F2", "F3", "F4"] {
    MainGui.Add("Text", "v" key " x" x " y30 w36 h36 +0x200 +0x400000 +Center", key)
    x += 40
}
x += 20  ; F4和F5之间的额外间距
for key in ["F5", "F6", "F7", "F8"] {
    MainGui.Add("Text", "v" key " x" x " y30 w36 h36 +0x200 +0x400000 +Center", key)
    x += 40
}
x += 20  ; F8和F9之间的额外间距
for key in ["F9", "F10", "F11", "F12"] {
    MainGui.Add("Text", "v" key " x" x " y30 w36 h36 +0x200 +0x400000 +Center", key)
    x += 40
}

; 数字键行
x := 16
MainGui.Add("Text", "vTilde x" x " y80 w36 h36 +0x200 +0x400000 +Center", "``")
x += 40
for key in ["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"] {
    MainGui.Add("Text", "v" key " x" x " y80 w36 h36 +0x200 +0x400000 +Center", key)
    x += 40
}
MainGui.Add("Text", "vSub x" x " y80 w36 h36 +0x200 +0x400000 +Center", "-")
x += 40
MainGui.Add("Text", "vAdd x" x " y80 w36 h36 +0x200 +0x400000 +Center", "=")
x += 40
MainGui.Add("Text", "vBackspace x" x " y80 w70 h36 +0x200 +0x400000 +Center", "←")

; 第三行（Q行）
x := 16
MainGui.Add("Text", "vTab x" x " y120 w36 h36 +0x200 +0x400000 +Center", "Tab")
x += 40
MainGui.Add("Text", "vQ x" x " y120 w36 h36 +0x200 +0x400000 +Center", "Q")
x += 40
MainGui.Add("Text", "vW x" x " y120 w36 h36 +0x200 +0x400000 +Center", "W")
x += 40
MainGui.Add("Text", "vE x" x " y120 w36 h36 +0x200 +0x400000 +Center", "E")
x += 40
MainGui.Add("Text", "vR x" x " y120 w36 h36 +0x200 +0x400000 +Center", "R")
x += 40
MainGui.Add("Text", "vT x" x " y120 w36 h36 +0x200 +0x400000 +Center", "T")
x += 40
MainGui.Add("Text", "vY x" x " y120 w36 h36 +0x200 +0x400000 +Center", "Y")
x += 40
MainGui.Add("Text", "vU x" x " y120 w36 h36 +0x200 +0x400000 +Center", "U")
x += 40
MainGui.Add("Text", "vI x" x " y120 w36 h36 +0x200 +0x400000 +Center", "I")
x += 40
MainGui.Add("Text", "vO x" x " y120 w36 h36 +0x200 +0x400000 +Center", "O")
x += 40
MainGui.Add("Text", "vP x" x " y120 w36 h36 +0x200 +0x400000 +Center", "P")
x += 40
MainGui.Add("Text", "vBracketL x" x " y120 w36 h36 +0x200 +0x400000 +Center", "[")
x += 40
MainGui.Add("Text", "vBracketR x" x " y120 w36 h36 +0x200 +0x400000 +Center", "]")
x += 40
MainGui.Add("Text", "vBackslash x" x " y120 w36 h36 +0x200 +0x400000 +Center", "\")

; 第四行（A行）
x := 16
MainGui.Add("Text", "vCapsLock x" x " y160 w36 h36 +0x200 +0x400000 +Center", "CapsLock")
x += 40
MainGui.Add("Text", "vA x" x " y160 w36 h36 +0x200 +0x400000 +Center", "A")
x += 40
MainGui.Add("Text", "vS x" x " y160 w36 h36 +0x200 +0x400000 +Center", "S")
x += 40
MainGui.Add("Text", "vD x" x " y160 w36 h36 +0x200 +0x400000 +Center", "D")
x += 40
MainGui.Add("Text", "vF x" x " y160 w36 h36 +0x200 +0x400000 +Center", "F")
x += 40
MainGui.Add("Text", "vG x" x " y160 w36 h36 +0x200 +0x400000 +Center", "G")
x += 40
MainGui.Add("Text", "vH x" x " y160 w36 h36 +0x200 +0x400000 +Center", "H")
x += 40
MainGui.Add("Text", "vJ x" x " y160 w36 h36 +0x200 +0x400000 +Center", "J")
x += 40
MainGui.Add("Text", "vK x" x " y160 w36 h36 +0x200 +0x400000 +Center", "K")
x += 40
MainGui.Add("Text", "vL x" x " y160 w36 h36 +0x200 +0x400000 +Center", "L")
x += 40
MainGui.Add("Text", "vSemicolon x" x " y160 w36 h36 +0x200 +0x400000 +Center", ";")
x += 40
MainGui.Add("Text", "vApostrophe x" x " y160 w36 h36 +0x200 +0x400000 +Center", "'")
x += 40
MainGui.Add("Text", "vEnter x" x " y160 w36 h36 +0x200 +0x400000 +Center", "Enter")

; 第五行（Z行）
x := 16
MainGui.Add("Text", "vShift x" x " y200 w36 h36 +0x200 +0x400000 +Center", "Shift")
x += 40
MainGui.Add("Text", "vZ x" x " y200 w36 h36 +0x200 +0x400000 +Center", "Z")
x += 40
MainGui.Add("Text", "vX x" x " y200 w36 h36 +0x200 +0x400000 +Center", "X")
x += 40
MainGui.Add("Text", "vC x" x " y200 w36 h36 +0x200 +0x400000 +Center", "C")
x += 40
MainGui.Add("Text", "vV x" x " y200 w36 h36 +0x200 +0x400000 +Center", "V")
x += 40
MainGui.Add("Text", "vB x" x " y200 w36 h36 +0x200 +0x400000 +Center", "B")
x += 40
MainGui.Add("Text", "vN x" x " y200 w36 h36 +0x200 +0x400000 +Center", "N")
x += 40
MainGui.Add("Text", "vM x" x " y200 w36 h36 +0x200 +0x400000 +Center", "M")
x += 40
MainGui.Add("Text", "vComma x" x " y200 w36 h36 +0x200 +0x400000 +Center", ",")
x += 40
MainGui.Add("Text", "vPeriod x" x " y200 w36 h36 +0x200 +0x400000 +Center", ".")
x += 40
MainGui.Add("Text", "vSlash x" x " y200 w36 h36 +0x200 +0x400000 +Center", "/")
x += 40
MainGui.Add("Text", "vRShift x" x " y200 w36 h36 +0x200 +0x400000 +Center", "RShift")

; 第六行（空格行）
x := 16
MainGui.Add("Text", "vCtrl x" x " y240 w36 h36 +0x200 +0x400000 +Center", "Ctrl")
x += 40
MainGui.Add("Text", "vWin x" x " y240 w36 h36 +0x200 +0x400000 +Center", "Win")
x += 40
MainGui.Add("Text", "vAlt x" x " y240 w36 h36 +0x200 +0x400000 +Center", "Alt")
x += 40
MainGui.Add("Text", "vSpace x" x " y240 w36 h36 +0x200 +0x400000 +Center", "Space")
x += 40
MainGui.Add("Text", "vRAlt x" x " y240 w36 h36 +0x200 +0x400000 +Center", "RAlt")
x += 40
MainGui.Add("Text", "vRWin x" x " y240 w36 h36 +0x200 +0x400000 +Center", "RWin")
x += 40
MainGui.Add("Text", "vAppsKey x" x " y240 w36 h36 +0x200 +0x400000 +Center", "AppsKey")
x += 40
MainGui.Add("Text", "vRCtrl x" x " y240 w36 h36 +0x200 +0x400000 +Center", "RCtrl")

; 方向键区域
MainGui.Add("Text", "vUp x670 y200 w36 h36 +0x200 +0x400000 +Center", "↑")
MainGui.Add("Text", "vLeft x630 y240 w36 h36 +0x200 +0x400000 +Center", "←")
MainGui.Add("Text", "vDown x670 y240 w36 h36 +0x200 +0x400000 +Center", "↓")
MainGui.Add("Text", "vRight x710 y240 w36 h36 +0x200 +0x400000 +Center", "→")

; 小键盘区域
MainGui.Add("Text", "vNumLk x770 y80 w36 h36 +0x200 +0x400000 +Center", "NumLk")
MainGui.Add("Text", "vNumSlash x810 y80 w36 h36 +0x200 +0x400000 +Center", "/")
MainGui.Add("Text", "vNumStar x850 y80 w36 h36 +0x200 +0x400000 +Center", "*")
MainGui.Add("Text", "vNumSub x890 y80 w36 h36 +0x200 +0x400000 +Center", "-")

; 存储按钮对象的数组
buttons := Map()

; 创建键盘布局
baseX := 16  ; 基础起始X坐标
standardRowWidth := 926  ; 标准行宽度（参考原代码的GroupBox宽度）

; 功能键行特殊处理
y := 30  ; 第一行Y坐标
x := baseX

; 第一行布局（Esc和功能键）
btn := MainGui.Add("Button", "x" x " y" y " w36 h36", "Esc")
x := 90  ; Esc和F1之间的特定间距
for i, key in ["F1", "F2", "F3", "F4"] {
    btn := MainGui.Add("Button", "x" x " y" y " w36 h36", key)
    x += 40
}
x += 20  ; F4和F5之间的额外间距
for i, key in ["F5", "F6", "F7", "F8"] {
    btn := MainGui.Add("Button", "x" x " y" y " w36 h36", key)
    x += 40
}
x += 20  ; F8和F9之间的额外间距
for i, key in ["F9", "F10", "F11", "F12"] {
    btn := MainGui.Add("Button", "x" x " y" y " w36 h36", key)
    x += 40
}

; 第二行（数字键行）
y := 80
x := baseX
keyRow := ["``", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "-", "=", "Backspace"]
widths := Map("Backspace", 70)  ; 特殊宽度键

for key in keyRow {
    width := widths.Has(key) ? widths[key] : 36
    btn := MainGui.Add("Button", "x" x " y" y " w" width " h36", key)
    x += width + 4
}

; 第三行（Q行）
y := 120
x := baseX
keyRow := ["Tab", "Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P", "[", "]", "\"]
widths := Map("Tab", 54, "\", 52)

for key in keyRow {
    width := widths.Has(key) ? widths[key] : 36
    btn := MainGui.Add("Button", "x" x " y" y " w" width " h36", key)
    x += width + 4
}

; 第四行（A行）
y := 160
x := baseX
keyRow := ["CapsLock", "A", "S", "D", "F", "G", "H", "J", "K", "L", ";", "'", "Enter"]
widths := Map("CapsLock", 64, "Enter", 82)

for key in keyRow {
    width := widths.Has(key) ? widths[key] : 36
    btn := MainGui.Add("Button", "x" x " y" y " w" width " h36", key)
    x += width + 4
}

; 第五行（Z行）
y := 200
x := baseX
keyRow := ["Shift", "Z", "X", "C", "V", "B", "N", "M", ",", ".", "/", "RShift"]
widths := Map("Shift", 86, "RShift", 100)

for key in keyRow {
    width := widths.Has(key) ? widths[key] : 36
    btn := MainGui.Add("Button", "x" x " y" y " w" width " h36", key)
    x += width + 4
}

; 第六行（空格行）
y := 240
x := baseX
keyRow := ["Ctrl", "Win", "Alt", "Space", "RAlt", "RWin", "AppsKey", "RCtrl"]
widths := Map("Ctrl", 48, "Win", 48, "Alt", 48, "Space", 226, "RAlt", 48, "RWin", 48, "AppsKey", 48, "RCtrl", 48)

for key in keyRow {
    width := widths.Has(key) ? widths[key] : 36
    btn := MainGui.Add("Button", "x" x " y" y " w" width " h36", key)
    x += width + 4
}

; 创建方向键区域
baseX := 650  ; 方向键起始X坐标
y := 200  ; 方向键起始Y坐标

; 创建方向键
for rowIndex, row in directionKeys {
    x := baseX
    for key in row {
        if key != "" {
            btn := MainGui.Add("Button", "x" x " y" y " w36 h36", key)
            btn.OnEvent("Click", ButtonClick)
            buttons[key] := {
                button: btn,
                state: false
            }
        }
        x += 40
    }
    y += 40
}

; 创建小键盘区域
baseX := 750  ; 小键盘起始X坐标
y := 80  ; 小键盘起始Y坐标

; 创建小键盘
for rowIndex, row in numpadLayout {
    x := baseX
    for key in row {
        width := 36
        height := 36
        
        ; 特殊按键处理
        if (key = "Num Enter") {
            height := 76  ; Enter键高度为两倍
        } else if (key = "Num +") {
            height := 76  ; +号键高度为两倍
        } else if (key = "Num 0") {
            width := 76  ; 0键宽度为两倍
        }
        
        btn := MainGui.Add("Button", "x" x " y" y " w" width " h" height, key)
        btn.OnEvent("Click", ButtonClick)
        buttons[key] := {
            button: btn,
            state: false
        }
        
        if (key != "Num Enter" && key != "Num +") {  ; Enter和+号键不影响下一个按键的x坐标
            x += width + 4
        }
    }
    y += 40
}

; 调整窗口大小以适应键盘布局
MainGui.OnEvent('Size', GuiSize)

GuiSize(thisGui, MinMax, Width, Height) {
    if MinMax = -1    ; 窗口被最小化
        return
    
    ; 可以在这里添加其他调整逻辑
}

; 按钮点击事件处理函数
ButtonClick(btn, *) {
    key := btn.Text
    if buttons.Has(key) {
        buttons[key].state := !buttons[key].state
        btn.Opt(buttons[key].state ? "+Background00FF00" : "+Background0000FF")  ; 切换绿色/蓝色
    }
}

; 显示GUI
MainGui.Show()
