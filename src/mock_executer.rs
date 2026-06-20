include!("remote_executer_trait.rs");

struct MockExecuter {}

impl RemoteExecuter for MockExecuter {
    fn retrieve_info(&self) -> Result<DisplayInfo, String> {
        Ok(DisplayInfo {
            uptime: self.uptime(),
            storage: self.storage(),
            revision: self.revision(),
            git_log: self.git_log(),
            git_branch: self.git_branch(),
            status: self.status(),
        })
    }

    fn file_content_list(&self) -> Vec<FileContent> {
        vec![
            self.uptime(),
            self.storage(),
            self.revision(),
            self.git_log(),
            self.git_branch(),
            self.status(),
        ]
    }

    fn get_file_content(&self, file_path: &str) -> FileContent {
        let full_path = format!("{}{}", self.root(), file_path);

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

    fn root(&self) -> String {
        "./fixtures/web-01/".to_string()
    }

    fn revision(&self) -> FileContent {
        self.get_file_content("REVISION")
    }

    fn git_log(&self) -> FileContent {
        self.get_file_content("git-log")
    }

    fn git_branch(&self) -> FileContent {
        self.get_file_content("git-branch")
    }

    fn status(&self) -> FileContent {
        self.get_file_content("status")
    }

    fn storage(&self) -> FileContent {
        self.get_file_content("df")
    }

    fn uptime(&self) -> FileContent {
        self.get_file_content("uptime")
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
