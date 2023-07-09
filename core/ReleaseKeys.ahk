ReleaseKeys(){
    SetKeyDelay, -1, 1
    Process, Priority,, High
    loop {
        if (GetKeyState("Shift") && !GetKeyState("Shift", "P")) {
            SendInput, {Blind}{LShift Up}
            SendInput, {Blind}{RShift Up}
        }
        if (GetKeyState("Ctrl") && !GetKeyState("Ctrl", "P")) {
            SendInput, {Blind}{LCtrl Up}
            SendInput, {Blind}{RCtrl Up}
        }
        if (GetKeyState("Alt") && !GetKeyState("Alt", "P")) {
            SendInput, {Blind}{LAlt Up}
            SendInput, {Blind}{RAlt Up}
        }
        Sleep, 1
    }
}