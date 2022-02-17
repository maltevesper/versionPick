mod tests {
    use fail::FailScenario;
    use version_pick::Git;
    use version_pick::VersionControlSystem;

    #[test]
    fn test_heads_fail_create_detached() {
        let scenario = FailScenario::setup();
        fail::cfg("git.heads.create_detached", "return('well then')").unwrap();
        let git = Git::from_url(".");
        assert_eq!(
            git.heads().unwrap_err().to_string(),
            "Injected create_detach error."
        );
        scenario.teardown();
    }
}
