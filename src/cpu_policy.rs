use std::{
    fs, os::unix::fs::PermissionsExt, path::{Path, PathBuf}
};

pub struct Policy {
    path: PathBuf,
    freqs: Vec<isize>,
    pos: usize,
}

impl Policy {
    pub fn new(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let freqs = path.as_ref().join("scaling_available_frequencies");
        let mut freqs: Vec<isize> = fs::read_to_string(freqs)?
            .split_whitespace()
            .map(|freq| freq.parse().unwrap())
            .collect();
        freqs.sort_unstable();
        Ok(Self {
            path: path.as_ref().to_path_buf(),
            pos: freqs.len() - 1,
            freqs,
        })
    }

    pub fn limit(&mut self) {
        if self.pos > 0 {
            self.pos -= 1;
        }

        self.write();
    }

    pub fn release(&mut self) {
        if self.pos < self.freqs.len() - 1 {
            self.pos += 1;
        }

        self.write();
    }

    fn write(&self) {
        if fs::write(
            self.path.join("scaling_max_freq"),
            self.freqs[self.pos].to_string(),
        ).is_err() {
            let _ = fs::set_permissions(self.path.join("scaling_max_freq"), PermissionsExt::from_mode(0o644));
            self.write();
        }
    }

    pub fn reset(&mut self) {
        self.pos = self.freqs.len() - 1;
        self.write();
    }
}
