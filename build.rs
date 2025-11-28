use std::{env::var, error::Error, fs, io, path};
use strum_macros::AsRefStr;

const APPLE_64: &str = "https://evermeet.cx/pub/ffmpeg/ffmpeg-8.0.1.zip";
const GIT_FFMPEG_BUILDS: &str =
    "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-n8.0-latest";

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() {
    if var("CARGO_FEATURE_STATIC").is_err() {
        return;
    }

    let bin = FfmpegBin::new();

    if let Err(e) = bin.download_and_extract() {
        panic!("Fail download and extract ffmpeg bin: {}", e);
    }

    let dir = bin.to_target_dir();
    println!("cargo:rustc-env=TARGET_DIR={dir}");
}

#[derive(AsRefStr)]
#[strum(serialize_all = "kebab-case")]
enum FfmpegBin {
    Apple64,
    Linux64,
    Win64,
}

impl FfmpegBin {
    fn new() -> FfmpegBin {
        match var("TARGET").unwrap_or_default().as_str() {
            t if t.starts_with("x86_64-apple-darwin") => Self::Apple64,
            t if t.starts_with("x86_64-unknown-linux") => Self::Linux64,
            t if t.starts_with("x86_64-pc-windows") => Self::Win64,
            t => panic!("Unsupported static target: {t}"),
        }
    }

    fn download_and_extract(&self) -> Result<()> {
        let dir = self.to_target_dir();
        if !fs::exists(&dir)? {
            let _ = fs::create_dir(&dir);
        }

        let archive = self.to_archive();
        if !fs::exists(&archive)? {
            self.download()?;
        }

        if self.to_download_url().ends_with(".tar.xz") {
            self.extract_tar_xz(&archive)?
        } else {
            extract_zip(&archive, &dir)?
        }

        self.ensure_ffmpeg_in_path()
    }

    fn download(&self) -> Result<()> {
        let url = self.to_download_url();
        let mut resp = reqwest::blocking::get(&url)?;
        let mut out = fs::File::create(self.to_archive())?;
        resp.copy_to(&mut out)?;
        Ok(())
    }

    fn extract_tar_xz(&self, fname: &str) -> Result<()> {
        let file = fs::File::open(fname)?;
        let decompressor = xz2::read::XzDecoder::new(file);
        let mut archive = tar::Archive::new(decompressor);
        archive.unpack(self.to_target_dir())?;
        Ok(())
    }

    fn ensure_ffmpeg_in_path(&self) -> Result<()> {
        let path = self.to_path();

        if fs::exists(&path)? {
            return Ok(());
        }

        let bin = find_ffmpeg_in_dir(&self.to_target_dir())
            .ok_or("Cannot find ffmpeg binary in extracted archive")?;

        fs::copy(bin, path)?;
        Ok(())
    }

    fn to_download_url(&self) -> String {
        match self {
            Self::Apple64 => APPLE_64.into(),
            Self::Linux64 => format!("{}-linux64-gpl-8.0.tar.xz", GIT_FFMPEG_BUILDS),
            Self::Win64 => format!("{}-win64-gpl-8.0.zip", GIT_FFMPEG_BUILDS),
        }
    }

    fn to_archive(&self) -> String {
        format!("{}/archive", self.to_target_dir())
    }

    fn to_target_dir(&self) -> String {
        format!("{}/assets/{}", env!("CARGO_MANIFEST_DIR"), self.as_ref())
    }

    fn to_path(&self) -> String {
        format!("{}/ffmpeg", self.to_target_dir())
    }
}

fn extract_zip(fname: &str, out_dir: &str) -> Result<()> {
    let file = fs::File::open(fname)?;
    let mut archive = zip::ZipArchive::new(file)?;
    let out_dir = path::Path::new(out_dir);

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;

        let out_path = match file.enclosed_name() {
            Some(path) => out_dir.join(path),
            None => continue,
        };

        if file.is_dir() {
            fs::create_dir_all(&out_path)?;
        } else {
            if let Some(p) = out_path.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)?;
                }
            }
            let mut outfile = fs::File::create(&out_path)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }
    Ok(())
}

fn find_ffmpeg_in_dir(dir: &str) -> Option<path::PathBuf> {
    for entry in walkdir::WalkDir::new(dir) {
        let entry = entry.ok()?;
        if entry.file_type().is_file() {
            let name = entry.file_name().to_string_lossy().to_lowercase();
            if name == "ffmpeg" || name == "ffmpeg.exe" {
                return Some(entry.path().into());
            }
        }
    }
    None
}
