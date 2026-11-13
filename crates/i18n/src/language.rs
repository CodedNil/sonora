use unic_langid::{LanguageIdentifier, langid};

pub const AUTO: &str = "auto";

/// Declares each shipped locale once, keeping its identifier, native name and resources together.
macro_rules! languages {
    ($(($variant:ident, $id:literal, $label:literal)),+ $(,)?) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(usize)]
        pub enum Language { $($variant),+ }

        impl Language {
            pub const ALL: [Self; [$(stringify!($variant)),+].len()] = [$(Self::$variant),+];

            pub fn id(self) -> &'static str {
                match self { $(Self::$variant => $id),+ }
            }

            pub fn label(self) -> &'static str {
                match self { $(Self::$variant => $label),+ }
            }

            pub(crate) fn tag(self) -> LanguageIdentifier {
                match self { $(Self::$variant => langid!($id)),+ }
            }

            pub(crate) fn source(self) -> &'static str {
                match self {
                    $(Self::$variant => concat!(
                        include_str!(concat!("../../../assets/i18n/", $id, "/main.ftl")),
                        "\n",
                        include_str!(concat!("../../../assets/i18n/", $id, "/settings.ftl")),
                        "\n",
                        include_str!(concat!("../../../assets/i18n/", $id, "/library.ftl")),
                        "\n",
                        include_str!(concat!("../../../assets/i18n/", $id, "/metadata.ftl")),
                        "\n",
                        include_str!(concat!("../../../assets/i18n/", $id, "/menus.ftl")),
                        "\n",
                        include_str!(concat!("../../../assets/i18n/", $id, "/playback.ftl")),
                        "\n",
                        include_str!(concat!("../../../assets/i18n/", $id, "/accounts.ftl")),
                        "\n",
                        include_str!(concat!("../../../assets/i18n/", $id, "/equalizer.ftl")),
                        "\n",
                        include_str!(concat!("../../../assets/i18n/", $id, "/notifications.ftl")),
                    )),+
                }
            }
        }
    };
}

languages! {
    (English, "en-US", "English"),
    (German, "de", "Deutsch"),
    (Spanish, "es", "Español"),
    (French, "fr", "Français"),
    (Italian, "it", "Italiano"),
    (Indonesian, "id", "Bahasa Indonesia"),
    (Japanese, "ja", "日本語"),
    (Russian, "ru", "Русский"),
    (Ukrainian, "uk", "Українська"),
    (Polish, "pl", "Polski"),
    (Czech, "cs", "Čeština"),
    (PortugueseBrazilian, "pt-BR", "Português (Brasil)"),
    (ChineseSimplified, "zh-CN", "简体中文"),
    (Turkish, "tr", "Türkçe"),
    (Albanian, "sq", "Shqip"),
}

impl Language {
    pub fn from_id(id: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|language| language.id() == id)
    }

    pub fn detect() -> Self {
        let Some(locale) = sys_locale::get_locale() else {
            return Self::English;
        };
        let primary = base(&locale);

        Self::ALL
            .into_iter()
            .find(|language| base(language.id()) == primary)
            .unwrap_or(Self::English)
    }
}

pub fn resolve(id: &str) -> Language {
    Language::from_id(id).unwrap_or_else(Language::detect)
}

fn base(tag: &str) -> String {
    tag.split(['-', '_'])
        .next()
        .unwrap_or_default()
        .to_lowercase()
}
