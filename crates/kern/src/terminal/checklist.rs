#[cfg(windows)]
fn platform_supports_emoji() -> bool {
    std::env::var("WT_SESSION").is_ok()
}

pub struct CheckList {
    current_item: String,
    in_progress: String,
    done: String,
    failed: String,
}

impl CheckList {
    pub fn new(title: &str) -> Self {
        eprintln!("{}", title);

        let supports_emoji = platform_supports_emoji();

        Self {
            current_item: String::new(),
            in_progress: if supports_emoji {
                "⌛".to_string()
            } else {
                "...".to_string()
            },
            done: if supports_emoji {
                "✅".to_string()
            } else {
                "✔".to_string()
            },
            failed: if supports_emoji {
                "❌".to_string()
            } else {
                "✖".to_string()
            },
        }
    }

    pub fn start_step(&mut self, step: String) {
        self.current_item = step;
        eprint!("  {} {}  ", self.in_progress, self.current_item);
    }

    pub fn finish_step(&mut self) {
        eprintln!("\r  {} {}  ", self.done, self.current_item);
    }

    pub fn fail(&mut self) {
        eprintln!("\r  {} {}  ", self.failed, self.current_item);
    }
}
