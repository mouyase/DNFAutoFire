AutoFire(key){
    Process, Priority,, High
    SetDNFWindowClass()
    keyCode := Key2NoVkSC(key)
    keySC := Key2SC(key)
    loop {
        if(WinActive("ahk_group DNF")) {
            while, GetKeyState(keySC, "P") {
                SendIP(keyCode)
            }
        }
        Sleep, 1
    }
}