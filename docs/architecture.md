# CineWeave 架构说明

## 1. 分层

### 1.1 Conversation Layer

负责接收用户自然语言请求，并输出结构化编辑意图：

- 编辑目标
- 风格目标
- 平台目标
- 质量约束
- 风险偏好

### 1.2 Planning Layer

把编辑意图编译为有向任务图：

- 每个任务都有输入、输出、依赖与前置能力要求
- 任务图可解释、可回放、可审计

### 1.3 Domain Layer

维护统一项目模型：

- Project
- Asset
- Timeline
- Track
- Clip
- Subtitle
- Filter
- Effect
- ExportPreset

### 1.4 Execution Layer

将任务图下发到本地执行器：

- Media analysis executor
- Subtitle executor
- Filter/effect compiler
- Render executor
- QC executor

### 1.5 UI Layer

向用户展示：

- 对话
- 计划
- 执行日志
- 时间线
- 预览

## 2. 当前仓内模块

- `packages/domain`
  负责统一的项目、意图、任务图模型
- `packages/agent`
  负责 prompt 解析和任务图编译
- `packages/render`
  负责样式和导出链路编译
- `packages/project`
  提供项目能力配置与工厂
- `apps/cli`
  提供命令行入口和诊断工具

## 3. 为什么先做中间层

真正容易返工的不是 UI，而是“中间表示”。

如果没有稳定的：

- 意图模型
- 项目模型
- 任务图
- 风格编译规则

那么后续无论接入 Rust 内核、桌面 UI 还是本地模型，都会不断推翻重来。

## 4. 下一阶段需要新增的模块

- `crates/media-core`
- `crates/project-store`
- `crates/render-engine`
- `apps/desktop`
- `packages/local-model-adapters`
