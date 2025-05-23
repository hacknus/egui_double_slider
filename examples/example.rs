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
    slider_f32_low: f32,
    slider_f32_high: f32,
    slider_f64_low: f64,
    slider_f64_high: f64,
    slider_i32_low: i32,
    slider_i32_high: i32,
    slider_f64_log_low: f64,
    slider_f64_log_high: f64,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            slider_f32_low: 30.0,
            slider_f32_high: 200.0,
            slider_f64_low: 10.0,
            slider_f64_high: 150.0,
            slider_i32_low: -20,
            slider_i32_high: 40,
            slider_f64_log_low: 3e-4,
            slider_f64_log_high: 7e12,
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        Window::new("Interactive Double Slider").show(ctx, |ui| {
            let width = ui.available_width();

            // Display slider, linked to the same range as the plot
            ui.label("f32 values:");
            ui.add(
                DoubleSlider::new(
                    &mut self.slider_f32_low,
                    &mut self.slider_f32_high,
                    10.0..=10.0,
                )
                .width(width)
                .separation_distance(0.0),
            );
            ui.label(format!("Lower Bound: {:.2}", self.slider_f32_low));
            ui.label(format!("Upper Bound: {:.2}", self.slider_f32_high));

            ui.separator();
            ui.label("f64 values (inverted highlight):");
            ui.add(
                DoubleSlider::new(
                    &mut self.slider_f64_low,
                    &mut self.slider_f64_high,
                    10.0..=300.0,
                )
                .width(width)
                .separation_distance(10.0)
                .invert_highlighting(true),
            );
            ui.label(format!("Lower Bound: {:.2}", self.slider_f64_low));
            ui.label(format!("Upper Bound: {:.2}", self.slider_f64_high));

            ui.separator();
            ui.label("f64 values (cannot push by dragging):");
            ui.add(
                DoubleSlider::new(
                    &mut self.slider_f64_low,
                    &mut self.slider_f64_high,
                    10.0..=300.0,
                )
                .width(width)
                .separation_distance(10.0)
                .push_by_dragging(false),
            );
            ui.label(format!("Lower Bound: {:.2}", self.slider_f64_low));
            ui.label(format!("Upper Bound: {:.2}", self.slider_f64_high));

            ui.separator();
            ui.label("i32 values:");
            ui.add(
                DoubleSlider::new(
                    &mut self.slider_i32_low,
                    &mut self.slider_i32_high,
                    -150..=150,
                )
                .width(width)
                .separation_distance(1),
            );
            ui.label(format!("Lower Bound: {}", self.slider_i32_low));
            ui.label(format!("Upper Bound: {}", self.slider_i32_high));

            ui.separator();
            ui.label("logarithmic f64:");
            ui.add(
                DoubleSlider::new(
                    &mut self.slider_f64_log_low,
                    &mut self.slider_f64_log_high,
                    1e-10..=1e20,
                )
                .width(width)
                .separation_distance(1e-10)
                .logarithmic(true)
                .scroll_factor(0.1)
                .zoom_factor(15.0),
            );
            ui.label(format!("Lower Bound: {:.3e}", self.slider_f64_log_low));
            ui.label(format!("Upper Bound: {:.3e}", self.slider_f64_log_high));
        });
    }
}
