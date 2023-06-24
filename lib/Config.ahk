SaveConfig(type, key, value) {
	IniWrite, %value%, config.ini, %type%, %key%
}

LoadConfig(type, key) {
	IniRead, value, config.ini, %type%, %key%
	return value
}

SavePresetConfig(presetsName, keys){
	keysString := ""
	for k,v in keys
	{
		keysString := keysString  . v . "|"
	}
	keysString := SubStr(keysString, 1, StrLen(keysString) - 1)
	IniWrite, %keysString%, config.ini, Preset, %presetsName%
}

LoadPresetConfig(presetsName){
	IniRead, config, config.ini, Preset, %presetsName%
	keys := []
	loop, Parse, config, `|
	{
		keys.Insert(A_LoopField)
	}
	return keys
}

DeletePresetConfig(presetsName){
	IniDelete,config.ini, Preset, %presetsName%
}

LoadPresetList(){
	IniRead, config, config.ini, Preset
	presetList := ""
	loop, Parse, config, `n, `r 
	{
		a := A_LoopField
		b := []
		loop, Parse, a, =
		{
			b.Insert(A_LoopField)
		}
		str := b[1]
		presetList := presetList  . str . "|"
	}
	presetList := SubStr(presetList, 1, StrLen(presetList) - 1)
	return presetList
}

GetDefaultPresetName(){
	IniRead, config, config.ini, Preset
	preset := ""
	presets := []
	loop, Parse, config, `n, `r
	{
		a := A_LoopField
		b := []
		loop, Parse, a, =
		{
			b.Insert(A_LoopField)
		}
		str := b[1]
		presets.Insert(str)
	}
	preset := presets[1]
	return preset
}