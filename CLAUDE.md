# CLAUDE.md

本文件为 Claude Code (claude.ai/code) 提供本仓库的开发指南。

## 快速参考

详细配置请查看 JSON 文档（AI 读取效果更好）：

- `docs/project-config.json` - 项目配置、技术栈、命令
- `docs/architecture.json` - 架构设计、模块结构、Tauri 命令
- `docs/dev-notes.json` - 踩坑记录、方案对比

## 开发规范

```json
{
  "语言": { "文档": "中文", "注释": "中文", "代码": "英文" },
  "文件命名": "kebab-case",
  "工作流程": ["设计先行", "反复校验", "零警告"],
  "代码原则": ["高内聚低耦合", "遵循官方最佳实践"]
}
```

## 常用命令

```bash
# 初始化
corepack enable && corepack prepare pnpm@latest --activate
pnpm install

# 开发
pnpm tauri dev      # 完整开发模式
pnpm dev            # 仅前端

# 代码质量
pnpm lint           # oxlint + eslint
pnpm format         # oxfmt 格式化
pnpm typecheck      # TypeScript 检查

# Rust
cd src-tauri && cargo clippy -- -D warnings
cd src-tauri && cargo fmt
```

## 技术栈

| 类别 | 工具 |
|------|------|
| 前端 | React 19 + Vite 6 + Tailwind 4 |
| 后端 | Tauri 2 + Rust |
| 检查 | oxlint (类型感知) + ESLint |
| 格式化 | oxfmt |
| 包管理 | pnpm |

## 项目结构

```
src/                          # React 前端
├── components/
│   ├── keyboard/            # 键盘组件
│   └── layout/              # 布局组件
├── hooks/                   # 自定义 Hooks
├── lib/                     # 工具函数
└── types/                   # 类型定义

src-tauri/src/               # Rust 后端
├── lib.rs                   # Tauri 命令
└── core/                    # 核心模块
    ├── autofire.rs          # 连发引擎
    ├── keyboard.rs          # 键盘输入
    └── window.rs            # 窗口检测

docs/                        # 文档 (JSON 格式)
├── project-config.json      # 项目配置
├── architecture.json        # 架构设计
└── dev-notes.json           # 开发笔记
```

## 运行要求

- Windows + 管理员权限（键盘钩子需要）
