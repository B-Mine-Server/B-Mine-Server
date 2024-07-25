use rust_raknet::Reliability;
use tokio::{fs::create_dir, io};
use tracing::{debug, info};
use util::{conf::ServerConfig, logger::init_log};
use B_Mine_SDK::packet::DataPacket;

const MC_PROTOCOL_VERSION: &str = "686";
const MC_VERSION: &str = "1.21.2";
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
    let mut plugin_manager = B_Mine_SDK::plugin::plugin_manager::PluginManager::new();
    plugin_manager.load_all();

    let server_ip_prot = format!("{}:{}", config.ip, config.v4port);
    let mut listener = rust_raknet::RaknetListener::bind(&server_ip_prot.parse().unwrap())
        .await
        .unwrap();
    listener
        .set_motd(
            &config.name,
            config.max_connection,
            MC_PROTOCOL_VERSION,
            MC_VERSION,
            &config.game_type,
            config.v4port,
        )
        .await;
    listener.listen().await;
    let _fu = tokio::spawn(async move {
        loop {
            let socket: rust_raknet::RaknetSocket = listener.accept().await.unwrap();
            let buf = socket.recv().await.unwrap();
            if buf[0] == 0xfe {
                plugin_manager.select_all(socket, &buf).await;
            }
        }
    })
    .await;
    Ok(())
}
