;@Ahk2Exe-SetMainIcon icon_main.ico
;@Ahk2Exe-AddResource icon_alert.ico, 160
;@Ahk2Exe-AddResource icon_green.ico, 206
;@Ahk2Exe-AddResource icon_red.ico, 207

;@Ahk2Exe-SetDescription DAF连发工具
;@Ahk2Exe-SetCopyright 某亚瑟
;@Ahk2Exe-SetLanguage 0x0804
;@Ahk2Exe-SetProductName DAF连发工具
;@Ahk2Exe-SetProductVersion 0.1.0
;@Ahk2Exe-SetVersion 0.1.0

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

global __Version := "0.1.2Fix1"

#Include <RunWithAdministrator>
#Include <MultipleThread>
#Include <Keys>
#Include <JSON>
#Include <Time>
#Include <GetPressKey>
#Include ./core/SendIP.ahk
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
#Include ./gui/Setting.ahk
#Include ./gui/ex/LvRen.ahk
#Include ./ex/ExLvRen.ahk
#Include ./gui/ex/ZhanFa.ahk
#Include ./ex/ExZhanFa.ahk
#Include ./gui/ex/JianZong.ahk
#Include ./ex/ExJianZong.ahk
#Include ./gui/ex/YuanDiAttack.ahk
#Include ./ex/ExYuanDiAttack.ahk

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
Menu, Tray , Add, 连发设置, ShowGuiMain
Menu, Tray , Add, 软件设置, ShowGuiSetting
Menu, Tray , Default, 连发设置
Menu, Tray , Add
Menu, Tray, Add, 退出连发,Exit

Exit(){
    ExitApp
}

global _AutoFireThreads := []
global _AutoFireEnableKeys := []
global _NowSelectPreset := LoadLastPreset()

ShowGuiMain()
if(_AutoStart){
    Gui Main:Hide
    StartAutoFire()
}

return