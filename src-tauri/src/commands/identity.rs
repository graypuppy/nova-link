use crate::config::{self, Identity};

/// 从配置文件加载 Identity
#[tauri::command]
pub fn load_identity() -> Result<Identity, String> {
    config::load_identity()
}

/// 保存 Identity 到配置文件
#[tauri::command]
pub fn save_identity(
    name: String,
    creature_type: String,
    temperament: String,
    emoji: String,
    avatar_path: String,
) -> Result<(), String> {
    let identity = Identity {
        name,
        creature_type,
        temperament,
        emoji,
        avatar_path,
    };
    config::save_identity(&identity)
}

/// 从 OpenClaw 工作目录加载 IDENTITY.md
#[tauri::command]
pub fn load_identity_from_file() -> Result<Identity, String> {
    let workspace_dir = get_openclaw_workspace_dir()?;
    let identity_path = workspace_dir.join("IDENTITY.md");

    if !identity_path.exists() {
        // 文件不存在，返回空的 Identity
        return Ok(Identity {
            name: String::new(),
            creature_type: String::new(),
            temperament: String::new(),
            emoji: String::new(),
            avatar_path: String::new(),
        });
    }

    let content = std::fs::read_to_string(&identity_path).map_err(|e| e.to_string())?;

    // 解析 IDENTITY.md 文件
    // 格式：键名：值
    let mut identity = Identity {
        name: String::new(),
        creature_type: String::new(),
        temperament: String::new(),
        emoji: String::new(),
        avatar_path: String::new(),
    };

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once('：') {
            let key = key.trim();
            let value = value.trim();
            match key {
                "名称" => identity.name = value.to_string(),
                "生物类型" => identity.creature_type = value.to_string(),
                "气质" => identity.temperament = value.to_string(),
                "表情符号" => identity.emoji = value.to_string(),
                "头像" => identity.avatar_path = value.to_string(),
                _ => {}
            }
        } else if let Some((key, value)) = line.split_once(':') {
            let key = key.trim();
            let value = value.trim();
            match key {
                "名称" | "Name" => identity.name = value.to_string(),
                "生物类型" | "Creature Type" => identity.creature_type = value.to_string(),
                "气质" | "Temperament" => identity.temperament = value.to_string(),
                "表情符号" | "Emoji" => identity.emoji = value.to_string(),
                "头像" | "Avatar" => identity.avatar_path = value.to_string(),
                _ => {}
            }
        }
    }

    Ok(identity)
}

/// 保存 Identity 到 OpenClaw 工作目录的 IDENTITY.md
#[tauri::command]
pub fn save_identity_to_file(
    name: String,
    creature_type: String,
    temperament: String,
    emoji: String,
    avatar_path: String,
) -> Result<String, String> {
    let workspace_dir = get_openclaw_workspace_dir()?;
    let identity_path = workspace_dir.join("IDENTITY.md");

    let content = format!(
        r#"# 角色身份

- **名称：** {}
- **生物类型：** {}
- **气质：** {}
- **专属emoji：** {}
- **头像：** {}
"#,
        name, creature_type, temperament, emoji, avatar_path
    );

    std::fs::write(&identity_path, content).map_err(|e| e.to_string())?;

    log::info!("Identity saved to: {:?}", identity_path);
    Ok(identity_path.to_string_lossy().to_string())
}

/// 获取默认 Identity 模板
#[tauri::command]
pub fn get_default_identity() -> String {
    config::IDENTITY_TEMPLATE.to_string()
}

/// 获取 OpenClaw 工作目录路径
fn get_openclaw_workspace_dir() -> Result<std::path::PathBuf, String> {
    let home_dir = dirs::home_dir().ok_or("Cannot find home directory")?;
    let workspace_dir = home_dir.join(".openclaw").join("workspace");
    std::fs::create_dir_all(&workspace_dir).map_err(|e| e.to_string())?;
    Ok(workspace_dir)
}
