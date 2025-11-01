use super::{FontAttachs, OtherAttachs};

to_json_args!(@tracks_or_attachs, FontAttachs, Fonts, NoFonts);
to_json_args!(@tracks_or_attachs, OtherAttachs, Attachs, NoAttachs);

/*
#[inline(always)]
fn shortest_track_of_nums(mut nums: BTreeSet<u64>, cnt: usize, cnt_init: usize) -> String {
    let inverse = cnt > (cnt_init / 2);

    if inverse {
        nums = (1..=cnt_init)
            .filter_map(|num| {
                let num = num as u64;
                (!nums.contains(&num)).then(|| num)
            })
            .collect();
    }

    let mut nums: String = nums
        .into_iter()
        .map(|aid| aid.to_string())
        .collect::<Vec<_>>()
        .join(",");

    if inverse {
        nums.insert(0, '!');
    }

    nums
}
*/
