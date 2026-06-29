include!("remote_executer.rs");

struct MockExecuter {
    // root_path: String,
}

impl RemoteExecuter for MockExecuter {
    fn server_list(&self) -> Vec<String> {
        vec!["web-01/".to_string(), "web-02/".to_string()]
    }

    fn root(&self) -> String {
        "./fixtures/".to_string()
    }

    fn retrieve_info(&self, server_index: usize) -> Result<DisplayInfo, String> {
        Ok(DisplayInfo {
            uptime: self.uptime(server_index),
            storage: self.storage(server_index),
            revision: self.revision(server_index),
            git_log: self.git_log(server_index),
            git_branch: self.git_branch(server_index),
            status: self.status(server_index),
        })
    }

    fn file_content_list(&self, server_index: usize) -> Vec<FileContent> {
        vec![
            self.uptime(server_index),
            self.storage(server_index),
            self.revision(server_index),
            self.git_log(server_index),
            self.git_branch(server_index),
            self.status(server_index),
        ]
    }

    fn get_file_content(&self, file_path: &str, server_index: usize) -> FileContent {
        let root_server_check = format!("{}{}", self.root(), self.server_list()[server_index]);
        if !std::path::Path::new(&root_server_check).exists() {
            return FileContent {
                label: file_path.to_string(),
                content: format!("Server not found: {}", root_server_check),
            };
        }

        let full_path = format!(
            "{}{}{}",
            self.root(),
            self.server_list()[server_index],
            file_path
        );

        if !std::path::Path::new(&full_path).exists() {
            return FileContent {
                label: file_path.to_string(),
                content: format!("File not found: {}", full_path),
            };
        }

        let content = std::fs::read_to_string(full_path);
        if let Err(e) = content {
            return FileContent {
                label: file_path.to_string(),
                content: format!("Error reading file: {}", e),
            };
        }

        let content = content.unwrap();

        if content.is_empty() {
            return FileContent {
                label: file_path.to_string(),
                content: "No content".to_string(),
            };
        }

        FileContent {
            label: file_path.to_string(),
            content: content.trim().to_string(),
        }
    }

    fn revision(&self, server_index: usize) -> FileContent {
        self.get_file_content("REVISION", server_index)
    }

    fn git_log(&self, server_index: usize) -> FileContent {
        self.get_file_content("git-log", server_index)
    }

    fn git_branch(&self, server_index: usize) -> FileContent {
        self.get_file_content("git-branch", server_index)
    }

    fn status(&self, server_index: usize) -> FileContent {
        self.get_file_content("status", server_index)
    }

    fn storage(&self, server_index: usize) -> FileContent {
        self.get_file_content("df", server_index)
    }

    fn uptime(&self, server_index: usize) -> FileContent {
        self.get_file_content("uptime", server_index)
    }
}

struct FileContent {
    label: String,
    content: String,
}

impl Clone for FileContent {
    fn clone(&self) -> Self {
        Self {
            label: self.label.clone(),
            content: self.content.clone(),
        }
    }
}

impl std::fmt::Display for FileContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.label, self.content)
    }
}

struct DisplayInfo {
    uptime: FileContent,
    storage: FileContent,
    revision: FileContent,
    git_log: FileContent,
    git_branch: FileContent,
    status: FileContent,
}
