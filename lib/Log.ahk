; 显示控制台并输出内容（默认自动添加换行）
log(ByRef str:="", rn:=1)
{
  if !DllCall("GetConsoleWindow")
  {
    IfWinExist, ahk_class ConsoleWindowClass
      WinGet, pid, PID
    else
    {
      Run, cmd.exe /k "echo off",,, pid
      WinWait, ahk_pid %pid%
    }
    DllCall("AttachConsole", "Int",pid)
  }
  FileAppend, % str (rn ? "`r`n":""), *
}

; 关闭控制台（手动关闭会退出AHK程序）
logoff()
{
  DllCall("FreeConsole")
}
