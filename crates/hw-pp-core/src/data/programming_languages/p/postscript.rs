//! Hello-world snippet for `ProgrammingLanguageName::Postscript`.
//!
//! Imported from leachim6/hello-world (`p/PostScript.ps` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Postscript => "PostScript",
    versions: [Postscript => "PostScript"],
    extensions: [Ps => "ps"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Postscript,
        extensions: &[FileExtension::Ps],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Postscript,
            syntax: concat!(
                r"% run> gs -q -sDEVICE=nullpage postscript.ps
(",
                output_for_hello_world!(),
                r"\n) print quit"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"% run> gs -q -sDEVICE=nullpage postscript.ps
(",
                    output_for_hello_world!(),
                    r"\n) print quit"
                ),
            )],
        }],
    };
