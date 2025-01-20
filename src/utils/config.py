import json
from typing import Set, List, Optional
from pathlib import Path

class Config:
    def __init__(self):
        # 配置文件路径（当前目录下的configs.json）
        self.config_file = Path('configs.json')
        
        # 初始化配置文件
        if not self.config_file.exists():
            self._save_config_data({
                'enabled_keys': [],
                'configs': {}
            })

    def _load_config_data(self) -> dict:
        """加载配置文件"""
        try:
            with open(self.config_file, 'r', encoding='utf-8') as f:
                return json.load(f)
        except Exception:
            return {'enabled_keys': [], 'configs': {}}

    def _save_config_data(self, data: dict) -> None:
        """保存配置文件"""
        try:
            with open(self.config_file, 'w', encoding='utf-8') as f:
                json.dump(data, f, indent=4)
        except Exception:
            pass

    def load_enabled_keys(self) -> Set[int]:
        """加载已启用的按键"""
        data = self._load_config_data()
        return set(data.get('enabled_keys', []))

    def save_enabled_keys(self, keys: Set[int]) -> None:
        """保存已启用的按键"""
        data = self._load_config_data()
        data['enabled_keys'] = list(keys)
        self._save_config_data(data)

    def get_config_list(self) -> List[str]:
        """获取所有配置名称列表"""
        data = self._load_config_data()
        return sorted(data.get('configs', {}).keys())

    def load_config(self, name: str) -> Optional[Set[int]]:
        """加载指定名称的配置"""
        data = self._load_config_data()
        config = data.get('configs', {}).get(name)
        return set(config) if config is not None else None

    def save_config(self, name: str, keys: Set[int]) -> bool:
        """保存配置"""
        try:
            data = self._load_config_data()
            if 'configs' not in data:
                data['configs'] = {}
            data['configs'][name] = list(keys)
            self._save_config_data(data)
            return True
        except Exception:
            return False

    def delete_config(self, name: str) -> bool:
        """删除配置"""
        try:
            data = self._load_config_data()
            if name in data.get('configs', {}):
                del data['configs'][name]
                self._save_config_data(data)
            return True
        except Exception:
            return False 