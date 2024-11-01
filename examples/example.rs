use eframe::{App, NativeOptions};
use egui::{Context, Window};
use egui_double_slider::DoubleSlider;

fn main() {
    let options = NativeOptions::default();
    eframe::run_native(
        "Interactive Double Slider",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
    .expect("GUI failed")
}

pub struct MyApp {
    slider_val_low_1: f32,
    slider_val_high_1: f32,
    slider_val_low_2: f32,
    slider_val_high_2: f32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            slider_val_low_1: 30.0,
            slider_val_high_1: 200.0,
            slider_val_low_2: 10.0,
            slider_val_high_2: 150.0,
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        Window::new("Interactive Double Slider").show(ctx, |ui| {
            let width = ui.available_width();

            // Display slider, linked to the same range as the plot
            ui.add(
                DoubleSlider::new(
                    &mut self.slider_val_low_1,
                    &mut self.slider_val_high_1,
                    10.0..=300.0,
                )
                .width(width)
                .separation_distance(10.0),
            );

            ui.label(format!("Lower Bound: {:.2}", self.slider_val_low_1));
            ui.label(format!("Upper Bound: {:.2}", self.slider_val_high_1));
            ui.add(
                DoubleSlider::new(
                    &mut self.slider_val_low_2,
                    &mut self.slider_val_high_2,
                    10.0..=300.0,
                )
                .width(width)
                .separation_distance(10.0)
                .invert_highlighting(true),
            );
            ui.label(format!("Lower Bound: {:.2}", self.slider_val_low_2));
            ui.label(format!("Upper Bound: {:.2}", self.slider_val_high_2));
        });
    }
}
