//! Hello-world snippet for `ProgrammingLanguageName::XslFo`.
//!
//! Imported from the Hello World Collection (`files/XSL-FO.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/XSL-FO.txt>
//! - <http://helloworldcollection.de/#XSL-FO>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: XslFo => "XSL-FO",
    versions: [XslFo => "XSL-FO"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::XslFo,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::XslFo,
            syntax: concat!(
                r#"<?xml version="1.0" encoding="utf-8"?>
<!-- "#,
                output_for_hello_world!(),
                r#" in XSL-FO -->
<fo:root xmlns:fo="http://www.w3.org/1999/XSL/Format">
    <fo:layout-master-set>
        <fo:simple-page-master master-name="LetterPage" page-width="8.5in" page-height="11in">?
            <fo:region-body region-name="PageBody" margin="0.7in"/>
        </fo:simple-page-master>
    </fo:layout-master-set>
    <fo:page-sequence master-reference="LetterPage">
        <fo:flow flow-name="PageBody">
            <fo:block font-size="12pt" font-family="courier">"#,
                output_for_hello_world!(),
                r"</fo:block>
        </fo:flow>
    </fo:page-sequence>
</fo:root>"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"<?xml version="1.0" encoding="utf-8"?>
<!-- "#,
                    output_for_hello_world!(),
                    r#" in XSL-FO -->
<fo:root xmlns:fo="http://www.w3.org/1999/XSL/Format">
    <fo:layout-master-set>
        <fo:simple-page-master master-name="LetterPage" page-width="8.5in" page-height="11in">?
            <fo:region-body region-name="PageBody" margin="0.7in"/>
        </fo:simple-page-master>
    </fo:layout-master-set>
    <fo:page-sequence master-reference="LetterPage">
        <fo:flow flow-name="PageBody">
            <fo:block font-size="12pt" font-family="courier">"#,
                    output_for_hello_world!(),
                    r"</fo:block>
        </fo:flow>
    </fo:page-sequence>
</fo:root>"
                ),
            )],
        }],
    };
