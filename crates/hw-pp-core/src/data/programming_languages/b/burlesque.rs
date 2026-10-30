//! Hello-world snippet for `ProgrammingLanguageName::Burlesque`.
//!
//! Imported from leachim6/hello-world (`b/Burlesque.burlesque` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/Burlesque.burlesque>
//! - <https://esolangs.org/wiki/Burlesque> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::StackBased;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Burlesque => "Burlesque",
    versions: [Burlesque => "Burlesque"],
    extensions: [Burlesque => "burlesque"],
}

const BURLESQUE: &str = concat!(r#"""#, output_for_hello_world!(), r#""Q"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Burlesque,
        extensions: &[FileExtension::Burlesque],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Burlesque,
            syntax: BURLESQUE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BURLESQUE,
                    [(
                        TokenKind::Plain,
                        concat!(r#"""#, output_for_hello_world!(), r#""Q"#),
                    )],
                )
            },
        }],
    };
