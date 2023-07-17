; 按键转换为只有扫描码
Key2NoVkSC(key){
    sc := GetKeySC(key)
    return Format("vkFFsc{1:02X}", sc)
}

; 按键转换为只有虚拟码
Key2VKNoSC(key){
    vk := GetKeyVK(key)
    return Format("vk{1:02X}scFF", vk)
}

; 按键转换扫描码
Key2SC(key){
    sc := GetKeySC(key)
    return Format("sc{1:02X}", sc)
}

; 按键转换为虚拟码
Key2VK(key){
    vk := GetKeyVK(key)
    return Format("vk{1:02X}", vk)
}