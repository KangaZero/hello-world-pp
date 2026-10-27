//! Hello-world snippet for `ProgrammingLanguageName::VisualBasicForApplications`.
//!
//! Imported from leachim6/hello-world (`v/Visual Basic for Applications.vba` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/v/Visual%20Basic%20for%20Applications.vba>
//! - <https://en.wikipedia.org/wiki/Visual_Basic_for_Applications> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::EventDriven;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Basic, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: VisualBasicForApplications => "Visual Basic for Applications",
    versions: [VisualBasicForApplications => "Visual Basic for Applications"],
    extensions: [Vba => "vba"],
}

const VISUAL_BASIC_FOR_APPLICATIONS: &str = concat!(
    r#"Sub HelloWorld()
    Call MsgBox(""#,
    output_for_hello_world!(),
    r#"")
End Sub"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::VisualBasicForApplications,
        extensions: &[FileExtension::Vba],
        categories: &[Scripting],
        families: &[Basic],
        paradigms: &[EventDriven],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::VisualBasicForApplications,
            syntax: VISUAL_BASIC_FOR_APPLICATIONS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    VISUAL_BASIC_FOR_APPLICATIONS,
                    [
                        (TokenKind::Keyword, r"Sub"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"HelloWorld"),
                        (TokenKind::Punctuation, r"()"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Keyword, r"Call"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"MsgBox"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"End"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"Sub"),
                    ],
                )
            },
        }],
    };
