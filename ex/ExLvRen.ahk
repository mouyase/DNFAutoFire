ExLvRen(){
    Process, Priority,, High
    presetName := LoadLastPreset()
    if(LoadPreset(LoadLastPreset(),"LvRenState")){
        ShotKey := LoadPreset(LoadLastPreset(), "LvRenShotKey")
        SkillKeys := LvRenLoadKeys(LoadLastPreset())
        keyCode := Key2NoVkSC(ShotKey)
        keySCs := []
        for _, key in SkillKeys{
            keySCs.Push(Key2SC(key))
        }
        loop {
            if(WinActive("ahk_class 地下城与勇士") or WinActive("ahk_exe DNF.exe")) {
                isNeedSend := false
                for _, keySC in keySCs{
                    if (GetKeyState(keySC, "P")) {
                        isNeedSend := true
                        break
                    }
                }
                if (isNeedSend) {
                    Sleep, 1
                    SendIP(keyCode)
                }
            }
            Sleep, 1
        }
    }
}

; 读取预设的连发按键
LvRenLoadKeys(presetName){
    skillKeysConfig := LoadPreset(presetName, "LvRenSkillKeys")
    keys := []
    loop, Parse, skillKeysConfig, `|
    {
        keys.Push(A_LoopField)
    }
    return keys
}
