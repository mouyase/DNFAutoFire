ExLvRen(){
    Process, Priority,, High
    SetDNFWindowClass()
    presetName := LoadLastPreset()
    if(LoadPreset(LoadLastPreset(),"LvRenState")){
        ShotKey := LoadPreset(LoadLastPreset(), "LvRenShotKey")
        SkillKeys := LvRenLoadKeys(LoadLastPreset())
        keyCode := Key2NoVkSC(ShotKey)
        pressKeys := []
        for _, key in SkillKeys{
            pressKeys.Push(Key2PressKey(key))
        }
        loop {
            if(WinActive("ahk_group DNF")) {
                isNeedSend := false
                for _, pressKey in pressKeys{
                    if (GetKeyState(pressKey, "P")) {
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
