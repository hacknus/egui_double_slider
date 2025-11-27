# egui Double Slider Widget

[![crates.io](https://img.shields.io/crates/v/egui_double_slider.svg)](https://crates.io/crates/egui_double_slider)
[![Docs](https://docs.rs/egui_double_slider/badge.svg)](https://docs.rs/egui_double_slider)
[![Rust](https://github.com/hacknus/egui_double_slider/actions/workflows/rust.yml/badge.svg)](https://github.com/hacknus/egui_double_slider/actions/workflows/rust.yml)

A double slider widget for the GUI rust library [egui](https://github.com/emilk/egui).

![screenshot dark](screenshot_dark.png)
![screenshot light](screenshot_light.png)

Add double slider along with `eframe` to your `cargo.toml`.

```toml
eframe = "0.33"
egui_double_slider = "1.0.0"
```

To run the example:
```shell
cargo run --example example
```

Example code:

```rust
use egui;
use egui_double_slider::DoubleSlider;

fn main() {
    egui::__run_test_ui(|ui| {
        ui.add(
            DoubleSlider::new(
                &mut self.slider_val_low_1,
                &mut self.slider_val_high_1,
                10.0..=300.0,
            )
                .width(width)
                .separation_distance(10.0),
        );
    });
}
```
