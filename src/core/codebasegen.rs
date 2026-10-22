use std::path::{Path, PathBuf};

use crate::Resrap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileType {
    Dir,
    File,
}

/// Describes either a directory or file entry in a codebase specification.
#[derive(Clone, Debug)]
pub struct Node {
    pub name: String,
    pub typ: FileType,
    /// E.g. `code_*.c`
    pub pattern: String,
    /// Number of files, e.g. 10.
    pub count: usize,
    /// Tokens per file, e.g. 20.
    pub token_count: usize,
    /// Language/type identifier, e.g. `C` or `sql`.
    pub file_type: String,
    pub children: Vec<Node>,
}

impl Node {
    fn dir(name: String) -> Self {
        Node {
            name,
            typ: FileType::Dir,
            pattern: String::new(),
            count: 0,
            token_count: 0,
            file_type: String::new(),
            children: Vec::new(),
        }
    }

    /// Creates the directory structure and files.
    pub fn generate_structure(&self, resrap: &Resrap, root: &str) -> Result<(), String> {
        self.generate_node(resrap, root)
    }

    fn generate_node(&self, resrap: &Resrap, root: &str) -> Result<(), String> {
        let loc = if self.name.is_empty() {
            PathBuf::from(root)
        } else {
            Path::new(root).join(&self.name)
        };

        match self.typ {
            FileType::Dir => {
                std::fs::create_dir_all(&loc).map_err(|err| {
                    format!("failed to create directory '{}': {}", loc.display(), err)
                })?;

                for child in &self.children {
                    child.generate_node(resrap, loc.to_str().unwrap_or(root))?;
                }
            }
            FileType::File => {
                if self.count == 0 {
                    return Err(format!(
                        "invalid file count {} for pattern '{}'",
                        self.count, self.pattern
                    ));
                }

                std::fs::create_dir_all(root).map_err(|err| {
                    format!("failed to create parent directory '{}': {}", root, err)
                })?;

                for i in 1..=self.count {
                    let file_name = self.pattern.replace('*', &i.to_string());
                    let full_path = Path::new(root).join(&file_name);

                    // Complete generation finishes the top-level rule instead
                    // of cutting the file in the middle of a statement.
                    let content =
                        resrap.generate_complete(&self.file_type, "program", self.token_count);

                    std::fs::write(&full_path, content).map_err(|err| {
                        format!("failed to write file '{}': {}", full_path.display(), err)
                    })?;

                    println!(
                        "Generated: {} ({} tokens)",
                        full_path.display(),
                        self.token_count
                    );
                }
            }
        }

        Ok(())
    }
}

/// Parses the directory DSL into a tree.
pub fn parse_dsl(input: &str) -> Result<Node, String> {
    let mut root = Node::dir(String::new());
    let mut stack: Vec<(usize, Vec<usize>)> = Vec::new();

    for (line_num, line) in input.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        let indent = line.chars().take_while(|c| *c == ' ').count();
        let name = line.trim();

        let node = if let Some(open) = name.find('[') {
            if !name.ends_with(']') {
                return Err(format!(
                    "line {}: invalid file spec format, expected 'type[countXtokens pattern]'",
                    line_num + 1
                ));
            }

            let file_type = name[..open].to_string();
            let rest = &name[open + 1..name.len() - 1];
            let parts: Vec<&str> = rest.split_whitespace().collect();

            if parts.len() < 2 {
                return Err(format!(
                    "line {}: invalid file spec format, expected 'type[countXtokens pattern]'",
                    line_num + 1
                ));
            }

            let xparts: Vec<&str> = parts[0].split('x').collect();
            if xparts.len() != 2 {
                return Err(format!(
                    "line {}: invalid count format '{}', expected 'NUMxNUM'",
                    line_num + 1,
                    parts[0]
                ));
            }

            let count = xparts[0].parse::<usize>().map_err(|err| {
                format!(
                    "line {}: invalid file count '{}': {}",
                    line_num + 1,
                    xparts[0],
                    err
                )
            })?;
            let token_count = xparts[1].parse::<usize>().map_err(|err| {
                format!(
                    "line {}: invalid token count '{}': {}",
                    line_num + 1,
                    xparts[1],
                    err
                )
            })?;

            if count == 0 || token_count == 0 {
                return Err(format!(
                    "line {}: count and token count must be positive numbers",
                    line_num + 1
                ));
            }

            Node {
                name: String::new(),
                typ: FileType::File,
                pattern: parts[1].to_string(),
                count,
                token_count,
                file_type,
                children: Vec::new(),
            }
        } else {
            Node::dir(name.to_string())
        };

        while let Some((stack_indent, _)) = stack.last() {
            if *stack_indent >= indent {
                stack.pop();
            } else {
                break;
            }
        }

        let parent_path = stack
            .last()
            .map(|(_, path)| path.clone())
            .unwrap_or_default();
        let parent = node_at_mut(&mut root, &parent_path)
            .ok_or_else(|| format!("line {}: indentation error", line_num + 1))?;
        parent.children.push(node);
        let child_path = {
            let mut path = parent_path;
            path.push(parent.children.len() - 1);
            path
        };

        if node_at_mut(&mut root, &child_path)
            .map(|n| n.typ == FileType::Dir)
            .unwrap_or(false)
        {
            stack.push((indent, child_path));
        }
    }

    Ok(root)
}

fn node_at_mut<'a>(root: &'a mut Node, path: &[usize]) -> Option<&'a mut Node> {
    let mut node = root;
    for &index in path {
        node = node.children.get_mut(index)?;
    }
    Some(node)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Resrap;

    #[test]
    fn parses_simple_directory() {
        let root = parse_dsl("src/").unwrap();
        assert_eq!(root.typ, FileType::Dir);
        assert!(root.name.is_empty());
        assert_eq!(root.children.len(), 1);
        assert_eq!(root.children[0].name, "src/");
    }

    #[test]
    fn parses_nested_file_spec() {
        let root = parse_dsl("src/\n  code/\n    c[10x20 code_*.c]").unwrap();
        let code = &root.children[0].children[0];
        assert_eq!(code.typ, FileType::Dir);
        assert_eq!(code.name, "code/");
        assert_eq!(code.children.len(), 1);
        let file = &code.children[0];
        assert_eq!(file.typ, FileType::File);
        assert_eq!(file.file_type, "c");
        assert_eq!(file.count, 10);
        assert_eq!(file.token_count, 20);
        assert_eq!(file.pattern, "code_*.c");
    }

    #[test]
    fn rejects_bad_specs() {
        for dsl in ["c[abc]", "c[10 code_*.c]", "c[ax20 *.c]", "c[0x20 *.c]"] {
            assert!(parse_dsl(dsl).is_err(), "accepted {:?}", dsl);
        }
    }

    #[test]
    fn accepts_blank_lines() {
        let root = parse_dsl("\n  \nsrc/\n\t\nmore/").unwrap();
        assert_eq!(root.children.len(), 2);
    }

    #[test]
    fn generates_files() {
        let mut r = Resrap::new();
        r.parse_grammar("c", "program:'int x;';\n").unwrap();
        let dir = std::env::temp_dir().join(format!("resrap-dsl-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let dsl = "out/\n  c[2x10 code_*.c]\n";
        let root = parse_dsl(dsl).unwrap();
        root.generate_structure(&r, dir.to_str().unwrap()).unwrap();
        for i in 1..=2 {
            assert!(dir.join("out").join(format!("code_{}.c", i)).exists());
        }
        std::fs::remove_dir_all(&dir).ok();
    }
}
