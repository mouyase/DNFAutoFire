class KeyCodes:
    # 字母键
    A = 0x1E
    B = 0x30
    C = 0x2E
    D = 0x20
    E = 0x12
    F = 0x21
    G = 0x22
    H = 0x23
    I = 0x17
    J = 0x24
    K = 0x25
    L = 0x26
    M = 0x32
    N = 0x31
    O = 0x18
    P = 0x19
    Q = 0x10
    R = 0x13
    S = 0x1F
    T = 0x14
    U = 0x16
    V = 0x2F
    W = 0x11
    X = 0x2D
    Y = 0x15
    Z = 0x2C

    # 数字键（键盘上方）
    KEY_1 = 0x02
    KEY_2 = 0x03
    KEY_3 = 0x04
    KEY_4 = 0x05
    KEY_5 = 0x06
    KEY_6 = 0x07
    KEY_7 = 0x08
    KEY_8 = 0x09
    KEY_9 = 0x0A
    KEY_0 = 0x0B

    # 功能键
    F1 = 0x3B
    F2 = 0x3C
    F3 = 0x3D
    F4 = 0x3E
    F5 = 0x3F
    F6 = 0x40
    F7 = 0x41
    F8 = 0x42
    F9 = 0x43
    F10 = 0x44
    F11 = 0x57
    F12 = 0x58

    # 特殊键
    ESC = 0x01
    TAB = 0x0F
    CAPS_LOCK = 0x3A
    LEFT_SHIFT = 0x2A
    RIGHT_SHIFT = 0x36
    LEFT_CTRL = 0x1D
    RIGHT_CTRL = 0x9D
    LEFT_ALT = 0x38
    RIGHT_ALT = 0xB8
    SPACE = 0x39
    ENTER = 0x1C
    BACKSPACE = 0x0E

    # 方向键
    UP = 0xC8
    DOWN = 0xD0
    LEFT = 0xCB
    RIGHT = 0xCD

    # 编辑键
    INSERT = 0xD2
    DELETE = 0xD3
    HOME = 0xC7
    END = 0xCF
    PAGE_UP = 0xC9
    PAGE_DOWN = 0xD1

    # 小键盘
    NUMPAD_0 = 0x52
    NUMPAD_1 = 0x4F
    NUMPAD_2 = 0x50
    NUMPAD_3 = 0x51
    NUMPAD_4 = 0x4B
    NUMPAD_5 = 0x4C
    NUMPAD_6 = 0x4D
    NUMPAD_7 = 0x47
    NUMPAD_8 = 0x48
    NUMPAD_9 = 0x49
    NUMPAD_MULTIPLY = 0x37
    NUMPAD_ADD = 0x4E
    NUMPAD_SUBTRACT = 0x4A
    NUMPAD_DECIMAL = 0x53
    NUMPAD_DIVIDE = 0xB5
    NUMPAD_ENTER = 0x9C

    # 其他常用键
    MINUS = 0x0C
    EQUALS = 0x0D
    LEFT_BRACKET = 0x1A
    RIGHT_BRACKET = 0x1B
    SEMICOLON = 0x27
    APOSTROPHE = 0x28
    GRAVE = 0x29  # `键
    BACKSLASH = 0x2B
    COMMA = 0x33
    PERIOD = 0x34
    SLASH = 0x35

    @classmethod
    def get_key_name(cls, scan_code: int) -> str:
        """
        根据扫描码获取按键名称
        """
        for key, value in vars(cls).items():
            if not key.startswith('_') and value == scan_code:
                return key
        return f"UNKNOWN_{hex(scan_code)}" 