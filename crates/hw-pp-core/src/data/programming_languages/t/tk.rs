//! Hello-world snippet for `ProgrammingLanguageName::Tk`.
//!
//! Imported from leachim6/hello-world (`t/Tk.tk` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/Tk.tk>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Framework,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Tcl, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Tk => "Tk",
    versions: [Tk => "Tk"],
    extensions: [Tk => "tk"],
}

const TK: &str = concat!(
    r#"label .l -text ""#,
    output_for_hello_world!(),
    r#""
pack .l"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Tk,
        extensions: &[FileExtension::Tk],
        categories: &[Framework],
        families: &[Tcl],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Tk,
            syntax: TK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    TK,
                    [
                        (TokenKind::Keyword, r"label"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Plain, r"l -text "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"pack"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Plain, r"l"),
                    ],
                )
            },
        }],
    };
