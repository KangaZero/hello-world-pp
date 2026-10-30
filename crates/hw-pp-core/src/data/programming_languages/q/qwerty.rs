//! Hello-world snippet for `ProgrammingLanguageName::Qwerty`.
//!
//! Imported from leachim6/hello-world (`q/QWERTY.qwerty` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/q/QWERTY.qwerty>
//! - <https://esolangs.org/wiki/Qwerty> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    CellBased, Imperative, Reflective, StackBased,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Qwerty => "QWERTY",
    versions: [Qwerty => "QWERTY"],
    extensions: [Qwerty => "qwerty"],
}

const QWERTY: &str = concat!(
    r#"begin
	?hello = ""#,
    output_for_hello_world!(),
    r#""
	if 1 == 1 then
		output ?hello
	fi
end"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Qwerty,
        extensions: &[FileExtension::Qwerty],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Imperative, Reflective, StackBased, CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Qwerty,
            syntax: QWERTY,
            tokenized_syntax: &const {
                checked_token_lengths(
                    QWERTY,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"begin
	?hello = ""#,
                            output_for_hello_world!(),
                            r#""
	if 1 == 1 then
		output ?hello
	fi
end"#
                        ),
                    )],
                )
            },
        }],
    };
