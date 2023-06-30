Gui LvRen:-MinimizeBox -MaximizeBox -SysMenu -Theme +Owner%A_DefaultGui%
Gui LvRen:Add, ListBox, vLvRenKeysListBox x8 y32 w80 h172
Gui LvRen:Add, Edit, vLvRenShotKey x96 y120 w80 h20 +ReadOnly -WantCtrlA
Gui LvRen:Add, Button, gLvRenAddKey x96 y40 w80 h22, 添加技能键
Gui LvRen:Add, Button, gLvRenDeleteKey x96 y70 w80 h22, 删除技能键
Gui LvRen:Add, Button, gLvRenSetShotKey x96 y148 w80 h22, 设置发射键
Gui LvRen:Add, Text, x8 y8 w80 h20 +0x200, 已添加键
Gui LvRen:Add, Text, x96 y100 w80 h20 +0x200, 发射键
Gui LvRen:Add, Button, gLvRenSave x96 y178 w80 h27, 保存
Gui LvRen:Add, Button, gLvRenHelp x158 y8 w18 h18, ?

ShowGuiLvRen(){
    Gui LvRen:Show, w184 h210, 旅人自动流星
    LvRenLoadConfig()
    DisableGuiMain()
}

HideGuiLvRen(){
    Gui LvRen:Hide
    EnableGuiMain()
}

LvRenHelp(){
    MsgBox 0x2020, 如何使用旅人自动流星, 1、添加你想要发射流星的技能键`n2、设置游戏中流星的发射键（默认为Z）`n3、保存配置，启动连发并使用`n`nPS：建议和连发功能一起打开，效果更好
}

global __LvRenSkillKeys := []

LvRenAddKey(){
    global __LvRenSkillKeys
    key := GetPressKey()
    if(IsValueInArray(key, __LvRenSkillKeys)){
        MsgBox 0x10, , 请勿重复添加按键
    }else{
        __LvRenSkillKeys.Push(key)
    }
    LvRenChangeListGui(__LvRenSkillKeys)
    GuiControl LvRen:ChooseString, LvRenKeysListBox, |%key%
}

LvRenDeleteKey(){
    global __LvRenSkillKeys
    global LvRenKeysListBox
    Gui LvRen:Submit, NoHide
    DeleteValueInArray(LvRenKeysListBox, __LvRenSkillKeys)
    LvRenChangeListGui(__LvRenSkillKeys)
}

LvRenSave(){
    LvRenSaveConfig()
    HideGuiLvRen()
}

LvRenSetShotKey(){
    key := GetPressKey()
    GuiControl LvRen:, LvRenShotKey, %key%
}

; 旅人功能模块修改列表
LvRenChangeListGui(keys){
    keysString := ""
    for k,v in keys
    {
        keysString := keysString . v . "|"
    }
    keysString := SubStr(keysString, 1, StrLen(keysString) - 1)
    GuiControl LvRen:, LvRenKeysListBox, |%keysString%
    GuiControl LvRen:Choose, LvRenKeysListBox, 1
}

; 旅人功能模块保存配置
LvRenSaveConfig(){
    global __LvRenSkillKeys
    global LvRenShotKey
    Gui LvRen:Submit, NoHide
    keysString := ""
    for k,v in __LvRenSkillKeys
    {
        keysString := keysString . v . "|"
    }
    keysString := SubStr(keysString, 1, StrLen(keysString) - 1)
    SavePreset(GetNowSelectPreset(),"LvRenSkillKeys", keysString)
    SavePreset(GetNowSelectPreset(),"LvRenShotKey", LvRenShotKey)
}

; 旅人功能模块读取配置
LvRenLoadConfig(){
    global __LvRenSkillKeys
    nowSelectPreset := GetNowSelectPreset()
    shotKey := LoadPreset(GetNowSelectPreset(), "LvRenShotKey")
    __LvRenSkillKeys := LvRenLoadKeys(GetNowSelectPreset())
    LvRenChangeListGui(__LvRenSkillKeys)
    GuiControl LvRen:, LvRenShotKey, %shotKey%
}