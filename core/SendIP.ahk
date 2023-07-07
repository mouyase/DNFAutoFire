SendIP(keycode){
    SendInput, {Blind}{%keycode% DownTemp}
    Sleep, 1
    SendInput, {Blind}{%keycode% Up}
    Sleep, 1
}