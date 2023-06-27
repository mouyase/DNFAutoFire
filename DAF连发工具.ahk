#NoEnv

#SingleInstance, Ignore
#MaxHotkeysPerInterval 9999
#InstallKeybdHook
SetWorkingDir, %A_ScriptDir%
SetBatchLines, -1
ListLines, Off
SendMode, Event
SetKeyDelay, -1, -1


#Include <RunWithAdministrator>
#Include <MultipleThread>
#Include <Keys>
#Include ./gui/GuiMain.ahk
#Include ./core/Config.ahk
#Include ./core/AutoFire.ahk
#Include ./core/Scripts.ahk
; #Include <Log>

#If WinActive("ahk_class 地下城与勇士") or WinActive("ahk_exe DNF.exe")

try
{
    Menu, Tray, Icon, %A_ScriptFullPath%, 1
}
; Menu, Tray, NoStandard
; Menu, Tray, DeleteAll
Menu, Tray, MainWindow
Menu, Tray, Tip, DAF连发工具
Menu, Tray , Add, 设置连发, ShowGUI
Menu, Tray , Default, 设置连发
Menu, Tray , Add
Menu, Tray, Add, 退出连发,Exit

Exit(){
    ExitApp
}

ShowGUI(){
    ShowGuiMain()
}

global __Version := "0.0.7"

global _AutoFireThreads := []
global _AutoFireEnableKeys := []
global _NowSelectPreset := ""

ShowGuiMain()

ClearAllKeys(){
    global _AutoFireEnableKeys
    for _, key in _AutoFireEnableKeys
    {
        SetOriginalDirect(key)
        SetGuiKeyState(key, false)
    }
    _AutoFireEnableKeys := []
    try
    {
        Menu, Tray, Icon, %A_ScriptFullPath%, 1
    }
}

SetAllKeys(keys){
    for _, key in keys
    {
        SetGuiKeyState(key, true)
        SetAutoFireKey(key)
    }
}

SavePreset(){
    global PresetName
    global _AutoFireEnableKeys
    Gui, Submit, NoHide
    if(PresetName != "")
    {
        SavePresetConfig(PresetName, _AutoFireEnableKeys)
    }
    else
    {
        MsgBox 0x2010, , 请输入配置名称
    }
    LoadPresetGUI()
    GuiControl, ChooseString, Preset, |%PresetName%
}

LoadPreset(){
    global Preset
    global PresetName
    global _AutoFireThreads
    Gui, Submit, NoHide
    GuiControl, , PresetName, %Preset%
    ClearAllKeys()
    ConfigKeys := LoadPresetConfig(Preset)
    SetAllKeys(ConfigKeys)
    _AutoFireThreads := []
}

DeletePreset(){
    global Preset
    global PresetName
    Gui, Submit, NoHide
    DeletePresetConfig(Preset)
    LoadPresetGUI()
    LoadDefaultPreset()
    GuiControl, Choose, Preset, |1
}

LoadPresetGUI(){
    global _NowSelectPreset
    presetList := LoadPresetList()
    GuiControl, , Preset, |%presetList%
    try
    {
        GuiControl, ChooseString, Preset, |%_NowSelectPreset%
    }
    catch
    {
        GuiControl, Choose, Preset, |1
    }
}

LoadDefaultPreset(){
    presetName := GetDefaultPresetName()
    GuiControl, , PresetName, %presetName%
    ClearAllKeys()
    ConfigKeys := LoadPresetConfig(presetName)
    SetAllKeys(ConfigKeys)
    GuiControl, Choose, Preset, |1
}

LoadPresetGUI()
LoadDefaultPreset()

; OnMessage(0x0006, "WM_ACTIVATE")

Gui ChangePreset:-MinimizeBox -MaximizeBox -SysMenu +AlwaysOnTop -Theme +0x800000
Gui ChangePreset:Font, s18, 微软雅黑
Gui ChangePreset:Add, ListBox, vChangePreset x10 y5 w200 h172
Gui ChangePreset:Font
Gui ChangePreset:Add, Button, x10 y185 w200 h86 +default, 启动连发

; KeyHistory
; ListHotkeys
; Log()

; IniRead, config, config.ini

; MsgBox, %config%
return

; WM_ACTIVATE(wParam)
; {
; ToolTip %wParam%
; }

!`::
    Gui ChangePreset:Show, w950 h510, aa

return

; 以下内容为对修饰键的单独处理，可以降低修饰键触发延迟
<+F24::
>+F24::
<^F24::
>^F24::
<!F24::
>!F24::
return
$LShift::
$RShift::
$LAlt::
$RAlt::
$LCtrl::
$RCtrl::
return