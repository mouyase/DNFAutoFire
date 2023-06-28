; 按键转换为只有扫描码
Key2NoVkSC(key){
    sc := GetKeySC(key)
    return Format("vkFFsc{1:02X}", sc)
}

; 按键转换为只有虚拟码
Key2VkNoSC(key){
    vk := GetKeyVK(key)
    return Format("vk{1:02X}scFF", vk)
}