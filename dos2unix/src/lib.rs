//! dos2unix 库模块
//!
//! 提供 CRLF 到 LF 转换的核心功能。

pub mod convert;

pub use convert::{contains_crlf, memory, stream, ConvertResult};
