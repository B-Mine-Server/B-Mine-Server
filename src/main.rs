use log::error;
use tokio::{fs::create_dir, io};
use tracing::{debug, info, warn};
use util::{conf::ServerConfig, logger::init_log};

mod util;

#[tokio::main]
async fn main() -> io::Result<()> {
    // 创建文件夹:log、worlds、players

    create_dir("log").await.err();
    create_dir("worlds").await.err();
    create_dir("players").await.err();
    create_dir("plugins").await.err();
    // 配置文件
    let config = ServerConfig::default();
    // 初始化日志
    let _guard = init_log(&config.log_level);

    // 加载插件
    

    Ok(())
}
