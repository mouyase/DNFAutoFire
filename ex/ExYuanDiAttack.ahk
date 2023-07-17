ExYuanDiAttack(){
    Process, Priority,, High
    presetName := LoadLastPreset()
    if(LoadPreset(LoadLastPreset(),"YuanDiAttackState")){
        skillKey := LoadPreset(presetName, "YuanDiAttackSkillKey")
        leftKey := LoadPreset(presetName, "YuanDiAttackLeftKey")
        rightKey := LoadPreset(presetName, "YuanDiAttackRightKey")
        delay1 := LoadPreset(presetName, "YuanDiAttackDelay1")
        delay2 := LoadPreset(presetName, "YuanDiAttackDelay2")
        delay3 := LoadPreset(presetName, "YuanDiAttackDelay3")
        counterTime1 := delay1
        counterTime2 := delay2
        counterTime3 := delay3 - delay2
        skillKeycode := Key2NoVkSC(skillKey)
        leftKeycode := Key2NoVkSC(leftKey)
        rightKeycode := Key2NoVkSC(rightKey)
        direction := "无"
        loop {
            if(WinActive("ahk_group DNF")) {
                while, GetKeyState(skillKey, "P"){
                    if(GetKeyState(leftKey, "P")){
                        direction := "面向左"
                    }
                    if(GetKeyState(rightKey, "P")){
                        direction := "面向右"
                    }
                    if(direction != "无"){
                        counterTime := 0
                        time := A_TickCount
                        while, counterTime < counterTime1{
                            SendIP(skillKeycode)
                            counterTime := A_TickCount - time
                            if(!GetKeyState(skillKey)){
                                SendInput, {Blind}{%rightKeycode% Up}
                                SendInput, {Blind}{%leftKeycode% Up}
                                break, 2
                            }
                        }
                        if(direction == "面向左"){
                            SendInput, {Blind}{%leftKeycode% Up}
                            SendInput, {Blind}{%rightKeycode% DownTemp}
                        }
                        if(direction == "面向右"){
                            SendInput, {Blind}{%rightKeycode% Up}
                            SendInput, {Blind}{%leftKeycode% DownTemp}
                        }
                        while, counterTime < counterTime2{
                            SendIP(skillKeycode)
                            counterTime := A_TickCount - time
                            if(!GetKeyState(skillKey)){
                                SendInput, {Blind}{%rightKeycode% Up}
                                SendInput, {Blind}{%leftKeycode% Up}
                                break, 2
                            }
                        }
                        if(direction == "面向左"){
                            SendInput, {Blind}{%rightKeycode% Up}
                        }
                        if(direction == "面向右"){
                            SendInput, {Blind}{%leftKeycode% Up}
                        }
                        if(!GetKeyState(skillKey)){
                            break
                        }
                        Sleep, counterTime3
                    }else{
                        SendIP(skillKeycode)
                    }
                }
                direction := "无"
            }
            Sleep, 1
        }
    }
}

ExYuanDiAttackSettingTool(){
    Process, Priority,, High
    log("啊？")
    presetName := LoadLastPreset()
    if(LoadPreset(LoadLastPreset(),"YuanDiAttackState")){
        skillKey := LoadPreset(LoadLastPreset(), "YuanDiAttackSkillKey")
        keyCode := Key2NoVkSC(skillKey)
        loop {
            if(WinActive("ahk_group DNF")) {
                while, GetKeyState(skillKey, "P") {
                    SendIP(keyCode)
                }
            }
            Sleep, 1
        }
    }
}

global _YuanDiAttackSettingToolThread

ExYuanDiAttackGetDelayTime(){
    global _YuanDiAttackSettingToolThread
    global YuanDiAttackSkillKey
    global YuanDiAttackLeftKey
    Gui YuanDiAttack:Submit, NoHide
    skillKey := GetOriginKeyName(YuanDiAttackSkillKey)
    _YuanDiAttackSettingToolThread := new Thread(skillKey)
    isNeedGetAttackDown := true
    isNeedGetLeftDown := false
    isNeedGetLeftUp := false
    ShowTip("开始检测平X方向延迟")
    loop{
        if(WinActive("ahk_group DNF")) {
            while, GetKeyState(YuanDiAttackSkillKey, "P") {
                if(isNeedGetAttackDown){
                    time1 := A_TickCount
                    isNeedGetAttackDown := false
                    isNeedGetLeftDown := true
                }
                if(GetKeyState(YuanDiAttackLeftKey, "P")){
                    if(isNeedGetLeftDown){
                        time2 := A_TickCount
                        isNeedGetLeftDown := false
                        isNeedGetLeftUp := true
                    }
                }else{
                    if(isNeedGetLeftUp){
                        time3 := A_TickCount
                        isNeedGetLeftUp := false
                    }
                }
            }
            if(time1 > 0 && time2 > 0 && time3 > 0){
                break
            }
        }
        Sleep, 1
        if (GetKeyState("``", "P")){
            break
        }
    }
    delay1 := Floor((time2 - time1)/10)*10
    delay2 := Floor((time3 - time1)/10)*10
    GuiControl YuanDiAttack:, YuanDiAttackDelay1, %delay1%
    GuiControl YuanDiAttack:, YuanDiAttackDelay2, %delay2%
    _YuanDiAttackSettingToolThread := ""
}

ExYuanDiAttackGetLoopTime(){
    global _YuanDiAttackSettingToolThread
    global YuanDiAttackSkillKey
    Gui YuanDiAttack:Submit, NoHide
    skillKey := GetOriginKeyName(YuanDiAttackSkillKey)
    _YuanDiAttackSettingToolThread := new Thread(skillKey)
    isNeedGetAttackDown := true
    isNeedGetAttackUp := false
    ShowTip("开始检测平X每轮时间")
    loop{
        if(WinActive("ahk_group DNF")) {
            while, GetKeyState(YuanDiAttackSkillKey, "P") {
                if(isNeedGetAttackDown){
                    time1 := A_TickCount
                    isNeedGetAttackDown := false
                    isNeedGetAttackUp := true
                }
            }
            if(isNeedGetAttackUp){
                time2 := A_TickCount
                isNeedGetAttackUp := false
            }
            if(time1 > 0 && time2 > 0){
                break
            }
        }
        Sleep, 1
        if (GetKeyState("``", "P")){
            break
        }
    }
    delay3 := Ceil((Ceil((time2 - time1) / 3))/10)*10
    GuiControl YuanDiAttack:, YuanDiAttackDelay3, %delay3%
    _YuanDiAttackSettingToolThread := ""
}