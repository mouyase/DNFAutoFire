ExLvRen(){
    presetName := LoadLastPreset()
    if(LoadPreset(LoadLastPreset(),"LvRenState")){
        SetKeyDelay, -1, 1
        Process, Priority,, High
        ShotKey := LoadPreset(LoadLastPreset(), "LvRenShotKey")
        SkillKeys := LvRenLoadKeys(LoadLastPreset())
        keycode := Key2NoVkSC(ShotKey)
        loop {
            if(WinActive("ahk_class 地下城与勇士") or WinActive("ahk_exe DNF.exe")) {
                isNeedSend := false
                for _, key in SkillKeys{
                    if (GetKeyState(key, "P")) {
                        isNeedSend := true
                        break
                    }
                }
                if (isNeedSend) {
                    Sleep, 1
                    SendEvent, {Blind}{%keycode%}
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