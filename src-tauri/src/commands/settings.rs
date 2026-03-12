use crate::config::{load_settings, save_settings};

#[tauri::command]
pub fn save_setting(key: String, value: String) -> Result<(), String> {
    // 加载现有设置
    let mut settings = load_settings().unwrap_or_default();

    // 如果是 app-settings 作为一个整体保存（前端发送的完整 JSON）
    if key == "app-settings" {
        // 解析 JSON 并应用每个字段
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&value) {
            if let Some(obj) = parsed.as_object() {
                if let Some(v) = obj.get("modelPath").and_then(|v| v.as_str()) {
                    settings.model_path = v.to_string();
                }
                if let Some(v) = obj.get("wsUrl").and_then(|v| v.as_str()) {
                    settings.ws_url = v.to_string();
                }
                if let Some(v) = obj.get("wsToken").and_then(|v| v.as_str()) {
                    settings.ws_token = v.to_string();
                }
                if let Some(v) = obj.get("chatProvider").and_then(|v| v.as_str()) {
                    settings.chat_provider = v.to_string();
                }
                if let Some(v) = obj.get("llmProvider").and_then(|v| v.as_str()) {
                    settings.llm_provider = v.to_string();
                }
                if let Some(v) = obj.get("llmApiKey").and_then(|v| v.as_str()) {
                    settings.llm_api_key = v.to_string();
                }
                if let Some(v) = obj.get("llmApiUrl").and_then(|v| v.as_str()) {
                    settings.llm_api_url = v.to_string();
                }
                if let Some(v) = obj.get("llmModel").and_then(|v| v.as_str()) {
                    settings.llm_model = v.to_string();
                }
                if let Some(v) = obj.get("bgColor").and_then(|v| v.as_str()) {
                    settings.bg_color = v.to_string();
                }
                if let Some(v) = obj.get("bgOpacity").and_then(|v| v.as_f64()) {
                    settings.bg_opacity = v as f32;
                }
                if let Some(v) = obj.get("bgBlur").and_then(|v| v.as_bool()) {
                    settings.bg_blur = v;
                }
                if let Some(v) = obj.get("alwaysOnTop").and_then(|v| v.as_bool()) {
                    settings.always_on_top = v;
                }
                return save_settings(&settings);
            }
        }
        return Err("Invalid app-settings JSON format".to_string());
    }

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
        "bg_color" => settings.bg_color = value,
        "bg_opacity" => settings.bg_opacity = value.parse().unwrap_or(0.8),
        "bg_blur" => settings.bg_blur = value.parse().unwrap_or(true),
        "always_on_top" => settings.always_on_top = value.parse().unwrap_or(true),
        _ => return Err(format!("Unknown setting key: {}", key)),
    }

    save_settings(&settings)
}

#[tauri::command]
pub fn get_setting(key: String) -> Result<Option<String>, String> {
    let settings = load_settings().unwrap_or_default();

    // 如果是获取完整的 app-settings
    if key == "app-settings" {
        // 转换为前端格式 (camelCase)
        let frontend_settings = serde_json::json!({
            "modelPath": settings.model_path,
            "wsUrl": settings.ws_url,
            "wsToken": settings.ws_token,
            "chatProvider": settings.chat_provider,
            "alwaysOnTop": settings.always_on_top,
            "llmProvider": settings.llm_provider,
            "llmApiKey": settings.llm_api_key,
            "llmApiUrl": settings.llm_api_url,
            "llmModel": settings.llm_model,
            "bgColor": settings.bg_color,
            "bgOpacity": settings.bg_opacity,
            "bgBlur": settings.bg_blur,
        });
        return Ok(Some(frontend_settings.to_string()));
    }

    let value = match key.as_str() {
        "model_path" => Some(settings.model_path),
        "ws_url" => Some(settings.ws_url),
        "ws_token" => Some(settings.ws_token),
        "chat_provider" => Some(settings.chat_provider),
        "llm_provider" => Some(settings.llm_provider),
        "llm_api_key" => Some(settings.llm_api_key),
        "llm_api_url" => Some(settings.llm_api_url),
        "llm_model" => Some(settings.llm_model),
        "bg_color" => Some(settings.bg_color),
        "bg_opacity" => Some(settings.bg_opacity.to_string()),
        "bg_blur" => Some(settings.bg_blur.to_string()),
        "always_on_top" => Some(settings.always_on_top.to_string()),
        _ => None,
    };

    Ok(value)
}
