//! 执行引擎模块
//!
//! 包含动作定义、配置管理和执行器

pub mod action;
pub mod executor;
pub mod profile;

// 重新导出类型供外部使用
#[allow(unused_imports)]
pub use action::{KeyAction, TriggerRule};
#[allow(unused_imports)]
pub use executor::{ActionExecutor, ExecutionContext, ExecutionResult};
#[allow(unused_imports)]
pub use profile::{Profile, ProfileManager};
