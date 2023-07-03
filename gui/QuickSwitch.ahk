Gui QuickSwitch:-MinimizeBox -MaximizeBox -SysMenu +AlwaysOnTop -Theme +0x800000
Gui QuickSwitch:Font, s18, 微软雅黑
Gui QuickSwitch:Add, ListBox, vQuickSwitchList x8 y8 w240 h292
Gui QuickSwitch:Font
Gui QuickSwitch:Add, Button, gQuickSwitchStart x8 y330 w100 h36 +Default, 切换并启动连发
Gui QuickSwitch:Add, Button, gQuickSwitchStop x150 y330 w100 h36, 停止连发
Gui QuickSwitch:Add, Text, x8 y300 w240 h30, 使用键盘上下键选择配置，按空格或回车快速切换，按ESC关闭窗口

QuickSwitchGuiEscape(){
    Gui QuickSwitch:Submit
}

QuickSwitchGuiClose(){
    Gui QuickSwitch:Submit
}

QuickSwitchStart(){
    global QuickSwitchList
    Gui QuickSwitch:Submit
    presetName := QuickSwitchList
    ChangePreset(presetName)
    StartAutoFire()
}

QuickSwitchStop(){
    Gui QuickSwitch:Submit
    StopAutoFire()
}

ShowGuiQuickSwitch(){
    HideGuiMain()
    Gui QuickSwitch:Show, w256 h370, 快速切换
    nowSelectPreset := GetNowSelectPreset()
    presetList := LoadAllPresetString()
    GuiControl QuickSwitch:, QuickSwitchList, |%presetList%
    GuiControl QuickSwitch:Focus, QuickSwitchList
    try{
        GuiControl QuickSwitch:ChooseString, QuickSwitchList, %nowSelectPreset%
    }catch{
        GuiControl QuickSwitch:Choose, QuickSwitchList, |1
    }
    OnMessage(0x0100, "QuickSwitchOnSpacePress")
}

HideGuiQuickSwitch(){
    Gui QuickSwitch:Hide
    OnMessage(0x0100, "")
}

QuickSwitchOnSpacePress(wParam, lParam){
    key := GetKeyName(Format("vk{1:02X}", wParam))
    if(key == "Space"){
        QuickSwitchStart()
    }
}