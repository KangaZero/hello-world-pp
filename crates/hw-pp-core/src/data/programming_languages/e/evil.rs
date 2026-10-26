//! Hello-world snippet for `ProgrammingLanguageName::Evil`.
//!
//! Imported from leachim6/hello-world (`e/Evil` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/e/Evil>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Evil => "Evil",
    versions: [Evil => "Evil"],
    extensions: [],
}

const EVIL: &str = r"zaeeeaeeew   (H)
zaeeaeeaeaw  (e)
zuueeueueaw  (l)
zuueeueueaw  (l)
zuueeueew    (o)
zaeeeeew     ( )
zaeeeeeue    (W)
zuueeueew    (o)
zuueeueeaaaw (r)
zuueeueueaw  (l)
zaeeaeeaew   (d)
zaeeuuuw     (\r)
zaeaaaaaaw   (\n)";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Evil,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Evil,
            syntax: EVIL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    EVIL,
                    [(
                        TokenKind::Plain,
                        r"zaeeeaeeew   (H)
zaeeaeeaeaw  (e)
zuueeueueaw  (l)
zuueeueueaw  (l)
zuueeueew    (o)
zaeeeeew     ( )
zaeeeeeue    (W)
zuueeueew    (o)
zuueeueeaaaw (r)
zuueeueueaw  (l)
zaeeaeeaew   (d)
zaeeuuuw     (\r)
zaeaaaaaaw   (\n)",
                    )],
                )
            },
        }],
    };
