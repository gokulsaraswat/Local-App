pub type CommandResult<T> = Result<T, String>;

pub fn to_command_error<E: std::fmt::Display>(context: &str, error: E) -> String {
    format!("{context}: {error}")
}
