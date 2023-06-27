Gui QuickSwitch:-MinimizeBox -MaximizeBox -SysMenu +AlwaysOnTop -Theme +0x800000
Gui QuickSwitch:Font, s18, 微软雅黑
Gui QuickSwitch:Add, ListBox, vQuickSwitchList x8 y8 w240 h292
Gui QuickSwitch:Font
Gui QuickSwitch:Add, Button, gQuickSwitchStart x8 y304 w100 h48 +Default, 切换并启动连发
Gui QuickSwitch:Add, Button, gQuickSwitchStop x150 y304 w100 h48, 停止连发

QuickSwitchStart(){
    global QuickSwitchList
    global _NowSelectPreset
    Gui QuickSwitch:Submit
    presetName := QuickSwitchList
    _NowSelectPreset := presetName
    ConfigKeys := LoadPresetKeys(presetName)
    SetAllKeysAutoFire(ConfigKeys)
    StartAutoFire()
}

QuickSwitchStop(){
    Gui QuickSwitch:Submit
    StopAutoFire()
}

ShowGuiQuickSwitch(){
    global _NowSelectPreset
    Gui QuickSwitch:Show, w256 h360, 快速切换
    presetList := LoadAllPresetString()
    GuiControl QuickSwitch:, QuickSwitchList, |%presetList%
    GuiControl QuickSwitch:Focus, QuickSwitchList
    try{
        GuiControl QuickSwitch:ChooseString, QuickSwitchList, %_NowSelectPreset%
    }catch{
        GuiControl QuickSwitch:Choose, QuickSwitchList, |1
    }
}

HideGuiQuickSwitch(){
    Gui QuickSwitch:Hide
}