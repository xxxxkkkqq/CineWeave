# CineWeave 项目计划书

更新时间：2026-04-01

## 1. 产品目标

构建一个真正开源、可本地部署、以对话驱动的视频编辑系统。用户输入自然语言后，系统能够完成素材分析、粗剪、精剪、字幕、滤镜、特效、节奏优化、导出和质量检查，并保留完整、可回滚的项目状态。

## 2. 非目标

- 不做单纯包装第三方云 API 的“壳”
- 不把现有上游仓库直接拼接成一个巨型仓库
- 不以“一次性 demo”代替长期工程化产品

## 3. 目标架构

### 3.1 技术选型

- 媒体内核：Rust
- 应用控制平面与桌面壳：TypeScript / Node 24
- 渲染工具链：FFmpeg + 本地媒体分析组件
- 本地模型层：可替换适配器，优先支持本地推理运行时
- 桌面产品形态：后续采用 Tauri 或等价本地桌面壳

### 3.2 设计原则

- Local-first
- Deterministic planning
- Auditability
- Reproducible rendering
- Capability-based execution
- Strong testing before feature expansion

## 4. 分阶段实施

### Phase 0：定义与基座

目标：

- 完成文献综述
- 完成系统架构定义
- 建立领域模型、任务图、编译器和测试骨架

验收物：

- `docs/literature-review.md`
- `docs/project-plan.md`
- `docs/architecture.md`
- 初始 CLI、任务编译器、风格编译器、测试

### Phase 1：统一项目模型

目标：

- 统一素材、时间线、轨道、片段、字幕、滤镜、特效、关键帧模型
- 定义项目文件格式和版本迁移策略
- 建立撤销/重做和事件日志

核心交付：

- Canonical project schema
- Timeline mutation engine
- Event-sourced history layer
- Snapshot and restore

### Phase 2：智能体规划层

目标：

- 将自然语言意图稳定转成执行图
- 支持目标平台约束、风格约束和质量约束
- 每个动作在执行前可验证、可预估风险、可回滚

核心交付：

- Intent parser
- Task graph compiler
- Safety rules
- Policy engine
- Explainable plan output

### Phase 3：本地分析与粗剪

目标：

- 接入本地 ASR、镜头切分、静音检测、语义摘要、片段评分
- 支持访谈、口播、播客、Vlog 等常见场景粗剪

核心交付：

- Speech analysis pipeline
- Shot detection
- Silence trimming
- Highlight ranking
- Rough cut generation

### Phase 4：精剪与风格层

目标：

- 支持滤镜、LUT、调色预设、转场、动效、字幕样式
- 将“加一点小心思”翻译成具体且受控的微调策略

核心交付：

- Filter/effect registry
- Style compiler
- Subtitle layout engine
- Rhythm polish engine
- Platform presets

### Phase 5：桌面编辑器

目标：

- 提供时间线 UI、属性面板、预览窗、任务面板、对话面板
- 用户可查看 AI 的计划、接受/拒绝、局部重跑

核心交付：

- Project explorer
- Timeline editor
- Preview player
- Agent conversation pane
- Execution trace inspector

### Phase 6：生产化

目标：

- 稳定打包、回归测试、性能测试、崩溃恢复、素材缓存、插件系统

核心交付：

- Packaging pipeline
- Crash-safe autosave
- Render cache
- Benchmark suite
- Extension SDK

## 5. 当前阶段的实现决策

当前机器现已安装 Rust，但当前可用的稳定编译路径是 `stable-x86_64-pc-windows-gnu`，因为系统中仍缺少 MSVC `link.exe`。因此本阶段的执行策略是：

- 用 Node 24 建立生产级控制平面骨架
- 把“聊天意图 -> 任务图 -> 渲染计划”的中间层先做正确
- 用 Rust `media-core` crate 验证关键领域模型和规划链路
- 把后续 Rust 媒体内核所需的数据边界提前固定下来

这不是退而求其次，而是在缺少工具链的环境里，先把最难返工的抽象层做稳定。

## 6. 风险清单

- 许可证风险：Dify 相关实现只能做结构借鉴，不能草率混编
- 本地模型风险：真正离线对话式编辑需要本地模型与推理资源
- 渲染复杂度风险：高质量特效与图形管线需要 Rust/C++ 级别性能
- UI 复杂度风险：时间线编辑器本身就是大型工程
- 跨平台风险：Windows、macOS、Linux 在编解码与硬件加速上差异大

## 7. 本轮交付目标

本轮不承诺“完成整个产品”，但会完成下面这些必须完成的起点：

- 形成明确可执行的研究结论
- 建立统一项目结构
- 实现可运行的任务编译器
- 实现可测试的风格编译器
- 为桌面产品和 Rust 媒体内核打好边界

## 8. 下一轮优先事项

- 安装 Rust 工具链并建立 `crates/media-core`
- 引入项目文件格式与事件日志
- 接入本地 FFmpeg 探测与执行层
- 补充真实素材分析流水线接口
- 开始桌面壳和预览层设计
