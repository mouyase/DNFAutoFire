;///////////////////////////////////////////////////////////////////////////////////////////
; WebViewToo.ahk
; Copyright (c) 2025 Ryan Dingman (known also as Panaku, The-CoDingman)
; https://github.com/The-CoDingman/WebViewToo
;
; MIT License
;
; Permission is hereby granted, free of charge, to any person obtaining a copy
; of this software and associated documentation files (the "Software"), to deal
; in the Software without restriction, including without limitation the rights
; to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
; copies of the Software, and to permit persons to whom the Software is
; furnished to do so, subject to the following conditions:
;
; The above copyright notice and this permission notice shall be included in all
; copies or substantial portions of the Software.
;
; THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
; IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
; FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
; AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
; LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
; OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
; SOFTWARE.
;///////////////////////////////////////////////////////////////////////////////////////////

#Requires AutoHotkey v2
#Include WebView2.ahk

class WebViewToo {
	static Template := {}
	static Template.Framework := "
	(
		<!DOCTYPE html>
		<html>
			<head>
				<meta http-equiv="X-UA-Compatible" content="IE=edge">
				<style>{2}</style>
			</head>

			<body>
				<div class="main">{1}</div>
				<script>{3}</script>
			</body>
		</html>
	)"
	
	static Template.Css := "html, body {width: 100%; height: 100%;margin: 0; padding: 0;font-family: sans-serif;} body {display: flex;flex-direction: column;} .main {flex-grow: 1;overflow: hidden;}"
	static Template.Name := "Template.html"
	static Template.Html := "<iframe style='height:100%;width:100%;border:0;' src='https://the-codingman.github.io/WebViewToo'></iframe>"
	static Template.JavaScript := ""

	static UniqueId => WebViewToo.CreateUniqueID()
	static CreateUniqueId() {
		SplitPath(A_ScriptName,,,, &OutNAmeNoExt)
		Loop Parse, OutNAmeNoExt {
			Id .= Mod(A_Index, 3) ? Format("{:X}", Ord(A_LoopField)) : "-" Format("{:X}", Ord(A_LoopField))
		}
		return RTrim(StrLower(Id), "-")
	}
	static TempDir := A_Temp "\" WebViewToo.UniqueId
	static DllPath := WebViewToo.TempDir "\" (A_PtrSize * 8) "bit\WebView2Loader.dll"

	__New(Html := WebViewToo.Template.Html, Css := WebViewToo.Template.Css, JavaScript := WebViewToo.Template.JavaScript, CustomCaption := False) {
		this.Gui := Gui("+Resize")
		this.Gui.BackColor := "000000"
		this.CustomCaption := CustomCaption
		this.Gui.MarginX := this.Gui.MarginY := 0
		this.Gui.OnEvent("Size", this.Size.Bind(this))
		this.LastKnownX := this.LastKnownY := this.LastKnownWidth := this.LastKnownHeight := 0
		this.Gui.Add("Text", "x0 y0 vWebViewTooContainer BackgroundTrans", "If you can see this, something went wrong.")
		this.wvc := !A_IsCompiled ? WebView2.CreateControllerAsync(this.Gui["WebViewTooContainer"].Hwnd).await2() : WebView2.CreateControllerAsync(this.Gui["WebViewTooContainer"].Hwnd,,,, WebViewToo.DllPath).await2()
		this.IsVisible := 1, this.wv := this.wvc.CoreWebView2, this.IsNonClientRegionSupportEnabled := 1
		this.wv.InjectAhkComponent().await2()
		this.AddCallbackToScript("Close", this.Close)
		this.AddCallbackToScript("Restore", this.Restore)
		this.AddCallbackToScript("Minimize", this.Minimize)
		this.AddCallbackToScript("Maximize", this.Maximize)
		A_IsCompiled ? (this.EnableInternalLoading()) : 0
		OnExit(this.OnExit := ObjBindMethod(this, "Destroy"), 1)
		this.NavigateToString(Format(WebViewToo.Template.Framework, Html, Css, JavaScript))
		if (this.CustomCaption) {
			this.CustomCaptionBarInit()
		}
	}
	
	__Delete() {
		;Placeholder for later on
	}

	;-------------------------------------------------------------------------------------------
	;Static WebViewToo Methods
	static ConvertColor(RGB) => (RGB := RGB ~= "^0x" ? RGB : "0x" RGB, (((RGB & 0xFF) << 16) | (RGB & 0xFF00) | (RGB >> 16 & 0xFF)) << 8 | 0xFF) ;Must be a string

	static CreateFileFromResource(ResourceName, DestinationDir := WebViewToo.TempDir) { ;Create a file from an installed resource -- works like a dynamic `FileInstall()`
		if (!A_IsCompiled) {
			return
		}

		ResourceName := StrReplace(ResourceName, "/", "\")
		SplitPath(ResourceName, &OutFileName, &OutDir, &OutExt)
		ResourceType := OutExt = "bmp" || OutExt = "dib" ? 2 : OutExt = "ico" ? 14 : OutExt = "htm" || OutExt = "html" || OutExt = "mht" ? 23 : OutExt = "manifest" ? 24 : 10
		Module := DllCall("GetModuleHandle", "Ptr", 0, "Ptr")
		Resource := DllCall("FindResource", "Ptr", Module, "Str", ResourceName, "UInt", ResourceType, "Ptr")
		ResourceSize := DllCall("SizeofResource", "Ptr", Module, "Ptr", Resource)
		ResourceData := DllCall("LoadResource", "Ptr", Module, "Ptr", Resource, "Ptr")
		ConvertedData := DllCall("LockResource", "Ptr", ResourceData, "Ptr")
		TextData := StrGet(ConvertedData, ResourceSize, "UTF-8")

		if (!DirExist(DestinationDir "\" OutDir)) {
			DirCreate(DestinationDir "\" OutDir)
		}

		if (FileExist(DestinationDir "\" ResourceName)) {
			ExistingFile := FileOpen(DestinationDir "\" ResourceName, "r")
			ExistingFile.RawRead(TempBuffer := Buffer(ResourceSize))
			ExistingFile.Close()
			if (DllCall("ntdll\memcmp", "Ptr", TempBuffer, "Ptr", ConvertedData, "Ptr", ResourceSize)) {
				FileSetAttrib("-R", DestinationDir "\" ResourceName)
				FileDelete(DestinationDir "\" ResourceName)
			}
		}

		if (!FileExist(DestinationDir "\" ResourceName)) {
			TempFile := FileOpen(DestinationDir "\" ResourceName, "w")
			TempFile.RawWrite(ConvertedData, ResourceSize)
			TempFile.Close()
			FileSetAttrib("+HR", DestinationDir "\" OutDir)
			FileSetAttrib("+HR", DestinationDir "\" ResourceName)
		}

		StartingPos := 1
		while (RegExMatch(TextData, "<script.*?src=[`"'](?!https)(.*?)[`"']>", &Match, StartingPos) || RegExMatch(TextData, "<link.*?href=[`"'](?!https)(.*?)[`"'] rel=`"stylesheet`">", &Match, StartingPos)) {
			WebViewToo.CreateFileFromResource(OutDir "\" StrReplace(Match[1], "/", "\"), DestinationDir)
			StartingPos := StrLen(Match[]) + Match.Pos
		}
	}

	static EscapeHtml(Text) => StrReplace(StrReplace(StrReplace(StrReplace(StrReplace(Text, "&", "&amp;"), "<", "&lt;"), ">", "&gt;"), "`"", "&quot;"), "'", "&#039;")

	static EscapeJavaScript(Text) => StrReplace(StrReplace(StrReplace(Text, '\', '\\'), '"', '\"'), '`n', '\n')

	static ForEach(Obj, Parent := "Default", Depth := 0) {
		if(!IsObject(Obj) || (Type(Obj) = "ComObject")) {
			return
		}

		Output := ""
		for Key, Value, in Obj.OwnProps() {
			try Output .= "`n" Parent " >> " Key
			try Output .= ": " Value
			try Output .= WebViewToo.ForEach(Value, Parent " >> " Key, Depth + 1)
		}
		for Key, Value in base_props(Obj) {
			try Output .= "`n" Parent " >> " Key
			try Output .= ": " Value
			try Output .= WebViewToo.ForEach(Value, Parent " >> " Key, Depth + 1)
		}
		return Depth ? Output : Trim(Output, "`n")
	
		base_props(Obj) {
			iter := Obj.Base.OwnProps(), iter() ;skip `__Class`
			return next
	
			next(&Key, &Value, *) {
				while (iter(&Key))
					try if !((Value := Obj.%Key%) is Func)
						return true
				return false
			}
		}
	}

	static FormatHtml(FormatStr, Values*) {
		for Index, Value, in Values {
			Values[Index] := WebViewToo.EscapeHtml(Value)
		}
		return Format(FormatStr, Values*)
	}

	;-------------------------------------------------------------------------------------------
	;WebViewToo class assignments	
	AddCallbackToScript(CallbackName, Callback) => this.AddHostObjectToScript(CallbackName, Callback.Bind(this)) ;Similar to `AddHostObjectToScript()`, but only registers a callback
	RemoveCallbackFromScript(CallbackName) => this.RemoveHostObjectFromScript(CallbackName) ;Removes a registered callback
	BorderColor {
		get => this.Border.Color
		set {
			this.Border.Color := Value
			if (this.CustomCaption) {
				this.Border.Gui.BackColor := Value
			}
		}
	}
	BorderMode {
		get => this.Border.Mode
		set {
			if (!this.CustomCaption) {
				return
			}

			OldMode := this.Border.Mode, this.Border.Mode := StrUpper(Value)
			if (this.Border.Mode = "OUTSET") {
				this.Border.Offset := 1
			} else if (this.Border.Mode = "MIDDLE") {
				this.Border.Offset := this.Border.Size / 2
			} else if (this.Border.Mode = "INSET") {
				this.Border.Offset := this.Border.Size
			} else if (this.Border.Mode = "OFF") {
				this.BorderHide()
			}
			this.BorderShow()
		}
	}
	BorderSize {
		/**
		 * A minimum BorderSize of 1 is required when using the CustomCaption
		 * This is what allows us to resize the window by grabbing the GUI border
		**/
		get => this.Border.Size
		set {
			this.Border.Size := Value
			this.BorderMode := this.Border.Mode
			this.BorderShow()
		}
	}
	BorderTransparency {
		get => this.Border.Transparency
		set {
			WinSetTransparent(this.Border.Transparency := Value, this.Border.Gui.Hwnd)
		}
	}

	BorderHide() {
		WinSetTransparent(0, this.Border.Gui.Hwnd)
		this.Border.Hidden := 0, this.Border.Gui.Hide()
		WinSetTransparent(this.Border.Transparency, this.Border.Gui.Hwnd)
	}

	BorderShow() {
		if (!GetKeyState("LButton") && (this.Border.Mode != "OFF")) {
			SetTimer(this.Border.Timer, 0)
			this.Gui.GetClientPos(&X, &Y, &Width, &Height)
			this.Border.Hidden := 0, WinSetTransparent(0, this.Border.Gui.Hwnd), this.Border.Gui.Show("NoActivate")
			this.Border.Gui.Move(X - this.Border.Size + this.Border.Offset, Y - this.Border.Size + this.Border.Offset, Width + (this.Border.Size * 2) - (this.Border.Offset * 2), Height + (this.Border.Size * 2) - (this.Border.Offset * 2))
			RegionString := Format("0-0 {1}-0 {1}-{2} 0-{2} 0-0 {3}-{3} {4}-{3} {4}-{5} {3}-{5} {3}-{3}", Width + (this.Border.Size * 2), Height + (this.Border.Size * 2), this.Border.Size, Width + this.Border.Size - (this.Border.Offset * 2), Height + this.Border.Size - (this.Border.Offset * 2))
			WinSetRegion(RegionString, this.Border.Gui.Hwnd)
			WinSetTransparent(this.BorderTransparency, this.Border.Gui.Hwnd)
		}
	}
	
	Close() {
		if (!WinExist("ahk_id" this.Hwnd)) {
			return
		}

		if (this.CustomCaption) {
			this.Gui.GetPos(&X, &Y, &Width, &Height)
		} else {
			this.Gui.GetClientPos(&X, &Y, &Width, &Height)
		}
		this.LastKnownX := X, this.LastKnownY := Y, this.LastKnownWidth := Width, this.LastKnownHeight := Height
		WinClose("ahk_id" this.Hwnd)
	}

	CustomCaptionBarInit(Redraw := 1) {
		this.Border := {Gui: Gui("+Resize"), Size: 8, Transparency: 1, Hidden: 0, Mode: "OFF", Color: 0x999999, Timeout: 100, Timer: ObjBindMethod(this, "BorderShow")}
		this.BorderMode := "MIDDLE"
		this.Border.Gui.Opt("+Owner" this.Gui.Hwnd)
		this.Border.Gui.BackColor := this.Border.Color
		this.Border.Gui.Show("x-1000000 y-1000000 w0 h0 NoActivate")
		this.CustomCaption := true
		this.Gui.OnEvent("Close", (*) => this.BorderHide())
		this.Border.NCHITTEST := (OnMessage(WM_NCHITTEST := 0x0084, WM_NCHITTEST_HANDLER))
		WM_NCHITTEST_HANDLER(wParam, lParam, Msg, Hwnd) {
			if ((Hwnd = this.Gui.Hwnd) || (Hwnd = this.Border.Gui.Hwnd)) {
				Critical(-1)
				this.Gui.GetPos(&gX, &gY, &gWidth, &gHeight)
				X := lParam << 48 >> 48, Y := lParam << 32 >> 48, HL := X < gX + (this.Border.Size * 2), HR := X >= gX + gWidth - (this.Border.Size * 2), HT := Y < gY + (this.Border.Size * 2), HB := Y >= gy + gHeight - (this.Border.Size * 2)
				ReturnCode := HT && HL ? 0xD : HT && HR ? 0xE : HT ? 0xC : HB && HL ? 0x10 : HB && HR ? 0x11 : HB ? 0xF : HL ? 0xA : HR ? 0xB : 0
				if (ReturnCode)
					return ReturnCode
			}
		}
		this.Border.NCACTIVATE := (OnMessage(WM_NCACTIVATE := 0x0086, WM_NCACTIVATE_HANDLER))
		WM_NCACTIVATE_HANDLER(wParam, lParam, Msg, Hwnd) {
			if ((Hwnd = this.Gui.Hwnd) || (Hwnd = this.Border.Gui.Hwnd)) {
				return 1
			}
		}
		this.Border.NACCALCSIZE := (OnMessage(WM_NCCALSIZE := 0x0083, WM_NCCALCSIZE_HANDLER))
		WM_NCCALCSIZE_HANDLER(wParam, lParam, Msg, Hwnd) {
			if ((Hwnd = this.Gui.Hwnd) || (Hwnd = this.Border.Gui.Hwnd)) {
				; Fill client area when not maximized, else crop borders to prevent screen overhang.
				if(!DllCall("IsZoomed", "UPtr", Hwnd)) {
					return 0
				}
				
				; Query for the window's border size
				WindowInfo := Buffer(60, 0)
				NumPut("UInt", 60, WindowInfo), DllCall("GetWindowInfo", "UPtr", Hwnd, "Ptr", WindowInfo)
				CxWindowBorders := NumGet(WindowInfo, 48, "Int"), CyWindowBorders := NumGet(WindowInfo, 52, "Int")
				
				; Inset the client rect by the border size
				NumPut("Int", NumGet(lParam + 0, "Int") + CxWindowBorders, "Int", NumGet(lParam + 4, "Int") + CyWindowBorders, "Int", NumGet(lParam + 8, "Int") - CxWindowBorders, "Int", NumGet(lParam + 12, "Int") - CyWindowBorders, lParam)
				return 0
			}
		}
		this.Border.Move := (OnMessage(WM_MOVE := 0x0003, WM_MOVE_HANDLER))
		WM_MOVE_HANDLER(wParam, lParam, Msg, Hwnd) {
			if ((Hwnd = this.Gui.Hwnd) && (GetKeyState("LButton")) && (!this.Border.Hidden)) {
				this.BorderHide()
				SetTimer(this.Border.Timer, this.Border.Timeout)
			}
		}
		this.Border.NCLBUTTONDOWN := (OnMessage(WM_NCLBUTTONDOWN := 0x00A1, WM_NCLBUTTONDOWN_HANDLER))
		WM_NCLBUTTONDOWN_HANDLER(wParam, lParam, Msg, Hwnd) {
			if (Hwnd = this.Border.Gui.Hwnd) {
				PostMessage(WM_NCLBUTTONDOWN, wParam, lParam, this.Gui.Hwnd)
				return 0
			}
		}
		DllCall("Dwmapi.dll\DwmSetWindowAttribute", "Ptr", this.Gui.Hwnd, "UInt", DWMWA_WINDOW_CORNER_PREFERENCE := 33, "Ptr*", pvAttribute := 2, "UInt", 4) ;May not work or even cause errors on Win10
		if (Redraw) {
			this.GetClientPos(&X, &Y, &Width, &Height)
			this.Show("x-1000000 y-1000000 w" Width " h" Height (WinActive(this.Hwnd) ? "" : " NoActivate"))
			this.Move(X, Y, Width, Height)
		}
	}

	Debug() {
		this.OpenDevToolsWindow()
	}

	EnableGlobal() {
		this.NavigationStarting(InstallGlobal)
		InstallGlobal(ICoreWebView2, Args) {
			static AllInstalled := 0
			if ((Args.Uri ~= "^https://ahk.localhost/" || Args.Uri ~= "^file:///") && (!AllInstalled)) {
				ICoreWebView2.AddHostObjectToScript("global", {__Get: (this, name, *) => %name%}), AllInstalled := 1
			} else if (!(Args.Uri ~= "^https://ahk.localhost/" || Args.Uri ~= "^file:///") && (AllInstalled)) {
				ICoreWebView2.RemoveHostObjectFromScript("global"), AllInstalled := 0
			}
		}
	}

	EnableInternalLoading() {
		if (A_IsCompiled) {
			this.AddWebResourceRequestedFilter("https://ahk.localhost/*", 0)
			this.WebResourceRequested(CreateWebpageFromWebResource)
			CreateWebpageFromWebResource(ICoreWebView2, Args) {
				ResourceName := StrUpper(StrReplace(StrReplace(Args.Request.Uri, "https://ahk.localhost/",,,, 1), "/", "\"))
				SplitPath(ResourceName, &OutFileName, &OutDir, &OutExt)
				ResourceType := OutExt = "bmp" || OutExt = "dib" ? 2 : OutExt = "ico" ? 14 : OutExt = "htm" || OutExt = "html" || OutExt = "mht" ? 23 : OutExt = "manifest" ? 24 : 10
				Module := DllCall("GetModuleHandle", "Ptr", 0, "Ptr")
				Resource := DllCall("FindResource", "Ptr", Module, "Str", ResourceName, "UInt", ResourceType, "Ptr")
				if (!Resource) {
					return
				}
				ResourceSize := DllCall("SizeofResource", "Ptr", Module, "Ptr", Resource)
				ResourceData := DllCall("LoadResource", "Ptr", Module, "Ptr", Resource, "Ptr")
				ConvertedData := DllCall("LockResource", "Ptr", ResourceData, "Ptr")
				Args.Response := ICoreWebView2.Environment.CreateWebResourceResponse(WebView2.CreateMemStream(ResourceData, ResourceSize), 200, "OK", "") ;
			}
		}
	}
	
	Load(Filename) => this.Navigate(Filename ~= "^https?:\/\/" ? Filename : A_IsCompiled ? "https://ahk.localhost/" Filename : A_WorkingDir "\" Filename)

	SimplePrintToPdf(FileName := "", Orientation := "Portrait", Timeout := 5000) {
		Loop {
			FileName := FileSelect("S", tFileName := IsSet(FileName) ? FileName : "",, "*.pdf")
			if (FileName = "") {
				return CancelMsg()
			}

			SplitPath(FileName, &OutFileName, &OutDir, &OutExt)
			FileName := OutExt = "" ? FileName ".pdf" : Filename
			if (FileExist(FileName)) {
				Overwrite := OverwriteMsg()
				if (Overwrite = "No") {
					continue
				} else if (Overwrite = "Cancel") {
					return CancelMsg()
				}
			}
			break
		}

		Settings := this.Environment.CreatePrintSettings()
		Settings.Orientation := Orientation = "Portrait" ? WebView2.PRINT_ORIENTATION.PORTRAIT : WebView2.PRINT_ORIENTATION.LANDSCAPE
		PrintPromise := this.PrintToPdfAsync(FileName, Settings)
		try PrintPromise.await2(Timeout)		
		if (!PrintPromise.Result) {
			ErrorMsg()
		} else {
			if (MsgBox("Would you like to open this PDF?", "Print to PDF", "262148") = "Yes") {
				Run(FileName)
			}
		}

		ErrorMsg() => MsgBox("An error occurred while attempting to save the file.`n" FileName, "Print to PDF", "262144")
		CancelMsg() => MsgBox("Print Canceled", "Print to PDF", "262144")
		OverwriteMsg() => MsgBox(OutFileName " already exist.`nWould you like to overwrite it?", "Confirm Save As", "262195")
	}
	
	Size(GuiObj, MinMax, Width, Height) {
		static LastCallTime := 0, Timeout := 25
		LastCallTime := A_TickCount
		SetTimer(TimeoutCheck, -Timeout)
		if (MinMax = 1) {
			if (this.CustomCaption) {
				this.BorderHide()
			}
			try this.ExecuteScriptAsync("document.body.classList.add('ahk-maximized')")
		} else if (MinMax != 1) {
			try this.ExecuteScriptAsync("document.body.classList.remove('ahk-maximized')")
		}
		
		if ((this.CustomCaption) && (MinMax != 1) && (!this.Border.Hidden)) {
			this.BorderHide()
			SetTimer(this.Border.Timer, this.Border.Timeout)
		}
		this.Gui["WebViewTooContainer"].Move(,, Width, Height)

		TimeoutCheck() {
			CurrentCallTime := A_TickCount
			if (CurrentCallTime - LastCallTime >= Timeout) {
				this.Fill()
			}
		}
	}

	;-------------------------------------------------------------------------------------------
	;Inherited GUI Methods
	Destroy(*) {
		if (!this.Gui) {
			return
		}

		OnExit(this.OnExit, 0), this.OnExit := 0
		this.wvc.Close() ;Close the WebView2Controller before destroying the Gui
		this.Gui.Destroy(), this.Gui := 0
		if (this.CustomCaption) {
			this.Border.Gui.Destroy(), this.Border.Gui := 0
			this.Border := 0
		}
	}

	Flash(Blink := True) {
		this.Gui.Flash(Blink)
	}

	GetClientPos(Params*) => this.Gui.GetClientPos(Params*)

	GetPos(Params*) => this.Gui.GetPos(Params*)

	Hide() {
		if (this.CustomCaption) {
			this.Gui.GetPos(&X, &Y, &Width, &Height)
		} else {
			this.Gui.GetClientPos(&X, &Y, &Width, &Height)
		}
		this.LastKnownX := X, this.LastKnownY := Y, this.LastKnownWidth := Width, this.LastKnownHeight := Height
		this.Gui.Hide()
		if (this.CustomCaption && !this.Border.Hidden) {
			this.BorderHide()
		}
	}

	Maximize() => this.Gui.Maximize()

	Minimize() => this.Gui.Minimize()

	Move(Params*) => this.Gui.Move(Params*)

	OnEvent(EventName, Callback, AddRemove := unset) {
		this.Gui.OnEvent(EventName, (p*) => (p[1] := this, Callback(p*)), AddRemove?)
	}

	Opt(Options) {
		this.Gui.Opt(Options)
	}

	Restore() => this.Gui.Restore()

	Show(Options := "", Title := this.Title) {
		if (!this.Gui) {
			throw Error("This WebViewToo does not have a window.", -1)
		}

		Width := RegExMatch(options, "w\s*\K\d+", &match) ? match[] : this.LastKnownWidth ? this.LastKnownWidth : A_ScreenWidth / 2
		Height := RegExMatch(options, "h\s*\K\d+", &match) ? match[] : this.LastKnownHeight ? this.LastKnownHeight : A_ScreenHeight / 2

		; AutoHotkey sizes the window incorrectly, trying to account for borders
		; that aren't actually there. Call the function AHK uses to offset and
		; apply the change in reverse to get the actual wanted size.
		if (this.CustomCaption) {
			rect := Buffer(16, 0)
			DllCall("AdjustWindowRectEx",
				"Ptr", rect,		; LPRECT lpRect
				"UInt", WinGetStyle(this.Gui),	; DWORD  dwStyle
				"UInt", 0,			; BOOL   bMenu
				"UInt", 0,			; DWORD  dwExStyle
				"UInt"				; BOOL
			)
			Width += (NumGet(rect, 0, "Int") - NumGet(rect, 8, "Int"))
			Height += (NumGet(rect, 4, "Int") - NumGet(rect, 12, "Int"))
		}
		this.Gui.Title := Title
		this.Gui.Show(Options " w" Width " h" Height)
		if (this.CustomCaption) {
			SetTimer(this.Border.Timer, this.Border.Timeout)
		}
	}

	;-------------------------------------------------------------------------------------------
	;Inherited GUI Properites
	BackColor {
		get => this.Gui.BackColor
		set => this.Gui.BackColor := Value
	}
	FocusedCtrl => this.Gui.FocusedCtrl
	Hwnd => this.Gui.Hwnd
	MarginX {
		get => this.Gui.MarginX
		set => this.Gui.MarginX := Value
	}
	MarginY {
		get => this.Gui.MarginY
		set => this.Gui.MarginY := Value
	}
	MenuBar {
		get => this.Gui.MenuBar
		set {
			if (!this.CustomCaption) {
				this.Gui.MenuBar := Value
			}
		}
	}
	Name {
		get => this.Gui.Name
		set => this.Gui.Name := Value
	}
	Title {
		get {
			if (!this.Gui) {
				return
			}

			return this.Gui.Title
		}
		set {
			if (!this.Gui) {
				return
			}

			this.Gui.Title := Value
			try this.ExecuteScriptAsync("document.querySelector('#ahkTitleBar').textContent = '" Value "'")
		}
	}

	;-------------------------------------------------------------------------------------------
	;Controller class assignments
	Fill() => this.wvc.Fill()
	CoreWebView2 => this.wvc.CoreWebView2 ;Gets the CoreWebView2 associated with this CoreWebView2Controller
	IsVisible { ;Boolean => Determines whether to show or hide the WebView
		get => this.wvc.IsVisible
		set => this.wvc.IsVisible := Value
	}
	Bounds { ;Rectangle => Gets or sets the WebView bounds
		/**
		 * Returns a Buffer()
		 * You can extract the X, Y, Width, and Height using NumGet()
		 * X is at offset 0, Y at offset 4, Width at offset 8, Height at offset 12.
		**/
		get => this.wvc.Bounds

		/**
		 * Value must be a Buffer(16) that you've inserted values into
		 * using NumPut(). See the above notes regarding the appropriate offsets.
		**/
		set => this.wvc.Bounds := Value
	}
	Bounds(X?, Y?, Width?, Height?) { ;Get: Object with X, Y, Width, Height properties; Set:
		tBounds := this.wvc.Bounds
		if (IsSet(X) || IsSet(Y) || IsSet(Width) || IsSet(Height)) {
			IsSet(X) ? NumPut("Int", X, tBounds, 0) : 0
			IsSet(Y) ? NumPut("Int", Y, tBounds, 4) : 0
			IsSet(Width) ? NumPut("Int", Width, tBounds, 8) : 0
			IsSet(Height) ? NumPut("Int", Height, tBounds, 12) : 0
			this.Bounds := tBounds
		} else {
			return Bounds := {
				X: NumGet(tBounds, 0, "Int"),
				Y: NumGet(tBounds, 4, "Int"),
				Width: NumGet(tBounds, 8, "Int"),
				Height: NumGet(tBounds, 12, "Int")
			}
		}
	}
	ZoomFactor { ;Double => Gets or sets the zoom factor for the WebView
		get => this.wvc.ZoomFactor
		set => this.wvc.ZoomFactor := Value
	}
	ParentWindow { ;Integer => Gets the parent window provided by the app or sets the parent window that this WebView is using to render content
		get => this.wvc.ParentWindow ;Returns this.Gui["WebViewTooContainer"].Hwnd
		set => this.wvc.ParentWindow := Value ;Not recommened to use set => because it dettaches the WebView2 window and can break the software
	}
	DefaultBackgroundColor { ;HexColorCode => Gets or sets the WebView default background color.
		get {
			BGRA := Format("{:X}", this.wvc.DefaultBackgroundColor)
			return SubStr(BGRA, 5, 2) SubStr(BGRA, 3, 2) SubStr(BGRA, 1, 2)
		}
		set => this.wvc.DefaultBackgroundColor := WebViewToo.ConvertColor(Value)
	}

	/**
	 * RasterizationScale, ShouldDetectMonitorScaleChanges, and BoundsMode all work together
	 * If you want to use set => (RasterizationScale||ShouldDetectMonitorScaleChanges||BoundsMode)
	 * you will need to turn on DPI Awareness for your script by using the following DllCall
	 * DllCall("SetThreadDpiAwarenessContext", "ptr", -3, "ptr") ;**NOTE: DpiAwareness Now causes fatal error, good luck**
	**/
	RasterizationScale { ;Double => Gets or sets the WebView rasterization scale
		get => this.wvc.RasterizationScale
		set => this.wvc.RasterizationScale := Value
	}
	ShouldDetectMonitorScaleChanges { ;Boolean => Determines whether the WebView will detect monitor scale changes
		get => this.wvc.ShouldDetectMonitorScaleChanges
		set => this.wvc.ShouldDetectMonitorScaleChanges := Value
	}
	BoundsMode { ;Boolean => Gets or sets the WebView bounds mode
		/**
		 * 0: UseRawPixels; Bounds property represents raw pixels. Physical size of Webview is not impacted by RasterizationScale
		 * 1: UseRasterizationScale; Bounds property represents logical pixels and the RasterizationScale property is used to get the physical size of the WebView.
		**/
		get => this.wvc.BoundsMode
		set => this.wvc.BoundsMode := Value
	}
	AllowExternalDrop { ;Boolean => Gets or sets the WebView allow external drop property
		get => this.wvc.AllowExternalDrop
		set => this.wvc.AllowExternalDrop := Value
	}
	SetBoundsAndZoomFactor(Bounds, ZoomFactor) => this.wvc.SetBoundsAndZoomFactor(Bounds, ZoomFactor) ;Updates Bounds and ZoomFactor properties at the same time

	/**
	 * MoveFocus()
	 * 1: Next; Specifies that the focus is moved due to Tab traversal forward
	 * 2: Previous; Specifies that the focus is moved due to Tab traversal backward
	 * 0: Programmatic; Specifies that the code is setting focus into WebView
	**/
	MoveFocus(Reason) => this.wvc.MoveFocus(Reason) ;Moves focus into WebView	

	/**
	 * NotifyParentWindowPositionChanged()
	 * Notifies the WebView that the parent (or any ancestor) HWND moved
	 * Example: Calling this method updates dialog windows such as the DownloadDialog
	**/
	NotifyParentWindowPositionChanged() => this.wvc.NotifyParentWindowPositionChanged()
	;-------------------------------------------------------------------------------------------
	;WebView2Core class assignments
	Settings => this.wv.Settings ;Returns Map() of Settings
		AreBrowserAcceleratorKeysEnabled { ;Boolean => Determines whether browser-specific accelerator keys are enabled
			get => this.Settings.AreBrowserAcceleratorKeysEnabled
			set => this.Settings.AreBrowserAcceleratorKeysEnabled := Value
		}
		AreDefaultContextMenusEnabled { ;Boolean => Determines whether the default context menus are shown to the user in WebView
			get => this.Settings.AreDefaultContextMenusEnabled
			set => this.Settings.AreDefaultContextMenusEnabled := Value
		}
		AreDefaultScriptDialogsEnabled { ;Boolean => Determines whether WebView renders the default JavaScript dialog box
			get => this.Settings.AreDefaultScriptDialogsEnabled
			set => this.Settings.AreDefaultScriptDialogsEnabled := Value
		}
		AreDevToolsEnabled { ;Boolean => Determines whether the user is able to use the context menu or keyboard shortcuts to open the DevTools window
			get => this.Settings.AreDevToolsEnabled
			set => this.Settings.AreDevToolsEnabled := Value
		}
		AreHostObjectsAllowed { ;Boolean => Determines whether host objects are accessible from the page in WebView
			get => this.Settings.AreHostObjectsAllowed
			set => this.Settings.AreHostObjectsAllowed := Value
		}
		HiddenPdfToolbarItems { ;Integer => Used to customize the PDF toolbar items
			/**
			 * None: 		 0
			 * Save: 		 1
			 * Print: 		 2
			 * SaveAs: 		 4
			 * ZoomIn: 		 8
			 * ZoomOut: 	 16
			 * Rotate: 		 32
			 * FitPage: 	 64
			 * PageLayout: 	 128
			 * Bookmarks: 	 256 ;This option is broken in the current runtime. See: https://github.com/MicrosoftEdge/WebView2Feedback/issues/2866
			 * PageSelector: 512
			 * Search: 		 1024
			 * FullScreen:   2048
			 * MoreSettings: 4096
			 * Add up numbers if you want to hide multiple items, Ex: 257 to hide Bookmarks and Save
			**/
			get => this.Settings.HiddenPdfToolbarItems
			set => this.Settings.HiddenPdfToolbarItems := Value
		}
		IsBuiltInErrorPageEnabled { ;Boolean => Determines whether to disable built in error page for navigation failure and render process failure
			get => this.Settings.IsBuiltInErrorPageEnabled
			set => this.Settings.IsBuiltInErrorPageEnabled := Value
		}
		IsGeneralAutofillEnabled { ;Boolean => Determines whether general form information will be saved and autofilled
			get => this.Settings.IsGeneralAutofillEnabled
			set => this.Settings.IsGeneralAutofillEnabled := Value
		}
		IsNonClientRegionSupportEnabled { ;Boolean => The IsNonClientRegionSupportEnabled property enables web pages to use the app-region CSS style
			get => this.Settings.IsNonClientRegionSupportEnabled
			set => this.Settings.IsNonClientRegionSupportEnabled := Value
		}
		IsPasswordAutosaveEnabled { ;Boolean => Determines whether password information will be autosaved
			get => this.Settings.IsPasswordAutosaveEnabled
			set => this.Settings.IsPasswordAutosaveEnabled := Value
		}
		IsPinchZoomEnabled { ;Boolean => Determines the ability of the end users to use pinching motions on touch input enabled devices to scale the web content in the WebView2
			get => this.Settings.IsPinchZoomEnabled
			set => this.Settings.IsPinchZoomEnabled := Value
		}
		IsReputationCheckingRequired { ;Boolean => Determines whether SmartScreen is enabled when visiting web pages
			get => this.Settings.IsReputationCheckingRequired
			set => this.Settings.IsReputationCheckingRequired := Value
		}
		IsScriptEnabled { ;Boolean => Determines whether running JavaScript is enabled in all future navigations in the WebView
			get => this.Settings.IsScriptEnabled
			set => this.Settings.IsScriptEnabled := Value
		}
		IsStatusBarEnabled { ;Boolean => Determines whether the status bar is displayed
			get => this.Settings.IsStatusBarEnabled
			set => this.Settings.IsStatusBarEnabled := Value
		}
		IsSwipeNavigationEnabled { ;Boolean => Determines whether the end user to use swiping gesture on touch input enabled devices to navigate in WebView2
			get => this.Settings.IsSwipeNavigationEnabled
			set => this.Settings.IsSwipeNavigationEnabled := Value
		}		
		IsWebMessageEnabled { ;Boolean => Determines whether communication from the host to the top-level HTML document of the WebView is allowed
			get => this.Settings.IsWebMessageEnabled
			set => this.Settings.IsWebMessageEnabled := Value
		}
		IsZoomControlEnabled { ;Boolean => Determines whether the user is able to impact the zoom of the WebView
			get => this.Settings.IsZoomControlEnabled
			set => this.Settings.IsZoomControlEnabled := Value
		}
		UserAgent { ;String => Determines WebView2's User Agent
			get => this.Settings.UserAgent
			set => this.Settings.UserAgent := Value
		}
		
	Source => this.wv.Source ;Returns Uri of current page
	Navigate(Uri) => this.wv.Navigate(Uri) ;Navigate to new Uri
	NavigateToString(HtmlContent) => this.wv.NavigateToString(HtmlContent) ;Navigate to text (essentially create a webpage from a string)
	AddScriptToExecuteOnDocumentCreatedAsync(JavaScript) => this.wv.AddScriptToExecuteOnDocumentCreatedAsync(JavaScript) ;Adds JavaScript to run when the DOM is created
	AddScriptToExecuteOnDocumentCreated(JavaScript) {
		AddScriptToExecuteOnDocumentCreatedPromise := this.wv.AddScriptToExecuteOnDocumentCreatedAsync(JavaScript)
		AddScriptToExecuteOnDocumentCreatedPromise.await2()
		return Trim(AddScriptToExecuteOnDocumentCreatedPromise.Result, "`"")
	}
	RemoveScriptToExecuteOnDocumentCreated(Id) => this.wv.RemoveScriptToExecuteOnDocumentCreated(Id)
	ExecuteScriptAsync(JavaScript) => this.wv.ExecuteScriptAsync(JavaScript) ;Execute code on the current Webpage
	ExecuteScript(JavaScript) {
		ExecuteScriptPromise := this.wv.ExecuteScriptAsync(JavaScript)
		ExecuteScriptPromise.await2()
		return Trim(ExecuteScriptPromise.Result, "`"")
	}
	CapturePreviewAsync(ImageFormat, ImageStream) => this.wv.CapturePreviewAsync(ImageFormat, ImageStream) ;Take a "screenshot" of the current WebView2 content
	CapturePreview(ImageFormat, ImageStream) {
		CapturePreviewPromise := this.wv.CapturePreviewAsync(ImageFormat, ImageStream)
		CapturePreviewPromise.await2()
		return CapturePreviewPromise.Result
	}
	Reload() => this.wv.Reload() ;Reloads the current page

	/**
	 * In order to use PostWebMessageAsJson() or PostWebMessageAsString(), you'll need to setup your webpage to listen to messages
	 * First, MyWindow.Settings.IsWebMessageEnabled must be set to true
	 * On your webpage itself, you'll need to setup an EventListner and Handler for the WebMessages
	 * 		window.chrome.webview.addEventListener('message', ahkWebMessage);
	 * 		function ahkWebMessage(Msg) {
	 * 			console.log(Msg);
	 * 		}
	**/
	PostWebMessageAsJson(WebMessageAsJson) => this.wv.PostWebMessageAsJson(WebMessageAsJson) ;Posts the specified JSON message to the top level document in this WebView
	PostWebMessageAsString(WebMessageAsString) => this.wv.PostWebMessageAsString(WebMessageAsString) ;Posts the specified STRING message to the top level document in this WebView
	CallDevToolsProtocolMethodAsync(MethodName, ParametersAsJson) => this.wv.CallDevToolsProtocolMethodAsync(MethodName, ParametersAsJson) ;Runs an DevToolsProtocol method
	BrowserProcessId => this.wv.BrowserProcessId ;Returns the process ID of the browser process that hosts the WebView2
	CanGoBack => this.wv.CanGoBack ;Returns true if the WebView is able to navigate to a previous page in the navigation history
	CanGoForward => this.wv.CanGoForward ;Returns true if the WebView is able to navigate to a next page in the navigation history
	GoBack() => this.wv.GoBack() ;GoBack to the previous page in the navigation history
	GoForward() => this.wv.GoForward() ;GoForward to the next page in the navigation history
	GetDevToolsProtocolEventReceiver(EventName) => this.wv.GetDevToolsProtocolEventReceiver(EventName) ;Gets a DevTools Protocol event receiver that allows you to subscribe to a DevToolsProtocol event
	Stop() => this.wv.Stop() ;Stops all navigations and pending resource fetches
	DocumentTitle => this.wv.DocumentTitle ;Returns the DocumentTitle of the current webpage
	AddHostObjectToScript(ObjName, Obj) => this.wv.AddHostObjectToScript(ObjName, Obj) ;Create object link between the WebView2 and the AHK Script
	RemoveHostObjectFromScript(ObjName) => this.wv.RemoveHostObjectFromScript(ObjName) ;Delete object link from the WebView2
	OpenDevToolsWindow() => this.wv.OpenDevToolsWindow() ;Opens DevTools for the current WebView2
	ContainsFullScreenElement => this.wv.ContainsFullScreenElement ;Returns true if the WebView contains a fullscreen HTML element
	AddWebResourceRequestedFilter(Uri, ResourceContext) => this.wv.AddWebResourceRequestedFilter(Uri, ResourceContext) ;Adds a URI and resource context filter for the WebResourceRequested event
	RemoveWebResourceRequestedFilter(Uri, ResourceContext) => this.wv.RemoveWebResourceRequestedFilter(Uri, ResourceContext) ;Removes a matching WebResource filter that was previously added for the WebResourceRequested event
	NavigateWithWebResourceRequest(Request) => this.wv.NavigateWithWebResourceRequest(Request) ;Navigates using a constructed CoreWebView2WebResourceRequest object
	CookieManager => this.wv.CookieManager ;Gets the CoreWebView2CookieManager object associated with this CoreWebView2
		GetCookiesAsync(Uri) => this.CookieManager.GetCookies(Uri) ;Gets a list of cookies matching the specific URI

	Environment => this.wv.Environment ;Returns Map() of Environment settings
		CreateCoreWebView2ControllerAsync(ParentWindow) => this.Environment.CreateWebView2ControllerAsync(ParentWindow)
		CreateWebResourceResponse(Content, StatusCode, ReasonPhrase, Headers) => this.Environment.CreateWebResourceResponse(Content, StatusCode, ReasonPhrase, Headers)
		BrowserVersionString => this.Environment.BrowserVersionString ;Returns the browser version info of the current CoreWebView2Environment, including channel name if it is not the stable channel
		FailureReportFolderPath => this.Environment.FailureReportFolderPath ;Returns the failure report folder that all CoreWebView2s created from this environment are using
		UserDataFolder => this.Environment.UserDataFolder ;Returns the user data folder that all CoreWebView2s created from this environment are using
		CreateWebResourceRequest(Uri, Method, PostData, Headers) => this.Environment.CreateWebResourceRequest(Uri, Method, PostData, Headers) ;Creates a new CoreWebView2WebResourceRequest object
		CreateCoreWebView2CompositionControllerAsync(ParentWindow) => this.Environment.CreateCoreWebView2CompositionControllerAsync(ParentWindow) ;Creates a new WebView for use with visual hosting
		CreateCoreWebView2PointerInfo() => this.Environment.CreateCoreWebView2PointerInfo() ;Returns Map() of a combined win32 POINTER_INFO, POINTER_TOUCH_INFO, and POINTER_PEN_INFO object
		GetAutomationProviderForWindow(Hwnd) => this.Environment.GetAutomationProviderForWindow(Hwnd) ;PRODUCES ERROR, REACH OUT TO THQBY
		CreatePrintSettings() => this.Environment.CreatePrintSettings() ;Creates the CoreWebView2PrintSettings used by the PrintToPdfAsync(String, CoreWebView2PrintSettings) method
		GetProcessInfos() => this.Environment.GetProcessInfos() ;Returns the list of all CoreWebView2ProcessInfo using same user data folder except for crashpad process
		CreateContextMenuItem(Label, IconStream, Kind) => this.Environment.CreateContextMenuItem(Label, IconStream, Kind) ;PRODUCES ERROR, REACH OUT TO THQBY
		CreateCoreWebView2ControllerOptions() => this.Environment.CreateCoreWebView2ControllerOptions() ;PRODUCES ERROR, REACH OUT TO THQBY
		CreateCoreWebView2ControllerWithOptionsAsync(ParentWindow, Options) => this.Environment.CreateCoreWebView2ControllerWithOptionsAsync(ParentWindow, Options) ;PRODUCES ERROR, REACH OUT TO THQBY -- I think the issue is part of the `CreateCoreWEbView2ControllerOptions()` method
		CreateCoreWebView2CompositionControllerWithOptionsAsync(ParentWindow, Options) => this.Environment.CreateCoreWebView2CompositionControllerWithOptionsAsync(ParentWindow, Options) ;PRODUCES ERROR, REACH OUT TO THQBY -- I think the issue is part of the `CreateCoreWEbView2ControllerOptions()` method
		CreateSharedBuffer(Size) => this.Environment.CreateSharedBuffer(Size) ;Create a shared memory based buffer with the specified size in bytes -- PRODUCES ERROR, REACH OUT TO THQBY
	
	TrySuspendAsync() => this.wv.TrySuspendAsync() ;Must set `IsVisible := 0` before trying to call
	Resume() => this.wv.Resume() ;Resumes the WebView so that it resumes activities on the web page. Will fail unless you set `IsVisible := 1`
	IsSuspended => this.wv.IsSuspended ;Returns true if the WebView is suspended
	SetVirtualHostNameToFolderMapping(HostName, FolderPath, AccessKind) => this.wv.SetVirtualHostNameToFolderMapping(HostName, FolderPath, AccessKind) ;Sets a mapping between a virtual host name and a folder path to make available to web sites via that host name
	ClearVirtualHostNameToFolderMapping(HostName) => this.wv.ClearVirtualHostNameToFolderMapping(HostName) ;Clears a host name mapping for local folder that was added by SetVirtualHostNameToFolderMapping()
	OpenTaskManagerWindow() => this.wv.OpenTaskManagerWindow() ;Opens the Browser Task Manager view as a new window in the foreground
	IsMuted { ;Indicates whether all audio output from this CoreWebView2 is muted or not. Set to true will mute this CoreWebView2, and set to false will unmute this CoreWebView2. true if audio is muted
		get => this.wv.IsMuted
		set => this.wv.IsMuted := Value
	}
	IsDocumentPlayingAudio => this.wv.IsDocumentPlayingAudio ;Returns true if audio is playing even if IsMuted is true
	IsDefaultDownloadDialogOpen => this.wv.IsDefaultDownloadDialogOpen ;Returns true if the default download dialog is currently open
	OpenDefaultDownloadDialog() => this.wv.OpenDefaultDownloadDialog() ;Opens the DownloadDialog Popup Window
	CloseDefaultDownloadDialog() => this.wv.CloseDefaultDownloadDialog() ;Closes the DownloadDialog Popup Window
	DefaultDownloadDialogCornerAlignment { ;Position of DownloadDialog does not update until after the WebView2 position or size has changed
		get => this.wv.DefaultDownloadDialogCornerAlignment ;Return the current corner the DownloadDialog will show up in (0 := TopLeft, 1 := TopRight, 2 := BottomLeft, 3 := BottomRight)
		set => this.wv.DefaultDownloadDialogCornerAlignment := Value ;Set the corner of the WebView2 that the DownloadDialog will show up in (0 := TopLeft, 1 := TopRight, 2 := BottomLeft, 3 := BottomRight)
	}
	DefaultDownloadDialogMargin { ;Working, but I don't know how to accurately assign a new Margin yet. We can assign one via an Integer, but it's hit and miss to get the position correct
		get => this.wv.DefaultDownloadDialogMargin
		set => this.wv.DefaultDownloadDialogMargin := Value
	}
	CallDevToolsProtocolMethodForSessionAsync(SessionId, MethodName, ParametersAsJson) => this.wv.CallDevToolsProtocolMethodForSessionAsync(SessionId, MethodName, ParametersAsJson) ;Runs a DevToolsProtocol method for a specific session of an attached target
	StatusBarText => this.wv.StatusBarText ;Returns the current text of the WebView2 StatusBar
	Profile => this.wv.Profile ;Returns the associated CoreWebView2Profile object of CoreWebView2
	ClearServerCertificateErrorActionsAsync() => this.wv.ClearServerCertificateErrorActionsAsync()
	FaviconUri => this.wv.FaviconUri ;Returns the Uri as a string of the current Favicon. This will be an empty string if the page does not have a Favicon
	GetFaviconAsync(Format) => this.wv.GetFaviconAsync(Format) ;Get the downloaded Favicon image for the current page and copy it to the image stream
	PrintAsync(PrintSettings) => this.wv.PrintAsync(PrintSettings) ;Print the current web page asynchronously to the specified printer with the provided settings
	PrintToPdfAsync(ResultFilePath, PrintSettings) => this.wv.PrintToPdfAsync(ResultFilePath, PrintSettings) ;Print the current page to PDF with the provided settings
	ShowPrintUI(PrintDialogKind) => this.wv.ShowPrintUI(PrintDialogKind) ;Opens the print dialog to print the current web page. Browser printDialogKind := 0, System printDialogKind := 1
	PrintToPdfStreamAsync(PrintSettings) => this.wv.PrintToPdfStreamAsync(PrintSettings) ;Provides the PDF data of current web page for the provided settings to a Stream
	PostSharedBufferToScript(SharedBuffer, Access, AdditionalDataAsJson) => this.wv.PostSharedBufferToScript(SharedBuffer, Access, AdditionalDataAsJson) ;Share a shared buffer object with script of the main frame in the WebView
	MemoryUsageTargetLevel { ;0 = Normal, 1 = Low; Low can be used for apps that are inactive to conserve memory usage
		get => this.wv.MemoryUsageTargetLevel
		set => this.wv.MemoryUsageTargetLevel := Value
	}
	
	;-------------------------------------------------------------------------------------------
	;Handler Assignments
	static PlaceholderHandler(Handler, ICoreWebView2, Args) {
		;MsgBox(handler, "WebviewWindow.PlaceholderHandler()", "262144")
	}

	;Controller
	ZoomFactorChanged(Handler) => this.wvc.add_ZoomFactorChanged(Handler)
	MoveFocusRequested(Handler) => this.wvc.add_MoveFocusRequested(Handler)
	GotFocus(Handler) => this.wvc.add_GotFocus(Handler)
	LostFocus(Handler) => this.wvc.add_LostFocus(Handler)
	AcceleratorKeyPressed(Handler) => this.wvc.add_AcceleratorKeyPressed(Handler)
	RasterizationScaleChanged(Handler) => this.wvc.add_RasterizationScaleChanged(Handler)

	;Core
	NavigationStarting(Handler) => this.wv.add_NavigationStarting(Handler)
	ContentLoading(Handler) => this.wv.add_ContentLoading(Handler)
	SourceChanged(Handler) => this.wv.add_SourceChanged(Handler)
	HistoryChanged(Handler) => this.wv.add_HistoryChanged(Handler)
	NavigationCompleted(Handler) => this.wv.add_NavigationCompleted(Handler)
	ScriptDialogOpening(Handler) => this.wv.add_ScriptDialogOpening(Handler)
	PermissionRequested(Handler) => this.wv.add_PermissionRequested(Handler)
	ProcessFailed(Handler) => this.wv.add_ProcessFailed(Handler)
	WebMessageReceived(Handler) => this.wv.add_WebMessageReceived(Handler)
	NewWindowRequested(Handler) => this.wv.add_NewWindowRequested(Handler)
	DocumentTitleChanged(Handler) => this.wv.add_DocumentTitleChanged(Handler)
	ContainsFullScreenElementChanged(Handler) => this.wv.add_ContainsFullScreenElementChanged(Handler)
	WebResourceRequested(Handler) => this.wv.add_WebResourceRequested(Handler)
	WindowCloseRequested(Handler) => this.wv.add_WindowCloseRequested(Handler)
	WebResourceResponseReceived(Handler) => this.wv.add_WebResourceResponseReceived(Handler)
	DOMContentLoaded(Handler) => this.wv.add_DOMContentLoaded(Handler)
	FrameCreated(Handler) => this.wv.add_FrameCreated(Handler)
	DownloadStarting(Handler) => this.wv.add_ownloadStarting(Handler)
	ClientCertificateRequested(Handler) => this.wv.add_ClientCertificateRequested(Handler)
	IsMutedChanged(Handler) => this.wv.add_IsMutedChanged(Handler)
	IsDocumentPlayingAudioChanged(Handler) => this.wv.add_IsDocumentPlayingAudioChanged(Handler)
	IsDefaultDownloadDialogOpenChanged(Handler) => this.wv.add_IsDefaultDownloadDialogOpenChanged(Handler)
	BasicAuthenticationRequested(Handler) => this.wv.add_BasicAuthenticationRequested(Handler)
	ContextMenuRequested(Handler) => this.wv.add_ContextMenuRequested(Handler)
	StatusBarTextChanged(Handler) => this.wv.add_StatusBarTextChanged(Handler)
	ServerCertificateErrorDetected(Handler) => this.wv.add_ServerCertificateErrorDetected(Handler)
	FaviconChanged(Handler) => this.wv.add_FaviconChanged(Handler)
	LaunchingExternalUriScheme(Handler) => this.wv.add_LaunchingExternalUriScheme(Handler)
	
	/**
	 * The following event handlers are commented out for the time being.
	 * Their assignments are not accurate and cannot be used directly,
	 * I'm leaving them here for easy tracking of event handlers and
	 * I may revisit fixing them in the future.
	**/
	/*
	;DownloadOperation
	BytesReceivedChanged(Handler) => this.BytesReceivedChangedHandler := this.do.BytesReceivedChanged(Handler)
	EstimatedEndTimeChanged(Handler) => this.EstimatedEndTimeChangedHandler := WebView2.Handler(Handler)
	StateChanged(Handler) => this.StateChangedHandler := WebView2.Handler(Handler)

	;Environment
	NewBrowserVersionAvailable(Handler) => this.NewBrowserVersionAvailableHandler := this.wv.NewBrowserVersionAvailable(Handler)
	BrowserProcessExited(Handler) => this.BrowserProcessExitedHandler := this.wv.BrowserProcessExited(Handler)
	ProcessInfosChanged(Handler) => this.ProcessInfosChangedHandler := this.wv.ProcessInfosChanged(Handler)

	;Frame
	FrameNameChanged(Handler) => this.FrameNameChangedHandler := this.wv.NameChanged(Handler)
	FrameDestroyed(Handler) => this.FrameDestroyedHandler := this.wv.FrameDestroyed(Handler)
	FrameNavigationStarting(Handler) => this.FrameNavigationStartingHandler := this.wv.FrameNavigationStarting(Handler)
	FrameNavigationCompleted(Handler) => this.FrameNavigationCompletedHandler := this.wv.FrameNavigationCompleted(Handler)
	FrameContentLoading(Handler) => this.FrameContentLoadingHandler := this.wv.FrameContentLoading(Handler)
	FrameDOMContentLoaded(Handler) => this.FrameDOMContentLoadedHandler := this.wv.FrameDOMContentLoaded(Handler)
	FrameWebMessageReceived(Handler) => this.FrameWebMessageReceivedHandler := this.wv.FrameWebMessageReceived(Handler)
	FramePermissionRequested(Handler) => this.FramePermissionRequestedHandler := this.wv.FramePermissionRequested(Handler)

	;Profile
	ClearBrowsingDataAll(Handler) => this.ClearBrowsingDataAllHandler := this.wv.ClearBrowsingDataAll(Handler)

	;CompositionController
	CursorChanged(Handler) => this.CursorChangedHandler := this.wv.CursorChanged(Handler)

	;ContextMenuItem
	CustomItemSelected(Handler) => this.CustomItemSelectedHandler := this.wv.CustomItemSelected(Handler)

	;DevToolsProtocolEventReceiver
	DevToolsProtocolEventReceived(Handler) => this.DevToolsProtocolEventReceivedHandler := this.wv.DevToolsProtocolEventReceived(Handler)

	;WebResourceResponseView
	GetContent(Handler) => this.GetContentHandler := this.wv.GetContent(Handler)
	*/
}