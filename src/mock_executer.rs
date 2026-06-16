impl RemoteExecuter for MockExecuter {
    async fn run(&self, command: &str) -> Result<String> {
        Ok(format!("Executed command: {}", command))
    }

    async fn read_file(&self, path: &Path) -> Result<String> {
        Ok(format!("Contents of file at path: {:?}", path))
    }
}
