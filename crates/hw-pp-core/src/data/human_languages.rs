//! The human-language catalog: languages, dialects, scripts and their greetings.

use crate::data::common::{
    DEFAULT_HUMAN_LANGUAGE, DEFAULT_HUMAN_LANGUAGE_DIALECT, DEFAULT_SCRIPT, unique,
};
use crate::data::lookup::LookupError;
use crate::data::macros::name_enum;

use thiserror::Error;

name_enum! {
    /// A classification of a human language; a language may have several.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum HumanLanguageCategory: ParseHumanLanguageCategoryError("category") {
        /// Evolved naturally among its speakers.
        Natural => "Natural",
        /// Deliberately designed (e.g. Esperanto, Klingon).
        Constructed => "Constructed",
        /// Expressed with the hands, face and body (e.g. American Sign Language).
        Signed => "Signed",
        /// No native speakers but still in use (e.g. Latin); see
        /// <https://www.globallanguageservices.co.uk/difference-extinct-language-dead-language/>.
        Dead => "Dead",
        /// No speakers and no longer in use (e.g. Sumerian).
        Extinct => "Extinct",
        /// No demonstrable relation to any other language (e.g. Basque).
        Isolate => "Isolate",
        /// A stable, natively spoken language formed from a contact language.
        Creole => "Creole",
        /// A simplified contact language with no native speakers.
        Pidgin => "Pidgin",
        /// Built for international communication (e.g. Esperanto, Interlingua).
        Auxiliary => "Auxiliary",
        /// Invented for a fictional world (e.g. Klingon).
        Fictional => "Fictional",
        /// Used in religious ceremony.
        Liturgical => "Liturgical",
        /// Spoken by whistling (e.g. Silbo Gomero).
        Whistled => "Whistled",
    }
}

name_enum! {
    /// A language family or major branch, as listed by Wikipedia's
    /// [language family](https://en.wikipedia.org/wiki/Language_family) and
    /// [list of language families](https://en.wikipedia.org/wiki/List_of_language_families);
    /// a language records its family and, where well known, its branch.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum HumanLanguageFamily: ParseHumanLanguageFamilyError("language family") {
        /// Most of Europe and South Asia (English, Hindi, Russian).
        IndoEuropean => "Indo-European",
        /// Chinese, Burmese, Tibetan.
        SinoTibetan => "Sino-Tibetan",
        /// Sub-Saharan Africa (Yoruba, Swahili, Zulu); Glottolog splits it, counting only Atlantic-Congo as proven.
        NigerCongo => "Niger-Congo",
        /// North Africa and the Middle East (Arabic, Hebrew, Amharic, Hausa).
        Afroasiatic => "Afroasiatic",
        /// Maritime Southeast Asia and the Pacific (Malay, Tagalog, Māori, Malagasy).
        Austronesian => "Austronesian",
        /// Highlands of New Guinea.
        TransNewGuinea => "Trans-New Guinea",
        /// Central and East Africa (Kanuri, Luo, Maasai); a contested grouping.
        NiloSaharan => "Nilo-Saharan",
        /// Mainland Southeast Asia and India (Vietnamese, Khmer, Santali).
        Austroasiatic => "Austroasiatic",
        /// Thai, Lao, Zhuang.
        KraDai => "Kra-Dai" | "Tai-Kadai",
        /// South India (Tamil, Telugu, Kannada, Malayalam).
        Dravidian => "Dravidian",
        /// Turkish, Kazakh, Uzbek, Yakut.
        Turkic => "Turkic",
        /// Finnish, Estonian, Hungarian, Sami.
        Uralic => "Uralic",
        /// Japanese and the Ryukyuan languages.
        Japonic => "Japonic",
        /// Korean and Jeju.
        Koreanic => "Koreanic",
        /// Mongolian, Buryat, Kalmyk.
        Mongolic => "Mongolic",
        /// Georgian, Mingrelian, Svan.
        Kartvelian => "Kartvelian",
        /// Chechen, Avar, Lezgian.
        NortheastCaucasian => "Northeast Caucasian",
        /// Abkhaz, Adyghe, Kabardian.
        NorthwestCaucasian => "Northwest Caucasian",
        /// Hmong, Mien.
        HmongMien => "Hmong-Mien" | "Miao-Yao",
        /// Greenlandic, Inuktitut, Yupik, Aleut.
        EskimoAleut => "Eskimo-Aleut",
        /// The Quechua languages of the Andes.
        Quechuan => "Quechuan",
        /// Nahuatl, Hopi, Comanche.
        UtoAztecan => "Uto-Aztecan",
        /// Cherokee, Mohawk, Seneca.
        Iroquoian => "Iroquoian",
        /// Yucatec Maya, Kʼicheʼ, Qʼeqchiʼ.
        Mayan => "Mayan",
        /// Zapotec, Mixtec, Otomi.
        OtoManguean => "Oto-Manguean",
        /// Guarani, Tupi.
        Tupian => "Tupian",
        /// Wayuu, Asháninka, Garifuna.
        Arawakan => "Arawakan",
        /// Navajo, Apache, Tlingit.
        NaDene => "Na-Dene",
        /// Cree, Ojibwe, Blackfoot.
        Algic => "Algic",
        /// Most Aboriginal Australian languages (Warlpiri, Pitjantjatjara).
        PamaNyungan => "Pama-Nyungan",
        /// Southern African click languages (ǃXóõ, ǀXam, ǁXegwi).
        Tuu => "Tuu",
        /// ǃKung and ǂʼAmkoe.
        Kxa => "Kxʼa" | "Kxa",
        /// Khoekhoe (Nama), Naro.
        KhoeKwadi => "Khoe-Kwadi",
        /// Sign languages descended from Old French Sign Language (ASL, LSF).
        FrenchSign => "French Sign",
        /// British, Australian and New Zealand Sign Language.
        Banzsl => "BANZSL",
        // Major branches, listed alongside their family.
        /// Branch of Indo-European (English, German, Swedish).
        Germanic => "Germanic",
        /// Branch of Indo-European: Latin and its Romance descendants.
        Italic => "Italic",
        /// Branch of Italic descended from Latin (Spanish, French, Italian).
        Romance => "Romance",
        /// Branch of Indo-European (Irish, Welsh, Breton).
        Celtic => "Celtic",
        /// Branch of Indo-European (Russian, Polish, Serbian).
        Slavic => "Slavic",
        /// Branch of Indo-European (Lithuanian, Latvian).
        Baltic => "Baltic",
        /// Branch of Indo-European: Greek, ancient and modern.
        Hellenic => "Hellenic",
        /// Branch of Indo-European (Hindi, Bengali, Sanskrit).
        IndoAryan => "Indo-Aryan",
        /// Branch of Indo-European (Persian, Kurdish, Pashto).
        Iranian => "Iranian",
        /// Branch of Afroasiatic (Arabic, Hebrew, Amharic).
        Semitic => "Semitic",
        /// Branch of Afroasiatic (Kabyle, Tamazight).
        Berber => "Berber",
        /// Branch of Afroasiatic (Oromo, Somali).
        Cushitic => "Cushitic",
        /// Branch of Afroasiatic (Hausa).
        Chadic => "Chadic",
        /// Branch of Afroasiatic: Ancient Egyptian and Coptic.
        Egyptian => "Egyptian",
        /// Branch of Niger-Congo (Swahili, Zulu, Shona).
        Bantu => "Bantu",
        /// Branch of Austronesian (Hawaiian, Māori, Samoan, Tongan).
        Polynesian => "Polynesian",
        /// Branch of Sino-Tibetan: the Chinese languages.
        Sinitic => "Sinitic",
    }
}

/// One recorded human language, as declared by its language file.
#[derive(Debug, Clone, Copy)]
pub struct HumanLanguage {
    /// The language's name.
    pub name: HumanLanguageName,
    /// Every category it belongs to.
    pub categories: &'static [HumanLanguageCategory],
    /// Its family and, where well known, its branch; empty for isolates, constructed
    /// languages, creoles and pidgins.
    pub families: &'static [HumanLanguageFamily],
    /// Every script it is written in; empty for unwritten (e.g. signed) languages.
    pub scripts: &'static [ScriptName],
    /// Its dialects; the first is the default.
    pub dialects: &'static [Dialect],
}

/// One dialect of a [`HumanLanguage`] and its greetings.
#[derive(Debug, Clone, Copy)]
pub struct Dialect {
    /// The dialect's name.
    pub label: DialectName,
    /// One greeting per spelling; the first is the default.
    pub greetings: &'static [Greeting],
}

/// A "Hello, World!" in one spelling.
#[derive(Debug, Clone, Copy)]
pub struct Greeting {
    /// The scripts this spelling is written in.
    pub scripts: &'static [ScriptName],
    /// The greeting itself.
    pub text: &'static str,
}

// Generated by `build.rs` from `human_languages/<shard>/<name>.rs`: one `mod`
// per language file, the `HumanLanguageName`, `DialectName` and `ScriptName`
// enums, and `HUMAN_LANGUAGES`. To add a language, add a file; nothing here changes.
include!(concat!(env!("OUT_DIR"), "/human_languages_registry.rs"));

/// Language/dialect resolution errors for the human-language catalog.
pub type HumanLookupError = LookupError<HumanLanguageName, DialectName>;

/// Errors that can occur while looking up localized greetings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum HumanLanguageError {
    /// The language or dialect could not be resolved.
    #[error(transparent)]
    Lookup(#[from] HumanLookupError),

    /// The dialect has no greeting in the requested script.
    #[error("no 'hello world' found for {language} ({dialect}) written in {script}")]
    UnknownScript {
        /// The resolved language.
        language: HumanLanguageName,
        /// The resolved dialect.
        dialect: DialectName,
        /// The requested script.
        script: ScriptName,
    },

    /// The dialect records no greetings at all.
    #[error("no 'hello world' recorded for {language} ({dialect})")]
    MissingGreeting {
        /// The resolved language.
        language: HumanLanguageName,
        /// The resolved dialect.
        dialect: DialectName,
    },
}

impl Greeting {
    /// Greeting for the default human language/dialect/script.
    ///
    /// # Panics
    ///
    /// Panics if the default language/dialect pair has no recorded greeting.
    #[must_use]
    pub fn default_text() -> &'static str {
        Greeting::text_for(
            Some(DEFAULT_HUMAN_LANGUAGE),
            Some(DEFAULT_HUMAN_LANGUAGE_DIALECT),
            Some(DEFAULT_SCRIPT),
        )
        .expect("default human language/dialect/script must resolve to a greeting")
    }

    /// The greeting for a language, dialect and script; each `None` falls back to a default.
    ///
    /// - `language`: `None` → inferred from `dialect`, else [`DEFAULT_HUMAN_LANGUAGE`].
    /// - `dialect`: `None` → the language's first dialect, or [`DEFAULT_HUMAN_LANGUAGE_DIALECT`]
    ///   when `language` is `None` too.
    /// - `script`: `None` → the dialect's first greeting.
    ///
    /// ```
    /// use hw_pp_core::data::human_languages::{Greeting, HumanLanguageName, ScriptName};
    /// let romaji =
    ///     Greeting::text_for(Some(HumanLanguageName::Japanese), None, Some(ScriptName::Romaji));
    /// assert_eq!(romaji, Ok("konnichiwa sekai!"));
    /// ```
    ///
    /// # Errors
    ///
    /// - [`LookupError::NoOwner`] if `dialect` belongs to no recorded language.
    /// - [`LookupError::UnknownLanguage`] if the language is not recorded.
    /// - [`LookupError::NoVariants`] if the language has no dialects to default to.
    /// - [`LookupError::UnknownVariant`] if the language does not record `dialect`.
    /// - [`HumanLanguageError::UnknownScript`] if the dialect has no greeting in `script`.
    /// - [`HumanLanguageError::MissingGreeting`] if the dialect has no greetings.
    ///
    /// The [`LookupError`]s are wrapped in [`HumanLanguageError::Lookup`].
    pub fn text_for(
        language: Option<HumanLanguageName>,
        dialect: Option<DialectName>,
        script: Option<ScriptName>,
    ) -> Result<&'static str, HumanLanguageError> {
        let (resolved_language, resolved_dialect) = match (language, dialect) {
            (None, None) => (DEFAULT_HUMAN_LANGUAGE, DEFAULT_HUMAN_LANGUAGE_DIALECT),
            (None, Some(existing_dialect)) => (existing_dialect.language()?, existing_dialect),
            (Some(existing_lang), None) => (existing_lang, existing_lang.default_dialect()?),
            (Some(existing_lang), Some(existing_dialect)) => (existing_lang, existing_dialect),
        };

        let lang = HUMAN_LANGUAGES
            .iter()
            .find(|lang| lang.name == resolved_language)
            .ok_or(LookupError::UnknownLanguage(resolved_language))?;
        let lang_dialect = lang
            .dialects
            .iter()
            .find(|d| d.label == resolved_dialect)
            .ok_or(LookupError::UnknownVariant {
                language: resolved_language,
                variant: resolved_dialect,
            })?;

        let greeting = match script {
            None => lang_dialect
                .greetings
                .first()
                .ok_or(HumanLanguageError::MissingGreeting {
                    language: resolved_language,
                    dialect: resolved_dialect,
                })?,
            Some(wanted) => lang_dialect
                .greetings
                .iter()
                .find(|greeting| greeting.scripts.contains(&wanted))
                .ok_or(HumanLanguageError::UnknownScript {
                    language: resolved_language,
                    dialect: resolved_dialect,
                    script: wanted,
                })?,
        };
        Ok(greeting.text)
    }
}

impl DialectName {
    /// Finds the language that records this dialect.
    ///
    /// # Errors
    ///
    /// Returns [`LookupError::NoOwner`] if no recorded language has
    /// the dialect.
    pub fn language(self) -> Result<HumanLanguageName, HumanLookupError> {
        HUMAN_LANGUAGES
            .iter()
            .find(|lang| lang.dialects.iter().any(|d| d.label == self))
            .map(|lang| lang.name)
            .ok_or(LookupError::NoOwner(self))
    }

    /// The dialects of `language` (or of every language if `None`), in catalog order.
    ///
    /// ```
    /// use hw_pp_core::data::human_languages::{DialectName, HumanLanguageName};
    /// let dialects: Vec<_> = DialectName::all_by(Some(HumanLanguageName::Japanese)).collect();
    /// assert_eq!(dialects, [DialectName::StandardJapanese, DialectName::KansaiJapanese]);
    /// ```
    pub fn all_by(language: Option<HumanLanguageName>) -> impl Iterator<Item = Self> {
        unique(
            HUMAN_LANGUAGES
                .iter()
                .filter(move |lang| language.is_none_or(|l| lang.name == l))
                .flat_map(|lang| lang.dialects.iter().map(|d| d.label)),
        )
    }
}

impl HumanLanguage {
    #[must_use]
    pub fn language_for(
        language_name: Option<&HumanLanguageName>,
        dialect_name: Option<&DialectName>,
    ) -> Option<&'static HumanLanguage> {
        if language_name.is_none() && dialect_name.is_none() {
            return None;
        }

        HUMAN_LANGUAGES.iter().find(|l| {
            language_name.is_none_or(|name| l.name == *name)
                && dialect_name.is_none_or(|d| l.dialects.iter().any(|dia| dia.label == *d))
        })
    }
}

impl HumanLanguageName {
    /// The first recorded dialect of this language.
    ///
    /// # Errors
    ///
    /// Returns [`LookupError::UnknownLanguage`] if the language is not recorded,
    /// or [`LookupError::NoVariants`] if it has no dialects.
    pub fn default_dialect(self) -> Result<DialectName, HumanLookupError> {
        let lang = HUMAN_LANGUAGES
            .iter()
            .find(|lang| lang.name == self)
            .ok_or(LookupError::UnknownLanguage(self))?;
        lang.dialects
            .first()
            .map(|dialect| dialect.label)
            .ok_or(LookupError::NoVariants(self))
    }

    /// The human languages matching every given filter, in catalog order; `None` matches all.
    ///
    /// ```
    /// use hw_pp_core::data::human_languages::{
    ///     HumanLanguageCategory, HumanLanguageFamily, HumanLanguageName, ScriptName,
    /// };
    /// assert!(
    ///     HumanLanguageName::all_by(
    ///         Some(ScriptName::Latin),
    ///         Some(HumanLanguageCategory::Dead),
    ///         None,
    ///     )
    ///     .any(|name| name == HumanLanguageName::Latin)
    /// );
    /// let romance: Vec<_> =
    ///     HumanLanguageName::all_by(None, None, Some(HumanLanguageFamily::Romance)).collect();
    /// assert!(
    ///     romance.contains(&HumanLanguageName::Spanish)
    ///         && !romance.contains(&HumanLanguageName::Latin)
    /// );
    /// ```
    pub fn all_by(
        script: Option<ScriptName>,
        category: Option<HumanLanguageCategory>,
        family: Option<HumanLanguageFamily>,
    ) -> impl Iterator<Item = Self> {
        unique(
            HUMAN_LANGUAGES
                .iter()
                .filter(move |lang| {
                    category.is_none_or(|c| lang.categories.contains(&c))
                        && script.is_none_or(|s| lang.scripts.contains(&s))
                        && family.is_none_or(|f| lang.families.contains(&f))
                })
                .map(|lang| lang.name),
        )
    }
}

impl ScriptName {
    /// The scripts `language` uses (or every language's if `None`), each once, in the order first
    /// recorded.
    ///
    /// ```
    /// use hw_pp_core::data::human_languages::{HumanLanguageName, ScriptName};
    /// let scripts: Vec<_> = ScriptName::all_by(Some(HumanLanguageName::Klingon)).collect();
    /// assert_eq!(scripts, [ScriptName::Latin, ScriptName::Piqad]);
    /// ```
    pub fn all_by(language: Option<HumanLanguageName>) -> impl Iterator<Item = Self> {
        unique(
            HUMAN_LANGUAGES
                .iter()
                .filter(move |lang| language.is_none_or(|l| lang.name == l))
                .flat_map(|lang| lang.scripts.iter().copied()),
        )
    }

    #[must_use]
    pub fn contains_script(
        language_name: Option<HumanLanguageName>,
        dialect: Option<DialectName>,
        target_script: ScriptName,
    ) -> bool {
        if let Some(language) =
            HumanLanguage::language_for(language_name.as_ref(), dialect.as_ref())
        {
            language.scripts.contains(&target_script)
        } else {
            false
        }
    }
}

impl HumanLanguageCategory {
    /// The categories of `language` (or of every language if `None`), each once, in the order
    /// first recorded.
    ///
    /// ```
    /// use hw_pp_core::data::human_languages::{HumanLanguageCategory, HumanLanguageName};
    /// let categories: Vec<_> =
    ///     HumanLanguageCategory::all_by(Some(HumanLanguageName::Sumerian)).collect();
    /// assert_eq!(categories, [HumanLanguageCategory::Extinct, HumanLanguageCategory::Isolate]);
    /// ```
    pub fn all_by(language: Option<HumanLanguageName>) -> impl Iterator<Item = Self> {
        unique(
            HUMAN_LANGUAGES
                .iter()
                .filter(move |lang| language.is_none_or(|l| lang.name == l))
                .flat_map(|lang| lang.categories.iter().copied()),
        )
    }
}

impl HumanLanguageFamily {
    /// The families of `language` (or of every language if `None`), each once, in the order first
    /// recorded.
    ///
    /// ```
    /// use hw_pp_core::data::human_languages::{HumanLanguageFamily, HumanLanguageName};
    /// let families: Vec<_> = HumanLanguageFamily::all_by(Some(HumanLanguageName::English)).collect();
    /// assert_eq!(families, [HumanLanguageFamily::IndoEuropean, HumanLanguageFamily::Germanic]);
    /// ```
    pub fn all_by(language: Option<HumanLanguageName>) -> impl Iterator<Item = Self> {
        unique(
            HUMAN_LANGUAGES
                .iter()
                .filter(move |lang| language.is_none_or(|l| lang.name == l))
                .flat_map(|lang| lang.families.iter().copied()),
        )
    }
}
