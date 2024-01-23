ExGuanYu(){
    Process, Priority,, High
    SetDNFWindowClass()
    presetName := LoadLastPreset()
    if(LoadPreset(LoadLastPreset(),"GuanYuState")){
        ShotKey := LoadPreset(LoadLastPreset(), "GuanYuShotKey")
        SkillKeys := GuanYuLoadKeys(LoadLastPreset())
        keyCode := Key2NoVkSC(ShotKey)
        delay := LoadPreset(LoadLastPreset(), "GuanYuDelay")
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
                    Sleep, delay
                    SendIP(keyCode)
                }
            }
            Sleep, 1
        }
    }
}

; 读取预设的连发按键
GuanYuLoadKeys(presetName){
    skillKeysConfig := LoadPreset(presetName, "GuanYuSkillKeys")
    keys := []
    loop, Parse, skillKeysConfig, `|
    {
        keys.Push(A_LoopField)
    }
    return keys
}