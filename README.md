# wp_err - 错误处理库

## 概述

`wp_err` 是专为 warp-parse 项目设计的综合性错误处理库，提供了针对配置、解析、数据源和分发等领域的结构化错误类型和实用工具。

## 主要功能

- **领域特定错误类型**:
  - 配置错误 (`config_error`)
  - 解析错误 (`parse_error`)
  - 数据源错误 (`source_error`)
  - 分发/接收端错误 (`dist_error`)
  - 运行时错误 (`run_error`)

- **错误处理策略**:
  - 多种健壮性模式 (Debug, Normal, Strict)
  - 可配置的错误处理策略 (重试、容忍、抛出等)

- **实用工具**:
  - 解析错误的位置转换
  - 字符串处理工具

## 模块说明

### 核心模块

| 模块 | 描述 |
|------|------|
| `config_error` | 配置相关错误类型和工具；`ConfReason` 现已实现 `ErrorCode`，直接返回 SysErrorCode |
| `parse_error` | 解析/OML相关错误类型 |
| `source_error` | 数据源错误类型 |
| `dist_error` | 分发/接收端错误类型 |
| `run_error` | 运行时错误类型和转换，涵盖 `SourceFocus::Other/Disconnect` 等细粒度原因 |

### 错误处理

| 模块 | 描述 |
|------|------|
| `error_handling` | 错误处理策略和健壮性模式 |
| `error_handling::strategy` | 错误处理策略实现 |
| `error_handling::target` | (保留供未来使用) |

## 使用示例

### 基本错误处理

```rust
use wp_err::config_error::{ConfError, ConfResult};

fn 加载配置() -> ConfResult<()> {
    // ... 配置加载逻辑
    Ok(())
}
```

### 错误转换

```rust
use wp_err::run_error::{RunErrorOwe, RunResult};

fn 处理数据() -> RunResult<()> {
    let 结果 = 某个操作();
    结果.转换为接收端错误()?;  // 失败时转换为接收端错误
    Ok(())
}
```

### 健壮性模式设置

```rust
use wp_err::error_handling::{RobustnessMode, 切换系统健壮性模式};

// 设置系统健壮性模式
let 先前模式 = 切换系统健壮性模式(RobustnessMode::严格模式);
```

## 错误类型说明

### 配置错误 (`config_error`)

- `ConfError`: 核心配置错误
- `FeatureConfError`: 功能特定配置错误
- `DynamicConfError`: 动态配置错误

### 解析错误 (`parse_error`)

- `OMLCodeError`: OML解析错误
- `DataErrKind`: 数据格式/验证错误

### 数据源错误 (`source_error`)

- `SourceError`: 数据源连接/可用性错误

### 分发错误 (`dist_error`)

- `SinkError`: 数据分发/接收端错误

### 运行时错误 (`run_error`)

- `RunError`: 综合数据源和接收端错误的通用运行时错误；`RunReason` 会保留 `SourceReason` 的详细原因（如 `Other` 及 `Disconnect`）并透过 `SysErrorCode` 暴露 50209/49901 等码值

## 依赖项

- `orion-error`: 核心错误处理框架
- `thiserror`: 用于方便的派生错误
- `serde`: 支持序列化
- `log`: 日志集成

## 更多示例

各模块文档中包含错误处理模式和转换的详细示例。

## 许可证

[项目许可证] (详情见项目根目录)

## 系统错误码契约（SysErrorCode）

为确保跨模块/对外的一致性，wp-error 在内部统一定义系统错误码（SysErrorCode）。所有领域错误类型都可以映射为稳定的 `u16` 码值，并在实现 `orion_error::ErrorCode` 时返回该码值。

- 访问方式：`use wp_err::SysErrorCode; reason.sys_code()`
- 与 `ErrorCode::error_code()`：对具体非泛型枚举（如 RunReason/SinkReason/SourceReason/OMLCodeReason）已返回对应 sys_code。

约定（示例，实际以 `src/codes.rs` 为准；更详细的规划见 `CODES.md`）：
- 配置错误（ConfReason<*>）
  - Core: Syntax=42201, NotFound=40401, Uvs=50001
  - Feature: Syntax=42202, NotFound=40402, Uvs=50002
  - Dynamic: Syntax=42203, NotFound=40403, Uvs=50003
- 解析/OML（OMLCodeReason/DataErrKind）
  - OML: Syntax=42211, NotFound=40411, Uvs=50011
  - Data: Format=42212, NotComplete=42213, UnParse=40412, Less/Empty=42214/42215, LessStc=42216, LessDef=42217
- 数据源（SourceReason）
  - NoData=20401, EOF=20402, Disconnect=49901, Supplier/Other=50201/50209, Uvs=50021
- 分发（SinkReason）
  - SinkError=50211, StgCtrl=50311, Mock=50312, Uvs=50031
- 运行聚合（RunReason）
  - Dist(SinkError/StgCtrl)=50211/50311
  - Source(NoData/Eof/Supplier/Disconnect)=20401/20402/50201/49901
  - Uvs=50041

使用建议：
- 对外接口/日志/指标统一使用 `sys_code()`；如需 HTTP 映射，可在上层做二次映射（例如 404xx→404，422xx→422，502xx→502）。
- 若需要扩展码值，请在 `src/codes.rs` 补充映射，并在 `CODES.md` 更新契约说明。
