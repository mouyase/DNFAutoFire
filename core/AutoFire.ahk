AutoFire(key){
    Process, Priority,, High
    SetDNFWindowClass()
    keyCode := Key2NoVkSC(key)
    pressKey := Key2PressKey(key)
    loop {
        if(WinActive("ahk_group DNF")) {
            while, GetKeyState(pressKey, "P") {
                SendIP(keyCode)
            }
        }
        Sleep, 1
    }
}