use zed_extension_api as zed;

struct CSVis {
    // ... state
}

impl zed::Extension for CSVis {
    // ...
}

zed::register_extension!(CSVis);
