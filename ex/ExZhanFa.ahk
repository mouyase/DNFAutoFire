ExZhanFa(){
    Process, Priority,, High
    presetName := LoadLastPreset()
    if(LoadPreset(LoadLastPreset(),"ZhanFaState")){
        ShotKey := LoadPreset(LoadLastPreset(), "ZhanFaShotKey")
        SkillKeys := ZhanFaLoadKeys(LoadLastPreset())
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
                    SendIP(keycode)
                }
            }
            Sleep, 1
        }
    }
}

; 读取预设的连发按键
ZhanFaLoadKeys(presetName){
    skillKeysConfig := LoadPreset(presetName, "ZhanFaSkillKeys")
    keys := []
    loop, Parse, skillKeysConfig, `|
    {
        keys.Push(A_LoopField)
    }
    return keys
}