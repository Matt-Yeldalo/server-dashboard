trait RemoteExecuter {
    fn root(&self) -> String;
    fn uptime(&self) -> FileContent;
    fn storage(&self) -> FileContent;
    fn revision(&self) -> FileContent;
    fn git_log(&self) -> FileContent;
    fn git_branch(&self) -> FileContent;
    fn status(&self) -> FileContent;
    fn retrieve_info(&self) -> Result<DisplayInfo, String>;
    fn get_file_content(&self, file_path: &str) -> FileContent;
}
