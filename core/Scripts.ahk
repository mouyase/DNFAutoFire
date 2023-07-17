; 切换按键连发状态
ChangeKeyAutoFireState(key){
    global _AutoFireEnableKeys
    if(IsKeyAutoFire(key)){
        needDeleteIndex := 0
        for index, element in _AutoFireEnableKeys
        {
            if(element == key){
                needDeleteIndex := index
            }
        }
        _AutoFireEnableKeys.Delete(needDeleteIndex)
        MainSetKeyState(key, false)
        SetOriginalDirect(key)
    } else {
        _AutoFireEnableKeys.Push(key)
        MainSetKeyState(key, true)
        SetOriginalBlocking(key)
    }
}

; 判断按键是否启用了连发
IsKeyAutoFire(key){
    global _AutoFireEnableKeys
    for _, element in _AutoFireEnableKeys
    {
        if(element == key){
            return true
        }
    }
    return false
}

; 把Gui上的key名转换为真实的键值
GetOriginKeyName(key){
    switch key {
    Case "Sub":
        keyName := "-"
    Case "Add":
        keyName := "="
    Case "Tilde":
        keyName := "``"
    Case "LeftBracket":
        keyName := "["
    Case "RightBracket":
        keyName := "]"
    Case "Backslash":
        keyName := "\"
    Case "Semicolon":
        keyName := ";"
    Case "Caps":
        keyName := "CapsLock"
    Case "QuotationMark":
        keyName := "'"
    Case "Comma":
        keyName := ","
    Case "Period":
        keyName := "."
    Case "Slash":
        keyName := "/"
    Case "PrtSc":
        keyName := "PrintScreen"
    Case "ScrLk":
        keyName := "ScrollLock"
    Case "Ins":
        keyName := "Insert"
    Case "Del":
        keyName := "Delete"
    Case "Num1":
        keyName := "Numpad1"
    Case "Num2":
        keyName := "Numpad2"
    Case "Num3":
        keyName := "Numpad3"
    Case "Num4":
        keyName := "Numpad4"
    Case "Num5":
        keyName := "Numpad5"
    Case "Num6":
        keyName := "Numpad6"
    Case "Num7":
        keyName := "Numpad7"
    Case "Num8":
        keyName := "Numpad8"
    Case "Num8":
        keyName := "Numpad8"
    Case "Num9":
        keyName := "Numpad9"
    Case "Num0":
        keyName := "Numpad0"
    Case "NumPeriod":
        keyName := "NumpadDot"
    Case "NumLk":
        keyName := "NumLock"
    Case "NumEnter":
        keyName := "NumpadEnter"
    Case "NumAdd":
        keyName := "NumpadAdd"
    Case "NumSub":
        keyName := "NumpadSub"
    Case "NumStar":
        keyName := "NumpadMult"
    Case "NumSlash":
        keyName := "NumpadDiv"
    Default:
        keyName := key
    }
    return keyName
}

; 用于屏蔽按键原始功能
OriginalBlocking(key){
    SendInput, {Blind}{%key% DownTemp}
    Sleep, 1
    KeyWait, %key%
    SendInput, {Blind}{%key% Up}
}

; 屏蔽按键原始功能
SetOriginalBlocking(key){
    keyName := GetOriginKeyName(key)
    keySC := Key2SC(keyName)
    fn := Func("OriginalBlocking").Bind(Format("{:L}", keySC))
    try{
        Hotkey, $*%keySC%, %fn%, On
    }
}

; 恢复按键原始功能
SetOriginalDirect(key){
    keyName := GetOriginKeyName(key)
    keyName := Key2SC(keyName)
    try{
        Hotkey, $*%keyName%, Off
    }
}

; 设置托盘图标状态
SetTrayRunningIcon(state){
    /*@Ahk2Exe-Keep
        if(state){
            Menu, Tray, Icon, %A_ScriptFullPath%, 3
        }else{
            Menu, Tray, Icon, %A_ScriptFullPath%, 4
        }
    */
}

; 启动连发功能
StartAutoFire(){
    global _AutoFireEnableKeys
    global _AutoFireThreads
    _AutoFireThreads := []
    for _, key in _AutoFireEnableKeys {
        SetOriginalBlocking(key)
        _AutoFireThreads.Push(new Thread(key))
    }
    Sleep, 10
    _AutoFireThreads.Push(new Thread("ReleaseKeys"))
    StartEx()
    SoundPlay *64
    SetTrayRunningIcon(true)
    nowSelectPreset := GetNowSelectPreset()
    ShowTip("连发已启动 - " . nowSelectPreset)
}

StartEx(){
    global _AutoFireThreads
    global YuanDiAttack
    global LvRen
    global ZhanFa
    global JianZong
    if(YuanDiAttack){
        skillKey := LoadPreset(GetNowSelectPreset(), "YuanDiAttackSkillKey")
        SetOriginalBlocking(skillKey)
        _AutoFireThreads.Push(new Thread("ExYuanDiAttack"))
    }
    if(LvRen){
        _AutoFireThreads.Push(new Thread("ExLvRen"))
    }
    if(ZhanFa){
        _AutoFireThreads.Push(new Thread("ExZhanFa"))
    }
    if(JianZong){
        skillKey := LoadPreset(GetNowSelectPreset(), "JianZongSkillKey")
        SetOriginalBlocking(skillKey)
        _AutoFireThreads.Push(new Thread("ExJianZong"))
    }
}

; 停止连发功能
StopAutoFire(){
    global _AutoFireThreads
    allKeys := GetAllKeys()
    for _, key in allKeys {
        SetOriginalDirect(key)
    }
    _AutoFireThreads := []
    SetTrayRunningIcon(false)
}

; 设置所有关闭连发
SetAllKeysDisable(){
    global _AutoFireEnableKeys
    allKeys := GetAllKeys()
    for _, key in allKeys {
        MainSetKeyState(key, false)
    }
    _AutoFireEnableKeys := []
}

; 设置所有按键开启连发
SetAllKeysAutoFire(keys){
    global _AutoFireEnableKeys
    SetAllKeysDisable()
    for _, key in keys {
        MainSetKeyState(key, true)
        _AutoFireEnableKeys.Push(key)
    }
}

; 设置当前选择预设名
SetNowSelectPreset(presetName){
    global _NowSelectPreset
    _NowSelectPreset := presetName
}

; 获取当前选择预设名
GetNowSelectPreset(){
    global _NowSelectPreset
    return _NowSelectPreset
}

; 切换预设
ChangePreset(presetName){
    StopAutoFire()
    presetKeys := LoadPresetKeys(presetName)
    SetAllKeysAutoFire(presetKeys)
    SetNowSelectPreset(presetName)
    SaveLastPreset(presetName)
    MainLoadEx()
}

; 判断数组中是否存在某值
IsValueInArray(value, array){
    for _, element in array
    {
        if(element == value){
            return true
        }
    }
    return false
}

; 删除数组中的某值
DeleteValueInArray(value, array){
    if(IsValueInArray(value, array)){
        needDeleteIndex := 0
        for k, v in array
        {
            if(v == value){
                needDeleteIndex := k
            }
        }
        array.Delete(needDeleteIndex)
    }
}

ShowTip(text){
    ToolTip, %text%
    SetTimer, CloseTip, -3000
    WinActivate, ahk_group DNF
}

CloseTip(){
    ToolTip,
}