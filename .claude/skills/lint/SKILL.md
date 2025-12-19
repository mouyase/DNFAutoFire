---
name: Clippy 代码检查
description: 使用 cargo clippy 进行零警告代码质量检查，识别潜在 bug、性能问题和不符合最佳实践的代码
---

# Clippy 代码检查

## 概述

运行 Clippy 对代码进行深度静态分析，检查代码质量、潜在 bug、性能问题和 Rust 最佳实践违规。

**零警告策略**：所有 Clippy 警告被视为错误（`-D warnings`），必须修复。

## 使用场景

- 每次编辑代码后必须运行
- 发现潜在的逻辑错误和 bug
- 识别性能优化机会
- 学习 Rust 最佳实践
- 提交代码前的质量验证

## 功能

执行 `cargo clippy --all-targets --all-features -- -D warnings`，检查：
- 代码逻辑错误和潜在 bug
- 性能问题（不必要的 clone、分配等）
- 代码风格和可读性问题
- 不符合 Rust 惯用法的代码
- 安全性问题

## 配置

Clippy 规则由项目根目录的 `.clippy.toml` 文件控制：
- 认知复杂度阈值：30
- 避免破坏性 API 更改

## 执行

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

## 常见问题处理

### 如何修复 Clippy 警告？

1. **优先**：按照 Clippy 建议修改代码
2. **必要时**：使用 `#[allow(clippy::specific_lint)]` 抑制特定警告
3. **必须**：在代码注释中说明抑制原因

### 零警告政策的意义

零警告确保：
- 代码质量持续改进
- 避免技术债务累积
- 团队遵循统一的代码标准
- 新警告能立即被注意到

## 最佳实践

建议每次编辑后立即运行，及时发现和修复问题，避免问题累积。
