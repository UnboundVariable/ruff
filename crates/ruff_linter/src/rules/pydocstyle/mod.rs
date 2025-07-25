//! Rules from [pydocstyle](https://pypi.org/project/pydocstyle/).
pub(crate) mod helpers;
pub(crate) mod rules;
pub mod settings;

#[cfg(test)]
mod tests {
    use std::path::Path;

    use anyhow::Result;
    use test_case::test_case;

    use crate::registry::Rule;

    use super::settings::{Convention, Settings};
    use crate::test::test_path;
    use crate::{assert_diagnostics, settings};

    #[test_case(Rule::MissingBlankLineAfterLastSection, Path::new("sections.py"))]
    #[test_case(Rule::NoBlankLineAfterSection, Path::new("sections.py"))]
    #[test_case(Rule::MissingBlankLineAfterLastSection, Path::new("D413.py"))]
    #[test_case(Rule::MissingBlankLineAfterSummary, Path::new("D.py"))]
    #[test_case(Rule::NoBlankLineBeforeSection, Path::new("sections.py"))]
    #[test_case(Rule::NonCapitalizedSectionName, Path::new("sections.py"))]
    #[test_case(Rule::MissingDashedUnderlineAfterSection, Path::new("sections.py"))]
    #[test_case(Rule::UndocumentedParam, Path::new("canonical_google_examples.py"))]
    #[test_case(Rule::UndocumentedParam, Path::new("canonical_numpy_examples.py"))]
    #[test_case(Rule::UndocumentedParam, Path::new("sections.py"))]
    #[test_case(Rule::MissingTrailingPeriod, Path::new("D.py"))]
    #[test_case(Rule::MissingTrailingPeriod, Path::new("D400.py"))]
    #[test_case(Rule::MissingTrailingPeriod, Path::new("D400_415.py"))]
    #[test_case(Rule::MissingTerminalPunctuation, Path::new("D.py"))]
    #[test_case(Rule::MissingTerminalPunctuation, Path::new("D400_415.py"))]
    #[test_case(Rule::FirstWordUncapitalized, Path::new("D.py"))]
    #[test_case(Rule::FirstWordUncapitalized, Path::new("D403.py"))]
    #[test_case(Rule::UnnecessaryMultilineDocstring, Path::new("D.py"))]
    #[test_case(Rule::DocstringTabIndentation, Path::new("D.py"))]
    #[test_case(Rule::UndocumentedMagicMethod, Path::new("D.py"))]
    #[test_case(Rule::MultiLineSummaryFirstLine, Path::new("D.py"))]
    #[test_case(Rule::MultiLineSummarySecondLine, Path::new("D.py"))]
    #[test_case(Rule::NewLineAfterLastParagraph, Path::new("D.py"))]
    #[test_case(Rule::MissingNewLineAfterSectionName, Path::new("sections.py"))]
    #[test_case(Rule::BlankLineAfterFunction, Path::new("D.py"))]
    #[test_case(Rule::UnnecessaryMultilineDocstring, Path::new("D200.py"))]
    #[test_case(Rule::BlankLineAfterFunction, Path::new("D202.py"))]
    #[test_case(Rule::BlankLineBeforeClass, Path::new("D.py"))]
    #[test_case(Rule::BlankLineBeforeFunction, Path::new("D.py"))]
    #[test_case(Rule::BlankLinesBetweenHeaderAndContent, Path::new("sections.py"))]
    #[test_case(Rule::BlankLinesBetweenHeaderAndContent, Path::new("sphinx.py"))]
    #[test_case(Rule::OverIndentation, Path::new("D.py"))]
    #[test_case(Rule::OverIndentation, Path::new("D208.py"))]
    #[test_case(Rule::SignatureInDocstring, Path::new("D.py"))]
    #[test_case(Rule::SignatureInDocstring, Path::new("D402.py"))]
    #[test_case(Rule::SurroundingWhitespace, Path::new("D.py"))]
    #[test_case(Rule::DocstringStartsWithThis, Path::new("D.py"))]
    #[test_case(Rule::UnderIndentation, Path::new("D.py"))]
    #[test_case(Rule::EmptyDocstring, Path::new("D.py"))]
    #[test_case(Rule::EmptyDocstringSection, Path::new("sections.py"))]
    #[test_case(Rule::NonImperativeMood, Path::new("D401.py"))]
    #[test_case(Rule::NoBlankLineAfterSection, Path::new("D410.py"))]
    #[test_case(Rule::IncorrectBlankLineAfterClass, Path::new("D.py"))]
    #[test_case(Rule::IncorrectBlankLineBeforeClass, Path::new("D.py"))]
    #[test_case(Rule::UndocumentedPublicClass, Path::new("D.py"))]
    #[test_case(Rule::UndocumentedPublicFunction, Path::new("D.py"))]
    #[test_case(Rule::UndocumentedPublicInit, Path::new("D.py"))]
    #[test_case(Rule::UndocumentedPublicMethod, Path::new("D.py"))]
    #[test_case(Rule::UndocumentedPublicMethod, Path::new("setter.py"))]
    #[test_case(Rule::UndocumentedPublicModule, Path::new("D.py"))]
    #[test_case(Rule::UndocumentedPublicModule, Path::new("D100.ipynb"))]
    #[test_case(
        Rule::UndocumentedPublicModule,
        Path::new("_unrelated/pkg/D100_pub.py")
    )]
    #[test_case(
        Rule::UndocumentedPublicModule,
        Path::new("_unrelated/pkg/_priv/no_D100_priv.py")
    )]
    #[test_case(
        Rule::UndocumentedPublicModule,
        Path::new("_unrelated/_no_pkg_priv.py")
    )]
    #[test_case(Rule::UndocumentedPublicNestedClass, Path::new("D.py"))]
    #[test_case(Rule::UndocumentedPublicPackage, Path::new("D.py"))]
    #[test_case(Rule::UndocumentedPublicPackage, Path::new("D104/__init__.py"))]
    #[test_case(Rule::MissingSectionNameColon, Path::new("D.py"))]
    #[test_case(Rule::OverindentedSection, Path::new("sections.py"))]
    #[test_case(Rule::OverindentedSection, Path::new("D214_module.py"))]
    #[test_case(Rule::OverindentedSectionUnderline, Path::new("D215.py"))]
    #[test_case(Rule::MissingSectionUnderlineAfterName, Path::new("sections.py"))]
    #[test_case(Rule::MismatchedSectionUnderlineLength, Path::new("sections.py"))]
    #[test_case(Rule::OverindentedSectionUnderline, Path::new("sections.py"))]
    #[test_case(Rule::OverloadWithDocstring, Path::new("D.py"))]
    #[test_case(Rule::EscapeSequenceInDocstring, Path::new("D.py"))]
    #[test_case(Rule::EscapeSequenceInDocstring, Path::new("D301.py"))]
    #[test_case(Rule::TripleSingleQuotes, Path::new("D.py"))]
    #[test_case(Rule::TripleSingleQuotes, Path::new("D300.py"))]
    fn rules(rule_code: Rule, path: &Path) -> Result<()> {
        let snapshot = format!("{}_{}", rule_code.noqa_code(), path.to_string_lossy());
        let diagnostics = test_path(
            Path::new("pydocstyle").join(path).as_path(),
            &settings::LinterSettings {
                pydocstyle: Settings {
                    ignore_decorators: ["functools.wraps".to_string()].into_iter().collect(),
                    property_decorators: ["gi.repository.GObject.Property".to_string()]
                        .into_iter()
                        .collect(),
                    ..Settings::default()
                },
                ..settings::LinterSettings::for_rule(rule_code)
            },
        )?;
        assert_diagnostics!(snapshot, diagnostics);
        Ok(())
    }

    #[test]
    fn bom() -> Result<()> {
        let diagnostics = test_path(
            Path::new("pydocstyle/bom.py"),
            &settings::LinterSettings::for_rule(Rule::TripleSingleQuotes),
        )?;
        assert_diagnostics!(diagnostics);
        Ok(())
    }

    #[test]
    fn d417_unspecified() -> Result<()> {
        let diagnostics = test_path(
            Path::new("pydocstyle/D417.py"),
            &settings::LinterSettings {
                // When inferring the convention, we'll see a few false negatives.
                // See: https://github.com/PyCQA/pydocstyle/issues/459.
                pydocstyle: Settings::default(),
                ..settings::LinterSettings::for_rule(Rule::UndocumentedParam)
            },
        )?;
        assert_diagnostics!(diagnostics);
        Ok(())
    }

    #[test]
    fn d417_unspecified_ignore_var_parameters() -> Result<()> {
        let diagnostics = test_path(
            Path::new("pydocstyle/D417.py"),
            &settings::LinterSettings {
                pydocstyle: Settings::default(),
                ..settings::LinterSettings::for_rule(Rule::UndocumentedParam)
            },
        )?;
        assert_diagnostics!(diagnostics);
        Ok(())
    }

    #[test]
    fn d417_google() -> Result<()> {
        let diagnostics = test_path(
            Path::new("pydocstyle/D417.py"),
            &settings::LinterSettings {
                // With explicit Google convention, we should flag every function.
                pydocstyle: Settings {
                    convention: Some(Convention::Google),
                    ..Settings::default()
                },
                ..settings::LinterSettings::for_rule(Rule::UndocumentedParam)
            },
        )?;
        assert_diagnostics!(diagnostics);
        Ok(())
    }

    #[test]
    fn d417_google_ignore_var_parameters() -> Result<()> {
        let diagnostics = test_path(
            Path::new("pydocstyle/D417.py"),
            &settings::LinterSettings {
                pydocstyle: Settings {
                    convention: Some(Convention::Google),
                    ignore_var_parameters: true,
                    ..Settings::default()
                },
                ..settings::LinterSettings::for_rule(Rule::UndocumentedParam)
            },
        )?;
        assert_diagnostics!(diagnostics);
        Ok(())
    }

    #[test]
    fn d417_numpy() -> Result<()> {
        let diagnostics = test_path(
            Path::new("pydocstyle/D417.py"),
            &settings::LinterSettings {
                // With explicit numpy convention, we shouldn't flag anything.
                pydocstyle: Settings {
                    convention: Some(Convention::Numpy),
                    ..Settings::default()
                },
                ..settings::LinterSettings::for_rule(Rule::UndocumentedParam)
            },
        )?;
        assert_diagnostics!(diagnostics);
        Ok(())
    }

    #[test]
    fn d209_d400() -> Result<()> {
        let diagnostics = test_path(
            Path::new("pydocstyle/D209_D400.py"),
            &settings::LinterSettings::for_rules([
                Rule::NewLineAfterLastParagraph,
                Rule::MissingTrailingPeriod,
            ]),
        )?;
        assert_diagnostics!(diagnostics);
        Ok(())
    }

    #[test]
    fn all() -> Result<()> {
        let diagnostics = test_path(
            Path::new("pydocstyle/all.py"),
            &settings::LinterSettings::for_rules([
                Rule::UndocumentedPublicModule,
                Rule::UndocumentedPublicClass,
                Rule::UndocumentedPublicMethod,
                Rule::UndocumentedPublicFunction,
                Rule::UndocumentedPublicPackage,
                Rule::UndocumentedMagicMethod,
                Rule::UndocumentedPublicNestedClass,
                Rule::UndocumentedPublicInit,
            ]),
        )?;
        assert_diagnostics!(diagnostics);
        Ok(())
    }
}
