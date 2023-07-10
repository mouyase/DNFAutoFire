; 保存软件设置
SaveConfig(type, value){
    IniWrite, %value%, config.ini, 设置, %type%
}

; 读取软件设置
LoadConfig(type, default := ""){
    if(default == ""){
        default := A_Space
    }
    IniRead, value, config.ini, 设置, %type%, %default%
    return value
}

; 删除软件设置
DeleteConfig(type){
    IniDelete, config.ini, 设置, %type%
}

; 保存预设
SavePreset(presetsName, type, value){
    presetsName := StrReplace(presetsName, "`|")
    IniWrite, %value%, config.ini, 预设:%presetsName%, %type%
}

; 读取预设
LoadPreset(presetsName, type, default := ""){
    if(default == ""){
        default := A_Space
    }
    presetsName := StrReplace(presetsName, "`|")
    IniRead, value, config.ini, 预设:%presetsName%, %type%, %default%
    return value
}

; 删除预设
DeletePreset(presetsName){
    presetsName := StrReplace(presetsName, "`|")
    IniDelete, config.ini, 预设:%presetsName%
}

; 保存预设的连发按键
SavePresetKeys(presetsName, keys){
    keysString := ""
    for k,v in keys
    {
        keysString := keysString . v . "|"
    }
    keysString := SubStr(keysString, 1, StrLen(keysString) - 1)
    SavePreset(presetsName, "keys", keysString)
}

; 读取预设的连发按键
LoadPresetKeys(presetsName){
    config := LoadPreset(presetsName, "keys")
    keys := []
    loop, Parse, config, `|
    {
        keys.Push(A_LoopField)
    }
    return keys
}

; 读取所有预设
LoadAllPreset(){
    IniRead, config, config.ini
    presetList := []
    loop, Parse, config, `n, `r
    {
        if(A_LoopField != "设置" && A_LoopField != "预设:"){
            presetName := StrReplace(A_LoopField, "预设:")
            presetList.Push(presetName)
        }
    }
    return presetList
}

; 以字符的方式读取所有预设
LoadAllPresetString(){
    presetList := LoadAllPreset()
    presetListStr := ""
    for _, value in presetList {
        presetListStr := presetListStr . value . "|"
    }
    presetListStr := SubStr(presetListStr, 1, StrLen(presetListStr) - 1)
    return presetListStr
}

; 保存上次的预设
SaveLastPreset(presetName){
    SaveConfig("LastPreset", presetName)
}

; 读取上次的预设
LoadLastPreset(){
    return LoadConfig("LastPreset")
}