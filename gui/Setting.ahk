Gui Setting:-MinimizeBox -MaximizeBox -Theme +Owner%A_DefaultGui%
Gui Setting:Add, Tab3, x0 y0 w400 h300, 通用设置|帮助说明|关于
Gui Setting:Tab, 通用设置
Gui Setting:Add, CheckBox, vSettingAutoStart x16 y32 h20, 软件打开后自动启动连发
Gui Setting:Add, CheckBox, vSettingOnSystemStart x16 y54 h20, 开机后自动启动
Gui Setting:Add, CheckBox, vSettingBlockWin x16 y76 h20, 游戏内屏蔽Win键
Gui Setting:Add, Button, gSettingSave x310 y250 w80 h40, 保存
Gui Setting:Tab, 帮助说明
Gui Setting:Add, Text, x16 y32 w368 h268, 如何使用DAF连发工具`n`n1、点击窗口中的键盘，将想启动连发的键位变成红色`n2、输入配置名称，保存配置`n3、点击启动连发，即可使用`n`nPS：在游戏中可以打开快速切换窗口，使用上下键和回车可以快速切换已经保存的配置，记得设置快捷键哦`n默认快捷键 Alt + `` （键盘1左边，Tab上边的那个）
Gui Setting:Tab, 关于
Gui Setting:Add, Link, x16 y32 w368 h268, 作者： 某亚瑟`n图标： Ousumu`n`n源码：<a href="https://github.com/mouyase/DNFAutoFire">https://github.com/mouyase/DNFAutoFire</a>

SettingGuiEscape(){
    HideGuiSetting()
}

SettingGuiClose(){
    HideGuiSetting()
}

ShowGuiSetting(){
    DisableGuiMain()
    Gui Setting:Show, w400 h300, 软件设置
    SettingLoad()
}

HideGuiSetting(){
    Gui Setting:Hide
    EnableGuiMain()
}

SettingSave(){
    global SettingAutoStart
    global SettingOnSystemStart
    global SettingBlockWin
    global _OnSystemStart
    global _BlockWin
    Gui Setting:Submit, NoHide
    SaveConfig("SettingAutoStart", SettingAutoStart)
    SaveConfig("SettingOnSystemStart", SettingOnSystemStart)
    SaveConfig("SettingBlockWin", SettingBlockWin)

    global _OnSystemStart := SettingOnSystemStart
    global _BlockWin := SettingBlockWin

    SettingNow()
    HideGuiSetting()
}

SettingLoad(){
    settingAutoStart := LoadConfig("SettingAutoStart", false)
    settingOnSystemStart := LoadConfig("SettingOnSystemStart", false)
    settingBlockWin := LoadConfig("SettingBlockWin", false)
    GuiControl Setting:, SettingAutoStart, %settingAutoStart%
    GuiControl Setting:, SettingOnSystemStart, %settingOnSystemStart%
    GuiControl Setting:, SettingBlockWin, %settingBlockWin%
}

SettingNow(){
    if(_OnSystemStart){
        FileCreateShortcut, %A_ScriptFullPath%, %A_Startup%\DAF连发工具.lnk
    }else{
        FileDelete, %A_Startup%\DAF连发工具.lnk
    }
    if(_BlockWin){
        fn := Func("BlockWin")
        Hotkey, $*LWin, %fn%, On
        Hotkey, $*RWin, %fn%, On
    }else{
        try {
            Hotkey, $*LWin, Off
            Hotkey, $*RWin, Off
        }
    }
}

global _AutoStart := LoadConfig("SettingAutoStart", false)
global _OnSystemStart := LoadConfig("SettingOnSystemStart", false)
global _BlockWin := LoadConfig("SettingBlockWin", false)

if(_BlockWin){
    fn := Func("")
    Hotkey, $*LWin, %fn%, On
    Hotkey, $*RWin, %fn%, On
}
