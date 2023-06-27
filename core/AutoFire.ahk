AutoFire(key){
    base := "vkFFsc"
    sc := GetKeySC(key)
    keycode := Format("{1}{2:02X}", base, sc)
    loop {
        if(WinActive("ahk_class 地下城与勇士") or WinActive("ahk_exe DNF.exe"))
        {
            if (GetKeyState(key, "P"))
            {
                Send, {Blind}{%keycode% down}{%keycode% up}
            }
        }
        Sleep, 1
    }
}