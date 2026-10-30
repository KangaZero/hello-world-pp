//! Hello-world snippets for `ProgrammingLanguageName::Refal`.
//!
//! Imported from leachim6/hello-world (`r/Refal.ref` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Imported from the Hello World Collection (`files/REFAL-2.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/Refal.ref>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/REFAL-2.txt>
//! - <http://helloworldcollection.de/#REFAL-2>
//! - <https://www.wikidata.org/wiki/Q2626418> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Functional;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Refal => "Refal",
    versions: [Refal => "Refal", Refal2 => "REFAL-2"],
    extensions: [Ref => "ref"],
}

const REFAL: &str = concat!(
    r"$ENTRY Go { = <Hello>;}
Hello {
   = <Prout '",
    output_for_hello_world!(),
    r"'>;
}"
);
const REFAL2: &str = concat!(
    r"* ",
    output_for_hello_world!(),
    r" in REFAL-2
start
entry go
extrn PROUT
go = <prout '",
    output_for_hello_world!(),
    r"'>
end"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Refal,
        extensions: &[FileExtension::Ref],
        categories: &[],
        families: &[],
        paradigms: &[Functional],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Refal,
                syntax: REFAL,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        REFAL,
                        [(
                            TokenKind::Plain,
                            concat!(
                                r"$ENTRY Go { = <Hello>;}
Hello {
   = <Prout '",
                                output_for_hello_world!(),
                                r"'>;
}"
                            ),
                        )],
                    )
                },
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Refal2,
                syntax: REFAL2,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        REFAL2,
                        [(
                            TokenKind::Plain,
                            concat!(
                                r"* ",
                                output_for_hello_world!(),
                                r" in REFAL-2
start
entry go
extrn PROUT
go = <prout '",
                                output_for_hello_world!(),
                                r"'>
end"
                            ),
                        )],
                    )
                },
            },
        ],
    };
