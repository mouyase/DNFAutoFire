Gui GuanYu:+ToolWindow +Owner%A_DefaultGui%
Gui GuanYu:Add, ListBox, vGuanYuKeysListBox x8 y32 w80 h210
Gui GuanYu:Add, Edit, vGuanYuShotKey x96 y120 w80 h20 +ReadOnly -WantCtrlA 
Gui GuanYu:Add, Button, gGuanYuAddKey x96 y40 w80 h22, 添加技能键
Gui GuanYu:Add, Button, gGuanYuDeleteKey x96 y70 w80 h22, 删除技能键
Gui GuanYu:Add, Button, gGuanYuSetShotKey x96 y149 w80 h22, 设置啪啪啪键
Gui GuanYu:Add, Text, x8 y8 w80 h20 +0x200  , 已添加技能键
Gui GuanYu:Add, Text, x96 y100 w80 h20 +0x200  , 啪啪啪键
Gui GuanYu:Add, Button, gGuanYuSave x97 y234 w80 h27, 保存
Gui GuanYu:Add, Button, gGuanYuHelp x158 y8 w18 h18, ?
Gui GuanYu:Add, Text, x96 y182 w80 h20 +0x200  , 啪啪啪延迟
Gui GuanYu:Add, Edit, vGuanYuDelay x96 y202 w80 h20 +Number

ShowGuiGuanYu(){
    Gui GuanYu:Show, w184 h270, 关羽自动啪啪啪
    GuanYuLoadConfig()
    DisableGuiMain()
}

HideGuiGuanYu(){
    Gui GuanYu:Hide
    EnableGuiMain()
}

GuanYuGuiEscape(){
    HideGuiGuanYu()
}

GuanYuGuiClose(){
    HideGuiGuanYu()
}

GuanYuHelp(){
    MsgBox 0x2020, 本功能基于战法炫纹功能做的修改，在原功能基础上增加delay参数，来实现关羽的自动战戟猛攻，防止空格键加速报废，最后感谢某亚瑟的代码贡献。
}

global __GuanYuSkillKeys := []

GuanYuAddKey(){
    global __GuanYuSkillKeys
    key := GetPressKey()
    if(IsValueInArray(key, __GuanYuSkillKeys)){
        MsgBox 0x10, , 请勿重复添加按键
    }else{
        __GuanYuSkillKeys.Push(key)
    }
    GuanYuChangeListGui(__GuanYuSkillKeys)
    GuiControl GuanYu:ChooseString, GuanYuKeysListBox, |%key%
}

GuanYuDeleteKey(){
    global __GuanYuSkillKeys
    global GuanYuKeysListBox
    Gui GuanYu:Submit, NoHide
    DeleteValueInArray(GuanYuKeysListBox, __GuanYuSkillKeys)
    GuanYuChangeListGui(__GuanYuSkillKeys)
}

GuanYuSave(){
    GuanYuSaveConfig()
    HideGuiGuanYu()
}

GuanYuSetShotKey(){
    key := GetPressKey()
    GuiControl GuanYu:, GuanYuShotKey, %key%
}

; 关羽功能模块修改列表
GuanYuChangeListGui(keys){
    keysString := ""
    for k,v in keys
    {
        keysString := keysString . v . "|"
    }
    keysString := SubStr(keysString, 1, StrLen(keysString) - 1)
    GuiControl GuanYu:, GuanYuKeysListBox, |%keysString%
    GuiControl GuanYu:Choose, GuanYuKeysListBox, 1
}

; 关羽功能模块保存配置
GuanYuSaveConfig(){
    global __GuanYuSkillKeys
    global GuanYuShotKey
    global GuanYuDelay
    Gui GuanYu:Submit, NoHide
    keysString := ""
    for k,v in __GuanYuSkillKeys
    {
        keysString := keysString . v . "|"
    }
    keysString := SubStr(keysString, 1, StrLen(keysString) - 1)
    SavePreset(GetNowSelectPreset(),"GuanYuSkillKeys", keysString)
    SavePreset(GetNowSelectPreset(),"GuanYuShotKey", GuanYuShotKey)
    SavePreset(GetNowSelectPreset(),"GuanYuDelay", GuanYuDelay)
}

; 关羽功能模块读取配置
GuanYuLoadConfig(){
    global __GuanYuSkillKeys
    nowSelectPreset := GetNowSelectPreset()
    shotKey := LoadPreset(GetNowSelectPreset(), "GuanYuShotKey", "Space")
    __GuanYuSkillKeys := GuanYuLoadKeys(GetNowSelectPreset())
    GuanYuChangeListGui(__GuanYuSkillKeys)
    GuiControl GuanYu:, GuanYuShotKey, %shotKey%
    
    delay := LoadPreset(GetNowSelectPreset(),"GuanYuDelay")
    GuiControl GuanYu:, GuanYuDelay, %delay%
}