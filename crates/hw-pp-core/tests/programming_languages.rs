use hw_pp_core::data::programming_languages::PROGRAMMING_LANGUAGES;

#[test]
fn every_language_has_a_version_and_extension() {
    for language in PROGRAMMING_LANGUAGES {
        assert!(
            !language.versions.is_empty(),
            "{} has no recorded versions",
            language.name
        );
        assert!(
            !language.extensions.is_empty(),
            "{} has no recorded extensions",
            language.name
        );
    }
}
