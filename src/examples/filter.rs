use globset::{Glob, GlobSet, GlobSetBuilder};
use std::path::Path;

/// 檔案過濾器結構體
pub struct FileFilter {
    // 使用 Option<T> 作為「開關」
    // None: 代表不執行過濾（全選或不排除）
    // Some(GlobSet): 代表啟動匹配引擎
    pub include: Option<GlobSet>,
    pub exclude: Option<GlobSet>,
}

impl FileFilter {
    /// 建立新的過濾器
    pub fn new(include_str: &str, exclude_str: &str) -> Self {
        // 方案 A：手動判定是否需要啟動引擎
        // 如果使用者輸入 "*" 或空，則視為 None (不進行過濾，效能最快)
        let include = if include_str == "*" || include_str.is_empty() {
            None
        } else {
            Self::build_set(include_str)
        };

        // 如果 exclude 為空，則視為 None (不排除任何檔案)
        let exclude = if exclude_str.is_empty() {
            None
        } else {
            Self::build_set(exclude_str)
        };

        Self { include, exclude }
    }

    /// 私有方法：將逗號分隔的字串編譯成 GlobSet
    fn build_set(patterns: &str) -> Option<GlobSet> {
        let mut builder = GlobSetBuilder::new();
        let mut has_pattern = false;

        for pattern in patterns.split(',') {
            let p = pattern.trim();
            if !p.is_empty() {
                // 處理目錄形式：若以 / 結尾，自動轉為該目錄下所有內容 (dir/**)
                let final_p = if p.ends_with('/') {
                    format!("{}**", p)
                } else {
                    p.to_string()
                };

                // 將字串編譯為 Glob 模式並加入 Builder
                if let Ok(glob) = Glob::new(&final_p) {
                    builder.add(glob);
                    has_pattern = true;
                }
            }
        }

        if has_pattern {
            Some(builder.build().expect("GlobSet 編譯失敗"))
        } else {
            None
        }
    }

    /// 核心匹配邏輯
    pub fn is_match(&self, path: &Path) -> bool {
        // 1. 優先處理排除 (Exclude) 邏輯
        // 如果是 Some，就跑匹配；如果是 None，直接跳過這段
        if let Some(ref ex_set) = self.exclude {
            if ex_set.is_match(path) {
                return false; // 匹配到排除模式，剔除
            }
        }

        // 2. 處理包含 (Include) 邏輯
        match &self.include {
            // 如果有設定特定模式（如 *.py），則必須匹配
            Some(inc_set) => inc_set.is_match(path),
            // 如果是 None（代表使用者輸入 *），直接過關，不跑運算
            None => true,
        }
    }
}