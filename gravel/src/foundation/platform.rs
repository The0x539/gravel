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

macro_rules! define_bool {
    ($(
        [$define:ident] $($device:ident)*
    )*) => {$(
        pub const fn $define(&self) -> bool {
            matches!(self, $(Self::$device)|*)
        }
    )*};
}

impl PlatformType {
    pub const CURRENT: Self = cfg_select! {
        target_device = "aplite" => Self::Aplite,
        target_device = "basalt" => Self::Basalt,
        target_device = "chalk" => Self::Chalk,
        target_device = "diorite" => Self::Diorite,
        target_device = "emery" => Self::Emery,
        target_device = "flint" => Self::Flint,
        target_device = "gabbro" => Self::Gabbro,
        _ => panic!("No device specified"),
    };

    define_bool! {
        [black_and_white] Aplite              Diorite       Flint
        [color]                  Basalt Chalk         Emery       Gabbro
        [microphone]             Basalt Chalk Diorite Emery Flint Gabbro
        [compass]         Aplite Basalt Chalk         Emery Flint Gabbro
        [health]                 Basalt Chalk Diorite Emery Flint Gabbro
        [rectangular]     Aplite Basalt       Diorite Emery Flint
        [round]                         Chalk                     Gabbro
        [speaker]                                     Emery Flint
        [touch]                                       Emery       Gabbro
    }

    pub const fn display_size(&self) -> (u16, u16) {
        match self {
            Self::Aplite | Self::Basalt | Self::Diorite | Self::Flint => (144, 168),
            Self::Chalk => (180, 180),
            Self::Emery => (200, 228),
            Self::Gabbro => (260, 260),
        }
    }

    pub const fn display_density(&self) -> u16 {
        match self {
            Self::Aplite | Self::Diorite | Self::Flint => 175,
            Self::Basalt | Self::Chalk => 182,
            Self::Emery => 202,
            Self::Gabbro => 200,
        }
    }
}
