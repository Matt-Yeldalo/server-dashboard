pub trait RemoteExecuter {
    asyc fn run(&self, command: &str) -> Result<String>;
    async fn read_file(&self, path: &Path) -> Result<String>;
}
