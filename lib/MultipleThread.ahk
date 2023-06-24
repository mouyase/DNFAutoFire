class Thread
{
	__New(RunLabelOrFunc)
	{
		args:= "/Run=" RunLabelOrFunc
		if (A_IsCompiled)
			Run, "%A_ScriptFullPath%" /f "%args%",,, pid
		else
			Run, "%A_AhkPath%" /f "%A_ScriptFullPath%" "%args%",,, pid
		this.pid:=pid
	}
	__Delete()
{
	Process, Close, % this.pid
}
ScriptStart()
{
	static init:=Thread.ScriptStart()
#NoEnv
#NoTrayIcon
	if !InStr(A_Args[1], "/Run=")
	{
		Menu, Tray, Icon
		return
	}
	Suspend, On
	if IsLabel(k:=SubStr(A_Args[1], 6))
		gosub, %k%
	else if IsFunc(k)
		%k%()
	ExitApp
}
}