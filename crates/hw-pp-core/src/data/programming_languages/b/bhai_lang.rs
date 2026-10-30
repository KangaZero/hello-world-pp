//! Hello-world snippet for `ProgrammingLanguageName::BhaiLang`.
//!
//! Imported from leachim6/hello-world (`b/Bhai Lang.bhai` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/Bhai%20Lang.bhai>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: BhaiLang => "Bhai Lang",
    versions: [BhaiLang => "Bhai Lang"],
    extensions: [Bhai => "bhai"],
}

const BHAI_LANG: &str = concat!(
    r#"hi bhai
  bol bhai ""#,
    output_for_hello_world!(),
    r#"";
bye bhai"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::BhaiLang,
        extensions: &[FileExtension::Bhai],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::BhaiLang,
            syntax: BHAI_LANG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BHAI_LANG,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"hi bhai
  bol bhai ""#,
                            output_for_hello_world!(),
                            r#"";
bye bhai"#
                        ),
                    )],
                )
            },
        }],
    };
