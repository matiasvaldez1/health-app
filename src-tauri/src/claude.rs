use crate::db::DbPool;
use crate::models::*;
use crate::services;
use serde::{Deserialize, Serialize};

const CLAUDE_API_URL: &str = "https://api.anthropic.com/v1/messages";
const MODEL: &str = "claude-sonnet-4-20250514";

#[derive(Serialize)]
struct ApiRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ApiResponse {
    content: Vec<ContentBlock>,
}

#[derive(Deserialize)]
struct ContentBlock {
    text: Option<String>,
}

pub struct HealthContext {
    pub recent_weights: Vec<WeightEntry>,
    pub recent_meditations: Vec<MeditationSession>,
    pub recent_feelings: Vec<FeelingEntry>,
}

pub fn gather_context(pool: &DbPool) -> Result<HealthContext, String> {
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let thirty_days_ago = (chrono::Local::now() - chrono::Duration::days(30))
        .format("%Y-%m-%d")
        .to_string();
    let seven_days_ago = (chrono::Local::now() - chrono::Duration::days(7))
        .format("%Y-%m-%d")
        .to_string();

    let recent_weights = services::list_weights(pool, Some(thirty_days_ago.clone()), Some(today.clone()))?;
    let recent_meditations = services::list_meditations(pool, Some(thirty_days_ago), Some(today.clone()))?;
    let recent_feelings = services::list_feelings(pool, Some(seven_days_ago), Some(today))?;

    Ok(HealthContext {
        recent_weights,
        recent_meditations,
        recent_feelings,
    })
}

fn build_prompt(ctx: &HealthContext) -> String {
    let mut prompt = String::from(
        "You are a supportive health and wellness assistant. Based on the following personal health data, provide 3-5 personalized, actionable health tips.\n\n",
    );

    prompt.push_str("## Weight Log (last 30 days)\n");
    if ctx.recent_weights.is_empty() {
        prompt.push_str("No weight entries recorded.\n");
    } else {
        for w in &ctx.recent_weights {
            prompt.push_str(&format!(
                "- {} | {:.1} kg{}\n",
                w.date,
                w.weight_kg,
                w.notes.as_ref().map(|n| format!(" | {}", n)).unwrap_or_default()
            ));
        }
    }

    prompt.push_str("\n## Meditation Sessions (last 30 days)\n");
    if ctx.recent_meditations.is_empty() {
        prompt.push_str("No meditation sessions recorded.\n");
    } else {
        for m in &ctx.recent_meditations {
            prompt.push_str(&format!(
                "- {} at {} | {} min{}\n",
                m.date,
                m.time,
                m.duration_min,
                m.notes.as_ref().map(|n| format!(" | {}", n)).unwrap_or_default()
            ));
        }
    }

    prompt.push_str("\n## Recent Feelings (last 7 days)\n");
    if ctx.recent_feelings.is_empty() {
        prompt.push_str("No feeling entries recorded.\n");
    } else {
        for f in &ctx.recent_feelings {
            prompt.push_str(&format!(
                "- {} | {}{}\n",
                f.date,
                f.content,
                f.mood_score
                    .map(|s| format!(" (mood: {}/10)", s))
                    .unwrap_or_default()
            ));
        }
    }

    prompt.push_str(
        "\nGuidelines:\n\
        - Be encouraging and non-judgmental\n\
        - Reference specific patterns you see in the data\n\
        - Suggest concrete, small actions\n\
        - If there's not enough data, encourage more tracking\n\
        - Keep tips concise (2-3 sentences each)\n\
        - Format each tip with a bold title and description\n",
    );

    prompt
}

pub async fn generate_tips(api_key: &str, pool: &DbPool) -> Result<String, String> {
    let ctx = gather_context(pool)?;
    let prompt = build_prompt(&ctx);

    let client = reqwest::Client::new();
    let body = ApiRequest {
        model: MODEL.to_string(),
        max_tokens: 1024,
        messages: vec![Message {
            role: "user".to_string(),
            content: prompt,
        }],
    };

    let resp = client
        .post(CLAUDE_API_URL)
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("API request failed: {}", e))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("API error ({}): {}", status, body));
    }

    let api_resp: ApiResponse = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse API response: {}", e))?;

    let text = api_resp
        .content
        .into_iter()
        .filter_map(|c| c.text)
        .collect::<Vec<_>>()
        .join("\n");

    Ok(text)
}
