import json
import os

class Config:
    def __init__(self):
        self.config_file = "configs.json"
        # 存储多套配置的字典
        self.configs = {}
        # 当前配置
        self.current_config = {
            'enabled_keys': [],    # 启用连发的按键列表
            'repeat_delay': 100,   # 按键重复延迟
            'repeat_interval': 50, # 按键重复间隔
        }
        # 确保加载已有配置
        self.load_configs()

    def load_configs(self):
        """加载所有配置"""
        if os.path.exists(self.config_file):
            try:
                with open(self.config_file, 'r', encoding='utf-8') as f:
                    self.configs = json.load(f)
            except Exception as e:
                print(f"加载配置失败: {e}")
                self.configs = {}

    def save_configs(self):
        """保存所有配置"""
        try:
            with open(self.config_file, 'w', encoding='utf-8') as f:
                json.dump(self.configs, f, ensure_ascii=False, indent=4)
            return True
        except Exception as e:
            print(f"保存配置失败: {e}")
            return False

    def save_current_config(self, name):
        """保存当前配置"""
        self.configs[name] = self.current_config.copy()
        return self.save_configs()

    def load_config(self, name):
        """加载指定配置"""
        if name in self.configs:
            self.current_config = self.configs[name].copy()
            return True
        return False

    def delete_config(self, name):
        """删除指定配置"""
        if name in self.configs:
            del self.configs[name]
            return self.save_configs()
        return False

    def get_config_names(self):
        """获取所有配置名称"""
        return list(self.configs.keys()) 