ReleaseKeys(){
    SetKeyDelay, -1, 1
    Process, Priority,, High
    loop {
        if (GetKeyState("Shift") && !GetKeyState("Shift", "P")) {
            SendEvent, {Blind}{LShift Up}
            SendEvent, {Blind}{RShift Up}
        }
        if (GetKeyState("Ctrl") && !GetKeyState("Ctrl", "P")) {
            SendEvent, {Blind}{LCtrl Up}
            SendEvent, {Blind}{RCtrl Up}
        }
        if (GetKeyState("Alt") && !GetKeyState("Alt", "P")) {
            SendEvent, {Blind}{LAlt Up}
            SendEvent, {Blind}{RAlt Up}
        }
        Sleep, 1
    }
}