trait RemoteExecuter {
    fn root(&self) -> String;
    fn uptime(&self, server_index: usize) -> FileContent;
    fn storage(&self, server_index: usize) -> FileContent;
    fn revision(&self, server_index: usize) -> FileContent;
    fn git_log(&self, server_index: usize) -> FileContent;
    fn git_branch(&self, server_index: usize) -> FileContent;
    fn status(&self, server_index: usize) -> FileContent;
    fn retrieve_info(&self, server_index: usize) -> Result<DisplayInfo, String>;
    fn get_file_content(&self, file_path: &str, server_index: usize) -> FileContent;
    fn file_content_list(&self, server_index: usize) -> Vec<FileContent>;
    fn server_list(&self) -> Vec<String>;
}
