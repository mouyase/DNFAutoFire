Gui YuanDiAttack:+ToolWindow +Owner%A_DefaultGui%
Gui YuanDiAttack:Add, Text, x8 y8 w80 h20 +0x200, 平X键
Gui YuanDiAttack:Add, Edit, vYuanDiAttackSkillKey x8 y32 w80 h20 +ReadOnly
Gui YuanDiAttack:Add, Button, gYuanDiAttackSetSkillKey x8 y56 w80 h22, 设置按键
Gui YuanDiAttack:Add, Text, x96 y8 w80 h20 +0x200, 方向左键
Gui YuanDiAttack:Add, Edit, vYuanDiAttackLeftKey x96 y32 w80 h20 +ReadOnly
Gui YuanDiAttack:Add, Button, gYuanDiAttackSetLeftKey x96 y56 w80 h22, 设置按键
Gui YuanDiAttack:Add, Text, x184 y8 w80 h20 +0x200, 方向右键
Gui YuanDiAttack:Add, Edit, vYuanDiAttackRightKey x184 y32 w80 h20 +ReadOnly
Gui YuanDiAttack:Add, Button, gYuanDiAttackSetRightKey x184 y56 w80 h22, 设置按键
; Gui YuanDiAttack:Add, Button, gYuanDiAttackHelp x94 y8 w18 h18, ?
Gui YuanDiAttack:Add, Text, x8 y80 w80 h20 +0x200, 按方向延迟
Gui YuanDiAttack:Add, Edit, vYuanDiAttackDelay1 x8 y104 w80 h20 +Number
Gui YuanDiAttack:Add, Text, x96 y80 w80 h20 +0x200, 抬方向延迟
Gui YuanDiAttack:Add, Edit, vYuanDiAttackDelay2 x96 y104 w80 h20 +Number
Gui YuanDiAttack:Add, Text, x184 y80 w80 h20 +0x200, 每轮时长
Gui YuanDiAttack:Add, Edit, vYuanDiAttackDelay3 x184 y104 w80 h20 +Number
Gui YuanDiAttack:Add, Button, gYuanDiAttackGetTime x8 y128 w80 h26, 获取延迟时间
Gui YuanDiAttack:Add, Button, gYuanDiAttackSave x184 y128 w80 h26, 保存

ShowGuiYuanDiAttack(){
    Gui YuanDiAttack:Show, w272 h162, 原地平X
    YuanDiAttackLoadConfig()
    DisableGuiMain()
}

HideGuiYuanDiAttack(){
    try{
        Hotkey, $1, Off
        Hotkey, $2, Off
    }
    Gui YuanDiAttack:Hide
    EnableGuiMain()
}

YuanDiAttackGuiEscape(){
    HideGuiYuanDiAttack()
}

YuanDiAttackGuiClose(){
    HideGuiYuanDiAttack()
}

YuanDiAttackHelp(){
    MsgBox 0x2020, 如何使用太宗帝剑延迟, 1、设置游戏中帝国剑术的技能按键`n2、设置帝国剑术第一刀后的延迟时间，单位毫秒键`n3、保存配置，启动连发并使用`n`nPS：该按键不能打开连发，否则功能失效
}

YuanDiAttackSave(){
    YuanDiAttackSaveConfig()
    HideGuiYuanDiAttack()
}

YuanDiAttackSetSkillKey(){
    key := GetPressKey()
    GuiControl YuanDiAttack:, YuanDiAttackSkillKey, %key%
}

YuanDiAttackSetLeftKey(){
    key := GetPressKey()
    GuiControl YuanDiAttack:, YuanDiAttackLeftKey, %key%
}

YuanDiAttackSetRightKey(){
    key := GetPressKey()
    GuiControl YuanDiAttack:, YuanDiAttackRightKey, %key%
}
; 原地平X功能模块保存配置
YuanDiAttackSaveConfig(){
    global YuanDiAttackSkillKey
    global YuanDiAttackLeftKey
    global YuanDiAttackRightKey
    global YuanDiAttackDelay1
    global YuanDiAttackDelay2
    global YuanDiAttackDelay3
    Gui YuanDiAttack:Submit, NoHide
    SavePreset(GetNowSelectPreset(), "YuanDiAttackSkillKey", YuanDiAttackSkillKey)
    SavePreset(GetNowSelectPreset(), "YuanDiAttackLeftKey", YuanDiAttackLeftKey)
    SavePreset(GetNowSelectPreset(), "YuanDiAttackRightKey", YuanDiAttackRightKey)
    SavePreset(GetNowSelectPreset(), "YuanDiAttackDelay1", YuanDiAttackDelay1)
    SavePreset(GetNowSelectPreset(), "YuanDiAttackDelay2", YuanDiAttackDelay2)
    SavePreset(GetNowSelectPreset(), "YuanDiAttackDelay3", YuanDiAttackDelay3)
}

; 原地平X功能模块读取配置
YuanDiAttackLoadConfig(){
    nowSelectPreset := GetNowSelectPreset()
    skillKey := LoadPreset(GetNowSelectPreset(), "YuanDiAttackSkillKey")
    GuiControl YuanDiAttack:, YuanDiAttackSkillKey, %skillKey%
    leftKey := LoadPreset(GetNowSelectPreset(), "YuanDiAttackLeftKey")
    GuiControl YuanDiAttack:, YuanDiAttackLeftKey, %leftKey%
    rightKey := LoadPreset(GetNowSelectPreset(), "YuanDiAttackRightKey")
    GuiControl YuanDiAttack:, YuanDiAttackRightKey, %rightKey%
    delay1 := LoadPreset(GetNowSelectPreset(), "YuanDiAttackDelay1")
    GuiControl YuanDiAttack:, YuanDiAttackDelay1, %delay1%
    delay2 := LoadPreset(GetNowSelectPreset(), "YuanDiAttackDelay2")
    GuiControl YuanDiAttack:, YuanDiAttackDelay2, %delay2%
    delay3 := LoadPreset(GetNowSelectPreset(), "YuanDiAttackDelay3")
    GuiControl YuanDiAttack:, YuanDiAttackDelay3, %delay3%
}

YuanDiAttackGetTime(){
    global YuanDiAttackSkillKey
    Gui YuanDiAttack:Submit, NoHide
    fn1 := Func("ExYuanDiAttackGetDelayTime")
    Hotkey, $1, %fn1%, On
    fn2 := Func("ExYuanDiAttackGetLoopTime")
    Hotkey, $2, %fn2%, On
}