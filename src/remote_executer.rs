trait RemoteExecuter {
    fn server_list(&self) -> Vec<String>;
    fn server_host_content(&self, server_index: usize) -> Vec<FileContent>;
    fn deployment_list(&self, server_index: usize) -> Vec<String>;
    fn deployment_content(&self, server_index: usize, deployment_index: usize) -> Vec<FileContent>;
}
