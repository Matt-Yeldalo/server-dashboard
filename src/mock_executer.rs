include!("remote_executer.rs");

struct MockExecuter {}

impl MockExecuter {
    fn root(&self) -> &str {
        "./fixtures/"
    }

    fn read_file(&self, server_index: usize, deployment: Option<&str>, filename: &str) -> FileContent {
        let path = match deployment {
            Some(d) => format!(
                "{}{}/{}/{}",
                self.root(),
                self.server_list()[server_index],
                d,
                filename
            ),
            None => format!(
                "{}{}/{}",
                self.root(),
                self.server_list()[server_index],
                filename
            ),
        };

        let label = filename.to_string();

        if !std::path::Path::new(&path).exists() {
            return FileContent {
                label,
                content: format!("File not found: {}", path),
            };
        }

        match std::fs::read_to_string(&path) {
            Err(e) => FileContent {
                label,
                content: format!("Error reading file: {}", e),
            },
            Ok(content) if content.trim().is_empty() => FileContent {
                label,
                content: "No content".to_string(),
            },
            Ok(content) => FileContent {
                label,
                content: content.trim().to_string(),
            },
        }
    }
}

impl RemoteExecuter for MockExecuter {
    fn server_list(&self) -> Vec<String> {
        vec![
            "web-01".to_string(),
            "web-02".to_string(),
            "web-03".to_string(),
        ]
    }

    fn server_host_content(&self, server_index: usize) -> Vec<FileContent> {
        vec![
            self.read_file(server_index, None, "uptime"),
            self.read_file(server_index, None, "df"),
        ]
    }

    fn deployment_list(&self, server_index: usize) -> Vec<String> {
        let server_path = format!("{}{}", self.root(), self.server_list()[server_index]);
        let mut dirs: Vec<String> = std::fs::read_dir(&server_path)
            .map(|rd| {
                rd.filter_map(|e| e.ok())
                    .filter(|e| e.path().is_dir())
                    .map(|e| e.file_name().to_string_lossy().to_string())
                    .collect()
            })
            .unwrap_or_default();
        dirs.sort();
        dirs
    }

    fn deployment_content(&self, server_index: usize, deployment_index: usize) -> Vec<FileContent> {
        let deployments = self.deployment_list(server_index);
        let deployment = &deployments[deployment_index];
        vec![
            self.read_file(server_index, Some(deployment), "REVISION"),
            self.read_file(server_index, Some(deployment), "git-log"),
            self.read_file(server_index, Some(deployment), "git-branch"),
            self.read_file(server_index, Some(deployment), "status"),
            self.read_file(server_index, Some(deployment), "releases"),
        ]
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
