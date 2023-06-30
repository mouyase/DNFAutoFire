ExJianZong(){
    presetName := LoadLastPreset()
    if(LoadPreset(LoadLastPreset(),"JianZongState")){
        SetKeyDelay, 1
        Process, Priority,, High
        skillKey := LoadPreset(LoadLastPreset(), "JianZongSkillKey")
        delay := LoadPreset(LoadLastPreset(), "JianZongDelay")
        keycode := Key2NoVkSC(skillKey)
        time := GetNewCounterTime()
        counterTime := 0
        loop {
            if(WinActive("ahk_class 地下城与勇士") or WinActive("ahk_exe DNF.exe")) {
                if (GetKeyState(skillKey, "P")) {
                    if(counterTime > delay){
                        SendEvent, {Blind}{%keycode% DownTemp}{%keycode% up}
                    }
                    counterTime := GetNewCounterTime() - time
                } else {
                    counterTime := 0
                    time := GetNewCounterTime()
                    Sleep, 1
                }
            } else {
                Sleep, 1
            }
        }
    }
}