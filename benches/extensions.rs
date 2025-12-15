#![feature(test)]

extern crate test;

use test::{Bencher, black_box};

#[bench]
fn phf_set_contains(b: &mut Bencher) {
    static SET: phf::Set<&[u8]> = phf::phf_set! {
        b"264", b"AVI", b"DTS", b"DTSHD", b"FLAC", b"SRT", b"SSA", b"WEBM"
    };
    let inputs = black_box(["264", "AVI", "DTS", "DTSHD", "FLAC", "SRT", "SSA", "WEBM"]);

    b.iter(|| {
        black_box({
            let mut count = 0;
            for ext in inputs {
                if SET.contains(ext.as_bytes()) {
                    count += 1;
                }
            }
            assert!(count != 0);
            black_box(count);
        })
    });
}

#[bench]
fn make_lowercase_and_match_webm(b: &mut Bencher) {
    let inputs = black_box(["264", "AVI", "DTS", "DTSHD", "FLAC", "SRT", "SSA", "WEBM"]);
    let mut buf = black_box([0u8; 6]);

    b.iter(|| {
        black_box({
            let mut count = 0;
            for ext in inputs {
                let len = ext.len();
                if !matches!(len, 4) {
                    continue;
                }
                for (dst, src) in buf[..len].iter_mut().zip(ext.as_bytes().iter()) {
                    *dst = src.to_ascii_lowercase();
                }
                let matched = match &buf[..len] {
                    b"webm" => true,
                    _ => false,
                };
                if matched {
                    count += 1;
                }
            }
            assert!(count != 0);
            black_box(count);
        })
    });
}

#[bench]
fn make_lowercase_and_match_subs(b: &mut Bencher) {
    let inputs = black_box(["264", "AVI", "DTS", "DTSHD", "FLAC", "SRT", "SSA", "WEBM"]);
    let mut buf = black_box([0u8; 6]);

    b.iter(|| {
        black_box({
            let mut count = 0;
            for ext in inputs {
                let len = ext.len();
                if !matches!(len, 3) {
                    continue;
                }
                for (dst, src) in buf[..len].iter_mut().zip(ext.as_bytes().iter()) {
                    *dst = src.to_ascii_lowercase();
                }
                let matched = match &buf[..len] {
                    b"ass" | b"mks" | b"srt" | b"ssa" | b"sub" | b"sup" | b"vtt" => true,
                    _ => false,
                };
                if matched {
                    count += 1;
                }
            }
            assert!(count != 0);
            black_box(count);
        })
    });
}

#[bench]
fn make_lowercase_and_match_media(b: &mut Bencher) {
    let inputs = black_box(["264", "AVI", "DTS", "DTSHD", "FLAC", "SRT", "SSA", "WEBM"]);
    let mut buf = black_box([0u8; 6]);

    b.iter(|| {
        black_box({
            let mut count = 0;

            for ext in inputs {
                let len = ext.len();
                if !matches!(len, 2 | 3 | 4 | 5 | 6) {
                    continue;
                }

                for (dst, src) in buf[..len].iter_mut().zip(ext.as_bytes().iter()) {
                    *dst = src.to_ascii_lowercase();
                }

                let matched = match &buf[..len] {
                    b"264" => true,
                    b"265" => true,
                    b"3gp" => true,
                    b"aac" => true,
                    b"ac3" => true,
                    b"ass" => true,
                    b"av1" => true,
                    b"avc" => true,
                    b"avi" => true,
                    b"caf" => true,
                    b"dts" => true,
                    b"dtshd" => true,
                    b"eac3" => true,
                    b"ec3" => true,
                    b"f4v" => true,
                    b"flac" => true,
                    b"flv" => true,
                    b"h264" => true,
                    b"h265" => true,
                    b"hevc" => true,
                    b"ivf" => true,
                    b"m2ts" => true,
                    b"m2v" => true,
                    b"m4a" => true,
                    b"m4v" => true,
                    b"mka" => true,
                    b"mks" => true,
                    b"mkv" => true,
                    b"mlp" => true,
                    b"mov" => true,
                    b"mp2" => true,
                    b"mp3" => true,
                    b"mp4" => true,
                    b"mpa" => true,
                    b"mpeg" => true,
                    b"mpg" => true,
                    b"mpv" => true,
                    b"obu" => true,
                    b"ogg" => true,
                    b"ogm" => true,
                    b"ogv" => true,
                    b"opus" => true,
                    b"ra" => true,
                    b"srt" => true,
                    b"ssa" => true,
                    b"sub" => true,
                    b"sup" => true,
                    b"thd" => true,
                    b"truehd" => true,
                    b"ts" => true,
                    b"tta" => true,
                    b"vc1" => true,
                    b"vtt" => true,
                    b"wav" => true,
                    b"weba" => true,
                    b"webm" => true,
                    b"webma" => true,
                    b"wma" => true,
                    b"wmv" => true,
                    b"x264" => true,
                    b"x265" => true,
                    _ => false,
                };

                if matched {
                    count += 1;
                }
            }
            assert!(count != 0);
            black_box(count);
        })
    });
}

#[bench]
fn match_webm(b: &mut Bencher) {
    let inputs = black_box(["264", "AVI", "DTS", "DTSHD", "FLAC", "SRT", "SSA", "WEBM"]);
    b.iter(|| {
        black_box({
            let mut count = 0;
            for ext in inputs {
                let matched = match ext.as_bytes() {
                    b"WEBM" => true,
                    b"WEBm" => true,
                    b"WEbM" => true,
                    b"WEbm" => true,
                    b"WeBM" => true,
                    b"WeBm" => true,
                    b"WebM" => true,
                    b"Webm" => true,
                    b"wEBM" => true,
                    b"wEBm" => true,
                    b"wEbM" => true,
                    b"wEbm" => true,
                    b"weBM" => true,
                    b"weBm" => true,
                    b"webM" => true,
                    b"webm" => true,
                    _ => false,
                };

                if matched {
                    count += 1;
                }
            }
            assert!(count != 0);
            black_box(count);
        })
    });
}

#[bench]
fn match_subs(b: &mut Bencher) {
    let inputs = black_box(["264", "AVI", "DTS", "DTSHD", "FLAC", "SRT", "SSA", "WEBM"]);
    b.iter(|| {
        black_box({
            let mut count = 0;
            for ext in inputs {
                let matched = match ext.as_bytes() {
                    b"ASS" => true,
                    b"ASs" => true,
                    b"AsS" => true,
                    b"Ass" => true,
                    b"aSS" => true,
                    b"aSs" => true,
                    b"asS" => true,
                    b"ass" => true,
                    b"MKS" => true,
                    b"MKs" => true,
                    b"MkS" => true,
                    b"Mks" => true,
                    b"mKS" => true,
                    b"mKs" => true,
                    b"mkS" => true,
                    b"mks" => true,
                    b"SRT" => true,
                    b"SRt" => true,
                    b"SrT" => true,
                    b"Srt" => true,
                    b"sRT" => true,
                    b"sRt" => true,
                    b"srT" => true,
                    b"srt" => true,
                    b"SSA" => true,
                    b"SSa" => true,
                    b"SsA" => true,
                    b"Ssa" => true,
                    b"sSA" => true,
                    b"sSa" => true,
                    b"ssA" => true,
                    b"ssa" => true,
                    b"SUB" => true,
                    b"SUb" => true,
                    b"SuB" => true,
                    b"Sub" => true,
                    b"sUB" => true,
                    b"sUb" => true,
                    b"suB" => true,
                    b"sub" => true,
                    b"SUP" => true,
                    b"SUp" => true,
                    b"SuP" => true,
                    b"Sup" => true,
                    b"sUP" => true,
                    b"sUp" => true,
                    b"suP" => true,
                    b"sup" => true,
                    b"VTT" => true,
                    b"VTt" => true,
                    b"VtT" => true,
                    b"Vtt" => true,
                    b"vTT" => true,
                    b"vTt" => true,
                    b"vtT" => true,
                    b"vtt" => true,
                    _ => false,
                };

                if matched {
                    count += 1;
                }
            }
            assert!(count != 0);
            black_box(count);
        })
    });
}

#[bench]
fn match_media(b: &mut Bencher) {
    let inputs = black_box(["264", "AVI", "DTS", "DTSHD", "FLAC", "SRT", "SSA", "WEBM"]);
    b.iter(|| {
        black_box({
            let mut count = 0;
            for ext in inputs {
                let matched = match ext.as_bytes() {
                    b"264" => true,
                    b"265" => true,
                    b"3GP" => true,
                    b"3Gp" => true,
                    b"3gP" => true,
                    b"3gp" => true,
                    b"AAC" => true,
                    b"AAc" => true,
                    b"AaC" => true,
                    b"Aac" => true,
                    b"aAC" => true,
                    b"aAc" => true,
                    b"aaC" => true,
                    b"aac" => true,
                    b"AC3" => true,
                    b"Ac3" => true,
                    b"aC3" => true,
                    b"ac3" => true,
                    b"ASS" => true,
                    b"ASs" => true,
                    b"AsS" => true,
                    b"Ass" => true,
                    b"aSS" => true,
                    b"aSs" => true,
                    b"asS" => true,
                    b"ass" => true,
                    b"AV1" => true,
                    b"Av1" => true,
                    b"aV1" => true,
                    b"av1" => true,
                    b"AVC" => true,
                    b"AVc" => true,
                    b"AvC" => true,
                    b"Avc" => true,
                    b"aVC" => true,
                    b"aVc" => true,
                    b"avC" => true,
                    b"avc" => true,
                    b"AVI" => true,
                    b"AVi" => true,
                    b"AvI" => true,
                    b"Avi" => true,
                    b"aVI" => true,
                    b"aVi" => true,
                    b"avI" => true,
                    b"avi" => true,
                    b"CAF" => true,
                    b"CAf" => true,
                    b"CaF" => true,
                    b"Caf" => true,
                    b"cAF" => true,
                    b"cAf" => true,
                    b"caF" => true,
                    b"caf" => true,
                    b"DTS" => true,
                    b"DTs" => true,
                    b"DtS" => true,
                    b"Dts" => true,
                    b"dTS" => true,
                    b"dTs" => true,
                    b"dtS" => true,
                    b"dts" => true,
                    b"DTSHD" => true,
                    b"DTSHd" => true,
                    b"DTShD" => true,
                    b"DTShd" => true,
                    b"DTsHD" => true,
                    b"DTsHd" => true,
                    b"DTshD" => true,
                    b"DTshd" => true,
                    b"DtSHD" => true,
                    b"DtSHd" => true,
                    b"DtShD" => true,
                    b"DtShd" => true,
                    b"DtsHD" => true,
                    b"DtsHd" => true,
                    b"DtshD" => true,
                    b"Dtshd" => true,
                    b"dTSHD" => true,
                    b"dTSHd" => true,
                    b"dTShD" => true,
                    b"dTShd" => true,
                    b"dTsHD" => true,
                    b"dTsHd" => true,
                    b"dTshD" => true,
                    b"dTshd" => true,
                    b"dtSHD" => true,
                    b"dtSHd" => true,
                    b"dtShD" => true,
                    b"dtShd" => true,
                    b"dtsHD" => true,
                    b"dtsHd" => true,
                    b"dtshD" => true,
                    b"dtshd" => true,
                    b"EAC3" => true,
                    b"EAc3" => true,
                    b"EaC3" => true,
                    b"Eac3" => true,
                    b"eAC3" => true,
                    b"eAc3" => true,
                    b"eaC3" => true,
                    b"eac3" => true,
                    b"EC3" => true,
                    b"Ec3" => true,
                    b"eC3" => true,
                    b"ec3" => true,
                    b"F4V" => true,
                    b"F4v" => true,
                    b"f4V" => true,
                    b"f4v" => true,
                    b"FLAC" => true,
                    b"FLAc" => true,
                    b"FLaC" => true,
                    b"FLac" => true,
                    b"FlAC" => true,
                    b"FlAc" => true,
                    b"FlaC" => true,
                    b"Flac" => true,
                    b"fLAC" => true,
                    b"fLAc" => true,
                    b"fLaC" => true,
                    b"fLac" => true,
                    b"flAC" => true,
                    b"flAc" => true,
                    b"flaC" => true,
                    b"flac" => true,
                    b"FLV" => true,
                    b"FLv" => true,
                    b"FlV" => true,
                    b"Flv" => true,
                    b"fLV" => true,
                    b"fLv" => true,
                    b"flV" => true,
                    b"flv" => true,
                    b"H264" => true,
                    b"h264" => true,
                    b"H265" => true,
                    b"h265" => true,
                    b"HEVC" => true,
                    b"HEVc" => true,
                    b"HEvC" => true,
                    b"HEvc" => true,
                    b"HeVC" => true,
                    b"HeVc" => true,
                    b"HevC" => true,
                    b"Hevc" => true,
                    b"hEVC" => true,
                    b"hEVc" => true,
                    b"hEvC" => true,
                    b"hEvc" => true,
                    b"heVC" => true,
                    b"heVc" => true,
                    b"hevC" => true,
                    b"hevc" => true,
                    b"IVF" => true,
                    b"IVf" => true,
                    b"IvF" => true,
                    b"Ivf" => true,
                    b"iVF" => true,
                    b"iVf" => true,
                    b"ivF" => true,
                    b"ivf" => true,
                    b"M2TS" => true,
                    b"M2Ts" => true,
                    b"M2tS" => true,
                    b"M2ts" => true,
                    b"m2TS" => true,
                    b"m2Ts" => true,
                    b"m2tS" => true,
                    b"m2ts" => true,
                    b"M2V" => true,
                    b"M2v" => true,
                    b"m2V" => true,
                    b"m2v" => true,
                    b"M4A" => true,
                    b"M4a" => true,
                    b"m4A" => true,
                    b"m4a" => true,
                    b"M4V" => true,
                    b"M4v" => true,
                    b"m4V" => true,
                    b"m4v" => true,
                    b"MKA" => true,
                    b"MKa" => true,
                    b"MkA" => true,
                    b"Mka" => true,
                    b"mKA" => true,
                    b"mKa" => true,
                    b"mkA" => true,
                    b"mka" => true,
                    b"MKS" => true,
                    b"MKs" => true,
                    b"MkS" => true,
                    b"Mks" => true,
                    b"mKS" => true,
                    b"mKs" => true,
                    b"mkS" => true,
                    b"mks" => true,
                    b"MKV" => true,
                    b"MKv" => true,
                    b"MkV" => true,
                    b"Mkv" => true,
                    b"mKV" => true,
                    b"mKv" => true,
                    b"mkV" => true,
                    b"mkv" => true,
                    b"MLP" => true,
                    b"MLp" => true,
                    b"MlP" => true,
                    b"Mlp" => true,
                    b"mLP" => true,
                    b"mLp" => true,
                    b"mlP" => true,
                    b"mlp" => true,
                    b"MOV" => true,
                    b"MOv" => true,
                    b"MoV" => true,
                    b"Mov" => true,
                    b"mOV" => true,
                    b"mOv" => true,
                    b"moV" => true,
                    b"mov" => true,
                    b"MP2" => true,
                    b"Mp2" => true,
                    b"mP2" => true,
                    b"mp2" => true,
                    b"MP3" => true,
                    b"Mp3" => true,
                    b"mP3" => true,
                    b"mp3" => true,
                    b"MP4" => true,
                    b"Mp4" => true,
                    b"mP4" => true,
                    b"mp4" => true,
                    b"MPA" => true,
                    b"MPa" => true,
                    b"MpA" => true,
                    b"Mpa" => true,
                    b"mPA" => true,
                    b"mPa" => true,
                    b"mpA" => true,
                    b"mpa" => true,
                    b"MPEG" => true,
                    b"MPEg" => true,
                    b"MPeG" => true,
                    b"MPeg" => true,
                    b"MpEG" => true,
                    b"MpEg" => true,
                    b"MpeG" => true,
                    b"Mpeg" => true,
                    b"mPEG" => true,
                    b"mPEg" => true,
                    b"mPeG" => true,
                    b"mPeg" => true,
                    b"mpEG" => true,
                    b"mpEg" => true,
                    b"mpeG" => true,
                    b"mpeg" => true,
                    b"MPG" => true,
                    b"MPg" => true,
                    b"MpG" => true,
                    b"Mpg" => true,
                    b"mPG" => true,
                    b"mPg" => true,
                    b"mpG" => true,
                    b"mpg" => true,
                    b"MPV" => true,
                    b"MPv" => true,
                    b"MpV" => true,
                    b"Mpv" => true,
                    b"mPV" => true,
                    b"mPv" => true,
                    b"mpV" => true,
                    b"mpv" => true,
                    b"OBU" => true,
                    b"OBu" => true,
                    b"ObU" => true,
                    b"Obu" => true,
                    b"oBU" => true,
                    b"oBu" => true,
                    b"obU" => true,
                    b"obu" => true,
                    b"OGG" => true,
                    b"OGg" => true,
                    b"OgG" => true,
                    b"Ogg" => true,
                    b"oGG" => true,
                    b"oGg" => true,
                    b"ogG" => true,
                    b"ogg" => true,
                    b"OGM" => true,
                    b"OGm" => true,
                    b"OgM" => true,
                    b"Ogm" => true,
                    b"oGM" => true,
                    b"oGm" => true,
                    b"ogM" => true,
                    b"ogm" => true,
                    b"OGV" => true,
                    b"OGv" => true,
                    b"OgV" => true,
                    b"Ogv" => true,
                    b"oGV" => true,
                    b"oGv" => true,
                    b"ogV" => true,
                    b"ogv" => true,
                    b"OPUS" => true,
                    b"OPUs" => true,
                    b"OPuS" => true,
                    b"OPus" => true,
                    b"OpUS" => true,
                    b"OpUs" => true,
                    b"OpuS" => true,
                    b"Opus" => true,
                    b"oPUS" => true,
                    b"oPUs" => true,
                    b"oPuS" => true,
                    b"oPus" => true,
                    b"opUS" => true,
                    b"opUs" => true,
                    b"opuS" => true,
                    b"opus" => true,
                    b"RA" => true,
                    b"Ra" => true,
                    b"rA" => true,
                    b"ra" => true,
                    b"SRT" => true,
                    b"SRt" => true,
                    b"SrT" => true,
                    b"Srt" => true,
                    b"sRT" => true,
                    b"sRt" => true,
                    b"srT" => true,
                    b"srt" => true,
                    b"SSA" => true,
                    b"SSa" => true,
                    b"SsA" => true,
                    b"Ssa" => true,
                    b"sSA" => true,
                    b"sSa" => true,
                    b"ssA" => true,
                    b"ssa" => true,
                    b"SUB" => true,
                    b"SUb" => true,
                    b"SuB" => true,
                    b"Sub" => true,
                    b"sUB" => true,
                    b"sUb" => true,
                    b"suB" => true,
                    b"sub" => true,
                    b"SUP" => true,
                    b"SUp" => true,
                    b"SuP" => true,
                    b"Sup" => true,
                    b"sUP" => true,
                    b"sUp" => true,
                    b"suP" => true,
                    b"sup" => true,
                    b"THD" => true,
                    b"THd" => true,
                    b"ThD" => true,
                    b"Thd" => true,
                    b"tHD" => true,
                    b"tHd" => true,
                    b"thD" => true,
                    b"thd" => true,
                    b"TRUEHD" => true,
                    b"TRUEHd" => true,
                    b"TRUEhD" => true,
                    b"TRUEhd" => true,
                    b"TRUeHD" => true,
                    b"TRUeHd" => true,
                    b"TRUehD" => true,
                    b"TRUehd" => true,
                    b"TRuEHD" => true,
                    b"TRuEHd" => true,
                    b"TRuEhD" => true,
                    b"TRuEhd" => true,
                    b"TRueHD" => true,
                    b"TRueHd" => true,
                    b"TRuehD" => true,
                    b"TRuehd" => true,
                    b"TrUEHD" => true,
                    b"TrUEHd" => true,
                    b"TrUEhD" => true,
                    b"TrUEhd" => true,
                    b"TrUeHD" => true,
                    b"TrUeHd" => true,
                    b"TrUehD" => true,
                    b"TrUehd" => true,
                    b"TruEHD" => true,
                    b"TruEHd" => true,
                    b"TruEhD" => true,
                    b"TruEhd" => true,
                    b"TrueHD" => true,
                    b"TrueHd" => true,
                    b"TruehD" => true,
                    b"Truehd" => true,
                    b"tRUEHD" => true,
                    b"tRUEHd" => true,
                    b"tRUEhD" => true,
                    b"tRUEhd" => true,
                    b"tRUeHD" => true,
                    b"tRUeHd" => true,
                    b"tRUehD" => true,
                    b"tRUehd" => true,
                    b"tRuEHD" => true,
                    b"tRuEHd" => true,
                    b"tRuEhD" => true,
                    b"tRuEhd" => true,
                    b"tRueHD" => true,
                    b"tRueHd" => true,
                    b"tRuehD" => true,
                    b"tRuehd" => true,
                    b"trUEHD" => true,
                    b"trUEHd" => true,
                    b"trUEhD" => true,
                    b"trUEhd" => true,
                    b"trUeHD" => true,
                    b"trUeHd" => true,
                    b"trUehD" => true,
                    b"trUehd" => true,
                    b"truEHD" => true,
                    b"truEHd" => true,
                    b"truEhD" => true,
                    b"truEhd" => true,
                    b"trueHD" => true,
                    b"trueHd" => true,
                    b"truehD" => true,
                    b"truehd" => true,
                    b"TS" => true,
                    b"Ts" => true,
                    b"tS" => true,
                    b"ts" => true,
                    b"TTA" => true,
                    b"TTa" => true,
                    b"TtA" => true,
                    b"Tta" => true,
                    b"tTA" => true,
                    b"tTa" => true,
                    b"ttA" => true,
                    b"tta" => true,
                    b"VC1" => true,
                    b"Vc1" => true,
                    b"vC1" => true,
                    b"vc1" => true,
                    b"VTT" => true,
                    b"VTt" => true,
                    b"VtT" => true,
                    b"Vtt" => true,
                    b"vTT" => true,
                    b"vTt" => true,
                    b"vtT" => true,
                    b"vtt" => true,
                    b"WAV" => true,
                    b"WAv" => true,
                    b"WaV" => true,
                    b"Wav" => true,
                    b"wAV" => true,
                    b"wAv" => true,
                    b"waV" => true,
                    b"wav" => true,
                    b"WEBA" => true,
                    b"WEBa" => true,
                    b"WEbA" => true,
                    b"WEba" => true,
                    b"WeBA" => true,
                    b"WeBa" => true,
                    b"WebA" => true,
                    b"Weba" => true,
                    b"wEBA" => true,
                    b"wEBa" => true,
                    b"wEbA" => true,
                    b"wEba" => true,
                    b"weBA" => true,
                    b"weBa" => true,
                    b"webA" => true,
                    b"weba" => true,
                    b"WEBM" => true,
                    b"WEBm" => true,
                    b"WEbM" => true,
                    b"WEbm" => true,
                    b"WeBM" => true,
                    b"WeBm" => true,
                    b"WebM" => true,
                    b"Webm" => true,
                    b"wEBM" => true,
                    b"wEBm" => true,
                    b"wEbM" => true,
                    b"wEbm" => true,
                    b"weBM" => true,
                    b"weBm" => true,
                    b"webM" => true,
                    b"webm" => true,
                    b"WEBMA" => true,
                    b"WEBMa" => true,
                    b"WEBmA" => true,
                    b"WEBma" => true,
                    b"WEbMA" => true,
                    b"WEbMa" => true,
                    b"WEbmA" => true,
                    b"WEbma" => true,
                    b"WeBMA" => true,
                    b"WeBMa" => true,
                    b"WeBmA" => true,
                    b"WeBma" => true,
                    b"WebMA" => true,
                    b"WebMa" => true,
                    b"WebmA" => true,
                    b"Webma" => true,
                    b"wEBMA" => true,
                    b"wEBMa" => true,
                    b"wEBmA" => true,
                    b"wEBma" => true,
                    b"wEbMA" => true,
                    b"wEbMa" => true,
                    b"wEbmA" => true,
                    b"wEbma" => true,
                    b"weBMA" => true,
                    b"weBMa" => true,
                    b"weBmA" => true,
                    b"weBma" => true,
                    b"webMA" => true,
                    b"webMa" => true,
                    b"webmA" => true,
                    b"webma" => true,
                    b"WMA" => true,
                    b"WMa" => true,
                    b"WmA" => true,
                    b"Wma" => true,
                    b"wMA" => true,
                    b"wMa" => true,
                    b"wmA" => true,
                    b"wma" => true,
                    b"WMV" => true,
                    b"WMv" => true,
                    b"WmV" => true,
                    b"Wmv" => true,
                    b"wMV" => true,
                    b"wMv" => true,
                    b"wmV" => true,
                    b"wmv" => true,
                    b"X264" => true,
                    b"x264" => true,
                    b"X265" => true,
                    b"x265" => true,
                    _ => false,
                };

                if matched {
                    count += 1;
                }
            }
            assert!(count != 0);
            black_box(count);
        })
    });
}
