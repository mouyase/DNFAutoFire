AutoFire(key){
    SetKeyDelay, 1
    Process, Priority,, High
    keycode := Key2NoVkSC(key)
    lastTimeCounter := GetNewCounterTime()
    loop {
        if(WinActive("ahk_class 地下城与勇士") or WinActive("ahk_exe DNF.exe")) {
            if (GetKeyState(key, "P")) {
                SendEvent, {Blind}{%keycode% DownTemp}{%keycode% up}
            } else {
                Sleep, 1
            }
        } else {
            Sleep, 1
        }
    }
}