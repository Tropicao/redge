use std::path::{Path, PathBuf};

pub struct Device {
    block_device: PathBuf,
    mount_point: PathBuf,
}

fn get_block_device_from_label(label: String) -> Option<Box<PathBuf>> {
    let search = Path::new(&label);
    match search.exists() {
        true => Some(Box::new(
            search.canonicalize().expect("Can not read symlink"),
        )),
        false => None,
    }
}

fn get_mount_point_from_block_device(path: &Path) -> Result<Box<PathBuf>, String> {
    match block_utils::get_mountpoint(Path::new(&path)) {
        Ok(option) => match option {
            Some(mount_point) => Ok(Box::new(mount_point)),
            None => Err(format!(
                "Can not find mount point for {}",
                path.to_str().unwrap()
            )),
        },
        Err(_e) => Err(format!(
            "Can not find mount point for {}",
            path.to_str().unwrap()
        )),
    }
}

impl Device {
    pub fn new() -> Result<Self, &'static str> {
        match get_block_device_from_label(String::from("/dev/disk/by-label/GARMIN")) {
            Some(block_device) => {
                let detected_mount_point =
                    get_mount_point_from_block_device(&block_device).unwrap();
                Ok(Device {
                    block_device: block_device.to_path_buf(),
                    mount_point: detected_mount_point.to_path_buf(),
                })
            }
            None => Err("No device found"),
        }
    }

    pub fn block_device(&self) -> String {
        String::from(self.block_device.to_str().unwrap())
    }

    pub fn mount_point(&self) -> String {
        String::from(self.mount_point.to_str().unwrap())
    }

    fn get_all_activities(&self) -> Vec<PathBuf> {
        let mut activities_path = PathBuf::new();
        activities_path.push(&self.mount_point);
        activities_path.push("Garmin/Activities");
        match activities_path.read_dir() {
            Ok(list) => {
                let mut result = list
                    .map(|entry| entry.unwrap().path())
                    .collect::<Vec<PathBuf>>();
                result.reverse();
                result
            }
            Err(_e) => vec![],
        }
    }

    pub fn get_activities(&self) -> Vec<PathBuf> {
        self.get_all_activities()
    }

    pub fn get_last_activity(&self) -> PathBuf {
        self.get_all_activities().first().unwrap().to_owned()
    }
}
