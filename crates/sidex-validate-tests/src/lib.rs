//! End-to-end tests for validation rules in generated Rust code.
//!
//! The fixture bundle at `bundle/` is generated at build time via
//! `sidex-build-rs`. The generated types carry their rules in their shapes,
//! which [`sidex::validation::validate`] checks.

sidex::include_bundle!(pub validate_tests);

#[cfg(test)]
mod tests {
    use sidex::types::Map;
    use sidex::types::Seq;
    use sidex::validation::ValidationReport;
    use sidex::validation::validate;

    use crate::validate_tests::data::*;

    fn ok_user() -> User {
        User {
            email: "test@example.com".into(),
            age: 30,
            nickname: Some("rusty".into()),
            tags: Seq::from(vec!["alpha".into()]),
            attributes: Map::new(),
        }
    }

    /// Returns the report of an invalid value.
    fn report(user: &User) -> ValidationReport {
        validate(user).expect_err("value should be invalid")
    }

    /// Returns whether the report has an error with the given path and code.
    fn has_error(report: &ValidationReport, path: &str, code: &str) -> bool {
        report
            .errors()
            .iter()
            .any(|error| error.path().to_string() == path && error.code() == code)
    }

    #[test]
    fn user_with_valid_inputs_passes() {
        validate(&ok_user()).unwrap();
    }

    #[test]
    fn user_with_invalid_email_size_emits_min_size() {
        let report = report(&User {
            email: "".into(),
            ..ok_user()
        });
        assert!(has_error(&report, "email", "min_size"), "{report}");
        // The message of the rule replaces the default message.
        assert!(
            report
                .errors()
                .iter()
                .any(|error| error.message() == "Email length is out of range."),
            "{report}"
        );
    }

    #[test]
    fn user_with_oversized_email_emits_max_size() {
        let report = report(&User {
            email: "a".repeat(255).into(),
            ..ok_user()
        });
        assert!(has_error(&report, "email", "max_size"), "{report}");
    }

    #[test]
    fn user_with_age_out_of_range_emits_max() {
        let report = report(&User {
            age: 200,
            ..ok_user()
        });
        assert!(has_error(&report, "age", "max"), "{report}");
    }

    #[test]
    fn nickname_validates_only_when_present() {
        validate(&User {
            nickname: None,
            ..ok_user()
        })
        .unwrap();

        let report = report(&User {
            nickname: Some("".into()),
            ..ok_user()
        });
        assert!(has_error(&report, "nickname", "min_size"), "{report}");
    }

    #[test]
    fn empty_tag_list_emits_min_size() {
        let report = report(&User {
            tags: Seq::new(),
            ..ok_user()
        });
        assert!(has_error(&report, "tags", "min_size"), "{report}");
    }

    #[test]
    fn oversized_tag_list_emits_max_size() {
        let tags = (0..16).map(|i| format!("t{i}").into()).collect();
        let report = report(&User { tags, ..ok_user() });
        assert!(has_error(&report, "tags", "max_size"), "{report}");
    }

    #[test]
    fn oversized_attribute_map_emits_max_size() {
        let attributes = (0..40)
            .map(|i| (format!("k{i}").into(), "".into()))
            .collect();
        let report = report(&User {
            attributes,
            ..ok_user()
        });
        assert!(has_error(&report, "attributes", "max_size"), "{report}");
    }

    #[test]
    fn wrapper_rules_apply_to_the_wrapped_value() {
        let report = validate(&Slug("a".repeat(65).into())).unwrap_err();
        assert!(has_error(&report, "", "max_size"), "{report}");
        validate(&Slug("hello-world".into())).unwrap();
    }

    #[test]
    fn regex_with_code_override_threads_through() {
        let report = validate(&CodeOnly {
            code: "lowercase".into(),
        })
        .unwrap_err();
        assert!(has_error(&report, "code", "format:order"), "{report}");
        validate(&CodeOnly {
            code: "AB-1234".into(),
        })
        .unwrap();
    }

    #[test]
    fn report_has_every_violation() {
        let report = report(&User {
            email: "".into(),
            age: 200,
            ..ok_user()
        });
        assert!(has_error(&report, "email", "min_size"), "{report}");
        assert!(has_error(&report, "age", "max"), "{report}");
    }
}
