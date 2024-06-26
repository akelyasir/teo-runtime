use serde::Serialize;
use teo_result::{Result, Error};

#[derive(Debug, Serialize, Clone)]
pub enum Language {
    EnUs,
    EnUk,
    De,
    Fr,
    Es,
    Hi,
    He,
    Ja,
    Ko,
    ZhCn,
    ZhTw,
}

impl Language {

    pub fn from_str(name: &str) -> Result<Self> {
        Ok(match name {
            "enUs" => Self::EnUs,
            "enUk" => Self::EnUk,
            "de" => Self::De,
            "fr" => Self::Fr,
            "es" => Self::Es,
            "hi" => Self::Hi,
            "he" => Self::He,
            "ja" => Self::Ja,
            "ko" => Self::Ko,
            "zhCn" => Self::ZhCn,
            "zhTw" => Self::ZhTw,
            _ => Err(Error::new("cannot convert this language code to language"))?
        })
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Language::EnUs => "enUs",
            Language::EnUk => "enUk",
            Language::De => "de",
            Language::Fr => "fr",
            Language::Es => "es",
            Language::Hi => "hi",
            Language::He => "he",
            Language::Ja => "ja",
            Language::Ko => "ko",
            Language::ZhCn => "zhCn",
            Language::ZhTw => "zhTw",
        }
    }

    pub fn display(&self) -> &'static str {
        match self {
            Language::EnUs => "English (United States)",
            Language::EnUk => "English (United Kingdom)",
            Language::De => "Deutsch",
            Language::Fr => "Français",
            Language::Es => "Español",
            Language::Hi => "हिन्दी",
            Language::He => "עברית",
            Language::Ja => "日本語",
            Language::Ko => "한국어",
            Language::ZhCn => "中文（简体）",
            Language::ZhTw => "中文（繁體）",
        }
    }
}