Gui GetPressKeyBox:-MinimizeBox -MaximizeBox -SysMenu -Caption +Owner +0x800000
Gui GetPressKeyBox:Font, s18, 微软雅黑
Gui GetPressKeyBox:Add, Text, vPressKey x0 y0 w200 h200 +0x200 +Center, 请按下按键
Gui GetPressKeyBox:Font

GetPressKey(){
    Gui GetPressKeyBox:Show,w200 h200
    key := GetUserInputKey(),
    GuiControl GetPressKeyBox:, PressKey, %key%
    Sleep, 500
    Gui GetPressKeyBox:Hide
    GuiControl GetPressKeyBox:, PressKey, 请按下按键
    return key
}

GetUserInputKey(){
    ih := InputHook("L0")
    ih.KeyOpt("{All}", "E")
    ih.KeyOpt("{LWin}{RWin}{AppsKey}", "-E")
    ih.Start()
    ih.Wait()
    key := ih.EndKey
    if(StrLen(key) == 1){
        key := Format("{:U}",key)
    }
    return key
}