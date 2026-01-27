use std::collections::BTreeMap;
use std::path::Path;

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub children: BTreeMap<String, Node>,
    pub is_file: bool,
}

impl Node {
    pub fn new(name: String, is_file: bool) -> Self {
        Self {
            name,
            children: BTreeMap::new(),
            is_file,
        }
    }

    // 插入路徑的邏輯
    pub fn insert_path(&mut self, path: &Path, is_file: bool) {
        let mut current = self;
        for component in path.components() {
            let name = component.as_os_str().to_string_lossy().to_string();
            current = current.children.entry(name.clone())
                .or_insert_with(|| Node::new(name, false));
        }
        current.is_file = is_file;
    }

    // 遞迴列印
    pub fn print(&self, indent: &str) {
        let len = self.children.len();
        for (i, (name, child)) in self.children.iter().enumerate() {
            let is_last = i == len - 1;
            let connector = if is_last { "└── " } else { "├── " };
            
            println!("{}{}{}", indent, connector, name);

            if !child.children.is_empty() {
                let new_indent = format!("{}{}", indent, if is_last { "    " } else { "│   " });
                child.print(&new_indent);
            }
        }
    }
}