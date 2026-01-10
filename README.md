# wp_err - 错误处理库

[![Crates.io](https://img.shields.io/crates/v/wp-error.svg)](https://crates.io/crates/wp-error)
[![Docs.rs](https://img.shields.io/docsrs/wp-error)](https://docs.rs/wp-error)
[![License](https://img.shields.io/badge/license-Elasticsearch_2.0-blue)](LICENSE)

`wp_err` 是从 warp-parse 数据处理平台拆分出来的综合性错误处理库，面向配置、解析、数据源和分发等领域，提供结构化错误类型、统一的系统错误码以及健壮性策略。该 crate 现在可以独立发布到 crates.io，用于任何需要领域化错误建模和策略化处理的 Rust 项目。

## 为什么发布为 crate？

- 复用 warp-parse 的生产级错误体系，而无需引入其它内部依赖。
- 与 [`orion-error`](https://crates.io/crates/orion-error) 系列库保持兼容，提供一致的 `ErrorCode` 行为。
- 为 CI/CD、插件或 CLI 工具提供可预测的系统错误码（SysErrorCode），方便日志与指标打通。

## 快速开始

### 添加依赖

```toml
[dependencies]
wp-error = "0.7"
```

### 最小示例

```rust
use wp_err::config_error::{ConfError, ConfResult};

fn load_conf() -> ConfResult<()> {
    // ... 业务配置加载逻辑
    Ok(())
}
```

## 主要功能

- **领域特定错误类型**：`config_error`、`parse_error`、`source_error`、`dist_error` 与 `run_error` 覆盖配置、解析、数据源、分发及运行态。
- **可配置的错误策略**：通过 `error_handling` 模块定义 Debug/Normal/Strict 等健壮性模式与策略，可组合重试、容忍或抛出等行为。
- **实用工具**：提供解析错误位置转换、字符串处理工具、SysErrorCode 的统一映射等能力。

## 模块说明

| 模块 | 描述 |
|------|------|
| `config_error` | 配置相关错误类型与工具，`ConfReason` 实现 `ErrorCode` 并返回 `SysErrorCode` |
| `parse_error` | 解析/OML 相关错误 |
| `source_error` | 数据源错误类型 |
| `dist_error` | 分发/接收端错误类型 |
| `run_error` | 运行时错误类型和转换，保留 `SourceFocus::Other/Disconnect` 等细粒度原因 |

### 错误处理

| 模块 | 描述 |
|------|------|
| `error_handling` | 错误处理策略与健壮性模式 |
| `error_handling::strategy` | 错误处理策略实现 |
| `error_handling::target` | 预留扩展位 |

## 更多使用示例

### 错误转换

```rust
use wp_err::run_error::{RunErrorOwe, RunResult};

fn process_data() -> RunResult<()> {
    let rs = do_something();
    rs.to_sink_error()?; // 失败时转换为接收端错误
    Ok(())
}
```

### 健壮性模式设置

```rust
use wp_err::error_handling::{RobustnessMode, set_system_robustness_mode};

let prev = set_system_robustness_mode(RobustnessMode::Strict);
```

## 系统错误码契约（SysErrorCode）

`wp_err` 统一定义跨模块/对外公开的系统错误码（SysErrorCode）。所有领域错误类型都可映射为稳定的 `u16` 码值，并在实现 `orion_error::ErrorCode` 时返回该码值。

- 访问方式：`use wp_err::SysErrorCode; reason.sys_code()`
- 对具体非泛型枚举（RunReason/SinkReason/SourceReason/OMLCodeReason 等）实现了 `ErrorCode::error_code()`，可直接得到 sys_code。

约定（示例，实际以 `src/codes.rs` 为准，更详细规划见 `CODES.md`）：
- 配置错误 (`ConfReason<*>`)
  - Core: Syntax=42201, NotFound=40401, Uvs=50001
  - Feature: Syntax=42202, NotFound=40402, Uvs=50002
  - Dynamic: Syntax=42203, NotFound=40403, Uvs=50003
- 解析/OML (`OMLCodeReason` / `DataErrKind`)
  - OML: Syntax=42211, NotFound=40411, Uvs=50011
  - Data: Format=42212, NotComplete=42213, UnParse=40412, Less/Empty=42214/42215, LessStc=42216, LessDef=42217
- 数据源 (`SourceReason`)
  - NoData=20401, EOF=20402, Disconnect=49901, Supplier/Other=50201/50209, Uvs=50021
- 分发 (`SinkReason`)
  - SinkError=50211, StgCtrl=50311, Mock=50312, Uvs=50031
- 运行聚合 (`RunReason`)
  - Dist(SinkError/StgCtrl)=50211/50311
  - Source(NoData/Eof/Supplier/Disconnect)=20401/20402/50201/49901
  - Uvs=50041

使用建议：

- 对外接口/日志/指标统一使用 `sys_code()`，若需 HTTP 映射可在上层做二次映射（例如 404xx→404，422xx→422，502xx→502）。
- 如需扩展码值，请在 `src/codes.rs` 补充映射并同步更新 `CODES.md`。

## 文档与支持

- API 文档：<https://docs.rs/wp-error>
- 错误码与契约：参见仓库内 `CODES.md`
- GitHub Issues：<https://github.com/wp-labs/wp-error/issues>

## 许可证

wp_err 在 Elastic License 2.0 下发布，详情参见项目根目录的 `LICENSE` 文件。
