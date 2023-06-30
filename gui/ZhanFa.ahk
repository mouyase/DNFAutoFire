Gui ZhanFa:-MinimizeBox -MaximizeBox -SysMenu -Theme +Owner%A_DefaultGui%
Gui ZhanFa:Add, ListBox, vZhanFaKeysListBox x8 y32 w80 h172
Gui ZhanFa:Add, Edit, vZhanFaShotKey x96 y120 w80 h20 +ReadOnly -WantCtrlA
Gui ZhanFa:Add, Button, gZhanFaAddKey x96 y40 w80 h22, 添加技能键
Gui ZhanFa:Add, Button, gZhanFaDeleteKey x96 y70 w80 h22, 删除技能键
Gui ZhanFa:Add, Button, gZhanFaSetShotKey x96 y148 w80 h22, 设置发射键
Gui ZhanFa:Add, Text, x8 y8 w80 h20 +0x200, 已添加键
Gui ZhanFa:Add, Text, x96 y100 w80 h20 +0x200, 发射键
Gui ZhanFa:Add, Button, gZhanFaSave x96 y178 w80 h27, 保存
Gui ZhanFa:Add, Button, gZhanFaHelp x158 y8 w18 h18, ?

ShowGuiZhanFa(){
    Gui ZhanFa:Show, w184 h210, 战法自动炫纹
    ZhanFaLoadConfig()
    DisableGuiMain()
}

HideGuiZhanFa(){
    Gui ZhanFa:Hide
    EnableGuiMain()
}

ZhanFaHelp(){
    MsgBox 0x2020, 你的数据很差, 你的数据很差，我现在玩战法每130s只要能射出300次炫纹，每次差不多34824％的等效百分比，就能有相当于10447200％的输出水平，换算过来狠狠地超越了精灵骑士的三觉数据。虽然我作为爆发职业没有一个技能超过3000000％，作为续航职业没有一个技能秒伤能超过90000％，但是我的炫纹已经超越了地下城绝大多数职业(包括你)的水平，这便是战斗法师给我的骄傲的资本。
}

global __ZhanFaSkillKeys := []

ZhanFaAddKey(){
    global __ZhanFaSkillKeys
    key := GetPressKey()
    if(IsValueInArray(key, __ZhanFaSkillKeys)){
        MsgBox 0x10, , 请勿重复添加按键
    }else{
        __ZhanFaSkillKeys.Push(key)
    }
    ZhanFaChangeListGui(__ZhanFaSkillKeys)
    GuiControl ZhanFa:ChooseString, ZhanFaKeysListBox, |%key%
}

ZhanFaDeleteKey(){
    global __ZhanFaSkillKeys
    global ZhanFaKeysListBox
    Gui ZhanFa:Submit, NoHide
    DeleteValueInArray(ZhanFaKeysListBox, __ZhanFaSkillKeys)
    ZhanFaChangeListGui(__ZhanFaSkillKeys)
}

ZhanFaSave(){
    ZhanFaSaveConfig()
    HideGuiZhanFa()
}

ZhanFaSetShotKey(){
    key := GetPressKey()
    GuiControl ZhanFa:, ZhanFaShotKey, %key%
}

; 战法功能模块修改列表
ZhanFaChangeListGui(keys){
    keysString := ""
    for k,v in keys
    {
        keysString := keysString . v . "|"
    }
    keysString := SubStr(keysString, 1, StrLen(keysString) - 1)
    GuiControl ZhanFa:, ZhanFaKeysListBox, |%keysString%
    GuiControl ZhanFa:Choose, ZhanFaKeysListBox, 1
}

; 战法功能模块保存配置
ZhanFaSaveConfig(){
    global __ZhanFaSkillKeys
    global ZhanFaShotKey
    Gui ZhanFa:Submit, NoHide
    keysString := ""
    for k,v in __ZhanFaSkillKeys
    {
        keysString := keysString . v . "|"
    }
    keysString := SubStr(keysString, 1, StrLen(keysString) - 1)
    SavePreset(GetNowSelectPreset(),"ZhanFaSkillKeys", keysString)
    SavePreset(GetNowSelectPreset(),"ZhanFaShotKey", ZhanFaShotKey)
}

; 战法功能模块读取配置
ZhanFaLoadConfig(){
    global __ZhanFaSkillKeys
    nowSelectPreset := GetNowSelectPreset()
    shotKey := LoadPreset(GetNowSelectPreset(), "ZhanFaShotKey")
    __ZhanFaSkillKeys := ZhanFaLoadKeys(GetNowSelectPreset())
    ZhanFaChangeListGui(__ZhanFaSkillKeys)
    GuiControl ZhanFa:, ZhanFaShotKey, %shotKey%
}