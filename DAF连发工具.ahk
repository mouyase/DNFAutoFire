;@Ahk2Exe-SetMainIcon icon_main.ico
;@Ahk2Exe-AddResource icon_alert.ico, 160
;@Ahk2Exe-AddResource icon_green.ico, 206
;@Ahk2Exe-AddResource icon_red.ico, 207

#NoEnv

#Persistent
#MenuMaskKey, vkFF
#SingleInstance, Ignore
#MaxHotkeysPerInterval, 9999
#InstallKeybdHook
#MaxThreadsBuffer, Off
SetWorkingDir, %A_ScriptDir%
SetBatchLines, -1
ListLines, Off
SetStoreCapslockMode, Off
SetKeyDelay, 1

global __Version := "0.0.11"

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
#Include ./core/ReleaseKeys.ahk
#Include ./gui/Main.ahk
#Include ./gui/QuickSwitch.ahk
#Include ./gui/UpdateProgress.ahk
#Include ./gui/LvRen.ahk
#Include ./ex/ExLvRen.ahk
#Include ./gui/ZhanFa.ahk
#Include ./ex/ExZhanFa.ahk
#Include ./gui/JianZong.ahk
#Include ./ex/ExJianZong.ahk

;@Ahk2Exe-IgnoreBegin
#Include <Log>
Log()
;@Ahk2Exe-IgnoreEnd

/*@Ahk2Exe-Keep
    Menu, Tray, Icon, %A_ScriptFullPath%, 4
    Menu, Tray, NoStandard
    Menu, Tray, DeleteAll
*/

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

global _AutoFireThreads := []
global _AutoFireEnableKeys := []
global _NowSelectPreset := LoadLastPreset()

ShowGui()
return

!`::
    ShowGuiQuickSwitch()
return

$LWin::
$RWin::
return