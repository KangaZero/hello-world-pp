//! Hello-world snippet for `ProgrammingLanguageName::Brzrk`.
//!
//! Imported from leachim6/hello-world (`b/BRZRK.brzrk` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/BRZRK.brzrk>
//! - <https://esolangs.org/wiki/Ans> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Functional;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Brzrk => "BRZRK",
    versions: [Brzrk => "BRZRK"],
    extensions: [Brzrk => "brzrk"],
}

const BRZRK: &str = concat!(r#"$""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Brzrk,
        extensions: &[FileExtension::Brzrk],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Brzrk,
            syntax: BRZRK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BRZRK,
                    [(
                        TokenKind::Plain,
                        concat!(r#"$""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };
