//! Hello-world snippet for `ProgrammingLanguageName::Clu`.
//!
//! Imported from leachim6/hello-world (`c/CLU.clu` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/CLU.clu>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Historical},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Clu => "CLU",
    versions: [Clu => "CLU"],
    extensions: [Clu => "clu"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Clu,
        extensions: &[FileExtension::Clu],
        categories: &[Compiled, Historical],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Clu,
            syntax: concat!(
                r#"start_up = proc ()
    po: stream := stream$primary_output ()
    stream$putl (po, ""#,
                output_for_hello_world!(),
                r#"")
    end start_up"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"start_up = proc ()
    po: stream := stream$primary_output ()
    stream$putl (po, ""#,
                    output_for_hello_world!(),
                    r#"")
    end start_up"#
                ),
            )],
        }],
    };
