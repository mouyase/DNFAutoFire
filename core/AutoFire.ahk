AutoFire(key){
    Process, Priority,, High
    keycode := Key2NoVkSC(key)
    loop {
        if(WinActive("ahk_class 地下城与勇士") or WinActive("ahk_exe DNF.exe")) {
            while, GetKeyState(key, "P") {
                SendIP(keycode)
            }
        }
        Sleep, 1
    }
}