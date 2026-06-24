use gravel_sys::graphics::geometry::GSize;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PlatformType {
    /// Pebble
    Aplite,
    /// Pebble Time
    Basalt,
    /// Pebble Time Round
    Chalk,
    /// Pebble 2
    Diorite,
    /// Pebble Time 2
    Emery,
    /// Pebble 2 Duo
    Flint,
    /// Pebble Round 2
    Gabbro,
}

macro_rules! devices {
    ($($name:literal => $variant:ident,)*) => {
        $(
            #[cfg(device_name = $name)]
            pub const CURRENT: Self = Self::$variant;
        )*

        #[cfg(not(any($(device_name = $name),*)))]
        pub const CURRENT: Self = panic!("Unknown device codename");
    };
}

impl PlatformType {
    devices! {
        "aplite"  => Aplite,
        "basalt"  => Basalt,
        "chalk"   => Chalk,
        "diorite" => Diorite,
        "emery"   => Emery,
        "flint"   => Flint,
        "gabbro"  => Gabbro,
    }
}

macro_rules! env_int {
    ($var:literal, $t:ty) => {
        'a: {
            let Some(s) = option_env!("PBL_DISPLAY_WIDTH") else {
                break 'a 0;
            };
            let Ok(n) = <$t>::from_str_radix(s, 10) else {
                break 'a 0;
            };
            n
        }
    };
}

pub const DISPLAY_SIZE: GSize = GSize {
    w: env_int!("PBL_DISPLAY_WIDTH", i16),
    h: env_int!("PBL_DISPLAY_HEIGHT", i16),
};
