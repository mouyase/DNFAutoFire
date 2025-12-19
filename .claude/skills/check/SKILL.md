---
name: 完整代码质量检查
description: 运行完整的代码质量检查流程，包括格式检查、Clippy检查、测试和编译，确保代码可以安全提交
---

# 完整代码质量检查

## 概述

运行完整的代码质量检查流程，验证代码是否满足提交标准。这是提交代码前的必要步骤。

## 使用场景

- 提交代码到版本控制前
- 创建 Pull Request 前
- 合并代码到主分支前
- CI/CD 流程中的质量门禁
- 发布新版本前

## 功能

按顺序执行 4 个检查步骤：

### 1. 格式化检查
```bash
cargo fmt --all -- --check
```
验证代码是否已格式化（不修改代码）

### 2. Clippy 检查
```bash
cargo clippy --all-targets --all-features -- -D warnings
```
零警告代码质量检查

### 3. 运行测试
```bash
cargo test --all
```
执行所有单元测试和集成测试

### 4. Release 编译
```bash
cargo build --release
```
验证 Release 版本可以成功构建

## 检查标准

所有检查都必须通过：
- ✅ 格式化：符合 `rustfmt.toml` 规范
- ✅ Clippy：零警告
- ✅ 测试：所有测试通过
- ✅ 编译：Release 版本成功构建

### 如果检查失败

1. **格式化失败**：运行 `/format` skill
2. **Clippy 失败**：运行 `/fix` skill 或手动修复
3. **测试失败**：修复测试或代码
4. **编译失败**：修复编译错误

修复后重新运行 `/check`。

## 等价命令

也可以使用脚本或 cargo-make：

```powershell
# PowerShell 脚本
.\scripts\check.ps1

# cargo-make
cargo make check
```

## 最佳实践

### 提交前检查流程

```
1. 完成代码编写
2. 运行 /format 和 /lint
3. 运行 /check
4. 所有检查通过后提交
```

### 团队协作

建议团队成员都在提交前运行此检查，确保代码库质量。

## CI/CD 集成

这个检查流程适合直接用于 CI/CD 管道：
- GitHub Actions
- GitLab CI
- Jenkins
- 其他 CI 系统

## 预期时间

根据项目大小，完整检查通常需要：
- 小项目：10-30 秒
- 中型项目：30-60 秒
- 大型项目：1-3 分钟
