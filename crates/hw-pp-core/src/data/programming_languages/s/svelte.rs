//! Hello-world snippet for `ProgrammingLanguageName::Svelte`.
//!
//! Imported from leachim6/hello-world (`s/Svelte.svelte` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Svelte => "Svelte",
    versions: [Svelte => "Svelte"],
    extensions: [Svelte => "svelte"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Svelte,
        extensions: &[FileExtension::Svelte],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Svelte,
            syntax: concat!(
                r"<script>
  let message = '",
                output_for_hello_world!(),
                r"';
</script>

<p>{ message }</p>"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"<script>
  let message = '",
                    output_for_hello_world!(),
                    r"';
</script>

<p>{ message }</p>"
                ),
            )],
        }],
    };
