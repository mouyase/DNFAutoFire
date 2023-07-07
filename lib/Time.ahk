; 解除系统的Time限制，让Time可以小于10ms
UnlockSystemTimeLimit(){
    DllCall("Winmm\timeBeginPeriod", "UInt", 1)
}

; 恢复系统的Time限制
RestoreSystemTimeLimit(){
    DllCall("Winmm\timeEndPeriod", "UInt", 1)
}

; 休眠，解除限制后休眠时间可以小于10ms
GrandSleep(delay){
    DllCall("Sleep", "UInt", delay)
}

; 超级休眠，解除限制后可以休眠的时间非常短
SuperSleep(delay){
    lastTime := GetNewCounterTime()
    while (GetNewCounterTime() - lastTime < delay){
    }
}

; 获取一个新的计时器时间
GetNewCounterTime(){
    DllCall("Winmm\timeBeginPeriod", "UInt", 1)
    time := DllCall("Winmm\timeGetTime")
    DllCall("Winmm\timeEndPeriod", "UInt", 1)
    return time
}