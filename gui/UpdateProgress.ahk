Gui UpdateProgress:-MinimizeBox -MaximizeBox -SysMenu -Caption +Owner%A_DefaultGui% +0x800000
Gui UpdateProgress:Add, Progress, w200 h20 vUpdateProgress, 0

ShowGuiUpdateProgress(){
    Gui UpdateProgress:Show
    DisableGuiMain()
}

; 设置进度条进度数据
UpdateProgressSetData(data){
    GuiControl UpdateProgress:, UpdateProgress, %data%
}