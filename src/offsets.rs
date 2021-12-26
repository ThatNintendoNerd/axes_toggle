use skyline::hooks::{getRegionAddress, Region};

// Default 13.0.1 offsets
pub static mut COORDINATE_AXES_PHOTO_OFFSET: usize = 0x5109dc;
pub static mut COORDINATE_AXES_REPLAY_OFFSET: usize = 0x135d438;

static COORDINATE_AXES_PHOTO_SEARCH_CODE: &[u8] = &[
    0x68, 0xc2, 0x51, 0x39,
    0x88, 0x0f, 0x00, 0x34,
];

static COORDINATE_AXES_REPLAY_SEARCH_CODE: &[u8] = &[
    0x88, 0x02, 0x40, 0xf9,
    0xe8, 0x2b, 0x40, 0xfd,
];

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

macro_rules! find_offsets {
    (
        $(
            ($out_variable:expr, $search_pattern:expr)
        ),*
        $(,)?
    ) => {
        $(
            unsafe {
                let text_ptr = getRegionAddress(Region::Text) as *const u8;
                let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
                let text = std::slice::from_raw_parts(text_ptr, text_size);

                if let Some(offset) = find_subsequence(text, $search_pattern) {
                    $out_variable = offset
                } else {
                    println!("Error: no offset found for '{}'. Defaulting to 13.0.1 offset. This most likely won't work.", stringify!($out_variable));
                }
            }
        )*
    };
}

pub fn search_offsets() {
    find_offsets!(
        (COORDINATE_AXES_PHOTO_OFFSET, COORDINATE_AXES_PHOTO_SEARCH_CODE), 
        (COORDINATE_AXES_REPLAY_OFFSET, COORDINATE_AXES_REPLAY_SEARCH_CODE)
    );
}
