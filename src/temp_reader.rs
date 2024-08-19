use std::{
    fs::{self, File},
    io::{Read, Seek},
};

pub struct TempReader {
    temp_files: Vec<File>,
}

impl TempReader {
    pub fn new() -> anyhow::Result<Self> {
        let mut temp_files = Vec::new();
        for zone in fs::read_dir("/sys/class/thermal")?
            .map(|entry| entry.unwrap())
            .filter(|entry| {
                entry
                    .file_name()
                    .to_str()
                    .unwrap()
                    .starts_with("thermal_zone")
            })
        {
            let zone_type = zone.path().join("type");
            let zone_type = fs::read_to_string(zone_type)?;
            if zone_type.contains("soc_max") || zone_type.contains("mtktscpu") {
                let file = File::open(zone.path().join("temp"))?;
                temp_files.push(file);
                break;
            } else if zone_type.contains("cpu-1-") {
                let file = File::open(zone.path().join("temp"))?;
                temp_files.push(file);
            }
        }

        Ok(Self { temp_files })
    }

    pub fn read_max_temp(&self) -> u32 {
        self.temp_files
            .iter()
            .map(|mut file| {
                file.rewind().unwrap();
                let mut buf = String::new();
                file.read_to_string(&mut buf).unwrap();
                buf
            })
            .map(|temp| temp.trim().parse::<u32>().unwrap())
            .max()
            .unwrap()
    }
}
