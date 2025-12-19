# 开发工作流程

本文档说明 DNF AutoFire 项目的开发工作流程和代码质量要求。

## 核心原则

1. **零警告政策** - 所有代码必须通过 `cargo clippy` 的零警告检查
2. **统一格式** - 所有代码必须经过 `cargo fmt` 格式化
3. **提交前检查** - 每次提交前必须运行完整检查

## 每次编辑代码后的流程

### 必须步骤（每次编辑后）

```bash
# 1. 格式化代码
cargo fmt --all

# 2. 运行 Clippy 检查
cargo clippy --all-targets --all-features -- -D warnings
```

或者使用快捷脚本：
```powershell
.\scripts\format.ps1
```

### 提交前检查

在提交代码前，运行完整检查：

```powershell
.\scripts\check.ps1
```

这会自动执行：
1. ✅ 格式化检查
2. ✅ Clippy 检查（零警告）
3. ✅ 运行所有测试
4. ✅ Release 编译

## 使用 cargo-make（推荐）

### 安装

```bash
cargo install cargo-make
```

### 常用命令

```bash
# 格式化代码
cargo make format

# 运行 Clippy
cargo make clippy

# 完整检查（格式化 + Clippy + 测试 + 编译）
cargo make check

# 自动修复问题
cargo make fix

# 提交前检查
cargo make pre-commit

# 开发模式（格式化 + Clippy + 运行）
cargo make dev
```

## 配置文件说明

### rustfmt.toml

Rust 代码格式化配置：
- 最大行宽：100 字符
- 缩进：4 空格
- 使用稳定特性

### .clippy.toml

Clippy lint 配置：
- 认知复杂度阈值：30
- 避免破坏性 API 更改

### .editorconfig

编辑器通用配置：
- UTF-8 编码
- LF 换行符
- 自动移除行尾空格
- 文件末尾空行

## 集成到编辑器

### VS Code

安装扩展：
- `rust-analyzer` - Rust 语言支持
- `EditorConfig for VS Code` - EditorConfig 支持

配置 settings.json：
```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.checkOnSave.extraArgs": ["--", "-D", "warnings"],
  "editor.formatOnSave": true,
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer",
    "editor.formatOnSave": true
  }
}
```

### 其他编辑器

确保编辑器支持：
1. EditorConfig
2. Rust 语言服务器（rust-analyzer）
3. 保存时自动格式化

## 常见问题

### Q: 为什么 clippy 要零警告？

A: 零警告政策确保代码质量，避免累积技术债务。Clippy 的警告通常指出真实的问题或更好的实现方式。

### Q: 格式化会改变我的代码逻辑吗？

A: 不会。`rustfmt` 只改变代码格式（空格、换行等），不会改变代码逻辑。

### Q: 如果 clippy 警告太严格怎么办？

A: 可以在特定代码段使用 `#[allow(clippy::specific_lint)]`，但必须有充分理由。建议在代码评审时讨论。

### Q: 提交前忘记运行检查怎么办？

A: 建议设置 Git pre-commit hook 自动运行检查（未来可以添加）。

## 自动化建议

### Git Pre-commit Hook（可选）

创建 `.git/hooks/pre-commit`：
```bash
#!/bin/sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

### CI/CD（未来计划）

在 CI 中自动运行：
- 格式化检查
- Clippy 检查
- 测试
- 编译

## 总结

**记住**：每次编辑代码后运行：

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
```

或者简单使用：
```bash
cargo make pre-commit
```

保持代码干净、一致、高质量！
