#NoEnv

#SingleInstance, Ignore
#MaxHotkeysPerInterval, 9999
#InstallKeybdHook
SetWorkingDir, %A_ScriptDir%
SetBatchLines, -1
ListLines, Off

#Include <RunWithAdministrator>
#Include <MultipleThread>
#Include <Keys>
#Include <JSON>
#Include <Time>
#Include <GetPressKey>
#Include ./core/CheckDNFWindow.ahk
#Include ./core/KeyConvert.ahk
#Include ./core/Config.ahk
#Include ./core/AutoFire.ahk
#Include ./core/Scripts.ahk
#Include ./core/Http.ahk
#Include ./gui/Main.ahk
#Include ./gui/QuickSwitch.ahk
#Include ./gui/UpdateProgress.ahk
#Include ./gui/LvRen.ahk
#Include ./ex/ExLvRen.ahk
#Include ./gui/ZhanFa.ahk
#Include ./ex/ExZhanFa.ahk

try
{
    if(A_IsCompiled){
        Menu, Tray, Icon, %A_ScriptFullPath%, 1
        Menu, Tray, NoStandard
        Menu, Tray, DeleteAll
    } else{
        #Include <Log>
        Log()
    }
}

Menu, Tray, MainWindow
Menu, Tray, Tip, DAF连发工具
Menu, Tray , Add, 设置连发, ShowGui
Menu, Tray , Default, 设置连发
Menu, Tray , Add
Menu, Tray, Add, 退出连发,Exit

Exit(){
    ExitApp
}

ShowGui(){
    ShowGuiMain()
}

global __Version := "0.0.9"

global _AutoFireThreads := []
global _AutoFireEnableKeys := []
global _NowSelectPreset := LoadLastPreset()

ShowGui()
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

; GuiClose(){
;     HideGuiMain()
; }