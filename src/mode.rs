use std::str::FromStr;

#[derive(Debug)]
pub enum Mode {
    Powersave,
    Balance,
    Performance,
    Fast,
}

impl FromStr for Mode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        Ok(match s {
            "powersave" => Self::Powersave,
            "balance" => Self::Balance,
            "performance" => Self::Performance,
            "fast" | "pedestal" => Self::Fast,
            _ => Mode::Balance,
        })
    }
}
