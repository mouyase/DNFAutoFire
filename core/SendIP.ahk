SendIP(keyCode){
    SendInput, {Blind}{%keyCode% DownTemp}
    Sleep, 1
    SendInput, {Blind}{%keyCode% Up}
    Sleep, 1
}