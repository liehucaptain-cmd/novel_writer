use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{TrayIconBuilder, TrayIconEvent},
    Manager,
};
use uuid::Uuid;

// Deserialize null as empty Vec
fn null_as_empty_vec<'de, T: Deserialize<'de> + Default, D: serde::Deserializer<'de>>(d: D) -> Result<T, D::Error> {
    let opt = Option::<T>::deserialize(d)?;
    Ok(opt.unwrap_or_default())
}

// ─────────────────────────────────────────────
// Data Models
// ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharRelationship {
    pub target_id: String,
    #[serde(rename = "type", alias = "rel_type")]
    pub rel_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub id: String,
    pub name: String,
    pub description: String,
    pub age: String,
    pub appearance: String,
    pub personality: String,
    pub background: String,
    pub status: String, // "待出场" | "已出场" | "重要" | "已下线"
    pub group: String,
    #[serde(default, rename = "chapters_appeared")]
    pub appear_chapters: Vec<String>,
    #[serde(default)]
    pub relationships: Vec<CharRelationship>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plotline {
    pub id: String,
    pub name: String,
    pub color: String,
    pub status: String, // "active" | "completed"
    pub description: String,
    #[serde(default)]
    pub status_summary: String,
    #[serde(default)]
    pub chapter_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    pub id: String,
    pub title: String,
    pub content: String,
    pub order: i32,
    #[serde(default)]
    pub characters_appeared: Vec<String>,
    #[serde(default)]
    pub plotline_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    pub id: String,
    pub name: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    pub chapters: Vec<Chapter>,
    pub characters: Vec<Character>,
    #[serde(default, rename(serialize = "storylines", deserialize = "storylines"), alias = "plotlines", deserialize_with = "null_as_empty_vec")]
    pub plotlines: Vec<Plotline>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookMeta {
    pub id: String,
    pub name: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookIndex {
    pub books: Vec<BookMeta>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub id: String,
    pub name: String,
    pub status: String, // "待写" | "进行中" | "已完成"
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutlineChapter {
    pub id: String,
    pub name: String,
    pub scenes: Vec<Scene>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Act {
    pub id: String,
    pub name: String,
    pub color: String,
    pub chapters: Vec<OutlineChapter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Outline {
    pub acts: Vec<Act>,
}

// ─────────────────────────────────────────────
// AI Config Models
// ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    pub api_key: String,
    pub model: String,
    pub provider: String,  // "minimax" | "openai" | "custom"
    #[serde(rename = "context_chapters")]
    pub context_chapters: i32,
    #[serde(rename = "context_characters")]
    pub context_characters: Vec<String>,
    #[serde(rename = "context_plotlines")]
    pub context_plotlines: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfigOutput {
    pub has_api_key: bool,
    pub model: String,
    pub provider: String,
    #[serde(rename = "context_chapters")]
    pub context_chapters: i32,
    #[serde(rename = "context_characters")]
    pub context_characters: Vec<String>,
    #[serde(rename = "context_plotlines")]
    pub context_plotlines: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIContextConfig {
    #[serde(rename = "context_chapters")]
    pub context_chapters: Option<i32>,
    #[serde(rename = "context_characters")]
    pub context_characters: Option<Vec<String>>,
    #[serde(rename = "context_plotlines")]
    pub context_plotlines: Option<Vec<String>>,
}

// ─────────────────────────────────────────────
// File Paths
// ─────────────────────────────────────────────

fn app_data_dir() -> Result<PathBuf, String> {
    dirs::data_dir()
        .map(|p| p.join("novel-writer-tauri"))
        .ok_or_else(|| "Could not find app data directory".to_string())
}

fn books_index_path() -> Result<PathBuf, String> {
    Ok(app_data_dir()?.join("books_index.json"))
}

fn book_path(book_id: &str) -> Result<PathBuf, String> {
    Ok(app_data_dir()?.join("books").join(format!("{book_id}.json")))
}

fn outlines_dir() -> Result<PathBuf, String> {
    Ok(app_data_dir()?.join("outlines"))
}

fn outline_path(book_id: &str) -> Result<PathBuf, String> {
    Ok(outlines_dir()?.join(format!("{book_id}.json")))
}

fn ai_config_path() -> Result<PathBuf, String> {
    Ok(app_data_dir()?.join("config.json"))
}

fn ensure_app_dirs() -> Result<(), String> {
    let base = app_data_dir()?;
    fs::create_dir_all(&base).map_err(|e| e.to_string())?;
    fs::create_dir_all(base.join("books")).map_err(|e| e.to_string())?;
    fs::create_dir_all(outlines_dir()?).map_err(|e| e.to_string())?;
    Ok(())
}

// ─────────────────────────────────────────────
// JSON helpers
// ─────────────────────────────────────────────

fn read_json<T: for<'de> Deserialize<'de>>(path: &PathBuf) -> Result<T, String> {
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    serde_json::from_str(&content).map_err(|e| e.to_string())
}

fn write_json<T: Serialize>(path: &PathBuf, value: &T) -> Result<(), String> {
    let content = serde_json::to_string_pretty(value).map_err(|e| e.to_string())?;
    fs::write(path, content).map_err(|e| e.to_string())
}

fn now_iso() -> String {
    chrono::Utc::now().to_rfc3339()
}

// ─────────────────────────────────────────────
// AI Config helpers
// ─────────────────────────────────────────────

fn default_ai_config() -> AIConfig {
    AIConfig {
        api_key: String::new(),
        model: "MiniMax-M2.7".to_string(),
        provider: "minimax".to_string(),
        context_chapters: 5,
        context_characters: vec![],
        context_plotlines: vec![],
    }
}

fn read_ai_config() -> Result<AIConfig, String> {
    let path = ai_config_path()?;
    if path.exists() {
        read_json(&path)
    } else {
        let cfg = default_ai_config();
        write_json(&path, &cfg)?;
        Ok(cfg)
    }
}

// ─────────────────────────────────────────────
// System Prompts for each feature
// ─────────────────────────────────────────────

fn get_feature_system_prompt(feature: &str) -> &'static str {
    match feature {
        "brainstorm" => {
            "你是一位专业的小说写作教练，擅长从一句话灵感出发，孵化出丰富多彩的故事方向。"
        }
        "trailer" => {
            "你是一位资深的小说营销专家，擅长为故事创作引人入胜的封面故事和预告文案。"
        }
        "outline" => {
            "你是一位专业的小说结构设计师，擅长根据主题和结构类型设计完整的分章大纲。"
        }
        "character_profile" => {
            "你是一位专业的人物塑造导师，擅长为小说角色创建立体、鲜活的人设档案。"
        }
        "world_building" => {
            "你是一位专业的小说世界观架构师，擅长构建完整、细致、富有深度的世界体系。"
        }
        "scene_description" => {
            "你是一位专业的小说环境描写大师，擅长用光影、气味、触感等多维感官描写场景。"
        }
        "dialogue" => {
            "你是一位专业的小说对话写作教练，擅长根据人物性格和情境创作生动、符合人物性格的对白。"
        }
        "transition" => {
            "你是一位专业的小说转场专家，擅长创作逻辑连贯、情感流畅的场景过渡描写。"
        }
        "polish" => {
            "你是一位专业的中文修辞大师，擅长对小说段落进行文学性和可读性的双重提升。"
        }
        "style_transfer" => {
            "你是一位专业的中文风格模仿大师，擅长以指定作家的风格重写文本，保留原意。"
        }
        "duplication_check" => {
            "你是一位专业的中文文本编辑，擅长检测文本中的重复用词和短语，并给出替换建议。"
        }
        "tags_generator" => {
            "你是一位专业的小说运营专家，擅长为小说生成精准的平台简介和标签，提升曝光度。"
        }
        "logic_audit" => "你是一位挑剔的读者，擅长扮演「杠精读者」角色，指出情节中的逻辑漏洞和时间线问题。",
        _ => {
            "你是一位专业的小说写作助理。"
        }
    }
}

fn build_user_prompt(feature: &str, params: serde_json::Value, context: Option<String>) -> String {
    let context_block = context.map(|c| format!("\n\n=== 故事上下文 ===\n{}\n=== 上下文结束 ===\n", c)).unwrap_or_default();

    match feature {
        "brainstorm" => {
            let idea = params.get("idea").and_then(|v| v.as_str()).unwrap_or("");
            format!(
                "{}\n\n请根据以下一句话灵感，生成10个不同方向的故事梗概（每个50-100字），涵盖不同类型（爱情、悬疑、科幻、奇幻、现实主义等）：\n\n「{}」\n\n请以编号列表形式输出。",
                context_block, idea
            )
        }
        "trailer" => {
            let theme = params.get("theme").and_then(|v| v.as_str()).unwrap_or("");
            format!(
                "{}\n\n请为以下主题创作一个封面故事（200字）和一段预告文案（50字以内，富有悬念感）：\n\n「{}」",
                context_block, theme
            )
        }
        "outline" => {
            let theme = params.get("theme").and_then(|v| v.as_str()).unwrap_or("");
            let structure = params.get("structure").and_then(|v| v.as_str()).unwrap_or("三幕式");
            format!(
                "{}\n\n请根据以下主题，使用「{}」结构，生成一份完整的分章大纲（10-20章）：\n\n「{}」\n\n请以表格形式输出，包含：章节序号 | 章节名称 | 核心事件 | 情感走向",
                context_block, structure, theme
            )
        }
        "character_profile" => {
            let name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let description = params.get("description").and_then(|v| v.as_str()).unwrap_or("");
            format!(
                "{}\n\n请根据以下信息，为角色「{}」创建一份完整的人设卡：\n\n「{}」\n\n请包含：外貌特征、性格剖析、人物背景、口头禅、MBTI类型、人际关系。",
                context_block, name, description
            )
        }
        "world_building" => {
            let world = params.get("world").and_then(|v| v.as_str()).unwrap_or("");
            let genre = params.get("genre").and_then(|v| v.as_str()).unwrap_or("现代");
            format!(
                "{}\n\n请为以下世界观添加详细扩充内容（类型：{}）：\n\n「{}」\n\n请包含：历史背景、地理环境、政治体系、势力分布、禁忌与规则、货币与科技水平。",
                context_block, genre, world
            )
        }
        "scene_description" => {
            let keywords = params.get("keywords").and_then(|v| v.as_str()).unwrap_or("");
            format!(
                "{}\n\n请根据以下关键词，创作一段沉浸式的场景描写（200-300字），必须包含光影、气味、触感等感官细节：\n\n「{}」",
                context_block, keywords
            )
        }
        "dialogue" => {
            let char_a = params.get("char_a").and_then(|v| v.as_str()).unwrap_or("");
            let char_b = params.get("char_b").and_then(|v| v.as_str()).unwrap_or("");
            let situation = params.get("situation").and_then(|v| v.as_str()).unwrap_or("");
            format!(
                "{}\n\n请模拟以下情境中人物A和人物B的对话：\n\n人物A：{}\n人物B：{}\n\n情境：{}\n\n请输出完整对白，并注明每句话的说话者。",
                context_block, char_a, char_b, situation
            )
        }
        "transition" => {
            let from = params.get("from").and_then(|v| v.as_str()).unwrap_or("");
            let to = params.get("to").and_then(|v| v.as_str()).unwrap_or("");
            format!(
                "{}\n\n请为以下场景转换创作过渡段落：\n\n当前场景：{}\n目标场景：{}\n\n要求：逻辑连贯，情感过渡自然，100-150字。",
                context_block, from, to
            )
        }
        "polish" => {
            let text = params.get("text").and_then(|v| v.as_str()).unwrap_or("");
            let direction = params.get("direction").and_then(|v| v.as_str()).unwrap_or("文学性");
            format!(
                "{}\n\n请将以下段落按照「{}」方向进行润色改写（保持原意，提升表达）：\n\n「{}」",
                context_block, direction, text
            )
        }
        "style_transfer" => {
            let text = params.get("text").and_then(|v| v.as_str()).unwrap_or("");
            let writer = params.get("writer").and_then(|v| v.as_str()).unwrap_or("");
            format!(
                "{}\n\n请以「{}」的写作风格，重写以下文本（保留原意）：\n\n「{}」",
                context_block, writer, text
            )
        }
        "duplication_check" => {
            let text = params.get("text").and_then(|v| v.as_str()).unwrap_or("");
            format!(
                "{}\n\n请检测以下文本中的重复用词和短语，并给出替换建议：\n\n「{}」\n\n请以表格形式输出：重复词/短语 | 出现次数 | 建议替换词",
                context_block, text
            )
        }
        "tags_generator" => {
            let summary = params.get("summary").and_then(|v| v.as_str()).unwrap_or("");
            format!(
                "{}\n\n请根据以下小说简介，生成平台简介（50字以内）和精准标签（10-15个）：\n\n「{}」",
                context_block, summary
            )
        }
        "logic_audit" => {
            let plot = params.get("plot").and_then(|v| v.as_str()).unwrap_or("");
            format!(
                "{}\n\n请扮演一位挑剔的读者，仔细审查以下情节，指出逻辑漏洞、时间线矛盾、人物行为不一致等问题：\n\n「{}」\n\n请列出所有发现的问题，并给出修改建议。",
                context_block, plot
            )
        }
        _ => {
            format!(
                "{}\n\n请处理以下请求：{}",
                context_block,
                params.to_string()
            )
        }
    }
}

// ─────────────────────────────────────────────
// MiniMax API call
// ─────────────────────────────────────────────

fn call_minimax_api(api_key: &str, model: &str, system_prompt: &str, user_prompt: &str) -> Result<String, String> {
    println!("[DEBUG call_minimax_api] api_key_len={} model={} sys_len={} user_len={}", api_key.len(), model, system_prompt.len(), user_prompt.len());
    #[derive(Serialize)]
    struct ChatMessage {
        role: String,
        content: String,
    }

    #[derive(Serialize)]
    struct ChatRequest {
        model: String,
        messages: Vec<ChatMessage>,
        temperature: f32,
    }

    let request_body = ChatRequest {
        model: model.to_string(),
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: user_prompt.to_string(),
            },
        ],
        temperature: 0.7,
    };

    let json_body = serde_json::to_string(&request_body).map_err(|e| format!("序列化失败: {}", e))?;

    // Use reqwest with explicit proxy
    let proxy_url = "http://127.0.0.1:7890";
    let client = reqwest::blocking::Client::builder()
        .proxy(reqwest::Proxy::http(proxy_url).unwrap())
        .proxy(reqwest::Proxy::https(proxy_url).unwrap())
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("创建HTTP客户端失败: {}", e))?;

    let response = client
        .post("https://api.minimax.chat/v1/text/chatcompletion_v2")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .body(json_body)
        .send()
        .map_err(|e| format!("API请求失败: {}", e))?;

    let status = response.status();
    let body = response.text().map_err(|e| format!("读取响应失败: {}", e))?;
    println!("[DEBUG curl] status={} body_len={}", status, body.len());


    // Try to parse as error response first
    #[derive(Deserialize)]
    struct AnyError {
        #[serde(rename = "type")]
        error_type: Option<String>,
        error: Option<AnyErrorDetail>,
        #[serde(rename = "base_resp")]
        base_resp: Option<BaseResp>,
    }
    #[derive(Deserialize)]
    struct AnyErrorDetail { message: Option<String> }
    #[derive(Deserialize)]
    struct BaseResp { status_code: Option<i32>, status_msg: Option<String> }
    if let Ok(any_err) = serde_json::from_str::<AnyError>(&body) {
        if any_err.error_type.as_deref() == Some("error")
            || any_err.error.as_ref().map(|e| e.message.is_some()).unwrap_or(false)
            || any_err.base_resp.as_ref().map(|b| b.status_code.unwrap_or(0) != 0).unwrap_or(false)
        {
            let msg = any_err.error.as_ref().and_then(|e| e.message.clone())
                .or_else(|| any_err.base_resp.as_ref().and_then(|b| b.status_msg.clone()))
                .unwrap_or_else(|| "API返回了错误响应".to_string());
            let code = any_err.base_resp.as_ref().and_then(|b| b.status_code).unwrap_or(0);
            return Err(format!("API错误 (code={}): {}", code, msg));
        }
    }
    // Also check for overloaded error
    if body.contains("overloaded_error") || body.contains("当前时段请求拥挤") {
        #[derive(Deserialize)]
        struct OverloadError { error: OverloadDetail }
        #[derive(Deserialize)]
        struct OverloadDetail { message: String }
        if let Ok(oe) = serde_json::from_str::<OverloadError>(&body) {
            return Err(format!("API拥挤: {}", oe.error.message));
        }
    }
    #[derive(Deserialize)]
    struct MiniMaxResponse {
        choices: Vec<MiniMaxChoice>,
    }

    #[derive(Deserialize)]
    struct MiniMaxChoice {
        message: MiniMaxMessage,
    }

    #[derive(Deserialize)]
    struct MiniMaxMessage {
        role: String,
        content: String,
    }

    let resp_json: MiniMaxResponse =
        serde_json::from_str(&body).map_err(|e| format!("响应解析失败: {} | body={}", e, &body))?;

    resp_json
        .choices
        .first()
        .map(|c| c.message.content.clone())
        .ok_or_else(|| "响应中未找到有效内容".to_string())
}

// OpenAI-compatible API call
fn call_openai_api(api_key: &str, model: &str, system_prompt: &str, user_prompt: &str) -> Result<String, String> {
    #[derive(Serialize)]
    struct ChatMessage { role: String, content: String }

    #[derive(Serialize)]
    struct ChatRequest { model: String, messages: Vec<ChatMessage>, temperature: f32 }

    let request_body = ChatRequest {
        model: model.to_string(),
        messages: vec![
            ChatMessage { role: "system".to_string(), content: system_prompt.to_string() },
            ChatMessage { role: "user".to_string(), content: user_prompt.to_string() },
        ],
        temperature: 0.7,
    };

    let json_body = serde_json::to_string(&request_body).map_err(|e| format!("序列化失败: {}", e))?;

    let output = std::process::Command::new("curl")
        .args([
            "-x", "http://127.0.0.1:7890",
            "-s", "--max-time", "30",
            "-X", "POST",
            "https://api.openai.com/v1/chat/completions",
            "-H", &format!("Authorization: Bearer {}", api_key),
            "-H", "Content-Type: application/json",
            "-d", &json_body,
        ])
        .output()
        .map_err(|e| format!("curl执行失败: {}", e))?;

    let stderr = String::from_utf8_lossy(&output.stderr);
    if !stderr.is_empty() {
        eprintln!("[curl stderr] {}", stderr);
    }

    let body = String::from_utf8_lossy(&output.stdout);
    if body.is_empty() {
        return Err("API返回空响应".to_string());
    }

    #[derive(Deserialize)]
    struct OpenAIResponse { choices: Vec<OpenAIChoice> }
    #[derive(Deserialize)]
    struct OpenAIChoice { message: OpenAIMessage }
    #[derive(Deserialize)]
    struct OpenAIMessage { role: String, content: String }

    let resp_json: OpenAIResponse =
        serde_json::from_str(&body).map_err(|e| format!("响应解析失败: {} | body={}", e, &body))?;
    resp_json.choices.first().map(|c| c.message.content.clone())
        .ok_or_else(|| "响应中未找到有效内容".to_string())
}

// ─────────────────────────────────────────────
// Tauri Commands
// ─────────────────────────────────────────────

#[tauri::command]
fn list_books() -> Result<BookIndex, String> {
    ensure_app_dirs()?;
    let path = books_index_path()?;
    let count = if path.exists() {
        read_json::<BookIndex>(&path).map(|idx| idx.books.len()).unwrap_or(0)
    } else {
        0
    };
    println!("[DEBUG list_books] path={:?} count={}", path, count);
    if path.exists() {
        read_json(&path)
    } else {
        Ok(BookIndex { books: vec![] })
    }
}

#[tauri::command]
fn get_book(book_id: String) -> Result<Book, String> {
    ensure_app_dirs()?;
    read_json(&book_path(&book_id)?)
}

#[tauri::command]
fn create_book(name: String) -> Result<Book, String> {
    ensure_app_dirs()?;
    let index_path = books_index_path()?;
    let mut index: BookIndex = if index_path.exists() {
        read_json(&index_path)?
    } else {
        BookIndex { books: vec![] }
    };

    let now = now_iso();
    let book = Book {
        id: Uuid::new_v4().to_string(),
        name,
        created_at: now.clone(),
        updated_at: now.clone(),
        chapters: vec![],
        characters: vec![],
        plotlines: vec![],
    };

    index.books.push(BookMeta {
        id: book.id.clone(),
        name: book.name.clone(),
        created_at: book.created_at.clone(),
        updated_at: book.updated_at.clone(),
    });

    write_json(&index_path, &index)?;
    write_json(&book_path(&book.id)?, &book)?;

    Ok(book)
}

#[tauri::command]
fn update_book(book_id: String, name: String) -> Result<Book, String> {
    ensure_app_dirs()?;
    let index_path = books_index_path()?;
    let mut index: BookIndex = read_json(&index_path)?;
    let mut book: Book = read_json(&book_path(&book_id)?)?;

    book.name = name;
    book.updated_at = now_iso();

    if let Some(meta) = index.books.iter_mut().find(|b| b.id == book_id) {
        meta.name = book.name.clone();
        meta.updated_at = book.updated_at.clone();
    }

    write_json(&index_path, &index)?;
    write_json(&book_path(&book_id)?, &book)?;

    Ok(book)
}

#[tauri::command]
fn delete_book(book_id: String) -> Result<(), String> {
    ensure_app_dirs()?;
    let index_path = books_index_path()?;
    let book_file_path = book_path(&book_id)?;

    let mut index: BookIndex = read_json(&index_path)?;
    index.books.retain(|b| b.id != book_id);
    write_json(&index_path, &index)?;

    if book_file_path.exists() {
        fs::remove_file(book_file_path).map_err(|e| e.to_string())?;
    }

    let outline_p = outline_path(&book_id)?;
    if outline_p.exists() {
        fs::remove_file(outline_p).map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
fn create_chapter(book_id: String, title: String) -> Result<Chapter, String> {
    ensure_app_dirs()?;
    let book_file_path = book_path(&book_id)?;
    let mut book: Book = read_json(&book_file_path)?;

    let order = book.chapters.len() as i32;
    let chapter = Chapter {
        id: Uuid::new_v4().to_string(),
        title,
        content: String::new(),
        order,
        characters_appeared: vec![],
        plotline_ids: vec![],
    };

    book.chapters.push(chapter.clone());
    book.updated_at = now_iso();
    write_json(&book_file_path, &book)?;

    Ok(chapter)
}

#[tauri::command]
fn update_chapter(book_id: String, chapter: Chapter) -> Result<Chapter, String> {
    ensure_app_dirs()?;
    let book_file_path = book_path(&book_id)?;
    let mut book: Book = read_json(&book_file_path)?;

    if let Some(idx) = book.chapters.iter().position(|c| c.id == chapter.id) {
        book.chapters[idx] = chapter.clone();
        book.updated_at = now_iso();
        write_json(&book_file_path, &book)?;
        Ok(chapter)
    } else {
        Err("Chapter not found".to_string())
    }
}

#[tauri::command]
fn delete_chapter(book_id: String, chapter_id: String) -> Result<(), String> {
    ensure_app_dirs()?;
    let book_file_path = book_path(&book_id)?;
    let mut book: Book = read_json(&book_file_path)?;

    book.chapters.retain(|c| c.id != chapter_id);
    for (i, c) in book.chapters.iter_mut().enumerate() {
        c.order = i as i32;
    }
    book.updated_at = now_iso();
    write_json(&book_file_path, &book)?;

    Ok(())
}

#[tauri::command]
fn reorder_books(book_ids: Vec<String>) -> Result<(), String> {
    ensure_app_dirs()?;
    let index_path = books_index_path()?;
    let mut index: BookIndex = read_json(&index_path)?;
    println!("[DEBUG reorder_books] before reorder: {:?}", index.books.iter().map(|b| &b.name).collect::<Vec<_>>());

    let mut reordered = Vec::new();
    let mut remaining: Vec<_> = index.books.iter().collect();
    for id in &book_ids {
        if let Some(pos) = remaining.iter().position(|b| &b.id == id) {
            reordered.push(remaining.remove(pos).clone());
        }
    }
    for b in remaining {
        reordered.push(b.clone());
    }
    index.books = reordered;
    println!("[DEBUG reorder_books] after reorder: {:?}", index.books.iter().map(|b| &b.name).collect::<Vec<_>>());
    write_json(&index_path, &index)?;
    println!("[DEBUG reorder_books] written to {:?}", index_path);
    Ok(())
}

#[tauri::command]
fn reorder_characters(book_id: String, char_ids: Vec<String>) -> Result<(), String> {
    ensure_app_dirs()?;
    let book_file_path = book_path(&book_id)?;
    let mut book: Book = read_json(&book_file_path)?;

    let mut reordered = Vec::new();
    let mut remaining: Vec<_> = book.characters.iter().collect();
    for id in &char_ids {
        if let Some(pos) = remaining.iter().position(|c| &c.id == id) {
            reordered.push(remaining.remove(pos).clone());
        }
    }
    for c in remaining {
        reordered.push(c.clone());
    }
    book.characters = reordered;
    book.updated_at = now_iso();
    write_json(&book_file_path, &book)?;
    Ok(())
}

#[tauri::command]
fn reorder_chapters(book_id: String, chapter_ids: Vec<String>) -> Result<(), String> {
    ensure_app_dirs()?;
    let book_file_path = book_path(&book_id)?;
    let mut book: Book = read_json(&book_file_path)?;

    let mut reordered = Vec::new();
    for (i, cid) in chapter_ids.iter().enumerate() {
        if let Some(ch) = book.chapters.iter().find(|c| &c.id == cid) {
            let mut ch = ch.clone();
            ch.order = i as i32;
            reordered.push(ch);
        }
    }
    book.chapters = reordered;
    book.updated_at = now_iso();
    write_json(&book_file_path, &book)?;

    Ok(())
}

#[tauri::command]
fn create_character(book_id: String, character: Character) -> Result<Character, String> {
    ensure_app_dirs()?;
    let book_file_path = book_path(&book_id)?;
    let mut book: Book = read_json(&book_file_path)?;

    let mut ch = character;
    if ch.id.is_empty() {
        ch.id = Uuid::new_v4().to_string();
    }
    book.characters.push(ch.clone());
    book.updated_at = now_iso();
    write_json(&book_file_path, &book)?;

    Ok(ch)
}

#[tauri::command]
fn update_character(book_id: String, character: Character) -> Result<Character, String> {
    ensure_app_dirs()?;
    let book_file_path = book_path(&book_id)?;
    let mut book: Book = read_json(&book_file_path)?;

    if let Some(idx) = book.characters.iter().position(|c| c.id == character.id) {
        book.characters[idx] = character.clone();
        book.updated_at = now_iso();
        write_json(&book_file_path, &book)?;
        Ok(character)
    } else {
        Err("Character not found".to_string())
    }
}

#[tauri::command]
fn delete_character(book_id: String, character_id: String) -> Result<(), String> {
    ensure_app_dirs()?;
    let book_file_path = book_path(&book_id)?;
    let mut book: Book = read_json(&book_file_path)?;

    book.characters.retain(|c| c.id != character_id);
    book.updated_at = now_iso();
    write_json(&book_file_path, &book)?;

    Ok(())
}

#[tauri::command]
fn get_plotlines(book_id: String) -> Result<Vec<Plotline>, String> {
    ensure_app_dirs()?;
    let book: Book = read_json(&book_path(&book_id)?)?;
    Ok(book.plotlines)
}

#[tauri::command]
fn save_plotlines(book_id: String, plotlines: Vec<Plotline>) -> Result<(), String> {
    ensure_app_dirs()?;
    let book_file_path = book_path(&book_id)?;
    let mut book: Book = read_json(&book_file_path)?;

    book.plotlines = plotlines;
    book.updated_at = now_iso();
    write_json(&book_file_path, &book)?;

    Ok(())
}

#[tauri::command]
fn get_outline(book_id: String) -> Result<Outline, String> {
    ensure_app_dirs()?;
    let path = outline_path(&book_id)?;
    if path.exists() {
        read_json(&path)
    } else {
        Ok(Outline { acts: vec![] })
    }
}

#[tauri::command]
fn save_outline(book_id: String, outline: Outline) -> Result<(), String> {
    ensure_app_dirs()?;
    let path = outline_path(&book_id)?;
    write_json(&path, &outline)
}

// ─────────────────────────────────────────────
// AI Commands
// ─────────────────────────────────────────────

#[tauri::command]
fn get_ai_config() -> Result<AIConfigOutput, String> {
    ensure_app_dirs()?;
    let cfg = read_ai_config()?;
    Ok(AIConfigOutput {
        has_api_key: !cfg.api_key.is_empty(),
        model: cfg.model,
        provider: cfg.provider,
        context_chapters: cfg.context_chapters,
        context_characters: cfg.context_characters,
        context_plotlines: cfg.context_plotlines,
    })
}

#[tauri::command]
fn get_api_key() -> Result<String, String> {
    let cfg = read_ai_config()?;
    if cfg.api_key.is_empty() {
        Err("API Key 未设置".to_string())
    } else {
        Ok(cfg.api_key)
    }
}

#[tauri::command]
fn set_ai_config(
    api_key: Option<String>,
    model: Option<String>,
    provider: Option<String>,
    context_chapters: Option<i32>,
    context_characters: Option<Vec<String>>,
    context_plotlines: Option<Vec<String>>,
) -> Result<(), String> {
    ensure_app_dirs()?;
    let mut cfg = read_ai_config()?;

    if let Some(key) = api_key {
        cfg.api_key = key;
    }
    if let Some(m) = model {
        cfg.model = m;
    }
    if let Some(p) = provider {
        cfg.provider = p;
    }
    if let Some(ch) = context_chapters {
        cfg.context_chapters = ch;
    }
    if let Some(chars) = context_characters {
        cfg.context_characters = chars;
    }
    if let Some(plots) = context_plotlines {
        cfg.context_plotlines = plots;
    }

    write_json(&ai_config_path()?, &cfg)?;
    // verify write
    let verify = read_ai_config()?;
    if verify.api_key != cfg.api_key || verify.model != cfg.model {
        eprintln!("[ERROR] config write verification failed!");
        return Err("配置写入验证失败，请检查文件权限".to_string());
    }
    println!("[OK] set_ai_config saved api_key len={}", cfg.api_key.len());
    Ok(())
}
fn build_context_internal(book_id: &str, cfg: AIContextConfig) -> Result<String, String> {
    ensure_app_dirs()?;
    let book: Book = read_json(&book_path(book_id)?)?;

    let mut context_parts = vec![];

    // Book overview
    context_parts.push(format!(
        "=== 书籍信息 ===\n书名：《{}》\n人物数量：{} | 章节数量：{} | 情节线数量：{}",
        book.name,
        book.characters.len(),
        book.chapters.len(),
        book.plotlines.len()
    ));

    // Characters
    let char_ids = cfg.context_characters.unwrap_or_default();
    let chars_to_include: Vec<&Character> = if char_ids.is_empty() {
        book.characters.iter().take(10).collect()
    } else {
        book.characters.iter().filter(|c| char_ids.contains(&c.id)).collect()
    };

    if !chars_to_include.is_empty() {
        let char_blocks: Vec<String> = chars_to_include
            .iter()
            .map(|c| {
                format!(
                    "- {}：{} | 状态：{} | 人设：{}\n  关系：{}",
                    c.name,
                    c.description,
                    c.status,
                    c.personality,
                    c.relationships
                        .iter()
                        .map(|r| format!("{}（{}）", r.target_id, r.rel_type))
                        .collect::<Vec<_>>()
                        .join("、")
                )
            })
            .collect();
        context_parts.push(format!("\n=== 人物档案 ===\n{}", char_blocks.join("\n")));
    }

    // Plotlines
    let plot_ids = cfg.context_plotlines.unwrap_or_default();
    let plots_to_include: Vec<&Plotline> = if plot_ids.is_empty() {
        book.plotlines.iter().take(10).collect()
    } else {
        book
            .plotlines
            .iter()
            .filter(|p| plot_ids.contains(&p.id))
            .collect()
    };

    if !plots_to_include.is_empty() {
        let plot_blocks: Vec<String> = plots_to_include
            .iter()
            .map(|p| {
                format!(
                    "- {} [{}]：「{}」| 状态：{} | 关联章节：{}",
                    p.name, p.color, p.description, p.status, p.chapter_ids.len()
                )
            })
            .collect();
        context_parts.push(format!("\n=== 情节线 ===\n{}", plot_blocks.join("\n")));
    }

    // Recent chapters
    let num_chapters = cfg.context_chapters.unwrap_or(5) as usize;
    let recent_chapters: Vec<&Chapter> = book.chapters.iter().rev().take(num_chapters).collect();
    if !recent_chapters.is_empty() {
        let chapter_blocks: Vec<String> = recent_chapters
            .iter()
            .rev()
            .map(|ch| {
                let preview = if ch.content.len() > 200 {
                    format!("{}...", &ch.content[..200])
                } else {
                    ch.content.clone()
                };
                format!("第{}章「{}」：\n{}\n---", ch.order + 1, ch.title, preview)
            })
            .collect();
        context_parts.push(format!(
            "\n=== 近{}章内容 ===\n{}",
            recent_chapters.len(),
            chapter_blocks.join("\n")
        ));
    }

    Ok(context_parts.join("\n"))
}

#[tauri::command]
fn build_context(book_id: String, context_config: Option<AIContextConfig>) -> Result<String, String> {
    let cfg = context_config.unwrap_or(AIContextConfig {
        context_chapters: None,
        context_characters: None,
        context_plotlines: None,
    });
    build_context_internal(&book_id, cfg)
}


#[tauri::command]
fn build_ai_prompt(
    feature: String,
    params: serde_json::Value,
    context_config: Option<AIContextConfig>,
    book_id: Option<String>,
) -> Result<(String, String), String> {
    // Build context if book_id provided
    let context_str = if let (Some(bid), Some(cc)) = (&book_id, &context_config) {
        let cfg = AIContextConfig {
            context_chapters: cc.context_chapters,
            context_characters: cc.context_characters.clone(),
            context_plotlines: cc.context_plotlines.clone(),
        };
        build_context_internal(bid, cfg).ok()
    } else if context_config.is_some() {
        let cc = context_config.unwrap();
        let cfg = AIContextConfig {
            context_chapters: cc.context_chapters,
            context_characters: cc.context_characters,
            context_plotlines: cc.context_plotlines,
        };
        build_context_internal("", cfg).ok()
    } else {
        None
    };

    let system_prompt = get_feature_system_prompt(&feature);
    let user_prompt = build_user_prompt(&feature, params, context_str);
    Ok((system_prompt.to_string(), user_prompt.to_string()))
}

#[tauri::command]
fn invoke_ai(
    feature: String,
    params: serde_json::Value,
    context_config: Option<AIContextConfig>,
    book_id: Option<String>,
    provider: Option<String>,
) -> Result<String, String> {
    println!("[DEBUG invoke_ai] feature={} params={} book_id={:?}", feature, params, book_id);
    ensure_app_dirs()?;
    let cfg = read_ai_config()?;
    let provider = provider.unwrap_or_else(|| cfg.provider.clone());

    if cfg.api_key.is_empty() {
        return Err("API Key 未设置，请在设置中配置 API Key".to_string());
    }

    // Build context
    let context_str = if let (Some(bid), Some(cc)) = (&book_id, &context_config) {
        let cc_owned = AIContextConfig {
            context_chapters: cc.context_chapters.or(Some(cfg.context_chapters)),
            context_characters: cc.context_characters.clone().filter(|v| !v.is_empty()).or_else(|| Some(cfg.context_characters.clone()).filter(|v| !v.is_empty())),
            context_plotlines: cc.context_plotlines.clone().filter(|v| !v.is_empty()).or_else(|| Some(cfg.context_plotlines.clone()).filter(|v| !v.is_empty())),
        };
        build_context_internal(&bid, cc_owned).ok()
    } else if context_config.is_some() {
        let cc_owned = AIContextConfig {
            context_chapters: context_config.as_ref().and_then(|c| c.context_chapters),
            context_characters: context_config.as_ref().and_then(|c| c.context_characters.clone()),
            context_plotlines: context_config.as_ref().and_then(|c| c.context_plotlines.clone()),
        };
        build_context_internal("", cc_owned).ok()
    } else {
        None
    };

    let system_prompt = get_feature_system_prompt(&feature);
    let user_prompt = build_user_prompt(&feature, params, context_str);

    match provider.as_str() {
        "openai" => call_openai_api(&cfg.api_key, &cfg.model, system_prompt, &user_prompt),
        _ => call_minimax_api(&cfg.api_key, &cfg.model, system_prompt, &user_prompt),
    }
}

#[tauri::command]
fn debug_books() -> Result<String, String> {
    let index_path = books_index_path()?;
    let base_path = app_data_dir()?;
    let index: BookIndex = read_json(&index_path)?;
    let mut result = format!(
        "Books count: {} | app_data_dir: {:?} | index_path: {:?}\n",
        index.books.len(),
        base_path,
        index_path
    );
    for meta in &index.books {
        let book: Book = read_json(&book_path(&meta.id)?)?;
        result.push_str(&format!(
            "  - {} (chapters: {}, characters: {})\n",
            book.name,
            book.chapters.len(),
            book.characters.len()
        ));
    }
    Ok(result)
}

// ─────────────────────────────────────────────
// App entry
// ─────────────────────────────────────────────

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            list_books,
            get_book,
            create_book,
            update_book,
            delete_book,
            reorder_books,
            create_chapter,
            update_chapter,
            delete_chapter,
            reorder_chapters,
            create_character,
            update_character,
            delete_character,
            reorder_characters,
            get_plotlines,
            save_plotlines,
            get_outline,
            save_outline,
            debug_books,
            get_api_key,
            get_ai_config,
            set_ai_config,
            build_ai_prompt,
            build_context,
            invoke_ai,
        ])
        .setup(|app| {
            // Setup system tray
            let show_item = MenuItemBuilder::with_id("show", "显示主窗口").build(app)?;
            let quit_item = MenuItemBuilder::with_id("quit", "退出").build(app)?;
            let menu = MenuBuilder::new(app)
                .item(&show_item)
                .separator()
                .item(&quit_item)
                .build()?;
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click { button, button_state, .. } = event {
                        if button == tauri::tray::MouseButton::Left && button_state == tauri::tray::MouseButtonState::Up {
                            let app = tray.app_handle();
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .build(app)?;

            // Hide window instead of closing on close button
            if let Some(window) = app.get_webview_window("main") {
                let win = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        let _ = win.hide();
                    }
                });
            }

            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
