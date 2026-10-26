//! Hello-world snippet for `ProgrammingLanguageName::Cpl`.
//!
//! Imported from the Hello World Collection (`files/cpl.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/cpl.htm>
//! - <http://helloworldcollection.de/#cpl>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Cpl => "cpl",
    versions: [Cpl => "cpl"],
    extensions: [],
}

const CPL: &str = concat!(
    r"title '",
    output_for_hello_world!(),
    r" in cpl on a Warrex Centurion'
system zhelloworld      (main,exp=d)
;
file crt:sysipt,class=0,seq
;
define m00:'",
    output_for_hello_world!(),
    r"'
format f00:c132
;
entrypoint crt
;
entry
;
open io crt
;
write (crt, f00) m00
stop 0
;
end"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cpl,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cpl,
            syntax: CPL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CPL,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"title '",
                            output_for_hello_world!(),
                            r" in cpl on a Warrex Centurion'
system zhelloworld      (main,exp=d)
;
file crt:sysipt,class=0,seq
;
define m00:'",
                            output_for_hello_world!(),
                            r"'
format f00:c132
;
entrypoint crt
;
entry
;
open io crt
;
write (crt, f00) m00
stop 0
;
end"
                        ),
                    )],
                )
            },
        }],
    };
