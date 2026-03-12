use log::info;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager, State, WindowEvent,
};

mod command_runner;
use command_runner::CommandRunner;

mod config;
use config::*;

/// 手动运行 OpenClaw Gateway
#[tauri::command]
fn run_gateway() -> Result<String, String> {
    match CommandRunner::run_openclaw_gateway() {
        Ok(()) => Ok("Gateway 启动命令已发送".to_string()),
        Err(e) => Err(format!("启动失败: {}", e)),
    }
}

// ============ 窗口大小设置命令（供前端调用） ============

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WindowSize {
    pub width: u32,
    pub height: u32,
}

/// 设置窗口大小（前端调用）
#[tauri::command]
async fn set_window_size(window: tauri::Window, width: u32, height: u32) -> Result<(), String> {
    // 确保最小尺寸
    let min_width = 300u32;
    let min_height = 400u32;
    let w = width.max(min_width);
    let h = height.max(min_height);

    window
        .set_size(tauri::Size::Physical(tauri::PhysicalSize::new(w, h)))
        .map_err(|e| e.to_string())?;

    info!("Window size set to {}x{}", w, h);
    Ok(())
}

/// 获取当前窗口大小
#[tauri::command]
async fn get_window_size(window: tauri::Window) -> Result<WindowSize, String> {
    let size = window.outer_size().map_err(|e| e.to_string())?;
    Ok(WindowSize {
        width: size.width,
        height: size.height,
    })
}

/// 根据屏幕尺寸设置默认窗口大小（首次启动时调用）
/// 宽度 = 屏幕宽度 / 6，高度 = 宽度 * 2
#[tauri::command]
async fn set_default_window_size(window: tauri::Window) -> Result<WindowSize, String> {
    // 获取主屏幕尺寸
    let monitor = window
        .current_monitor()
        .map_err(|e| e.to_string())?
        .ok_or("Cannot get monitor")?;

    let screen_size = monitor.size();
    let width = (screen_size.width as f64 / 6.0) as u32;
    let height = width * 2;

    // 确保最小尺寸
    let min_width = 300u32;
    let min_height = 400u32;
    let w = width.max(min_width);
    let h = height.max(min_height);

    window
        .set_size(tauri::Size::Physical(tauri::PhysicalSize::new(w, h)))
        .map_err(|e| e.to_string())?;

    info!(
        "Default window size set to {}x{} (1/6 screen width, height = width * 2)",
        w, h
    );

    Ok(WindowSize {
        width: w,
        height: h,
    })
}

/// 检查是否有保存的窗口状态
#[tauri::command]
fn has_window_state() -> bool {
    config::has_window_state()
}

#[tauri::command]
fn save_setting(key: String, value: String) -> Result<(), String> {
    // 加载现有设置
    let mut settings = load_settings().unwrap_or_default();

    // 根据 key 更新对应字段
    match key.as_str() {
        "model_path" => settings.model_path = value,
        "ws_url" => settings.ws_url = value,
        "ws_token" => settings.ws_token = value,
        "chat_provider" => settings.chat_provider = value,
        "llm_provider" => settings.llm_provider = value,
        "llm_api_key" => settings.llm_api_key = value,
        "llm_api_url" => settings.llm_api_url = value,
        "llm_model" => settings.llm_model = value,
        "window_width" => settings.window_width = value.parse().unwrap_or(400),
        "window_height" => settings.window_height = value.parse().unwrap_or(800),
        "bg_color" => settings.bg_color = value,
        "bg_opacity" => settings.bg_opacity = value.parse().unwrap_or(0.8),
        "bg_blur" => settings.bg_blur = value.parse().unwrap_or(true),
        "always_on_top" => settings.always_on_top = value.parse().unwrap_or(true),
        _ => return Err(format!("Unknown setting key: {}", key)),
    }

    save_settings(&settings)
}

/// 保存人格设定
#[tauri::command]
fn save_soul(content: String) -> Result<(), String> {
    // 创建一个临时的 Soul 对象并保存
    let soul = config::Soul {
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
fn load_soul() -> Result<String, String> {
    // 从配置文件加载并返回 Markdown
    let soul = config::load_soul().unwrap_or_default();
    Ok(soul.to_markdown())
}

/// 同步人格设定到 OpenClaw 工作目录
/// 将 soul.md 内容复制到用户目录下的 .openclaw/workspace/SOUL.md
#[tauri::command]
fn sync_soul_to_openclaw(content: String) -> Result<String, String> {
    // 获取用户主目录
    let home_dir = dirs::home_dir().ok_or("Cannot find home directory")?;
    let openclaw_dir = home_dir.join(".openclaw").join("workspace");

    // 创建目录（如果不存在）
    std::fs::create_dir_all(&openclaw_dir).map_err(|e| e.to_string())?;

    // 写入 SOUL.md 文件
    let soul_path = openclaw_dir.join("SOUL.md");
    std::fs::write(&soul_path, &content).map_err(|e| e.to_string())?;

    info!("Soul synced to OpenClaw: {:?}", soul_path);
    Ok(soul_path.to_string_lossy().to_string())
}

// ============ Identity & User 相关命令 ============

/// 获取 OpenClaw 工作目录路径
fn get_openclaw_workspace_dir() -> Result<PathBuf, String> {
    let home_dir = dirs::home_dir().ok_or("Cannot find home directory")?;
    let workspace_dir = home_dir.join(".openclaw").join("workspace");
    std::fs::create_dir_all(&workspace_dir).map_err(|e| e.to_string())?;
    Ok(workspace_dir)
}

/// 从配置文件加载 Identity
#[tauri::command]
fn load_identity() -> Result<config::Identity, String> {
    config::load_identity()
}

/// 保存 Identity 到配置文件
#[tauri::command]
fn save_identity(
    name: String,
    creature_type: String,
    temperament: String,
    emoji: String,
    avatar_path: String,
) -> Result<(), String> {
    let identity = config::Identity {
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
fn load_identity_from_file() -> Result<Identity, String> {
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
fn save_identity_to_file(
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

    info!("Identity saved to: {:?}", identity_path);
    Ok(identity_path.to_string_lossy().to_string())
}

/// 获取默认 Identity 模板
#[tauri::command]
fn get_default_identity() -> String {
    config::IDENTITY_TEMPLATE.to_string()
}

/// 获取默认 User 模板
#[tauri::command]
fn get_default_user() -> String {
    config::USER_TEMPLATE.to_string()
}

/// 从 OpenClaw 工作目录加载 USER.md
#[tauri::command]
fn load_user_from_file() -> Result<config::User, String> {
    let workspace_dir = get_openclaw_workspace_dir()?;
    let user_path = workspace_dir.join("USER.md");

    if !user_path.exists() {
        return Ok(config::User::default());
    }

    let content = std::fs::read_to_string(&user_path).map_err(|e| e.to_string())?;

    // 解析 USER.md 文件
    let mut user = config::User::default();
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
fn save_user_to_file(
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

    info!("User saved to: {:?}", user_path);
    Ok(user_path.to_string_lossy().to_string())
}

/// 从 OpenClaw 工作目录加载 SOUL.md（优先读取用户目录）
#[tauri::command]
fn load_soul_from_file() -> Result<String, String> {
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
fn save_soul_to_file(content: String) -> Result<String, String> {
    let workspace_dir = get_openclaw_workspace_dir()?;
    let soul_path = workspace_dir.join("SOUL.md");

    std::fs::write(&soul_path, &content).map_err(|e| e.to_string())?;

    info!("Soul saved to: {:?}", soul_path);
    Ok(soul_path.to_string_lossy().to_string())
}

/// 同步 Soul 到 OpenClaw（从本地存储同步到用户目录）
#[tauri::command]
fn sync_soul_from_local() -> Result<String, String> {
    // 从配置目录读取 soul.md
    let content =
        config::load_soul_markdown().unwrap_or_else(|_| config::SOUL_TEMPLATE.to_string());

    // 写入 OpenClaw 目录
    let workspace_dir = get_openclaw_workspace_dir()?;
    let soul_path = workspace_dir.join("SOUL.md");
    std::fs::write(&soul_path, &content).map_err(|e| e.to_string())?;

    info!("Soul synced from local to OpenClaw: {:?}", soul_path);
    Ok(soul_path.to_string_lossy().to_string())
}

/// 从 soul.md 内容中提取系统指令
/// 提取 "## 系统指令" 后的内容
fn extract_system_instruction(soul_content: &str) -> String {
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

#[tauri::command]
fn get_setting(key: String) -> Result<Option<String>, String> {
    let settings = load_settings().unwrap_or_default();

    let value = match key.as_str() {
        "model_path" => Some(settings.model_path),
        "ws_url" => Some(settings.ws_url),
        "ws_token" => Some(settings.ws_token),
        "chat_provider" => Some(settings.chat_provider),
        "llm_provider" => Some(settings.llm_provider),
        "llm_api_key" => Some(settings.llm_api_key),
        "llm_api_url" => Some(settings.llm_api_url),
        "llm_model" => Some(settings.llm_model),
        "window_width" => Some(settings.window_width.to_string()),
        "window_height" => Some(settings.window_height.to_string()),
        "bg_color" => Some(settings.bg_color),
        "bg_opacity" => Some(settings.bg_opacity.to_string()),
        "bg_blur" => Some(settings.bg_blur.to_string()),
        "always_on_top" => Some(settings.always_on_top.to_string()),
        _ => None,
    };

    Ok(value)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct LlmMessage {
    role: String,
    content: String,
}

#[derive(Clone)]
struct LlmConfig {
    provider: String,
    api_key: String,
    api_url: String,
    model: String,
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            provider: "none".to_string(),
            api_key: String::new(),
            api_url: String::new(),
            model: String::new(),
        }
    }
}

struct AppState {
    llm_config: Mutex<LlmConfig>,
    http_client: Client,
}

#[tauri::command]
fn update_llm_config(
    state: State<AppState>,
    provider: String,
    api_key: String,
    api_url: String,
    model: String,
) {
    let mut config = state.llm_config.lock().unwrap();
    config.provider = provider;
    config.api_key = api_key;
    config.api_url = api_url;
    config.model = model;
    info!("LLM config updated");
}

#[tauri::command]
async fn chat_with_llm(
    state: State<'_, AppState>,
    provider: String,
    api_key: String,
    api_url: String,
    model: String,
    message: String,
    system_prompt: Option<String>,
) -> Result<String, String> {
    if provider == "none" || api_key.is_empty() || api_url.is_empty() {
        return Err("LLM not configured".to_string());
    }

    let url = format!("{}/chat/completions", api_url.trim_end_matches('/'));

    // 构建消息列表，支持注入 system prompt
    let mut messages: Vec<LlmMessage> = Vec::new();

    // 如果提供了 system prompt，添加到消息列表
    if let Some(system_content) = system_prompt {
        // 从 soul.md 中提取系统指令部分
        let system_instruction = extract_system_instruction(&system_content);
        if !system_instruction.is_empty() {
            messages.push(LlmMessage {
                role: "system".to_string(),
                content: system_instruction,
            });
        }
    }

    messages.push(LlmMessage {
        role: "user".to_string(),
        content: message,
    });

    let request_body = serde_json::json!({
        "model": model,
        "messages": messages,
        "temperature": 0.7,
        "max_tokens": 2048
    });

    let response = state
        .http_client
        .post(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("API error ({}): {}", status, error_text));
    }

    let response_json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let content = response_json["choices"]
        .as_array()
        .and_then(|arr| arr.first())
        .and_then(|c| c.get("message"))
        .and_then(|m| m.get("content"))
        .and_then(|c| c.as_str())
        .unwrap_or("")
        .to_string();

    if content.is_empty() {
        return Err("Empty response from LLM".to_string());
    }

    Ok(content)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            llm_config: Mutex::new(LlmConfig::default()),
            http_client: Client::new(),
        })
        .invoke_handler(tauri::generate_handler![
            chat_with_llm,
            update_llm_config,
            set_window_size,
            get_window_size,
            set_default_window_size,
            has_window_state,
            save_setting,
            get_setting,
            run_gateway,
            save_soul,
            load_soul,
            sync_soul_to_openclaw,
            load_identity,
            save_identity,
            load_identity_from_file,
            save_identity_to_file,
            get_default_identity,
            get_default_user,
            load_user_from_file,
            save_user_to_file,
            load_soul_from_file,
            save_soul_to_file,
            sync_soul_from_local,
        ])
        .setup(|app| {
            println!("[DEBUG] Nova Link setup starting...");

            // 尝试从文件恢复窗口状态
            if let Ok(Some(state)) = config::load_window_state() {
                if let Some(window) = app.get_webview_window("main") {
                    // 先设置位置
                    let _ = window.set_position(tauri::Position::Physical(
                        tauri::PhysicalPosition::new(state.x, state.y),
                    ));
                    // 再设置大小
                    let _ = window.set_size(tauri::Size::Physical(tauri::PhysicalSize::new(
                        state.width,
                        state.height,
                    )));
                    println!(
                        "[DEBUG] Window state restored: {}x{} at ({}, {})",
                        state.width, state.height, state.x, state.y
                    );
                }
            } else {
                // 如果没有保存的状态，根据屏幕尺寸计算默认大小
                if let Some(window) = app.get_webview_window("main") {
                    if let Ok(monitor) = window.current_monitor() {
                        if let Some(monitor) = monitor {
                            let screen_size = monitor.size();
                            let width = (screen_size.width as f64 / 6.0) as u32;
                            let height = width * 2;
                            let min_width = 300u32;
                            let min_height = 400u32;
                            let w = width.max(min_width);
                            let h = height.max(min_height);
                            let _ = window
                                .set_size(tauri::Size::Physical(tauri::PhysicalSize::new(w, h)));
                            println!("[DEBUG] Default window size set: {}x{}", w, h);
                        }
                    }
                }
            }

            // 平台特定的透明窗口处理
            #[cfg(target_os = "macos")]
            {
                if let Some(window) = app.get_webview_window("main") {
                    // macOS 上启用透明需要设置 NSWindow 的相关属性
                    // 通过 JavaScript 注入来确保透明生效（多层防护机制）
                    let js = r#"
                        document.body.style.background = 'transparent';
                        document.documentElement.style.background = 'transparent';
                        var app = document.getElementById('app');
                        if (app) { app.style.background = 'transparent'; }
                        var canvas = document.getElementById('live2d-canvas');
                        if (canvas) { canvas.style.background = 'transparent'; }
                    "#;
                    if let Err(e) = window.eval(js) {
                        println!("[WARN] Failed to set transparent style: {}", e);
                    }
                    println!("[DEBUG] macOS transparent window setup complete");
                }
            }

            // Windows 透明窗口处理
            #[cfg(target_os = "windows")]
            {
                if let Some(window) = app.get_webview_window("main") {
                    // Windows 透明窗口处理（多层防护机制）
                    let js = r#"
                        document.body.style.background = 'transparent';
                        document.documentElement.style.background = 'transparent';
                        var app = document.getElementById('app');
                        if (app) { app.style.background = 'transparent'; }
                        var canvas = document.getElementById('live2d-canvas');
                        if (canvas) { canvas.style.background = 'transparent'; }
                    "#;
                    if let Err(e) = window.eval(js) {
                        println!("[WARN] Failed to set transparent style: {}", e);
                    }
                    println!("[DEBUG] Windows transparent window setup complete");
                }
            }

            // Linux 透明窗口处理
            #[cfg(target_os = "linux")]
            {
                if let Some(window) = app.get_webview_window("main") {
                    let js = r#"
                        document.body.style.background = 'transparent';
                        document.documentElement.style.background = 'transparent';
                        var app = document.getElementById('app');
                        if (app) { app.style.background = 'transparent'; }
                        var canvas = document.getElementById('live2d-canvas');
                        if (canvas) { canvas.style.background = 'transparent'; }
                    "#;
                    if let Err(e) = window.eval(js) {
                        println!("[WARN] Failed to set transparent style: {}", e);
                    }
                    println!("[DEBUG] Linux transparent window setup complete");
                }
            }

            // 创建系统托盘
            // 注意：窗口位置和大小由 tauri-plugin-window-state 自动保存和恢复
            let show_item = MenuItem::with_id(app, "show", "显示", true, None::<&str>).unwrap();
            let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>).unwrap();
            let menu = Menu::with_items(app, &[&show_item, &quit_item]).unwrap();

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        // 保存窗口状态后退出
                        if let Some(window) = app.get_webview_window("main") {
                            if let Ok(pos) = window.outer_position() {
                                if let Ok(size) = window.outer_size() {
                                    let _ = config::save_window_state(
                                        pos.x,
                                        pos.y,
                                        size.width,
                                        size.height,
                                    );
                                }
                            }
                        }
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            // println!("[DEBUG] Window event: {:?}", event);
            if let WindowEvent::CloseRequested { api, .. } = event {
                println!("[DEBUG] Close requested, hiding window");
                // 保存窗口状态后再隐藏
                if let Ok(pos) = window.outer_position() {
                    if let Ok(size) = window.outer_size() {
                        let _ = config::save_window_state(pos.x, pos.y, size.width, size.height);
                    }
                }
                api.prevent_close();
                let _ = window.hide();
            }
            if let WindowEvent::Resized(size) = event {
                println!("[DEBUG] Window resized: {:?}", size);
                // 如果大小异常（标题栏大小），尝试恢复
                if size.width < 100 || size.height < 100 {
                    println!("[DEBUG] Window size too small, ignoring");
                    return;
                }
                // 保存窗口状态
                if let Ok(pos) = window.outer_position() {
                    let _ = config::save_window_state(pos.x, pos.y, size.width, size.height);
                }
            }
            if let WindowEvent::Moved(pos) = event {
                // println!("[DEBUG] Window moved: {:?}", pos);
                // 如果移动到隐藏位置，恢复
                if pos.x < -10000 || pos.y < -10000 {
                    println!("[DEBUG] Window moved to hidden position, ignoring");
                    return;
                }
                // 保存窗口状态
                if let Ok(size) = window.outer_size() {
                    let _ = config::save_window_state(pos.x, pos.y, size.width, size.height);
                }
            }
            if let WindowEvent::Focused(focused) = event {
                println!("[DEBUG] Window focused: {}", focused);
                // 当窗口获得焦点时，通知前端可能需要重新加载 Live2D
                if *focused {
                    let _ = window.emit("window-shown", ());
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
