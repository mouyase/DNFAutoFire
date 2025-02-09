#Requires AutoHotkey v2.0
#SingleInstance Off

; 隐藏托盘图标
A_IconHidden := true

; 获取命令行参数
scanCode := A_Args[1]
keyName := A_Args[2]
offset := Integer(A_Args[3])

; 打开共享内存
hMapFile := DllCall("OpenFileMapping", "UInt", 0x0002, "Int", 0, "Str", "DAFSharedMem", "Ptr")
if !hMapFile {
    ExitApp
}

; 映射共享内存视图
pBuf := DllCall("MapViewOfFile", "Ptr", hMapFile, "UInt", 0x0002, "UInt", 0, "UInt", 0, "UInt", 4096, "Ptr")
if !pBuf {
    DllCall("CloseHandle", "Ptr", hMapFile)
    ExitApp
}

; 发送按键按下
SendKeyDown(sc) {
    SendInput("{VKFFsc" Format("{:03X}", sc) " down}")
}

; 发送按键抬起
SendKeyUp(sc) {
    SendInput("{VKFFsc" Format("{:03X}", sc) " up}")
}

; 检查是否是DNF窗口
IsDNFWindow() {
    return WinActive("ahk_exe dnf.exe")
}

; 检查按键状态
IsKeyPressed() {
    return NumGet(pBuf + offset, "UChar")
}

; 按键连发循环
while true {
    if IsKeyPressed() && IsDNFWindow() {
        SendKeyDown(scanCode)
        DllCall("Sleep", "UInt", 2)
        SendKeyUp(scanCode)
        DllCall("Sleep", "UInt", 2)
    } else {
        Sleep(50)  ; 非激活状态时降低CPU占用
    }
}

; 清理共享内存
OnExit(*) {
    DllCall("UnmapViewOfFile", "Ptr", pBuf)
    DllCall("CloseHandle", "Ptr", hMapFile)
} 