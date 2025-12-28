use crate::common::*;
use mux_media::*;

#[test]
fn test_from_output() {
    [
        (Container::Matroska, "avi"),
        (Container::Matroska, "mp4"),
        (Container::Matroska, "mkv"),
        (Container::Matroska, "webm"),
        (Container::Matroska, "x"),
        (Container::Matroska, "abc"),
        (Container::Matroska, "rand"),
    ]
    .into_iter()
    .for_each(|(expected, ext)| {
        let s = format!(",.{}", ext);
        let out = Output::try_from_path(data(s)).unwrap();
        assert_eq!(expected, Container::new(&out));
    })
}
