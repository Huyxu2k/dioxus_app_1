use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use dioxus_i18n::unic_langid::{langid, LanguageIdentifier};
use serde::{Deserialize,Serialize};

const VI_VN: &str = include_str!("../../../assets/i18n/vi-VN.ftl");
const EN_US: &str = include_str!("../../../assets/i18n/en-US.ftl");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum AppLanguage {
    Vi,
    En
}

impl AppLanguage {
    pub const ALL: [Self;2] = [Self::Vi, Self::En];

    pub fn code(self) -> &'static str{
        match self {
            Self::Vi => "vi",
            Self::En => "en"
        }
    }

    pub fn flag_asset(self) -> Asset{
        match self {
            Self::Vi => todo!(),
            Self::En => todo!(),
        }
    }

    pub fn language_id(self) -> LanguageIdentifier{
        match self {
            Self::Vi => langid!("vi-VN"),
            Self::En => langid!("en-US"),
        }
    }

    pub fn from_code(value: &str) -> Self {
        match value {
            "en" => Self::En,
            _ => Self::Vi
        }
    }
}

impl Default for AppLanguage {
    fn default() -> Self {
        Self::Vi
    }
}

pub fn config(init_language: AppLanguage) -> I18nConfig{
    I18nConfig::new(init_language.language_id())
            .with_fallback(AppLanguage::Vi.language_id())
            .with_locale((AppLanguage::Vi.language_id(), VI_VN))
            .with_locale((AppLanguage::En.language_id(), EN_US))
}