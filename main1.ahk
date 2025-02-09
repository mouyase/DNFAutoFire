#Requires AutoHotkey v2.0
#SingleInstance Force

; 定义按键的扫描码
KEY_J := 0x24
KEY_L := 0x26
KEY_H := 0x23
KEY_P := 0x19

; 创建共享内存
hMapFile := DllCall("CreateFileMapping", "Ptr", -1, "Ptr", 0, "UInt", 0x04, "UInt", 0, "UInt", 4096, "Str", "DAFSharedMem")
if !hMapFile {
    MsgBox "Could not create file mapping"
    ExitApp
}

; 映射共享内存视图
pBuf := DllCall("MapViewOfFile", "Ptr", hMapFile, "UInt", 0x0002, "UInt", 0, "UInt", 0, "UInt", 4096, "Ptr")
if !pBuf {
    DllCall("CloseHandle", "Ptr", hMapFile)
    MsgBox "Could not map view of file"
    ExitApp
}

; 记录进程对象
global g_processes := Map()

; 检查是否是DNF窗口
IsDNFWindow() {
    return WinActive("ahk_exe dnf.exe")
}

; 启动所有按键处理进程
InitKeyProcesses() {
    g_processes["j"] := Run(Format('AutoHotkey.exe "keyfire.ahk" {1} "j" 0', KEY_J), , "Hide")
    g_processes["l"] := Run(Format('AutoHotkey.exe "keyfire.ahk" {1} "l" 1', KEY_L), , "Hide")
    g_processes["h"] := Run(Format('AutoHotkey.exe "keyfire.ahk" {1} "h" 2', KEY_H), , "Hide")
    g_processes["p"] := Run(Format('AutoHotkey.exe "keyfire.ahk" {1} "p" 3', KEY_P), , "Hide")
}

; 设置按键状态
SetKeyState(offset, state) {
    DllCall("RtlFillMemory", "Ptr", pBuf + offset, "UInt", 1, "UChar", state)
}

; 处理按键按下
HandleKeyDown(key, offset) {
    if IsDNFWindow() {
        SetKeyState(offset, 1)
        return true
    }
    return false
}

; 处理按键抬起
HandleKeyUp(key, offset) {
    SetKeyState(offset, 0)
}

; 注册退出回调
OnExit(*) {
    ; 清理所有进程
    for key, pid in g_processes {
        if pid {
            ProcessClose(pid)
        }
    }
    ; 清理共享内存
    DllCall("UnmapViewOfFile", "Ptr", pBuf)
    DllCall("CloseHandle", "Ptr", hMapFile)
}

; 初始化进程
InitKeyProcesses()

; 注册热键
j::{
    if !HandleKeyDown("j", 0) {
        Send("{j down}")
    }
}

j up::{
    HandleKeyUp("j", 0)
    if !IsDNFWindow() {
        Send("{j up}")
    }
}

l::{
    if !HandleKeyDown("l", 1) {
        Send("{l down}")
    }
}

l up::{
    HandleKeyUp("l", 1)
    if !IsDNFWindow() {
        Send("{l up}")
    }
}

h::{
    if !HandleKeyDown("h", 2) {
        Send("{h down}")
    }
}

h up::{
    HandleKeyUp("h", 2)
    if !IsDNFWindow() {
        Send("{h up}")
    }
}

p::{
    if !HandleKeyDown("p", 3) {
        Send("{p down}")
    }
}

p up::{
    HandleKeyUp("p", 3)
    if !IsDNFWindow() {
        Send("{p up}")
    }
}

