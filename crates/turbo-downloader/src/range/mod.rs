//! Range 请求模块
//!
//! 提供 HTTP Range 请求支持，用于多线程下载和断点续传

mod client;
mod support;

pub use client::{RangeClient, RangeClientConfig};
pub use support::RangeSupport;