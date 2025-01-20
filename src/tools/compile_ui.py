import os
import subprocess
from pathlib import Path

def compile_ui():
    """编译UI文件为Python代码"""
    # 获取项目根目录 (从tools目录往上两级)
    root_dir = Path(__file__).parent.parent.parent
    
    # UI文件目录
    qt_dir = root_dir / 'src' / 'qt'
    # 生成的Python文件目录
    ui_dir = root_dir / 'src' / 'ui'
    
    print(f'项目根目录: {root_dir}')
    print(f'UI文件目录: {qt_dir}')
    print(f'生成目录: {ui_dir}')
    
    # 确保ui目录存在
    ui_dir.mkdir(exist_ok=True)
    
    # 遍历所有.ui文件
    for ui_file in qt_dir.glob('*.ui'):
        # 生成的py文件名去掉.ui后缀，加上ui_前缀
        py_filename = f'ui_{ui_file.stem}.py'
        py_file = ui_dir / py_filename
        
        print(f'正在编译: {ui_file.name} -> {py_filename}')
        
        try:
            # 使用pyside6-uic命令编译UI文件
            subprocess.run(['pyside6-uic', str(ui_file), '-o', str(py_file)], check=True)
            print(f'编译成功: {py_filename}')
        except subprocess.CalledProcessError as e:
            print(f'编译失败: {ui_file.name}')
            print(f'错误信息: {str(e)}')
        except FileNotFoundError:
            print(f'错误: 找不到pyside6-uic，请确保已安装PySide6')
            print('可以使用以下命令安装：pip install PySide6')
            break

if __name__ == '__main__':
    compile_ui() 
