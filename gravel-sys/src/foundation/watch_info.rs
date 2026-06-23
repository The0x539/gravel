unsafe extern "C" {
    pub fn watch_info_get_model() -> WatchInfoModel;
    pub fn watch_info_get_firmware_version() -> WatchInfoVersion;
    pub fn watch_info_get_color() -> WatchInfoColor;
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct WatchInfoVersion {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

c_enum! {
    pub enum WatchInfoModel;

    UNKNOWN = 0;
    PEBBLE_ORIGINAL = 1;
    PEBBLE_STEEL = 2;
    PEBBLE_TIME = 3;
    PEBBLE_TIME_STEEL = 4;
    PEBBLE_TIME_ROUND_14 = 5;
    PEBBLE_TIME_ROUND_20 = 6;
    PEBBLE_2_HR = 7;
    PEBBLE_2_SE = 8;
    PEBBLE_TIME_2 = 9;
    CORE_P2D = 10;
    CORE_PT2 = 11;
    CORE_PR2 = 12;
}

c_enum! {
    pub enum WatchInfoColor;

    UNKNOWN = 0;
    BLACK = 1;
    WHITE = 2;
    RED = 3;
    ORANGE = 4;
    GRAY = 5;
    STAINLESS_STEEL = 6;
    MATTE_BLACK = 7;
    BLUE = 8;
    GREEN = 9;
    PINK = 10;
    TIME_WHITE = 11;
    TIME_BLACK = 12;
    TIME_RED = 13;
    TIME_STEEL_SILVER = 14;
    TIME_STEEL_BLACK = 15;
    TIME_STEEL_GOLD = 16;
    TIME_ROUND_SILVER_14 = 17;
    TIME_ROUND_BLACK_14 = 18;
    TIME_ROUND_SILVER_20 = 19;
    TIME_ROUND_BLACK_20 = 20;
    TIME_ROUND_ROSE_GOLD_14 = 21;
    PEBBLE_2_HR_BLACK = 22;
    PEBBLE_2_HR_LIME = 23;
    PEBBLE_2_HR_WHITE = 24;
    PEBBLE_2_HR_AQUA = 25;
    PEBBLE_2_SE_BLACK = 26;
    PEBBLE_2_SE_WHITE = 27;
    TIME_2_BLACK = 28;
    TIME_2_SILVER = 29;
    TIME_2_GOLD = 30;
    CORE_P2D_BLACK = 31;
    CORE_P2D_WHITE = 32;
    CORE_PT2_BLACK_GREY = 33;
    CORE_PT2_BLACK_RED = 34;
    CORE_PT2_SILVER_BLUE = 35;
    CORE_PT2_SILVER_GREY = 36;
    CORE_PR2_BLACK_20 = 37;
    CORE_PR2_SILVER_20 = 38;
    CORE_PR2_GOLD_14 = 39;
    CORE_PR2_SILVER_14 = 40;
}
