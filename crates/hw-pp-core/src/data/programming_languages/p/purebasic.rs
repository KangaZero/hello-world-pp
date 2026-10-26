//! Hello-world snippets for `ProgrammingLanguageName::Purebasic`.
//!
//! Imported from leachim6/hello-world (`p/PureBasic.pb` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Imported from the Hello World Collection (`files/PureBasic&nbsp;(Console).txt`, `files/PureBasic&nbsp;(Messagebox).txt`, `files/PureBasic&nbsp;(Window).txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/PureBasic.pb>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/PureBasic%26nbsp%3B%28Console%29.txt>
//! - <http://helloworldcollection.de/#PureBasic%C2%A0(Console)>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/PureBasic%26nbsp%3B%28Messagebox%29.txt>
//! - <http://helloworldcollection.de/#PureBasic%C2%A0(Messagebox)>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/PureBasic%26nbsp%3B%28Window%29.txt>
//! - <http://helloworldcollection.de/#PureBasic%C2%A0(Window)>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Basic, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Purebasic => "PureBasic",
    versions: [Purebasic => "PureBasic", PurebasicConsole => "PureBasic (Console)", PurebasicMessagebox => "PureBasic (Messagebox)", PurebasicWindow => "PureBasic (Window)"],
    extensions: [Pb => "pb"],
}

const PUREBASIC: &str = concat!(
    r#"If OpenConsole()
  PrintN(""#,
    output_for_hello_world!(),
    r#"")
EndIf"#
);
const PUREBASIC_CONSOLE: &str = concat!(
    r"; ",
    output_for_hello_world!(),
    r#" in PureBasic (console program)

OpenConsole()
   ConsoleTitle (""#,
    output_for_hello_world!(),
    r#"")
   PrintN (""#,
    output_for_hello_world!(),
    r#"")
CloseConsole()"#
);
const PUREBASIC_MESSAGEBOX: &str = concat!(
    r"; ",
    output_for_hello_world!(),
    r#" in PureBasic (message box)

MessageRequester(""#,
    output_for_hello_world!(),
    r#" Messagebox",""#,
    output_for_hello_world!(),
    r#"")"#
);
const PUREBASIC_WINDOW: &str = concat!(
    r"; ",
    output_for_hello_world!(),
    r#" in PureBasic (Window)

If OpenWindow(0, 216, 0, 268, 133,  #PB_Window_SystemMenu | #PB_Window_TitleBar | #PB_Window_ScreenCentered , ""#,
    output_for_hello_world!(),
    r#" Window")
 If CreateGadgetList(WindowID())
   TextGadget(1, 100, 60, 60, 20, ""#,
    output_for_hello_world!(),
    r#"")
 EndIf
EndIf

Repeat    ; Message Loop
Until WaitWindowEvent() = #PB_EventCloseWindow"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Purebasic,
        extensions: &[FileExtension::Pb],
        categories: &[Compiled],
        families: &[Basic],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Purebasic,
                syntax: PUREBASIC,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        PUREBASIC,
                        [
                            (TokenKind::Keyword, r"If"),
                            (TokenKind::Plain, r" "),
                            (TokenKind::Keyword, r"OpenConsole"),
                            (TokenKind::Punctuation, r"()"),
                            (
                                TokenKind::Plain,
                                r"
  ",
                            ),
                            (TokenKind::Keyword, r"PrintN"),
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
                            (TokenKind::Keyword, r"EndIf"),
                        ],
                    )
                },
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::PurebasicConsole,
                syntax: PUREBASIC_CONSOLE,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        PUREBASIC_CONSOLE,
                        [
                            (
                                TokenKind::Comment,
                                concat!(
                                    r"; ",
                                    output_for_hello_world!(),
                                    r" in PureBasic (console program)"
                                ),
                            ),
                            (
                                TokenKind::Plain,
                                r"

",
                            ),
                            (TokenKind::Keyword, r"OpenConsole"),
                            (TokenKind::Punctuation, r"()"),
                            (
                                TokenKind::Plain,
                                r"
   ",
                            ),
                            (TokenKind::Function, r"ConsoleTitle"),
                            (TokenKind::Plain, r" "),
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
                            (TokenKind::Keyword, r"PrintN"),
                            (TokenKind::Plain, r" "),
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
                            (TokenKind::Keyword, r"CloseConsole"),
                            (TokenKind::Punctuation, r"()"),
                        ],
                    )
                },
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::PurebasicMessagebox,
                syntax: PUREBASIC_MESSAGEBOX,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        PUREBASIC_MESSAGEBOX,
                        [
                            (
                                TokenKind::Comment,
                                concat!(
                                    r"; ",
                                    output_for_hello_world!(),
                                    r" in PureBasic (message box)"
                                ),
                            ),
                            (
                                TokenKind::Plain,
                                r"

",
                            ),
                            (TokenKind::Keyword, r"MessageRequester"),
                            (TokenKind::Punctuation, r"("),
                            (
                                TokenKind::String,
                                concat!(r#"""#, output_for_hello_world!(), r#" Messagebox""#),
                            ),
                            (TokenKind::Punctuation, r","),
                            (
                                TokenKind::String,
                                concat!(r#"""#, output_for_hello_world!(), r#"""#),
                            ),
                            (TokenKind::Punctuation, r")"),
                        ],
                    )
                },
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::PurebasicWindow,
                syntax: PUREBASIC_WINDOW,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        PUREBASIC_WINDOW,
                        [
                            (
                                TokenKind::Comment,
                                concat!(
                                    r"; ",
                                    output_for_hello_world!(),
                                    r" in PureBasic (Window)"
                                ),
                            ),
                            (
                                TokenKind::Plain,
                                r"

",
                            ),
                            (TokenKind::Keyword, r"If"),
                            (TokenKind::Plain, r" "),
                            (TokenKind::Function, r"OpenWindow"),
                            (TokenKind::Punctuation, r"("),
                            (TokenKind::Number, r"0"),
                            (TokenKind::Punctuation, r","),
                            (TokenKind::Plain, r" "),
                            (TokenKind::Number, r"216"),
                            (TokenKind::Punctuation, r","),
                            (TokenKind::Plain, r" "),
                            (TokenKind::Number, r"0"),
                            (TokenKind::Punctuation, r","),
                            (TokenKind::Plain, r" "),
                            (TokenKind::Number, r"268"),
                            (TokenKind::Punctuation, r","),
                            (TokenKind::Plain, r" "),
                            (TokenKind::Number, r"133"),
                            (TokenKind::Punctuation, r","),
                            (
                                TokenKind::Plain,
                                r"  #PB_Window_SystemMenu | #PB_Window_TitleBar | #PB_Window_ScreenCentered ",
                            ),
                            (TokenKind::Punctuation, r","),
                            (TokenKind::Plain, r" "),
                            (
                                TokenKind::String,
                                concat!(r#"""#, output_for_hello_world!(), r#" Window""#),
                            ),
                            (TokenKind::Punctuation, r")"),
                            (
                                TokenKind::Plain,
                                r"
 ",
                            ),
                            (TokenKind::Keyword, r"If"),
                            (TokenKind::Plain, r" "),
                            (TokenKind::Function, r"CreateGadgetList"),
                            (TokenKind::Punctuation, r"("),
                            (TokenKind::Function, r"WindowID"),
                            (TokenKind::Punctuation, r"())"),
                            (
                                TokenKind::Plain,
                                r"
   ",
                            ),
                            (TokenKind::Function, r"TextGadget"),
                            (TokenKind::Punctuation, r"("),
                            (TokenKind::Number, r"1"),
                            (TokenKind::Punctuation, r","),
                            (TokenKind::Plain, r" "),
                            (TokenKind::Number, r"100"),
                            (TokenKind::Punctuation, r","),
                            (TokenKind::Plain, r" "),
                            (TokenKind::Number, r"60"),
                            (TokenKind::Punctuation, r","),
                            (TokenKind::Plain, r" "),
                            (TokenKind::Number, r"60"),
                            (TokenKind::Punctuation, r","),
                            (TokenKind::Plain, r" "),
                            (TokenKind::Number, r"20"),
                            (TokenKind::Punctuation, r","),
                            (TokenKind::Plain, r" "),
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
                            (TokenKind::Keyword, r"EndIf"),
                            (
                                TokenKind::Plain,
                                r"
",
                            ),
                            (TokenKind::Keyword, r"EndIf"),
                            (
                                TokenKind::Plain,
                                r"

Repeat    ",
                            ),
                            (TokenKind::Comment, r"; Message Loop"),
                            (
                                TokenKind::Plain,
                                r"
Until ",
                            ),
                            (TokenKind::Function, r"WaitWindowEvent"),
                            (TokenKind::Punctuation, r"()"),
                            (TokenKind::Plain, r" = #PB_EventCloseWindow"),
                        ],
                    )
                },
            },
        ],
    };
