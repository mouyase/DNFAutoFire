;Environment Controls
;///////////////////////////////////////////////////////////////////////////////////////////
#Requires Autohotkey v2
#SingleInstance Force
#Include Lib/WebViewToo.ahk
ScriptPID := DllCall("GetCurrentProcessId")
GroupAdd("ScriptGroup", "ahk_pid" ScriptPID)
;///////////////////////////////////////////////////////////////////////////////////////////

;Create the WebviewWindow/GUI
;///////////////////////////////////////////////////////////////////////////////////////////
if (A_IsCompiled) {
	WebViewToo.CreateFileFromResource((A_PtrSize * 8) "bit\WebView2Loader.dll", WebViewToo.TempDir)
}
MyWindow := WebViewToo(,,, True) ;You can omit the final parameter or switch 'True' to 'False' to use a Native Window's Titlebar
MyWindow.EnableGlobal() ;Opens all native ahk functions and script variables to the Webpage **USE WITH CAUTION**
MyWindow.OnEvent("Close", (*) => ExitApp())
MyWindow.Load("Pages/index.html")
MyWindow.Debug()
MyWindow.AddCallBackToScript("Tooltip", WebTooltipEvent)
MyWindow.AddCallbackToScript("SubmitForm", SubmitFormHandler)
MyWindow.AddCallBackToScript("CopyGlyphCode", CopyGlyphCodeEvent)
MyWindow.AddHostObjectToScript("ButtonClick", {func:WebButtonClickEvent})
MyWindow.Show("w1000 h720 Center", "WebViewToo Example")
;///////////////////////////////////////////////////////////////////////////////////////////

;Hotkeys
;///////////////////////////////////////////////////////////////////////////////////////////
#HotIf WinActive("ahk_group ScriptGroup")
F1:: {
	MsgBox(MyWindow.Title)
	MyWindow.Title := "New Title!"
	MsgBox(MyWindow.Title)
}

F2:: {
    static Toggle := 0
    Toggle := !Toggle
    if (Toggle) {
	    MyWindow.PostWebMessageAsString("Hello World")
    } else {
        MyWindow.PostWebMessageAsJson('{"key1": "value1"}')
    }
}

F3:: {
	MyWindow.SimplePrintToPdf()
}
#HotIf
;///////////////////////////////////////////////////////////////////////////////////////////

;Web Functions
;///////////////////////////////////////////////////////////////////////////////////////////
WebButtonClickEvent(button) {
	MsgBox(button)
}

CopyGlyphCodeEvent(WebView, Title) {
	GlyphCode := "<span class='glyphicon glyphicon-" Title "' aria-hidden='true'></span>"
	MsgBox(A_Clipboard := GlyphCode, "OuterHTML Copied to Clipboard")
}

WebTooltipEvent(WebView, Msg) {
	ToolTip(Msg)
	SetTimer((*) => ToolTip(), -1000)
}

SubmitFormHandler(WebView, FormData) {
	Output := ""
	Output .= "Email: " FormData.Email "`n"
	Output .= "Password: " FormData.Password "`n"
	Output .= "Address: " FormData.Address "`n"
	Output .= "Address2: " FormData.Address2 "`n"
	Output .= "City: " FormData.City "`n"
	Output .= "State: " FormData.State "`n"
	Output .= "Zip: " FormData.Zip "`n"
	try Output .=  "Check: " FormData.Check "`n" ;Only works when checked
	MsgBox(Output)
}
;///////////////////////////////////////////////////////////////////////////////////////////


;Resources for Compiled Scripts
;///////////////////////////////////////////////////////////////////////////////////////////
;@Ahk2Exe-AddResource Lib\32bit\WebView2Loader.dll, 32bit\WebView2Loader.dll
;@Ahk2Exe-AddResource Lib\64bit\WebView2Loader.dll, 64bit\WebView2Loader.dll
;@Ahk2Exe-AddResource Pages\index.html, Pages\index.html
;@Ahk2Exe-AddResource Pages\Bootstrap\bootstrap.bundle.min.js, Pages\Bootstrap\bootstrap.bundle.min.js
;@Ahk2Exe-AddResource Pages\Bootstrap\bootstrap.min.css, Pages\Bootstrap\bootstrap.min.css
;@Ahk2Exe-AddResource Pages\Bootstrap\color-modes.js, Pages\Bootstrap\color-modes.js
;@Ahk2Exe-AddResource Pages\Bootstrap\sidebars.css, Pages\Bootstrap\sidebars.css
;@Ahk2Exe-AddResource Pages\Bootstrap\sidebars.js, Pages\Bootstrap\sidebars.js
;@Ahk2Exe-AddResource Pages\Bootstrap\fonts\glyphicons-halflings-regular.ttf, Pages\Bootstrap\fonts\glyphicons-halflings-regular.ttf
;@Ahk2Exe-AddResource Pages\Bootstrap\fonts\glyphicons-halflings-regular.woff, Pages\Bootstrap\fonts\glyphicons-halflings-regular.woff
;@Ahk2Exe-AddResource Pages\Bootstrap\fonts\glyphicons-halflings-regular.woff2, Pages\Bootstrap\fonts\glyphicons-halflings-regular.woff2
;///////////////////////////////////////////////////////////////////////////////////////////
