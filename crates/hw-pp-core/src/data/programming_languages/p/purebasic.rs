//! Hello-world snippet for `ProgrammingLanguageName::Purebasic`.
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
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Purebasic => "PureBasic",
    versions: [Purebasic => "PureBasic", PurebasicConsole => "PureBasic (Console)", PurebasicMessagebox => "PureBasic (Messagebox)", PurebasicWindow => "PureBasic (Window)"],
    extensions: [Pb => "pb"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Purebasic,
        extensions: &[FileExtension::Pb],
        categories: &[Compiled],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Purebasic,
                syntax: concat!(
                    r#"If OpenConsole()
  PrintN(""#,
                    output_for_hello_world!(),
                    r#"")
EndIf"#
                ),
                tokenized_syntax: &[(
                    TokenKind::Plain,
                    concat!(
                        r#"If OpenConsole()
  PrintN(""#,
                        output_for_hello_world!(),
                        r#"")
EndIf"#
                    ),
                )],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::PurebasicConsole,
                syntax: concat!(
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
                ),
                tokenized_syntax: &[(
                    TokenKind::Plain,
                    concat!(
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
                    ),
                )],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::PurebasicMessagebox,
                syntax: concat!(
                    r"; ",
                    output_for_hello_world!(),
                    r#" in PureBasic (message box)

MessageRequester(""#,
                    output_for_hello_world!(),
                    r#" Messagebox",""#,
                    output_for_hello_world!(),
                    r#"")"#
                ),
                tokenized_syntax: &[(
                    TokenKind::Plain,
                    concat!(
                        r"; ",
                        output_for_hello_world!(),
                        r#" in PureBasic (message box)

MessageRequester(""#,
                        output_for_hello_world!(),
                        r#" Messagebox",""#,
                        output_for_hello_world!(),
                        r#"")"#
                    ),
                )],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::PurebasicWindow,
                syntax: concat!(
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
                ),
                tokenized_syntax: &[(
                    TokenKind::Plain,
                    concat!(
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
                    ),
                )],
            },
        ],
    };
