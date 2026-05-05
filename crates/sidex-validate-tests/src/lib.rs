//! End-to-end runtime tests for the validate plugin's generated Rust code.
//!
//! The fixture bundle at `bundle/` is generated at build time via
//! `sidex-build-rs`; the resulting Rust types live under
//! [`validate_tests::data`] and provide `Validate` impls.

sidex::include_bundle!(pub validate_tests);

#[cfg(test)]
mod tests {
    use sidex_validate::Validate;

    use crate::validate_tests::data::*;

    #[test]
    fn user_with_valid_inputs_passes() {
        let user = User {
            email: "test@example.com".to_owned(),
            age: 30,
            nickname: Some("rusty".to_owned()),
        };
        let report = user.validate();
        assert!(report.is_ok(), "expected ok, got: {report}");
    }

    #[test]
    fn user_with_invalid_email_length_emits_min_length() {
        let user = User {
            email: String::new(),
            age: 30,
            nickname: None,
        };
        let report = user.validate();
        let codes: Vec<_> = report.errors().iter().map(|e| e.code.as_ref()).collect();
        assert!(
            codes.contains(&"min_length"),
            "expected min_length, got {codes:?}"
        );
        // The user-supplied message is threaded through verbatim.
        assert!(
            report
                .errors()
                .iter()
                .any(|e| e.message == "Email length is out of range."),
            "expected user message in {report}"
        );
    }

    #[test]
    fn user_with_oversized_email_emits_max_length() {
        let user = User {
            email: "a".repeat(255),
            age: 30,
            nickname: None,
        };
        let report = user.validate();
        let codes: Vec<_> = report.errors().iter().map(|e| e.code.as_ref()).collect();
        assert!(
            codes.contains(&"max_length"),
            "expected max_length, got {codes:?}"
        );
    }

    #[test]
    fn user_with_age_out_of_range_emits_max() {
        let user = User {
            email: "a".to_owned(),
            age: 200,
            nickname: None,
        };
        let report = user.validate();
        let codes: Vec<_> = report.errors().iter().map(|e| e.code.as_ref()).collect();
        assert!(codes.contains(&"max"), "expected max, got {codes:?}");
        assert!(
            report.errors().iter().any(|e| e.path.to_string() == "/age"),
            "expected /age path"
        );
    }

    #[test]
    fn nickname_validates_only_when_present() {
        // None → no error from the optional rule.
        let user = User {
            email: "a".to_owned(),
            age: 30,
            nickname: None,
        };
        let report = user.validate();
        let codes: Vec<_> = report
            .errors()
            .iter()
            .map(|e| (e.path.to_string(), e.code.clone()))
            .collect();
        assert!(
            !codes.iter().any(|(path, _)| path == "/nickname"),
            "absent optional should not be validated, got {codes:?}"
        );

        // Some("") → length rule fires.
        let user = User {
            email: "a".to_owned(),
            age: 30,
            nickname: Some(String::new()),
        };
        let report = user.validate();
        assert!(
            report
                .errors()
                .iter()
                .any(|e| e.path.to_string() == "/nickname" && e.code == "min_length"),
            "present empty optional should fail min_length, got {report}"
        );
    }

    #[test]
    fn slug_try_new_rejects_oversized_input() {
        let too_long = "a".repeat(65);
        let result = Slug::try_new(too_long);
        let report = result.expect_err("oversized slug should fail try_new");
        assert!(report.errors().iter().any(|e| e.code == "max_length"));
    }

    #[test]
    fn slug_try_new_accepts_valid_input() {
        let slug = Slug::try_new("hello-world".to_owned()).expect("valid slug accepted");
        assert!(slug.validate().is_ok());
    }

    #[test]
    fn regex_with_code_override_threads_through() {
        let bad = CodeOnly {
            code: "lowercase".to_owned(),
        };
        let report = bad.validate();
        let codes: Vec<_> = report.errors().iter().map(|e| e.code.as_ref()).collect();
        assert!(
            codes.contains(&"format:order"),
            "expected format:order code override, got {codes:?}"
        );
    }

    #[test]
    fn regex_passes_for_matching_input() {
        let ok = CodeOnly {
            code: "AB-1234".to_owned(),
        };
        assert!(ok.validate().is_ok());
    }

    #[test]
    fn report_paths_use_jsonpointer() {
        let user = User {
            email: String::new(),
            age: 200,
            nickname: None,
        };
        let report = user.validate();
        let paths: Vec<_> = report.errors().iter().map(|e| e.path.to_string()).collect();
        assert!(paths.iter().any(|p| p == "/email"));
        assert!(paths.iter().any(|p| p == "/age"));
    }

    #[test]
    fn into_result_returns_err_for_invalid_inputs() {
        let user = User {
            email: String::new(),
            age: 30,
            nickname: None,
        };
        assert!(user.validate().into_result().is_err());

        let user = User {
            email: "a".to_owned(),
            age: 1,
            nickname: None,
        };
        assert!(user.validate().into_result().is_ok());
    }
}
