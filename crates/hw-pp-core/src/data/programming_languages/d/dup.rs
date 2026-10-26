//! Hello-world snippet for `ProgrammingLanguageName::Dup`.
//!
//! Imported from leachim6/hello-world (`d/DUP.dup` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/d/DUP.dup>
//! - <https://esolangs.org/wiki/DUP> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::StackBased;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Dup => "DUP",
    versions: [Dup => "DUP"],
    extensions: [Dup => "dup"],
}

const DUP: &str = concat!(
    r#"[\[^^>][$;,1+]#%%]p:
0$""#,
    output_for_hello_world!(),
    r#""p;!10,"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dup,
        extensions: &[FileExtension::Dup],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dup,
            syntax: DUP,
            tokenized_syntax: &const {
                checked_token_lengths(
                    DUP,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"[\[^^>][$;,1+]#%%]p:
0$""#,
                            output_for_hello_world!(),
                            r#""p;!10,"#
                        ),
                    )],
                )
            },
        }],
    };
