Gui JianZong:+ToolWindow +Owner%A_DefaultGui%
Gui JianZong:Add, Edit, vJianZongSkillKey x8 y70 w80 h20 +ReadOnly
Gui JianZong:Add, Button, gJianZongSetSkillKey x8 y94 w80 h22, 设置按键
Gui JianZong:Add, Text, x8 y48 w80 h20 +0x200, 帝国剑术键
Gui JianZong:Add, Button, gJianZongSave x8 y118 w80 h26, 保存
Gui JianZong:Add, Button, gJianZongHelp x94 y8 w18 h18, ?
Gui JianZong:Add, Text, x8 y8 w80 h20 +0x200, 延迟时间(ms)
Gui JianZong:Add, Edit, vJianZongDelay x8 y28 w80 h20 +Number

ShowGuiJianZong(){
    Gui JianZong:Show, w120 h148, 太宗帝剑延迟
    JianZongLoadConfig()
    DisableGuiMain()
}

HideGuiJianZong(){
    Gui JianZong:Hide
    EnableGuiMain()
}


JianZongGuiEscape(){
    HideGuiJianZong()
}

JianZongGuiClose(){
    HideGuiJianZong()
}

JianZongHelp(){
    MsgBox 0x2020, 如何使用太宗帝剑延迟, 1、设置游戏中帝国剑术的技能按键`n2、设置帝国剑术第一刀后的延迟时间，单位毫秒键`n3、保存配置，启动连发并使用`n`nPS：该按键不能打开连发，否则功能失效
}

JianZongSave(){
    JianZongSaveConfig()
    HideGuiJianZong()
}

JianZongSetSkillKey(){
    key := GetPressKey()
    GuiControl JianZong:, JianZongSkillKey, %key%
}

; 剑宗功能模块保存配置
JianZongSaveConfig(){
    global JianZongSkillKey
    global JianZongDelay
    Gui JianZong:Submit, NoHide
    SavePreset(GetNowSelectPreset(), "JianZongSkillKey", JianZongSkillKey)
    SavePreset(GetNowSelectPreset(), "JianZongDelay", JianZongDelay)
}

; 剑宗功能模块读取配置
JianZongLoadConfig(){
    nowSelectPreset := GetNowSelectPreset()
    skillKey := LoadPreset(GetNowSelectPreset(), "JianZongSkillKey")
    GuiControl JianZong:, JianZongSkillKey, %skillKey%
    delay := LoadPreset(GetNowSelectPreset(), "JianZongDelay")
    GuiControl JianZong:, JianZongDelay, %delay%
}