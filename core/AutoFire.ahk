AutoFire(key){
    Process, Priority,, High
    keyCode := Key2NoVkSC(key)
    keySC := Key2Sc(key)
    loop {
        if(WinActive("ahk_class 地下城与勇士") or WinActive("ahk_exe DNF.exe")) {
            while, GetKeyState(keySC, "P") {
                SendIP(keyCode)
            }
        }
        Sleep, 1
    }
}