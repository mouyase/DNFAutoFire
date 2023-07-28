ExJianZong(){
    Process, Priority,, High
    SetDNFWindowClass()
    presetName := LoadLastPreset()
    if(LoadPreset(LoadLastPreset(),"JianZongState")){
        skillKey := LoadPreset(LoadLastPreset(), "JianZongSkillKey")
        delay := LoadPreset(LoadLastPreset(), "JianZongDelay")
        keyCode := Key2NoVkSC(skillKey)
        pressKey := Key2PressKey(key)
        counterTime := 0
        time := A_TickCount
        loop {
            if(WinActive("ahk_group DNF")) {
                while, GetKeyState(pressKey, "P"){
                    counterTime := A_TickCount - time
                    if(counterTime > delay){
                        SendIP(keyCode)
                    }
                }
                counterTime := 0
                time := A_TickCount
            }
            Sleep, 1
        }
    }
}