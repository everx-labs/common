pub fn library_commit() -> Option<&'static str> {
    std::option_env!("BUILD_GIT_COMMIT")
}
