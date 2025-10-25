use std::{
    env,
    fs::{self, File},
    io::{Read, Seek, SeekFrom},
    path::PathBuf,
};

fn main() {
    if env::var("CARGO_FEATURE_WITH_EMBEDDED_BINS").is_err() {
        return;
    }

    let target = env::var("TARGET").unwrap_or_default();

    if !target.starts_with("x86_64-pc-windows") {
        return;
    }

    let mut assets = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    assets.push("assets");
    assets.push("win64");

    copy_tool_to_dir(&assets, "ffmpeg");
    copy_tool_to_dir(&assets, "mkvmerge");
}

fn copy_tool_to_dir(dir: &PathBuf, tool: &str) {
    let path = dir.join(format!("{}.exe", tool));

    if path.exists() && verify_arch(&path).is_ok() {
        return;
    }

    let panic = |e: String| {
        panic!(
            "Fail verify tool '{}': {}. Please copy tool for current Rust target to '{}'",
            tool,
            e,
            path.display()
        );
    };

    if let Err(e) = try_copy_from_os_path(tool, &path) {
        panic(e);
    }

    if let Err(e) = verify_arch(&path) {
        panic(e);
    }
}

#[inline(always)]
fn try_copy_from_os_path(tool: &str, tool_path: &PathBuf) -> Result<(), String> {
    let path =
        which::which(tool).map_err(|e| format!("tool '{}' not found in PATH: {}", tool, e))?;

    let bytes = fs::read(path).map_err(|e| format!("{}", e))?;

    match fs::write(tool_path, bytes) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Fail write tool to out dir: {}", e)),
    }
}

fn verify_arch(bin: &PathBuf) -> Result<(), String> {
    let fail = |e: &str| -> String { String::from("Bin verification fail: ") + e };

    let mut file = File::open(bin).map_err(|_| fail("Can't open"))?;

    let mut mz_header = [0u8; 2];
    file.read_exact(&mut mz_header)
        .map_err(|_| fail("Failed to read MZ header"))?;

    if b"MZ" != &mz_header {
        return Err(fail("Not a valid PE executable"));
    }

    file.seek(SeekFrom::Start(0x3C))
        .map_err(|_| fail("Seek failed"))?;

    let mut pe_offset = [0u8; 4];
    file.read_exact(&mut pe_offset)
        .map_err(|_| fail("Failed to read PE offset"))?;

    let pe_offset = u32::from_le_bytes(pe_offset);
    file.seek(SeekFrom::Start(pe_offset as u64))
        .map_err(|_| fail("Seek to PE header failed"))?;

    let mut pe_signature = [0u8; 4];
    file.read_exact(&mut pe_signature)
        .map_err(|_| fail("Read PE signature failed"))?;

    if b"PE\0\0" != &pe_signature {
        return Err(fail("Invalid PE signature"));
    }

    let mut machine = [0u8; 2];
    file.read_exact(&mut machine)
        .map_err(|_| fail("Read machine failed"))?;

    let machine = u16::from_le_bytes(machine);
    let expected = try_expected_machine()?;

    if expected != machine {
        let s = format!(
            "Bin architecture mismatch: expected '{}' found '{}'",
            expected, machine
        );
        return Err(fail(&s));
    }

    Ok(())
}

#[inline(always)]
fn try_expected_machine() -> Result<u16, String> {
    match env::var("CARGO_CFG_TARGET_ARCH").as_deref() {
        Ok("x86_64") => Ok(0x8664),
        arch => Err(format!("Unknown or unsupported target arch: {:?}", arch)),
    }
}
