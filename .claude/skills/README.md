# DNF AutoFire - Claude Code Skills

本项目包含 4 个自定义 Claude Code Skills，用于简化 Rust 代码质量管理工作流程。

## 📋 Skills 列表

| Skill | 命令 | 功能 | 使用时机 |
|-------|------|------|----------|
| 🎨 **format** | `/format` | 格式化所有 Rust 代码 | 每次编辑后 |
| 🔍 **lint** | `/lint` | Clippy 零警告检查 | 每次编辑后 |
| 🔧 **fix** | `/fix` | 自动修复格式和 Clippy 问题 | 发现问题时 |
| ✅ **check** | `/check` | 完整质量检查流程 | 提交代码前 |

## 🚀 快速开始

在 Claude Code 对话中直接输入：

```
/format
/lint
/fix
/check
```

## 📖 详细说明

### `/format` - 格式化代码

**功能**：使用 `cargo fmt` 格式化所有 Rust 代码

**执行**：
```bash
cargo fmt --all
```

**适用场景**：
- 每次编辑代码后
- 保存文件后
- 提交代码前

---

### `/lint` - 代码质量检查

**功能**：使用 `cargo clippy` 进行零警告检查

**执行**：
```bash
cargo clippy --all-targets --all-features -- -D warnings
```

**检查内容**：
- 潜在 bug 和逻辑错误
- 性能问题
- 代码风格和可读性
- Rust 最佳实践

**适用场景**：
- 每次编辑代码后
- 发现问题及时修复
- 学习 Rust 最佳实践

---

### `/fix` - 自动修复

**功能**：自动格式化 + 自动修复 Clippy 问题

**执行**：
```bash
cargo fmt --all
cargo clippy --fix --allow-dirty --allow-staged
```

**注意事项**：
- 可能修改代码逻辑
- 修复后建议检查 git diff
- 运行测试确保功能正常

**适用场景**：
- 发现格式或 lint 问题时
- 批量清理代码
- 快速规范化代码

---

### `/check` - 完整检查

**功能**：运行完整的代码质量检查流程

**执行步骤**：
1. 格式化检查（不修改代码）
2. Clippy 零警告检查
3. 运行所有测试
4. Release 版本编译

**质量标准**：
- ✅ 代码已格式化
- ✅ Clippy 零警告
- ✅ 所有测试通过
- ✅ Release 编译成功

**适用场景**：
- 提交代码前
- 创建 Pull Request 前
- 发布新版本前

## 🔄 推荐工作流程

### 日常开发循环
```
编辑代码 → /format → /lint → 继续开发
```

### 提交前验证
```
/check
```

如果检查失败：
```
/fix → /check
```

## ⚙️ 配置文件

这些 Skills 使用以下配置文件：

- `rustfmt.toml` - 格式化规则
- `.clippy.toml` - Clippy 检查规则
- `.editorconfig` - 编辑器配置

## 🎯 零警告策略

本项目采用**零警告策略**：
- 所有 Clippy 警告视为错误
- 必须修复才能提交
- 保证代码质量持续改进

## 📦 Skills 结构

```
.claude/skills/
├── format/
│   └── SKILL.md
├── lint/
│   └── SKILL.md
├── fix/
│   └── SKILL.md
├── check/
│   └── SKILL.md
└── README.md (本文件)
```

每个 Skill 的 `SKILL.md` 遵循 Claude Code 标准格式：
- YAML frontmatter（name、description）
- 详细的功能说明
- 使用场景和最佳实践

## 🔗 等价命令

如果不使用 Skills，也可以手动运行：

```bash
# 格式化
cargo fmt --all

# Lint 检查
cargo clippy --all-targets --all-features -- -D warnings

# 自动修复
cargo fmt --all && cargo clippy --fix --allow-dirty

# 完整检查
.\scripts\check.ps1  # Windows
cargo make check      # cargo-make
```

## 📚 了解更多

- 查看各个 Skill 的 `SKILL.md` 获取详细说明
- 阅读 `docs/development-workflow.md` 了解完整开发流程
- 参考主 `README.md` 了解项目整体信息

## 💡 提示

- Skills 会被 Claude 自动识别和调用
- 也可以在对话中提及"格式化代码"、"运行 lint" 等触发
- 每次编辑代码后记得运行 `/format` 和 `/lint`
- 提交前必须运行 `/check` 确保所有检查通过
