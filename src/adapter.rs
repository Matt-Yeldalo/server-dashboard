pub trait DepoloyAdapter {
    fn name(&self) -> String;
    fn fetch_info(&self, dir: &str) -> Result<DeployInfo>;
}
