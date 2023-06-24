;DNF连发: v0.0.1
;作者：某亚瑟


#NoEnv

#SingleInstance,Force
#MaxHotkeysPerInterval 9999
#InstallKeybdHook
#Include <MultipleThread>
#Include <AutoFire>
#Include <Config>
SetWorkingDir %A_ScriptDir%
SetBatchLines,-1
ListLines Off
SendMode,Event
SetKeyDelay,0,-1
CoordMode, ToolTip, Screen

full_command_line := DllCall("GetCommandLine", "str")

if not (A_IsAdmin or RegExMatch(full_command_line, " /restart(?!\S)"))
{
	try
	{
		if A_IsCompiled
			Run *RunAs "%A_ScriptFullPath%" /restart
		else
			Run *RunAs "%A_AhkPath%" /restart "%A_ScriptFullPath%"
	}
	ExitApp
}

Menu, Tray, Icon, icon.ico
Menu,Tray,NoStandard
Menu,Tray,DeleteAll
Menu,Tray,Add,设置连发,ShowGUI
Menu,Tray,Add,关闭连发,Exit

Exit(){
	ExitApp
}

ShowGUI(){
    Gui, Show
}

global _ThreadArray := []
global EnableKeys := []

isKeyEnable(key){
	global EnableKeys
	for _, element in EnableKeys
	{
		if(element == key){
		return true
	}
}
return false
}

setKeyEnable(key){
	global EnableKeys
	if(isKeyEnable(key)){
	needDeleteIndex := 0
	for index, element in EnableKeys
	{
		if(element == key){
		needDeleteIndex := index
	}
}
EnableKeys.Delete(needDeleteIndex)
SetGUIKeyEnable(key, false)
}
else
{
	EnableKeys.Push(key)
	SetGUIKeyEnable(key, true)
}

}

KeyClick() {
	setKeyEnable(A_GuiControl)
}

ClearAllKeys(){
	global EnableKeys
	for _, key in EnableKeys
	{
		SetGUIKeyEnable(key, false)
	}
	EnableKeys := []
}

SetAllKeys(keys){
	global EnableKeys
	for _, key in keys
	{
		SetGUIKeyEnable(key, true)
	}
	EnableKeys := keys
}

SetGUIKeyEnable(key, status){
	color := "cRed"
	size := "s12"
	weight := "Bold"
	if(!status)
	{
		weight := "Norm"
		color := "cBlue"
	}
if(key == "PrtSc" || key == "ScrLk" || key == "Pause" || key == "NumEnter" || key == "NumLk")
{
	size := "s7"
}
if(key == "Ins" || key == "Home" || key == "PgUp" || key == "Del" || key == "End" || key == "PgDn" || key == "Num1" || key == "Num2" || key == "Num3" || key == "Num4" || key == "Num5" || key == "Num6" || key == "Num7" || key == "Num8" || key == "Num9")
{
	size := "s9"
}
Gui, Font,%size% %color% %weight%
GuiControl, Font, %key%
}

SavePreset(){
	global PresetName
	global EnableKeys
	Gui, Submit, NoHide
	if(PresetName != "")
	{
		SavePresetConfig(PresetName, EnableKeys)
	}
else
{
	MsgBox 0x2010, , 请输入配置名称
}
LoadPresetGUI()
GuiControl, ChooseString, Preset, |%PresetName%
}

LoadPreset(){
	global Preset
	global EnableKeys
	global PresetName
	Gui, Submit, NoHide
	GuiControl, , PresetName, %Preset%
	ClearAllKeys()
	EnableKeys := LoadPresetConfig(Preset)
	SetAllKeys(EnableKeys)
}

DeletePreset(){
	global Preset
	global PresetName
	Gui, Submit, NoHide
	DeletePresetConfig(Preset)
	LoadPresetGUI()
	LoadDefaultPreset()
	GuiControl, Choose, Preset, |1
}

LoadPresetGUI(){
	presetList := LoadPresetList()
	GuiControl, , Preset, |%presetList%
}

StartAutoFire(){
	global EnableKeys
	global _ThreadArray
	_ThreadArray := []
	for _, key in EnableKeys
	{
		_ThreadArray.Insert(new Thread(key))
	}
	ToolTip, 连发开启, 0, 0
}

LoadDefaultPreset(){
	presetName := GetDefaultPresetName()
	GuiControl, , PresetName, %presetName%
	ClearAllKeys()
	EnableKeys := LoadPresetConfig(presetName)
	SetAllKeys(EnableKeys)
	GuiControl, Choose, Preset, 1
}

Gui -MinimizeBox -MaximizeBox -Theme
Gui Add, GroupBox, x8 y8 w936 h276, 按键设置 - [ 红色为启用连发 蓝色为关闭连发 ]
Gui Font, s12 cBlue
Gui Add, Text, vEsc gKeyClick x16 y30 w36 h36 +0x200 +0x400000 +Center , Esc
Gui Add, Text, vF1 gKeyClick x100 y30 w36 h36 +0x200 +0x400000 +Center , F1
Gui Add, Text, vF2 gKeyClick x140 y30 w36 h36 +0x200 +0x400000 +Center , F2
Gui Add, Text, vF3 gKeyClick x180 y30 w36 h36 +0x200 +0x400000 +Center , F3
Gui Add, Text, vF4 gKeyClick x220 y30 w36 h36 +0x200 +0x400000 +Center , F4
Gui Add, Text, vF5 gKeyClick x280 y30 w36 h36 +0x200 +0x400000 +Center , F5
Gui Add, Text, vF6 gKeyClick x320 y30 w36 h36 +0x200 +0x400000 +Center , F6
Gui Add, Text, vF7 gKeyClick x360 y30 w36 h36 +0x200 +0x400000 +Center , F7
Gui Add, Text, vF8 gKeyClick x400 y30 w36 h36 +0x200 +0x400000 +Center , F8
Gui Add, Text, vF9 gKeyClick x460 y30 w36 h36 +0x200 +0x400000 +Center , F9
Gui Add, Text, vF10 gKeyClick x500 y30 w36 h36 +0x200 +0x400000 +Center , F10
Gui Add, Text, vF11 gKeyClick x540 y30 w36 h36 +0x200 +0x400000 +Center , F11
Gui Add, Text, vF12 gKeyClick x580 y30 w36 h36 +0x200 +0x400000 +Center , F12

Gui Add, Text, vTilde gKeyClick x16 y80 w36 h36 +0x200 +0x400000 +Center, ``
Gui Add, Text, v1 gKeyClick x60 y80 w36 h36 +0x200 +0x400000 +Center, 1
Gui Add, Text, v2 gKeyClick x104 y80 w36 h36 +0x200 +0x400000 +Center, 2
Gui Add, Text, v3 gKeyClick x148 y80 w36 h36 +0x200 +0x400000 +Center, 3
Gui Add, Text, v4 gKeyClick x192 y80 w36 h36 +0x200 +0x400000 +Center, 4
Gui Add, Text, v5 gKeyClick x236 y80 w36 h36 +0x200 +0x400000 +Center, 5
Gui Add, Text, v6 gKeyClick x280 y80 w36 h36 +0x200 +0x400000 +Center, 6
Gui Add, Text, v7 gKeyClick x324 y80 w36 h36 +0x200 +0x400000 +Center, 7
Gui Add, Text, v8 gKeyClick x368 y80 w36 h36 +0x200 +0x400000 +Center, 8
Gui Add, Text, v9 gKeyClick x412 y80 w36 h36 +0x200 +0x400000 +Center, 9
Gui Add, Text, vSub gKeyClick x456 y80 w36 h36 +0x200 +0x400000 +Center, -
Gui Add, Text, vAdd gKeyClick x500 y80 w36 h36 +0x200 +0x400000 +Center, +
Gui Add, Text, vBackspace gKeyClick x544 y80 w72 h36 +0x200 +0x400000 +Center, ←

Gui Add, Text, vTab gKeyClick x16 y120 w50 h36 +0x200 +0x400000 +Center, Tab
Gui Add, Text, vQ gKeyClick x73 y120 w36 h36 +0x200 +0x400000 +Center, Q
Gui Add, Text, vW gKeyClick x114 y120 w36 h36 +0x200 +0x400000 +Center, W
Gui Add, Text, vE gKeyClick x155 y120 w36 h36 +0x200 +0x400000 +Center, E
Gui Add, Text, vR gKeyClick x196 y120 w36 h36 +0x200 +0x400000 +Center, R
Gui Add, Text, vT gKeyClick x237 y120 w36 h36 +0x200 +0x400000 +Center, T
Gui Add, Text, vY gKeyClick x278 y120 w36 h36 +0x200 +0x400000 +Center, Y
Gui Add, Text, vU gKeyClick x319 y120 w36 h36 +0x200 +0x400000 +Center, U
Gui Add, Text, vI gKeyClick x360 y120 w36 h36 +0x200 +0x400000 +Center, I
Gui Add, Text, vO gKeyClick x401 y120 w36 h36 +0x200 +0x400000 +Center, O
Gui Add, Text, vP gKeyClick x442 y120 w36 h36 +0x200 +0x400000 +Center, P
Gui Add, Text, vLeftBracket gKeyClick x483 y120 w36 h36 +0x200 +0x400000 +Center, `[
Gui Add, Text, vRightBracket gKeyClick x524 y120 w36 h36 +0x200 +0x400000 +Center, `]
Gui Add, Text, vBackslash gKeyClick x565 y120 w51 h36 +0x200 +0x400000 +Center, `\

Gui Add, Text, vCaps gKeyClick x16 y160 w64 h36 +0x200 +0x400000 +Center, Caps
Gui Add, Text, vA gKeyClick x85 y160 w36 h36 +0x200 +0x400000 +Center, A
Gui Add, Text, vS gKeyClick x126 y160 w36 h36 +0x200 +0x400000 +Center, S
Gui Add, Text, vD gKeyClick x167 y160 w36 h36 +0x200 +0x400000 +Center, D
Gui Add, Text, vF gKeyClick x208 y160 w36 h36 +0x200 +0x400000 +Center, F
Gui Add, Text, vG gKeyClick x249 y160 w36 h36 +0x200 +0x400000 +Center, G
Gui Add, Text, vH gKeyClick x290 y160 w36 h36 +0x200 +0x400000 +Center, H
Gui Add, Text, vJ gKeyClick x331 y160 w36 h36 +0x200 +0x400000 +Center, J
Gui Add, Text, vK gKeyClick x372 y160 w36 h36 +0x200 +0x400000 +Center, K
Gui Add, Text, vL gKeyClick x413 y160 w36 h36 +0x200 +0x400000 +Center, L
Gui Add, Text, vSemicolon gKeyClick x454 y160 w36 h36 +0x200 +0x400000 +Center, `;
Gui Add, Text, vQuotationMark gKeyClick x495 y160 w36 h36 +0x200 +0x400000 +Center, `'
Gui Add, Text, vEnter gKeyClick x536 y160 w80 h36 +0x200 +0x400000 +Center, Enter

Gui Add, Text, vLShift gKeyClick x16 y200 w85 h36 +0x200 +0x400000 +Center, Shift
Gui Add, Text, vZ gKeyClick x106 y200 w36 h36 +0x200 +0x400000 +Center, Z
Gui Add, Text, vX gKeyClick x147 y200 w36 h36 +0x200 +0x400000 +Center, X
Gui Add, Text, vC gKeyClick x188 y200 w36 h36 +0x200 +0x400000 +Center, C
Gui Add, Text, vV gKeyClick x229 y200 w36 h36 +0x200 +0x400000 +Center, V
Gui Add, Text, vB gKeyClick x270 y200 w36 h36 +0x200 +0x400000 +Center, B
Gui Add, Text, vN gKeyClick x311 y200 w36 h36 +0x200 +0x400000 +Center, N
Gui Add, Text, vM gKeyClick x352 y200 w36 h36 +0x200 +0x400000 +Center, M
Gui Add, Text, vComma gKeyClick x393 y200 w36 h36 +0x200 +0x400000 +Center, `,
Gui Add, Text, vPeriod gKeyClick x434 y200 w36 h36 +0x200 +0x400000 +Center, `.
Gui Add, Text, vSlash gKeyClick x475 y200 w36 h36 +0x200 +0x400000 +Center, `/
Gui Add, Text, vRShift gKeyClick x516 y200 w100 h36 +0x200 +0x400000 +Center, Shift

Gui Add, Text, vLCtrl gKeyClick x16 y240 w48 h36 +0x200 +0x400000 +Center, Ctrl
Gui Add, Text, x70 y240 w48 h36 +0x200 +0x400000 +Center  +Disabled, Win
Gui Add, Text, vLAlt gKeyClick x124 y240 w48 h36 +0x200 +0x400000 +Center, Alt
Gui Add, Text, vSpace gKeyClick x178 y240 w224 h36 +0x200 +0x400000 +Center, Space
Gui Add, Text, vAlt gKeyClick x408 y240 w48 h36 +0x200 +0x400000 +Center, Alt
Gui Add, Text, x462 y240 w48 h36 +0x200 +0x400000 +Center +Disabled, Fn
Gui Add, Text, x516 y240 w48 h36 +0x200 +0x400000 +Center +Disabled, List
Gui Add, Text, vRCtrl gKeyClick x568 y240 w48 h36 +0x200 +0x400000 +Center, Ctrl
Gui Add, Text, vUp gKeyClick x680 y200 w36 h36 +0x200 +0x400000 +Center, ↑
Gui Add, Text, vLeft gKeyClick x640 y240 w36 h36 +0x200 +0x400000 +Center, ←
Gui Add, Text, vDown gKeyClick x680 y240 w36 h36 +0x200 +0x400000 +Center, ↓
Gui Add, Text, vRight gKeyClick x720 y240 w36 h36 +0x200 +0x400000 +Center, →
Gui Add, Text, vNum0 gKeyClick x780 y240 w76 h36 +0x200 +0x400000 +Center, Num0
Gui Add, Text, vNumPeriod gKeyClick x860 y240 w36 h36 +0x200 +0x400000 +Center, .
Gui Add, Text, vNumSlash gKeyClick x820 y80 w36 h36 +0x200 +0x400000 +Center, /
Gui Add, Text, vNumStar gKeyClick x860 y80 w36 h36 +0x200 +0x400000 +Center, *
Gui Add, Text, vNumSub gKeyClick x900 y80 w36 h36 +0x200 +0x400000 +Center, -
Gui Add, Text, vNumAdd gKeyClick x900 y120 w36 h75 +0x200 +0x400000 +Center, +


Gui Font
Gui Font, s9 cBlue
Gui Add, Text, vIns gKeyClick x640 y70 w36 h36 +0x200 +0x400000 +Center, Ins
Gui Add, Text, vHome gKeyClick x680 y70 w36 h36 +0x200 +0x400000 +Center, Home
Gui Add, Text, vPgUp gKeyClick x720 y70 w36 h36 +0x200 +0x400000 +Center, PgUp
Gui Add, Text, vDel gKeyClick x640 y110 w36 h36 +0x200 +0x400000 +Center, Del
Gui Add, Text, vEnd gKeyClick x680 y110 w36 h36 +0x200 +0x400000 +Center, End
Gui Add, Text, vPgDn gKeyClick x720 y110 w36 h36 +0x200 +0x400000 +Center, PgDn

Gui Add, Text, vNum1 gKeyClick x780 y200 w36 h36 +0x200 +0x400000 +Center, Num1
Gui Add, Text, vNum2 gKeyClick x820 y200 w36 h36 +0x200 +0x400000 +Center, Num2
Gui Add, Text, vNum3 gKeyClick x860 y200 w36 h36 +0x200 +0x400000 +Center, Num3
Gui Add, Text, vNum4 gKeyClick x780 y160 w36 h36 +0x200 +0x400000 +Center, Num4
Gui Add, Text, vNum5 gKeyClick x820 y160 w36 h36 +0x200 +0x400000 +Center, Num5
Gui Add, Text, vNum6 gKeyClick x860 y160 w36 h36 +0x200 +0x400000 +Center, Num6
Gui Add, Text, vNum7 gKeyClick x780 y120 w36 h36 +0x200 +0x400000 +Center, Num7
Gui Add, Text, vNum8 gKeyClick x820 y120 w36 h36 +0x200 +0x400000 +Center, Num8
Gui Add, Text, vNum9 gKeyClick x860 y120 w36 h36 +0x200 +0x400000 +Center, Num9

Gui Font

Gui Font, s7 cBlue
Gui Add, Text, vPrtSc gKeyClick x640 y30 w36 h36 +0x200 +0x400000 +Center, PrtSc
Gui Add, Text, vScrLk gKeyClick x680 y30 w36 h36 +0x200 +0x400000 +Center, ScrLk
Gui Add, Text, vPause gKeyClick x720 y30 w36 h36 +0x200 +0x400000 +Center, Pause
Gui Add, Text, vNumEnter gKeyClick x900 y200 w36 h75 +0x400000 +Center, `n`n`nNum`nEnter
Gui Add, Text, vNumLk gKeyClick x780 y80 w36 h36 +0x200 +0x400000 +Center, NumLk
Gui Font

Gui Font, s9
Gui Add, Button, gClearAllKeys x900 y30 w36 h36 +0x200 +Center, 清空
Gui Font

Gui Add, GroupBox, x8 y300 w275 h200, 配置设置

Gui Add, ListBox, vPreset x16 y320 w120 h180
Gui Add, Text, x150 y320 w120 h24 +0x200, 配置名称
Gui Add, Button, gLoadPreset x150 y380 w120 h30, 读取配置
Gui Add, Button, gSavePreset x150 y420 w120 h30, 保存配置
Gui Add, Button, gDeletePreset x150 y460 w120 h30, 删除配置
Gui Add, Button, gStartAutoFire x744 y294 w200 h200, 启动连发
Gui Add, Edit, vPresetName x150 y350 w120 h22

Gui Show, w950 h510, DNF连发工具 - DNF:DNF Normative Fire - v0.0.1

LoadPresetGUI()
LoadDefaultPreset()
return

GuiEscape:
GuiClose:
	Gui, Hide
