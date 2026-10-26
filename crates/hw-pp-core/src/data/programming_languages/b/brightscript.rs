//! Hello-world snippet for `ProgrammingLanguageName::Brightscript`.
//!
//! Imported from the Hello World Collection (`files/BrightScript.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/BrightScript.htm>
//! - <http://helloworldcollection.de/#BrightScript>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Brightscript => "BrightScript",
    versions: [Brightscript => "BrightScript"],
    extensions: [],
}

const BRIGHTSCRIPT: &str = concat!(
    r"' ",
    output_for_hello_world!(),
    r#" in BrightScript

sub Main()
port = CreateObject("roMessagePort")
screen = CreateObject("roTextScreen")
screen.SetText(""#,
    output_for_hello_world!(),
    r#"")
screen.setMessagePort(port)
screen.Show()
while true
  msg = wait(0, port)
  if(msg.isScreenClosed())
    return -1
  end if
end while
end sub"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Brightscript,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Brightscript,
            syntax: BRIGHTSCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BRIGHTSCRIPT,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"' ",
                            output_for_hello_world!(),
                            r#" in BrightScript

sub Main()
port = CreateObject("roMessagePort")
screen = CreateObject("roTextScreen")
screen.SetText(""#,
                            output_for_hello_world!(),
                            r#"")
screen.setMessagePort(port)
screen.Show()
while true
  msg = wait(0, port)
  if(msg.isScreenClosed())
    return -1
  end if
end while
end sub"#
                        ),
                    )],
                )
            },
        }],
    };
