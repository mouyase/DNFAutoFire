import { Button } from "@heroui/button";
import { useState } from "react";

interface KeyProps {
    width?: number;  // 按键宽度倍数
    label: string;   // 按键显示文本
    enabled?: boolean;
    onClick?: () => void;
}

const Key = ({ label,  enabled = false, onClick }: KeyProps) => {
    return (
        <Button
            className={'h-9 w-9'}
            color={enabled ? "success" : "default"}
            variant={"solid"}
            onPress={onClick}
            size="sm"
        >
            {label}
        </Button>
    );
};

export const KeyboardLayout = () => {
    const [activeKeys, setActiveKeys] = useState<Set<string>>(new Set());

    const toggleKey = (key: string) => {
        const newActiveKeys = new Set(activeKeys);
        if (newActiveKeys.has(key)) {
            newActiveKeys.delete(key);
        } else {
            newActiveKeys.add(key);
        }
        setActiveKeys(newActiveKeys);
    };

    // 键盘布局数据
    const keyboardLayout = {
        functionRow: [
            { label: "Esc" }, { label: "F1" }, { label: "F2" }, { label: "F3" }, { label: "F4" },
            { label: "F5" }, { label: "F6" }, { label: "F7" }, { label: "F8" },
            { label: "F9" }, { label: "F10" }, { label: "F11" }, { label: "F12" }
        ],
        numberRow: [
            { label: "`" }, { label: "1" }, { label: "2" }, { label: "3" }, { label: "4" },
            { label: "5" }, { label: "6" }, { label: "7" }, { label: "8" }, { label: "9" },
            { label: "0" }, { label: "-" }, { label: "=" }, { label: "Backspace", width: 2 }
        ],
        qwertyRow: [
            { label: "Tab", width: 1.5 }, { label: "Q" }, { label: "W" }, { label: "E" },
            { label: "R" }, { label: "T" }, { label: "Y" }, { label: "U" }, { label: "I" },
            { label: "O" }, { label: "P" }, { label: "[" }, { label: "]" },
            { label: "\\", width: 1.5 }
        ],
        asdfRow: [
            { label: "Caps", width: 1.75 }, { label: "A" }, { label: "S" }, { label: "D" },
            { label: "F" }, { label: "G" }, { label: "H" }, { label: "J" }, { label: "K" },
            { label: "L" }, { label: ";" }, { label: "'" }, { label: "Enter", width: 2.25 }
        ],
        zxcvRow: [
            { label: "Shift", width: 2.25 }, { label: "Z" }, { label: "X" }, { label: "C" },
            { label: "V" }, { label: "B" }, { label: "N" }, { label: "M" }, { label: "," },
            { label: "." }, { label: "/" }, { label: "Shift", width: 2.75 }
        ],
        spaceRow: [
            { label: "Ctrl", width: 1.25 }, { label: "Win", width: 1.25 },
            { label: "Alt", width: 1.25 }, { label: "Space", width: 6.25 },
            { label: "Alt", width: 1.25 }, { label: "Win", width: 1.25 },
            { label: "Menu", width: 1.25 }, { label: "Ctrl", width: 1.25 }
        ]
    };

    return (
        <div className="p-4 select-none">
            {/* 功能键行 */}
            <div className="flex gap-0.5 mb-2">
                {keyboardLayout.functionRow.map((key, index) => (
                    <Key
                        key={`fn-${index}`}
                        label={key.label}
                        enabled={activeKeys.has(key.label)}
                        onClick={() => toggleKey(key.label)}
                    />
                ))}
            </div>

            {/* 数字键行 */}
            <div className="flex gap-0.5 mb-0.5">
                {keyboardLayout.numberRow.map((key, index) => (
                    <Key
                        key={`num-${index}`}
                        label={key.label}
                        width={key.width}
                        enabled={activeKeys.has(key.label)}
                        onClick={() => toggleKey(key.label)}
                    />
                ))}
            </div>

            {/* QWERTY 行 */}
            <div className="flex gap-0.5 mb-0.5">
                {keyboardLayout.qwertyRow.map((key, index) => (
                    <Key
                        key={`qwerty-${index}`}
                        label={key.label}
                        width={key.width}
                        enabled={activeKeys.has(key.label)}
                        onClick={() => toggleKey(key.label)}
                    />
                ))}
            </div>

            {/* ASDF 行 */}
            <div className="flex gap-0.5 mb-0.5">
                {keyboardLayout.asdfRow.map((key, index) => (
                    <Key
                        key={`asdf-${index}`}
                        label={key.label}
                        width={key.width}
                        enabled={activeKeys.has(key.label)}
                        onClick={() => toggleKey(key.label)}
                    />
                ))}
            </div>

            {/* ZXCV 行 */}
            <div className="flex gap-0.5 mb-0.5">
                {keyboardLayout.zxcvRow.map((key, index) => (
                    <Key
                        key={`zxcv-${index}`}
                        label={key.label}
                        width={key.width}
                        enabled={activeKeys.has(key.label)}
                        onClick={() => toggleKey(key.label)}
                    />
                ))}
            </div>

            {/* 空格行 */}
            <div className="flex gap-0.5">
                {keyboardLayout.spaceRow.map((key, index) => (
                    <Key
                        key={`space-${index}`}
                        label={key.label}
                        width={key.width}
                        enabled={activeKeys.has(key.label)}
                        onClick={() => toggleKey(key.label)}
                    />
                ))}
            </div>
        </div>
    );
}; 
