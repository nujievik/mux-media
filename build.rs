use std::{
    env,
    fs::{self, File},
    io::{Read, Seek, SeekFrom},
    path::PathBuf,
};

fn main() {
    let target = env::var("TARGET").unwrap_or_default();

    if !(target.starts_with("x86_64-pc-windows") || target.starts_with("i686-pc-windows")) {
        return;
    }

    let mut assets = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    if target.starts_with("x86_64") {
        assets.push("assets\\win64");
    } else {
        assets.push("assets\\win32");
    }

    cp_tool_to_assets(&assets, "mkvmerge");
}

fn cp_tool_to_assets(assets_dir: &PathBuf, tool: &str) {
    let path = assets_dir.join(format!("{}.exe", tool));

    if path.exists() && check_arch(&path).is_ok() {
        dbg!("check arch ok");
        return;
    }

    let panic = |e: String| {
        panic!(
            "Fail check tool '{}': {}. Please download it for current target and copy to '{}'",
            tool,
            e,
            path.display()
        );
    };

    if let Err(e) = try_cp_from_os_path(tool, &path) {
        panic(e);
    }

    if let Err(e) = check_arch(&path) {
        panic(e);
    }
}

fn try_cp_from_os_path(tool: &str, tool_path: &PathBuf) -> Result<(), String> {
    let path =
        which::which(tool).map_err(|e| format!("tool '{}' not found in PATH: {}", e, tool))?;

    let bytes = fs::read(path).map_err(|e| format!("{}", e))?;

    match fs::write(tool_path, bytes) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Fail write tool to out dir: {}", e)),
    }
}

fn check_arch(bin: &PathBuf) -> Result<(), String> {
    let mut file = File::open(bin).map_err(|_| "Can't open")?;

    let mut mz_header = [0u8; 2];
    file.read_exact(&mut mz_header)
        .map_err(|_| "Failed to read MZ header")?;

    if b"MZ" != &mz_header {
        return Err("Not a valid PE executable".into());
    }

    file.seek(SeekFrom::Start(0x3C))
        .map_err(|_| "Seek failed")?;
    let mut pe_offset = [0u8; 4];
    file.read_exact(&mut pe_offset)
        .map_err(|_| "Failed to read PE offset")?;
    let pe_offset = u32::from_le_bytes(pe_offset);

    file.seek(SeekFrom::Start(pe_offset as u64))
        .map_err(|_| "Seek to PE header failed")?;
    let mut pe_signature = [0u8; 4];
    file.read_exact(&mut pe_signature)
        .map_err(|_| "Read PE signature failed")?;

    if b"PE\0\0" != &pe_signature {
        return Err("Invalid PE signature".into());
    }

    let mut machine = [0u8; 2];
    file.read_exact(&mut machine)
        .map_err(|_| "Read machine failed")?;
    let machine = u16::from_le_bytes(machine);

    if expected_machine() != machine {
        return Err("bin architecture mismatch".into());
    }

    Ok(())
}

fn expected_machine() -> u16 {
    match env::var("CARGO_CFG_TARGET_ARCH").as_deref() {
        Ok("x86_64") => 0x8664,
        Ok("x86") => 0x014c,
        arch => panic!("Unknown or unsupported target arch: {:?}", arch),
    }
}
