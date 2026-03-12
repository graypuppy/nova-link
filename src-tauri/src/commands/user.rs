use crate::config::User;

/// 获取默认 User 模板
#[tauri::command]
pub fn get_default_user() -> String {
    crate::config::USER_TEMPLATE.to_string()
}

/// 从 OpenClaw 工作目录加载 USER.md
#[tauri::command]
pub fn load_user_from_file() -> Result<User, String> {
    let workspace_dir = get_openclaw_workspace_dir()?;
    let user_path = workspace_dir.join("USER.md");

    if !user_path.exists() {
        return Ok(User::default());
    }

    let content = std::fs::read_to_string(&user_path).map_err(|e| e.to_string())?;

    // 解析 USER.md 文件
    let mut user = User::default();
    let mut in_context = false;
    let mut context_lines: Vec<String> = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == "## Context" {
            in_context = true;
            continue;
        }
        if in_context {
            if trimmed.starts_with("---") || trimmed.is_empty() {
                continue;
            }
            context_lines.push(line.to_string());
            continue;
        }
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with('_') {
            continue;
        }
        // 解析键值对
        if let Some((key, value)) = trimmed.split_once(':') {
            let key = key.trim().trim_start_matches('-').trim();
            let value = value
                .trim()
                .trim_start_matches('_')
                .trim()
                .trim_end_matches('_');
            match key {
                "Name" => user.name = value.to_string(),
                "What to call them" => user.call_name = value.to_string(),
                "Pronouns" => user.pronouns = value.to_string(),
                "Timezone" => user.timezone = value.to_string(),
                "Notes" => user.notes = value.to_string(),
                _ => {}
            }
        }
    }

    user.context = context_lines.join("\n");
    Ok(user)
}

/// 保存 User 到 OpenClaw 工作目录的 USER.md
#[tauri::command]
pub fn save_user_to_file(
    name: String,
    call_name: String,
    pronouns: String,
    timezone: String,
    notes: String,
    context: String,
) -> Result<String, String> {
    let workspace_dir = get_openclaw_workspace_dir()?;
    let user_path = workspace_dir.join("USER.md");

    let content = format!(
        r#"# USER.md - About Your Human

_Learn about the person you're helping. Update this as you go._

- **Name:** {}
- **What to call them:** {}
- **Pronouns:** {} _(optional)_
- **Timezone:** {}
- **Notes:** {}

## Context

{}

---

The more you know, the better you can help. But remember — you're learning about a person, not building a dossier. Respect the difference.
"#,
        name, call_name, pronouns, timezone, notes, context
    );

    std::fs::write(&user_path, content).map_err(|e| e.to_string())?;

    log::info!("User saved to: {:?}", user_path);
    Ok(user_path.to_string_lossy().to_string())
}

/// 获取 OpenClaw 工作目录路径
fn get_openclaw_workspace_dir() -> Result<std::path::PathBuf, String> {
    let home_dir = dirs::home_dir().ok_or("Cannot find home directory")?;
    let workspace_dir = home_dir.join(".openclaw").join("workspace");
    std::fs::create_dir_all(&workspace_dir).map_err(|e| e.to_string())?;
    Ok(workspace_dir)
}
