ExJianZong(){
    Process, Priority,, High
    presetName := LoadLastPreset()
    if(LoadPreset(LoadLastPreset(),"JianZongState")){
        skillKey := LoadPreset(LoadLastPreset(), "JianZongSkillKey")
        delay := LoadPreset(LoadLastPreset(), "JianZongDelay")
        keyCode := Key2NoVkSC(skillKey)
        keySC := Key2NoVkSC(skillKey)
        counterTime := 0
        time := A_TickCount
        loop {
            if(WinActive("ahk_class 地下城与勇士") or WinActive("ahk_exe DNF.exe")) {
                while, GetKeyState(keySC, "P"){
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