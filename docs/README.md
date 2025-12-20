# DNF AutoFire 文档中心

## 📚 文档索引

### 核心规范文档

- **[nwg-gui-spec.json](./nwg-gui-spec.json)** ⭐ 主要参考
  - NWG GUI 完整实现规范（JSON 格式，512行）
  - 包含架构、API、配置、事件系统等所有细节
  - **推荐阅读**：作为 NWG 开发的主要参考文档

### 项目记录

- **[pitfalls.json](./pitfalls.json)**
  - 开发过程中的踩坑记录和解决方案
  - 包含 4 个主要问题记录和最佳实践
  - ID 4: NWG 架构实现记录

### 开发指南

- **[development-workflow.md](./development-workflow.md)**
  - 开发工作流程指南

## 🎯 快速导航

### GUI 开发相关

| 需求 | 文档位置 |
|------|---------|
| NWG 完整规范 | `nwg-gui-spec.json` |
| 键盘布局定义 | `nwg-gui-spec.json` → `keyboard_layout` |
| 配置系统设计 | `nwg-gui-spec.json` → `configuration_system` |
| 事件绑定指南 | `nwg-gui-spec.json` → `event_system` |
| UI 规范 | `nwg-gui-spec.json` → `ui_specifications` |
| VK 常量列表 | `nwg-gui-spec.json` → `virtual_key_codes` |

### 问题解决

| 问题类型 | 文档位置 |
|---------|---------|
| 连发逻辑问题 | `pitfalls.json` → ID 1 |
| 管理员权限 | `pitfalls.json` → ID 2 |
| Skills 格式 | `pitfalls.json` → ID 3 |
| NWG 实现 | `pitfalls.json` → ID 4 |

## 📖 文档使用建议

### 对于 AI 助手

1. **优先读取 JSON 格式文档**
   - `nwg-gui-spec.json` 结构化数据，易于解析
   - `pitfalls.json` 包含历史问题和解决方案

2. **开发 NWG GUI 时**
   - 先读 `nwg-gui-spec.json` 了解完整架构
   - 参考 `event_system` 部分实现事件绑定
   - 查看 `pending_tasks` 了解待完成工作

3. **遇到问题时**
   - 先查 `pitfalls.json` 看是否有类似问题
   - 查看 `best_practices` 部分的最佳实践

### 对于开发者

1. **快速了解项目**
   - 阅读 `nwg-implementation.md` 获取概览
   - 查看 `nwg-gui-spec.json` → `metadata` 了解状态

2. **开始开发**
   - 参考 `nwg-gui-spec.json` → `architecture` 了解模块结构
   - 查看 `pending_tasks` 部分的待办事项
   - 遵循 `development_guidelines` 的开发流程

3. **API 参考**
   - 配置管理 API: `nwg-gui-spec.json` → `configuration_system`
   - 键盘布局 API: `nwg-gui-spec.json` → `keyboard_layout`
   - 事件处理 API: `nwg-gui-spec.json` → `event_system`

## 🔄 文档更新

每次重要更新后，需要同步更新：

1. **代码变更**
   - 更新 `nwg-gui-spec.json` 对应部分
   - 如有新问题，添加到 `pitfalls.json`

2. **版本发布**
   - 更新 `nwg-gui-spec.json` → `changelog`
   - 更新 `metadata` → `version` 和 `status`

3. **文档变更**
   - 保持 JSON 和 Markdown 文档同步
   - 更新本 README.md 索引

## 📝 文档规范

### JSON 文档格式

```json
{
  "metadata": {
    "title": "文档标题",
    "version": "版本号",
    "date": "YYYY-MM-DD",
    "status": "当前状态"
  },
  // ... 其他内容
}
```

### 更新日志格式

```json
{
  "changelog": [
    {
      "date": "YYYY-MM-DD",
      "version": "x.y.z",
      "changes": ["变更1", "变更2"],
      "status": "当前状态"
    }
  ]
}
```

## 🔗 相关链接

- 项目主仓库: `/DNFAutoFire`
- History 实现: `/DNFAutoFire/history` (Tauri + React 参考版本)
- 构建产物: `/DNFAutoFire/target/release`

---

**最后更新**: 2025-12-21
**文档版本**: 1.0.0
**项目状态**: NWG GUI 架构完成，待事件绑定
