use crate::machine_hiding::os_detection::detect_OS;
use std::fs::{self, Permissions};
use std::io;

#[cfg(any(target_os = "linux", target_os = "macos"))]
use std::os::unix::fs::PermissionsExt;

#[cfg(target_os = "windows")]
use std::os::windows::fs::MetadataExt;

#[derive(Debug)]
pub struct Permission {
    pub readable: bool,
    pub writable: bool,
}

pub fn get_permissions(file_path: &str) -> io::Result<Permission> {
    let os = detect_OS();
    let metadata = fs::metadata(file_path)?;
    match os {
        "linux" | "macos" => {
            #[cfg(any(target_os = "linux", target_os = "macos"))]
            {
                let mode = metadata.permissions().mode();
                Ok(Permission {
                    readable: mode & 0o400 != 0,
                    writable: mode & 0o200 != 0,
                })
            }
            #[cfg(not(any(target_os = "linux", target_os = "macos")))]
            {
                Err(io::Error::new(io::ErrorKind::Other, "Unsupported OS for Unix-like permissions"))
            }
        },
        "windows" => {
            #[cfg(target_os = "windows")]
            {
                let readonly = metadata.permissions().readonly();
                Ok(Permission {
                    readable: true, 
                    writable: !readonly,
                })
            }
            #[cfg(not(target_os = "windows"))]
            {
                Err(io::Error::new(io::ErrorKind::Other, "Unsupported OS for Windows permissions"))
            }
        },
        _ => Err(io::Error::new(io::ErrorKind::Other, "Unsupported OS")),
    }
}

pub fn set_permissions(file_path: &str, permissions: Permission) -> io::Result<()> {
    let os = detect_OS();
    match os{
        "linux" | "macos" => {
            #[cfg(any(target_os = "linux", target_os = "macos"))]
            {
                let mut perms = fs::metadata(file_path)?.permissions();
                let mut mode = perms.mode();

                if permissions.readable {
                    mode |= 0o400;
                } else {
                    mode &= !0o400;
                }

                if permissions.writable {
                    mode |= 0o200;
                } else {
                    mode &= !0o200;
                }

                perms.set_mode(mode);
                fs::set_permissions(file_path, perms)
            }
            #[cfg(not(any(target_os = "linux", target_os = "macos")))]
            {
                Err(io::Error::new(io::ErrorKind::Other, "Unsupported OS for Unix-like permissions"))
            }
        },
        "windows" => {
            let mut perms = fs::metadata(file_path)?.permissions();
            perms.set_readonly(!permissions.writable);
            fs::set_permissions(file_path, perms)
        },
        _ => Err(io::Error::new(io::ErrorKind::Other, "Unsupported OS")),
    }
}

// pub fn set_permissions(file_path: &str, permissions: Permission) -> io::Result<()>{
//     let os=detect_OS();
//     match os {
//         "windows" => {
//             let mut perms = fs::metadata(file_path)?.permissions();
//             perms.set_readonly(!permissions.writable);
//             fs::set_permissions(file_path, perms)
//         },
//         "linux" | "macos" => {
//             let mut perms = fs::metadata(file_path)?.permissions();
//             let mut mode = perms.mode();

//             if permissions.readable {
//                 mode |= 0o400;
//             } else {
//                 mode &= !0o400;
//             }

//             if permissions.writable {
//                 mode |= 0o200;
//             } else {
//                 mode &= !0o200;
//             }

//             perms.set_mode(mode);
//             fs::set_permissions(file_path, perms)
//         },
//         _ => Err(io::Error::new(io::ErrorKind::Other, "Unsupported OS")),
//     }
// }
