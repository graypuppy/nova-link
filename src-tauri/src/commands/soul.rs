use crate::config::{self, Soul};

/// 保存人格设定
#[tauri::command]
pub fn save_soul(content: String) -> Result<(), String> {
    // 创建一个临时的 Soul 对象并保存
    let soul = Soul {
        name: "Nova".to_string(),
        personality: "活泼、可爱、友好".to_string(),
        style: "轻松可爱".to_string(),
        emoticons: "◕‿◕".to_string(),
        tone: "简洁有趣".to_string(),
        content,
    };
    config::save_soul(&soul)
}

/// 加载人格设定
#[tauri::command]
pub fn load_soul() -> Result<String, String> {
    // 从配置文件加载并返回 Markdown
    let soul = config::load_soul().unwrap_or_default();
    Ok(soul.to_markdown())
}

/// 同步人格设定到 OpenClaw 工作目录
/// 将 soul.md 内容复制到用户目录下的 .openclaw/workspace/SOUL.md
#[tauri::command]
pub fn sync_soul_to_openclaw(content: String) -> Result<String, String> {
    // 获取用户主目录
    let home_dir = dirs::home_dir().ok_or("Cannot find home directory")?;
    let openclaw_dir = home_dir.join(".openclaw").join("workspace");

    // 创建目录（如果不存在）
    std::fs::create_dir_all(&openclaw_dir).map_err(|e| e.to_string())?;

    // 写入 SOUL.md 文件
    let soul_path = openclaw_dir.join("SOUL.md");
    std::fs::write(&soul_path, &content).map_err(|e| e.to_string())?;

    log::info!("Soul synced to OpenClaw: {:?}", soul_path);
    Ok(soul_path.to_string_lossy().to_string())
}

/// 从 OpenClaw 工作目录加载 SOUL.md（优先读取用户目录）
#[tauri::command]
pub fn load_soul_from_file() -> Result<String, String> {
    let workspace_dir = get_openclaw_workspace_dir()?;
    let soul_path = workspace_dir.join("SOUL.md");

    if soul_path.exists() {
        std::fs::read_to_string(&soul_path).map_err(|e| e.to_string())
    } else {
        // 如果文件不存在，返回默认 soul
        Ok(config::SOUL_TEMPLATE.to_string())
    }
}

/// 保存 Soul 到 OpenClaw 工作目录
#[tauri::command]
pub fn save_soul_to_file(content: String) -> Result<String, String> {
    let workspace_dir = get_openclaw_workspace_dir()?;
    let soul_path = workspace_dir.join("SOUL.md");

    std::fs::write(&soul_path, &content).map_err(|e| e.to_string())?;

    log::info!("Soul saved to: {:?}", soul_path);
    Ok(soul_path.to_string_lossy().to_string())
}

/// 同步 Soul 到 OpenClaw（从本地存储同步到用户目录）
#[tauri::command]
pub fn sync_soul_from_local() -> Result<String, String> {
    // 从配置目录读取 soul.md
    let content =
        config::load_soul_markdown().unwrap_or_else(|_| config::SOUL_TEMPLATE.to_string());

    // 写入 OpenClaw 目录
    let workspace_dir = get_openclaw_workspace_dir()?;
    let soul_path = workspace_dir.join("SOUL.md");
    std::fs::write(&soul_path, &content).map_err(|e| e.to_string())?;

    log::info!("Soul synced from local to OpenClaw: {:?}", soul_path);
    Ok(soul_path.to_string_lossy().to_string())
}

/// 从 soul.md 内容中提取系统指令
/// 提取 "## 系统指令" 后的内容
pub fn extract_system_instruction(soul_content: &str) -> String {
    // 查找 "## 系统指令" 部分
    if let Some(idx) = soul_content.find("## 系统指令") {
        let after_title = &soul_content[idx + "## 系统指令".len()..];
        // 找到下一个 ## 标题之前的内容，或者文件结尾
        let end_idx = after_title.find("##").unwrap_or(after_title.len());
        let instruction = after_title[..end_idx].trim();
        return instruction.to_string();
    }
    // 如果没有找到系统指令部分，返回整个内容
    soul_content.to_string()
}

/// 获取 OpenClaw 工作目录路径
fn get_openclaw_workspace_dir() -> Result<std::path::PathBuf, String> {
    let home_dir = dirs::home_dir().ok_or("Cannot find home directory")?;
    let workspace_dir = home_dir.join(".openclaw").join("workspace");
    std::fs::create_dir_all(&workspace_dir).map_err(|e| e.to_string())?;
    Ok(workspace_dir)
}
