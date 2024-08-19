use std::{
    fs,
    path::Path,
    str::FromStr,
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

use config::Config;
use cpu_policy::Policy;
use inotify::{Inotify, WatchMask};
use mode::Mode;
use temp_reader::TempReader;

mod config;
mod cpu_policy;
mod mode;
mod temp_reader;

fn main() -> anyhow::Result<()> {
    let config = fs::read_to_string("/data/adb/cpulimiter_rs/config.toml")?;
    let config = toml::from_str(&config)?;
    let config = Arc::new(RwLock::new(config));

    {
        let config = config.clone();
        thread::spawn(move || config_reader(config).unwrap());
    }

    let temp_reader = TempReader::new()?;
    let mut cpus: Vec<_> = fs::read_dir("/sys/devices/system/cpu/cpufreq")?
        .map(|entry| entry.unwrap())
        .filter(|entry| entry.file_name().to_str().unwrap().starts_with("policy"))
        .map(|entry| Policy::new(entry.path()).unwrap())
        .collect();

    let mut reseted = false;

    loop {
        thread::sleep(Duration::from_millis(33));

        if Path::new("/data/adb/cpulimiter_rs/fas_rs_on").exists() {
            if !reseted {
                cpus.iter_mut().for_each(|cpu| cpu.reset());
            }
            continue;
        }

        reseted = false;

        let mode = read_mode()?;
        let target_temp = {
            let config = config.read().unwrap();
            match mode {
                Mode::Powersave => config.powersave,
                Mode::Balance => config.balance,
                Mode::Performance => config.performance,
                Mode::Fast => config.fast,
            }
        };
        let current_temp = temp_reader.read_max_temp();

        if current_temp > target_temp {
            cpus.iter_mut().for_each(|cpu| cpu.limit());
        } else {
            cpus.iter_mut().for_each(|cpu| cpu.release());
        }
    }
}

fn config_reader(config_data: Arc<RwLock<Config>>) -> anyhow::Result<()> {
    loop {
        let mut inotify = Inotify::init()?;
        let _ = inotify.watches().add(
            "/data/adb/cpulimiter_rs/config.toml",
            WatchMask::CLOSE_WRITE | WatchMask::MODIFY,
        );

        let _ = inotify.read_events_blocking(&mut []);
        read_config(&config_data)?;
    }
}

fn read_config(config_data: &RwLock<Config>) -> anyhow::Result<()> {
    let config = fs::read_to_string("/data/adb/cpulimiter_rs/config.toml")?;
    if let Ok(config) = toml::from_str::<Config>(&config) {
        *config_data.write().unwrap() = config;
    } else {
        read_config(config_data)?;
    }
    Ok(())
}

fn read_mode() -> anyhow::Result<Mode> {
    let mode;
    let fas_rs_mode = Path::new("/dev/fas_rs/mode");
    if fas_rs_mode.exists() {
        let raw = fs::read_to_string(fas_rs_mode)?;
        mode = Mode::from_str(raw.trim())?;
    } else {
        let default_mode = Path::new("/data/adb/cpulimiter_rs/mode");
        if default_mode.exists() {
            let raw = fs::read_to_string(default_mode)?;
            mode = Mode::from_str(raw.trim())?;
        } else {
            fs::write(default_mode, "balance")?;
            let raw = fs::read_to_string(default_mode)?;
            mode = Mode::from_str(raw.trim())?;
        }
    }

    Ok(mode)
}
