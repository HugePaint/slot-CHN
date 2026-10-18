use std::collections::HashMap;
use std::fmt;
use std::path::{Path, PathBuf};

use pinyin::ToPinyin;

use crate::gba::header_title_code;
use crate::platform::Platform;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Cart {
    pub platform: Platform,
    pub stem: String,
    pub rom: PathBuf,
    pub label: Option<PathBuf>,
    pub title: String,
    pub code: String,
    pub shell: Option<crate::ShellChoice>,
}

impl Cart {
    pub fn read_header(&mut self) {
        match self.platform {
            Platform::Gba => {
                if let Some((title, code)) = header_title_code(&self.rom) {
                    self.title = title;
                    self.code = code;
                }
            }
            _ => {
                if let Some(title) = crate::gb::title(&self.rom) {
                    self.title = title;
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum StoreError {
    Io(std::io::Error),
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StoreError::Io(e) => write!(f, "io: {e}"),
        }
    }
}

impl std::error::Error for StoreError {}

impl From<std::io::Error> for StoreError {
    fn from(e: std::io::Error) -> Self {
        StoreError::Io(e)
    }
}

pub fn scan(root: &Path) -> Result<Vec<Cart>, StoreError> {
    let read = |file: &str| std::fs::read_to_string(root.join(file)).unwrap_or_default();
    let shells = crate::cart_shell::layered(
        &read(crate::CART_SHELL_FILE),
        &read(crate::LABELS_SHELL_FILE),
    );
    let mut carts = Vec::new();
    for platform in Platform::ALL {
        let dir = root.join("Games").join(platform.dir_name());
        let entries = match std::fs::read_dir(&dir) {
            Ok(d) => d,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
            Err(e) => {
                eprintln!("slot: scan: {}: {e}", dir.display());
                continue;
            }
        };
        let labels_dir = root.join("Labels").join(platform.dir_name());
        let labels = listing(&labels_dir);
        for entry in entries {
            let Ok(entry) = entry else {
                continue;
            };
            let rom = entry.path();
            if is_hidden(&rom) || !platform.accepts(&rom) || !is_file(&entry) {
                continue;
            }
            let Some(stem) = rom.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };
            let label = labels
                .get(&format!("{stem}.png").to_lowercase())
                .map(|name| labels_dir.join(name));
            carts.push(Cart {
                platform,
                stem: stem.to_string(),
                title: String::new(),
                code: String::new(),
                shell: shells.get(&crate::cart_shell::key(stem)).copied(),
                label,
                rom,
            });
        }
    }
    carts.sort_by(|a, b| {
        (a.platform as u8, sort_key(&a.stem)).cmp(&(b.platform as u8, sort_key(&b.stem)))
    });
    Ok(carts)
}

pub fn listing(dir: &Path) -> HashMap<String, String> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return HashMap::new();
    };
    entries
        .filter_map(|e| e.ok()?.file_name().into_string().ok())
        .filter(|name| !name.starts_with('.'))
        .map(|name| (name.to_lowercase(), name))
        .collect()
}

fn is_file(entry: &std::fs::DirEntry) -> bool {
    match entry.file_type() {
        Ok(t) if t.is_symlink() => entry.path().is_file(),
        Ok(t) => t.is_file(),
        Err(_) => false,
    }
}

pub fn sort_key(stem: &str) -> (u8, String) {
    (bucket_of(stem), stem.to_uppercase())
}

pub const INITIAL_BUCKETS: [char; 27] = [
    '#', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
    'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

fn bucket_of(stem: &str) -> u8 {
    INITIAL_BUCKETS
        .iter()
        .position(|bucket| *bucket == initial(stem))
        .unwrap_or(0) as u8
}

pub fn initial(stem: &str) -> char {
    let Some(first) = stem.chars().find(|c| !c.is_whitespace()) else {
        return '#';
    };
    if first.is_ascii_alphabetic() {
        return first.to_ascii_uppercase();
    }
    first
        .to_pinyin()
        .and_then(|pinyin| pinyin.plain().chars().next())
        .filter(char::is_ascii_alphabetic)
        .map(|letter| letter.to_ascii_uppercase())
        .unwrap_or('#')
}

pub fn is_hidden(p: &Path) -> bool {
    p.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|n| n.starts_with('.'))
}

#[cfg(test)]
mod tests {
    use super::{initial, sort_key, INITIAL_BUCKETS};

    #[test]
    fn hanzi_are_filed_by_their_pinyin_initial() {
        assert_eq!(initial("宝可梦"), 'B');
        assert_eq!(initial("恶魔城"), 'E');
        assert_eq!(initial("洛克人"), 'L');
    }

    #[test]
    fn digits_and_marks_use_the_catch_all_bucket() {
        assert_eq!(initial("1080 Snowboarding"), '#');
        assert_eq!(initial("[BIOS] Test"), '#');
        assert_eq!(initial("ポケットモンスター"), '#');
        assert!(!INITIAL_BUCKETS.iter().any(char::is_ascii_digit));
    }

    #[test]
    fn pinyin_buckets_are_the_primary_sort_order() {
        let mut names = [
            "洛克人",
            "宝可梦",
            "Advance Wars",
            "恶魔城",
            "1080 Snowboarding",
        ];
        names.sort_by_key(|name| sort_key(name));
        assert_eq!(
            names,
            [
                "1080 Snowboarding",
                "Advance Wars",
                "宝可梦",
                "恶魔城",
                "洛克人"
            ]
        );
    }
}
