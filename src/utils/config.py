import json
from pathlib import Path
from typing import Set, Dict, Any


class Config:
    """配置管理类"""
    
    def __init__(self) -> None:
        # 配置文件保存在当前目录
        self.config_file = Path('./configs.json')
        
        # 默认配置
        self.default_config: Dict[str, Any] = {
            'enabled_keys': [],  # 启用的按键扫描码列表
        }

    def load(self) -> Dict[str, Any]:
        """加载完整配置"""
        try:
            if self.config_file.exists():
                with open(self.config_file, 'r', encoding='utf-8') as f:
                    config = json.load(f)
                # 确保所有默认配置项都存在
                return {**self.default_config, **config}
        except Exception as e:
            print(f"加载配置文件失败: {e}")
        
        return self.default_config.copy()

    def save(self, config: Dict[str, Any]) -> None:
        """保存完整配置"""
        try:
            with open(self.config_file, 'w', encoding='utf-8') as f:
                json.dump(config, f, ensure_ascii=False, indent=2)
        except Exception as e:
            print(f"保存配置文件失败: {e}")

    def load_enabled_keys(self) -> Set[int]:
        """加载已启用的按键列表"""
        config = self.load()
        return set(config['enabled_keys'])

    def save_enabled_keys(self, enabled_keys: Set[int]) -> None:
        """保存已启用的按键列表"""
        config = self.load()
        config['enabled_keys'] = list(enabled_keys)
        self.save(config)


def get_config() -> Config:
    """获取配置实例（单例模式）"""
    if not hasattr(get_config, '_instance'):
        get_config._instance = Config()
    return get_config._instance 