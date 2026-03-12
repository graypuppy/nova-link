use crate::command_runner::CommandRunner;

/// 手动运行 OpenClaw Gateway
#[tauri::command]
pub fn run_gateway() -> Result<String, String> {
    match CommandRunner::run_openclaw_gateway() {
        Ok(()) => Ok("Gateway 启动命令已发送".to_string()),
        Err(e) => Err(format!("启动失败: {}", e)),
    }
}
