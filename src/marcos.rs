
#[macro_export]
macro_rules! mb {
    ($e:expr) => {
        $e / 1_000_000
    };
}

#[macro_export]
macro_rules! gb {
    ($e:expr) => {
        $e / 1_000_000_000
    };
}

#[macro_export]
macro_rules! mb_or_gb {
    ($arg:ident, $e:expr) => {
        if $arg.in_gb {
            $e / 1_000_000_000
        } else {
            $e / 1_000_000
        }
    };
}

#[macro_export]
macro_rules! mb_or_gb_label {
    ($arg:ident) => {
        if $arg.in_gb {
            "GB"
        } else {
            "MB"
        }
    };
}