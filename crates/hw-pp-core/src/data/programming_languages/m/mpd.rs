//! Hello-world snippet for `ProgrammingLanguageName::Mpd`.
//!
//! Imported from the Hello World Collection (`files/MPD.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/MPD.txt>
//! - <http://helloworldcollection.de/#MPD>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageFamily::Sr,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Mpd => "MPD",
    versions: [Mpd => "MPD"],
    extensions: [],
}

const MPD: &str = concat!(
    r"# ",
    output_for_hello_world!(),
    r#" in MPD.

resource helloworld()
  write(""#,
    output_for_hello_world!(),
    r#"")
end"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mpd,
        extensions: &[],
        categories: &[],
        families: &[Sr],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mpd,
            syntax: MPD,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MPD,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"# ",
                            output_for_hello_world!(),
                            r#" in MPD.

resource helloworld()
  write(""#,
                            output_for_hello_world!(),
                            r#"")
end"#
                        ),
                    )],
                )
            },
        }],
    };
