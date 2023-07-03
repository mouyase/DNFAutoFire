AutoFire(key){
    SetKeyDelay, 0, 1
    Process, Priority,, High
    keycode := Key2NoVkSC(key)
    keySC := Key2SC(key)
    time := GetNewCounterTime()
    loop {
        if(WinActive("ahk_class 地下城与勇士") or WinActive("ahk_exe DNF.exe")) {
            if (GetKeyState(keySC, "P")) {
                SendEvent, {Blind}{%keycode%}
                counterTime := GetNewCounterTime() - time
                time := GetNewCounterTime()
            }
        }
        Sleep, 1
    }
}