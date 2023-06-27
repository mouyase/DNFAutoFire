#NoEnv

#SingleInstance, Ignore
#MaxHotkeysPerInterval, 9999
#InstallKeybdHook
SetWorkingDir, %A_ScriptDir%
SetBatchLines, -1
ListLines, Off
SendMode, Event
SetKeyDelay, -1, -1

#Include <RunWithAdministrator>
#Include <MultipleThread>
#Include <Keys>
#Include ./core/CheckDNFWindow.ahk
#Include ./core/Config.ahk
#Include ./core/AutoFire.ahk
#Include ./core/Scripts.ahk
#Include ./gui/Main.ahk
#Include ./gui/QuickSwitch.ahk

try
{
    if(A_IsCompiled){
        Menu, Tray, Icon, %A_ScriptFullPath%, 1
        Menu, Tray, NoStandard
        Menu, Tray, DeleteAll
    } else{
        ; #Include <Log>
        ; Log()
    }
}

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
global _NowSelectPreset := LoadLastPreset()

ShowGuiMain()
return

!`::
    ShowGuiQuickSwitch()
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

GuiEscape:
GuiClose:
    HideGuiMain()