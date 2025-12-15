use zed_extension_api::{self as zed};

struct RedExtension {}

impl zed::Extension for RedExtension {
    fn new() -> Self {
        Self {}
    }
}

zed::register_extension!(RedExtension);
