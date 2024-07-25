use std::str::FromStr;

use chrono::Local;
use log::info;
use tracing_log::LogTracer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{filter::LevelFilter, fmt};
pub fn init_log(log_level: &str) -> tracing_appender::non_blocking::WorkerGuard {
    // 消费log门面日志 转为 tracing Event日志
    LogTracer::builder()
        .with_max_level(log::LevelFilter::Error)
        .init()
        .expect("[PEAR] LogTracer 初始化失败");

    let level = LevelFilter::from_str(log_level).expect("Invalid log level");
    // 标准控制台输出layer
    let fmt_layer = fmt::layer()
        .with_level(true)
        .with_writer(std::io::stdout)
        .with_filter(level);

    // 文件 appender 指定日志文件输出目录和文件名前缀
    // daily 指定生成文件名日期到年月日
    // 如： test-log.2023-08-30
    let fmt = "%Y-%m-%d-%H-%M";
    let now = Local::now().format(fmt);
    let file_name = format!("{}.log", now);

    let file_appender = tracing_appender::rolling::never("log/", file_name);
    // 生成非阻塞写入器
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    // 文件输出层
    let file_layer = fmt::layer()
        // 移除输出内容中的 颜色或其它格式相关转义字符
        .with_ansi(false)
        .with_writer(non_blocking)
        // 日志等级过滤
        .with_filter(level);

    // 生成注册中心 Registry 绑定多个输出层
    let collector = tracing_subscriber::registry()
        .with(file_layer)
        .with(fmt_layer);

    // 订阅者全局注册
    tracing::subscriber::set_global_default(collector).expect("Tracing collect error");

    guard
}
