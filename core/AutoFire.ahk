AutoFire(key){
    SetKeyDelay, 0, 1
    Process, Priority,, High
    keycode := Key2NoVkSC(key)
    loop {
        if(WinActive("ahk_class 地下城与勇士") or WinActive("ahk_exe DNF.exe")) {
            if (GetKeyState(key, "P")) {
                SendEvent, {Blind}{%keycode%}
            }
        }
        Sleep, 1
    }
}