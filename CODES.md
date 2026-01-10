// 系统错误码划分规划（契约）

本文定义 wp-error 在系统内的错误码划分与分配原则，作为跨模块/对外接口的稳定契约。所有领域错误均应可映射为 `u16` 的系统错误码（sys_code）。

## 设计目标
- 统一：不同领域（配置/解析/源/分发/运行）使用统一的码系与语义
- 稳定：码值作为对外契约，升级时保持兼容；新增仅在预留范围内扩展
- 可观测：便于日志/指标聚合、HTTP 映射与告警策略

## 码位结构与约定
- 数字格式：5 位（示例：42211）。按语义分段如下：
  - 首位（HTTP 映射建议）：
    - 2xxxx：非异常/无内容（例如 NoData/EOF）
    - 4xxxx：客户端/输入侧错误（例如 NotFound/Syntax/Validation）
    - 5xxxx：服务/运行时错误（内部异常、不可修复）
  - 中间两位（领域/子域）：
    - 20：配置（01 Core / 02 Feature / 03 Dynamic）
    - 21：解析（11 OML / 12 Data）
    - 22：数据源（01 源通用）
    - 23：分发（11 分发通用）
    - 24：运行聚合（41 运行聚合）
  - 末两位（具体原因）：按领域内子类定义

说明：实际分配在 `src/codes.rs` 落地，以代码为准；本文提供一份规划说明，便于查询与扩展。

## 领域分配（当前-已落地）

### 配置（ConfReason）
- Core：Syntax=42201，NotFound=40401，Uvs=50001
- Feature：Syntax=42202，NotFound=40402，Uvs=50002
- Dynamic：Syntax=42203，NotFound=40403，Uvs=50003

### 解析/OML
- OMLCodeReason：Syntax=42211，NotFound=40411，Uvs=50011
- DataErrKind：
  - FormatError=42212，NotComplete=42213，UnParse=40412
  - LessData=42214，EmptyData=42215，LessStc=42216，LessDef=42217

### 数据源（SourceReason）
- NotData=20401，EOF=20402
- Disconnect=49901（网络/连接异常）
- SupplierError=50201，Other=50209
- Uvs=50021

### 分发（SinkReason）
- SinkError=50211，StgCtrl=50311，Mock=50312
- Uvs=50031

### 运行聚合（RunReason）
- Dist(SinkError)=50211，Dist(StgCtrl)=50311
- Source(NoData/Eof/Supplier/Disconnect)=20401/20402/50201/49901
- Uvs=50041

## HTTP 映射建议
- 从 sys_code 推导 HTTP 状态（仅建议，按上层 API 需求可微调）：
  - 404xx → 404 Not Found
  - 422xx → 422 Unprocessable Entity
  - 204xx → 204 No Content
  - 499xx → 499 Client Closed Request（或 503 Service Unavailable）
  - 502xx/503xx/500xx → 5xx（按 502/503/500 对应）

## 扩展与预留
- 各领域在 末两位 预留 50 个码（01～50）供未来扩充
- 新增码值需：
  1. 在 `src/codes.rs` 补充映射；
  2. 更新本文档表格；
  3. 在变更日志/发布说明注明新增码与语义

## 迁移与兼容
- orion_error::ErrorCode::error_code 已对 RunReason/SinkReason/SourceReason/OMLCodeReason 返回 sys_code，兼容旧接口
- 对 ConfReason<T> 的通用 ErrorCode 默认返回 500（泛型限制）；调用端应使用 `SysErrorCode::sys_code()` 获取系统码
- 日志/指标/HTTP 建议统一使用 sys_code，以确保一致性

## 备注
- 本文档与 `src/codes.rs` 一致性由代码评审保障；如有冲突，以代码为准，并在文档修正
- 若需国际化/本地化消息，请在上层映射层做 code→message 的语言适配
