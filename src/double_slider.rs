use egui::emath::{Numeric, Pos2, Rect, RectTransform, Vec2};
use egui::epaint::{CircleShape, Color32, PathShape, RectShape, Shape, Stroke};
use egui::{Sense, StrokeKind, Ui, Widget};
use std::ops::RangeInclusive;

// offset for stroke highlight
const OFFSET: f32 = 2.0;

/// Control two numbers with a double slider.
///
/// The slider range defines the values you get when pulling the slider to the far edges.
///
/// The range can include any numbers, and go from low-to-high or from high-to-low.
///
///
/// ```
/// use egui_double_slider::DoubleSlider;
///
/// egui::__run_test_ui(|ui| {
///     let mut my_val: f32 = 0.0;
///     let mut my_other_val: f32 = 0.0;
///         ui.add(DoubleSlider::new(&mut my_val,&mut my_other_val, 0.0..=100.0));
/// });
/// ```
///
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct DoubleSlider<'a, T: Numeric> {
    left_slider: &'a mut T,
    right_slider: &'a mut T,
    separation_distance: T,
    control_point_radius: f32,
    inverted_highlighting: bool,
    vertical_scroll: bool,
    horizontal_scroll: bool,
    scroll_factor: f32,
    zoom_factor: f32,
    width: f32,
    color: Option<Color32>,
    cursor_fill: Option<Color32>,
    stroke: Option<Stroke>,
    range: RangeInclusive<T>,
    logarithmic: bool,
    push_by_dragging: bool,
}

impl<'a, T: Numeric> DoubleSlider<'a, T> {
    pub fn new(lower_value: &'a mut T, upper_value: &'a mut T, range: RangeInclusive<T>) -> Self {
        DoubleSlider {
            left_slider: lower_value,
            right_slider: upper_value,
            separation_distance: T::from_f64(1.0),
            control_point_radius: 7.0,
            inverted_highlighting: false,
            vertical_scroll: true,
            horizontal_scroll: true,
            scroll_factor: if T::INTEGRAL { 0.04 } else { 0.01 },
            zoom_factor: 10.0,
            width: 100.0,
            cursor_fill: None,
            color: None,
            stroke: None,
            range,
            logarithmic: false,
            push_by_dragging: true,
        }
    }

    /// Set the primary width for the slider.
    /// Default is 100.0
    #[inline]
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Set the zoom factor (multiplied with cursor zoom). This depends on the responsiveness that you would like to have for zooming
    /// Default is 10.0
    #[inline]
    pub fn zoom_factor(mut self, zoom_factor: f32) -> Self {
        self.zoom_factor = zoom_factor;
        self
    }

    /// Set the scroll factor (multiplied with cursor scroll). This depends on the responsiveness that you would like to have for scrolling
    /// Default is 0.01
    #[inline]
    pub fn scroll_factor(mut self, scroll_factor: f32) -> Self {
        self.scroll_factor = scroll_factor;
        self
    }

    /// Enable the horizontal scroll axis.
    /// Default is true
    #[inline]
    pub fn horizontal_scroll(mut self, enable: bool) -> Self {
        self.horizontal_scroll = enable;
        self
    }

    /// Enable the vertical scroll axis.
    /// Default is true
    #[inline]
    pub fn vertical_scroll(mut self, enable: bool) -> Self {
        self.vertical_scroll = enable;
        self
    }

    /// Invert the highlighted part.
    /// Default is false.
    #[inline]
    pub fn invert_highlighting(mut self, inverted_highlighting: bool) -> Self {
        self.inverted_highlighting = inverted_highlighting;
        self
    }

    /// Set the separation distance for the two sliders.
    /// Default is 1.
    #[inline]
    pub fn separation_distance(mut self, separation_distance: T) -> Self {
        self.separation_distance = separation_distance;
        self
    }

    /// Set the primary color for the slider
    /// Default color is taken from `inactive.bg_fill` in [`egui::style::Widgets`], the same as [`egui::Slider`].
    #[inline]
    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    /// Set the stroke for the main line.
    /// Default color is taken from `selection.bg_fill` in [`egui::Visuals`], the same as [`egui::Slider`]
    #[inline]
    pub fn stroke(mut self, stroke: Stroke) -> Self {
        self.stroke = Some(stroke);
        self
    }

    /// Set the color fill for the slider cursor.
    /// Default fill is taken from `visuals.bg_fill` in [`egui::style::WidgetVisuals`], the same as [`egui::Slider`]
    #[inline]
    pub fn cursor_fill(mut self, cursor_fill: Color32) -> Self {
        self.cursor_fill = Some(cursor_fill);
        self
    }

    /// Set the control point radius
    /// Default is 7.0
    #[inline]
    pub fn control_point_radius(mut self, control_point_radius: f32) -> Self {
        self.control_point_radius = control_point_radius;
        self
    }

    /// Use a logarithmic scale.
    /// Default is false.
    #[inline]
    pub fn logarithmic(mut self, logarithmic: bool) -> Self {
        if logarithmic {
            let range_f64 = self.range_f64();
            assert!(
                *range_f64.start() > 0.0 && range_f64.start().is_finite() &&
                    *range_f64.end() > 0.0 && range_f64.end().is_finite(),
                "Logarithmic scale can only be used with a range of finite, strictly positive values (both start and end)"
            );
        }
        self.logarithmic = logarithmic;
        self
    }

    /// Allow to drag the lower value to the right of the upper value, and vice versa.
    /// Default is true.
    #[inline]
    pub fn push_by_dragging(mut self, push_by_dragging: bool) -> Self {
        self.push_by_dragging = push_by_dragging;
        self
    }

    fn val_to_x(&self, val: T) -> f32 {
        let offset = self.control_point_radius + OFFSET;
        // Calculate usable visual width of the slider track, ensuring it's not negative
        let visual_slider_width = (self.width - 2.0 * offset).max(0.0);

        let mut current_val_f64 = val.to_f64();
        let mut range_min_f64 = self.range.start().to_f64();
        let mut range_max_f64 = self.range.end().to_f64();

        if self.logarithmic {
            // Values are asserted to be > 0 in the logarithmic() setter.
            // If range_min_f64 or range_max_f64 were <=0, log10 would produce NaN or Inf.
            // current_val_f64 should also be > 0.
            if current_val_f64 <= 0.0 || range_min_f64 <= 0.0 || range_max_f64 <= 0.0 {
                // This case should ideally be prevented by assertions or clamping
                // For safety, if inputs are invalid for log, default to a non-NaN behavior
                // though this indicates a deeper issue if reached.
                // Given the problem context (1..=1), this branch isn't the primary issue.
                return offset; // Fallback to avoid NaN from log
            }
            current_val_f64 = current_val_f64.log10();
            range_min_f64 = range_min_f64.log10();
            range_max_f64 = range_max_f64.log10();
        }

        let range_span_f64 = range_max_f64 - range_min_f64;

        let ratio = if range_span_f64 == 0.0 {
            // If the range is a single point (e.g., 1..=1, or log(1)..=log(1)),
            // the value is conceptually at that point.
            // Map this to the start (0.0) of the visual slider part.
            0.0
        } else {
            // Normalize current_val_f64 to a [0, 1] ratio within the range.
            let normalized_val = (current_val_f64 - range_min_f64) / range_span_f64;
            normalized_val.clamp(0.0, 1.0) // Clamp to handle potential floating point inaccuracies.
        };

        // Map the ratio to the screen coordinate.
        (ratio as f32 * visual_slider_width) + offset
    }

    fn x_to_val(&self, x_in_widget: f32) -> T {
        let offset = self.control_point_radius + OFFSET;
        // Calculate usable visual width of the slider track, ensuring it's not negative
        let visual_slider_width = (self.width - 2.0 * offset).max(0.0) as f64;

        let range_min_f64 = self.range.start().to_f64();
        let range_max_f64 = self.range.end().to_f64();

        let value_f64 = if range_min_f64 == range_max_f64 {
            // If the range is a single point, any x position maps to this single value.
            range_min_f64
        } else {
            // Position of x relative to the start of the slider track
            let x_on_track = (x_in_widget - offset) as f64;

            let ratio = if visual_slider_width == 0.0 {
                // If visual width is zero (e.g. self.width is too small),
                // effectively all points map to the start of the range.
                0.0
            } else {
                (x_on_track / visual_slider_width).clamp(0.0, 1.0)
            };

            if self.logarithmic {
                // Values are asserted to be > 0 in the logarithmic() setter.
                if range_min_f64 <= 0.0 || range_max_f64 <= 0.0 {
                    // Fallback, though assertions should prevent this.
                    return self.f64_to_val(self.range.start().to_f64());
                }
                let log_min = range_min_f64.log10();
                let log_max = range_max_f64.log10();
                // This path is taken only if range_min_f64 != range_max_f64.
                // If they are different and positive, their logs will also be different.
                10.0f64.powf(log_min + (log_max - log_min) * ratio)
            } else {
                range_min_f64 + (range_max_f64 - range_min_f64) * ratio
            }
        };

        self.f64_to_val(value_f64)
    }

    fn left_slider_f64(&self) -> f64 {
        self.left_slider.to_f64()
    }

    fn right_slider_f64(&self) -> f64 {
        self.right_slider.to_f64()
    }

    fn separation_distance_f64(&self) -> f64 {
        self.separation_distance.to_f64()
    }

    fn range_f64(&self) -> RangeInclusive<f64> {
        self.range.start().to_f64()..=self.range.end().to_f64()
    }

    // Rounds decimal values when casting to integers (instead of truncating like a native float-to-int cast)
    fn f64_to_val(&self, float: f64) -> T {
        T::from_f64(if T::INTEGRAL { float.round() } else { float })
    }

    fn clamp_to_range(&self, val: &T) -> T {
        let (start, end) = (self.range.start().to_f64(), self.range.end().to_f64());
        self.f64_to_val(val.to_f64().clamp(start, end))
    }
}

impl<'a, T: Numeric> Widget for DoubleSlider<'a, T> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        // calculate height
        let height = 2.0 * self.control_point_radius + 2.0 * OFFSET;

        let (mut response, painter) =
            ui.allocate_painter(Vec2::new(self.width, height), Sense::click_and_drag());
        let mut left_edge = response.rect.left_center();
        left_edge.x += self.control_point_radius;
        let mut right_edge = response.rect.right_center();
        right_edge.x -= self.control_point_radius;

        let visuals = ui.style().interact(&response);
        let widget_visuals = &ui.visuals().widgets;

        let color = self.color.unwrap_or(widget_visuals.inactive.bg_fill);
        let cursor_fill = self.cursor_fill.unwrap_or(visuals.bg_fill);
        let stroke_style = self
            .stroke
            .unwrap_or(Stroke::new(7.0, ui.visuals().selection.bg_fill));

        // draw the line
        painter.add(PathShape::line(
            vec![left_edge, right_edge],
            Stroke::new(stroke_style.width, color),
        ));

        let to_screen = RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.size()),
            response.rect,
        );
        let mut shapes = vec![];
        let stroke = if !self.inverted_highlighting {
            let in_between_rect = Rect::from_min_max(
                to_screen.transform_pos(Pos2 {
                    x: self.val_to_x(*self.left_slider) + self.control_point_radius,
                    y: height / 2.0 - stroke_style.width / 2.0,
                }),
                to_screen.transform_pos(Pos2 {
                    x: self.val_to_x(*self.right_slider) - self.control_point_radius,
                    y: height / 2.0 + stroke_style.width / 2.0,
                }),
            );
            let in_between_id = response.id.with(2);
            let in_between_response =
                ui.interact(in_between_rect, in_between_id, Sense::click_and_drag());

            // drag both sliders by dragging the highlighted part (only when not highlighting is not inverted)
            if in_between_response.dragged() {
                *self.right_slider = self.x_to_val(
                    self.val_to_x(*self.right_slider) + in_between_response.drag_delta().x,
                );
                *self.left_slider = self.x_to_val(
                    self.val_to_x(*self.left_slider) + in_between_response.drag_delta().x,
                );
                response.mark_changed();
            }

            response |= in_between_response.clone();

            if in_between_response.hovered() {
                let mut stroke = ui.style().interact(&in_between_response).fg_stroke;
                stroke.width /= 2.0;
                stroke
            } else {
                Stroke::new(1.0, stroke_style.color)
            }
        } else {
            Stroke::new(0.0, stroke_style.color)
        };

        // handle lower bound
        // get the control point
        let size = Vec2::splat(2.0 * self.control_point_radius);
        let left_point_in_screen = to_screen.transform_pos(Pos2 {
            x: self.val_to_x(*self.left_slider),
            y: self.control_point_radius + OFFSET,
        });
        let point_rect = Rect::from_center_size(left_point_in_screen, size);
        let point_id = response.id.with(0);
        let point_response = ui.interact(point_rect, point_id, Sense::click_and_drag());

        if point_response.dragged() {
            if let Some(pointer_pos) = point_response.interact_pointer_pos() {
                *self.left_slider = self.x_to_val(pointer_pos.x - response.rect.left());
                response.mark_changed();
            }
        }

        // handle logic
        if self.right_slider_f64() < self.left_slider_f64() + self.separation_distance_f64() {
            if self.push_by_dragging {
                *self.right_slider =
                    self.f64_to_val(self.left_slider_f64() + self.separation_distance_f64());
            } else {
                *self.left_slider =
                    self.f64_to_val(self.right_slider_f64() - self.separation_distance_f64());
            }
        }
        *self.left_slider = self.clamp_to_range(self.left_slider);
        *self.right_slider = self.clamp_to_range(self.right_slider);

        let left_circle_stroke = ui.style().interact(&point_response).fg_stroke;
        response |= point_response;

        // handle upper bound
        // get the control point
        let right_point_in_screen = to_screen.transform_pos(Pos2 {
            x: self.val_to_x(*self.right_slider),
            y: self.control_point_radius + OFFSET,
        });
        let point_rect = Rect::from_center_size(right_point_in_screen, size);
        let point_id = response.id.with(1);
        let point_response = ui.interact(point_rect, point_id, Sense::click_and_drag());

        if point_response.dragged() {
            if let Some(pointer_pos) = point_response.interact_pointer_pos() {
                *self.right_slider = self.x_to_val(pointer_pos.x - response.rect.left());
                response.mark_changed();
            }
        }

        // handle logic
        if self.left_slider_f64() > self.right_slider_f64() - self.separation_distance_f64() {
            if self.push_by_dragging {
                *self.left_slider =
                    self.f64_to_val(self.right_slider_f64() - self.separation_distance_f64());
            } else {
                *self.right_slider =
                    self.f64_to_val(self.left_slider_f64() + self.separation_distance_f64());
            }
        }
        *self.left_slider = self.clamp_to_range(self.left_slider);
        *self.right_slider = self.clamp_to_range(self.right_slider);

        let right_circle_stroke = ui.style().interact(&point_response).fg_stroke;
        response |= point_response;

        // override all shapes before drawing, due to logic limits (calculated above)
        if !self.inverted_highlighting {
            let in_between_rect = Rect::from_min_max(
                to_screen.transform_pos(Pos2 {
                    x: self.val_to_x(*self.left_slider),
                    y: height / 2.0 - stroke_style.width / 2.0 + OFFSET / 2.0,
                }),
                to_screen.transform_pos(Pos2 {
                    x: self.val_to_x(*self.right_slider),
                    y: height / 2.0 + stroke_style.width / 2.0 - OFFSET / 2.0,
                }),
            );
            shapes.push(Shape::Rect(RectShape::new(
                in_between_rect,
                0.0,
                stroke_style.color,
                stroke,
                StrokeKind::Middle,
            )));
        } else {
            let left_rect = Rect::from_min_max(
                to_screen.transform_pos(Pos2 {
                    x: self.control_point_radius,
                    y: height / 2.0 - stroke_style.width / 2.0,
                }),
                to_screen.transform_pos(Pos2 {
                    x: self.val_to_x(*self.left_slider),
                    y: height / 2.0 + stroke_style.width / 2.0,
                }),
            );

            let right_rect = Rect::from_min_max(
                to_screen.transform_pos(Pos2 {
                    x: self.val_to_x(*self.right_slider),
                    y: height / 2.0 - stroke_style.width / 2.0,
                }),
                to_screen.transform_pos(Pos2 {
                    x: self.width - self.control_point_radius,
                    y: height / 2.0 + stroke_style.width / 2.0,
                }),
            );
            shapes.push(Shape::Rect(RectShape::new(
                left_rect,
                0.0,
                stroke_style.color,
                Stroke::new(0.0, stroke_style.color),
                StrokeKind::Middle,
            )));
            shapes.push(Shape::Rect(RectShape::new(
                right_rect,
                0.0,
                stroke_style.color,
                Stroke::new(0.0, stroke_style.color),
                StrokeKind::Middle,
            )));
        }

        let left_point_in_screen = to_screen.transform_pos(Pos2 {
            x: self.val_to_x(*self.left_slider),
            y: self.control_point_radius + OFFSET,
        });

        let right_point_in_screen = to_screen.transform_pos(Pos2 {
            x: self.val_to_x(*self.right_slider),
            y: self.control_point_radius + OFFSET,
        });

        shapes.push(Shape::Circle(CircleShape {
            center: left_point_in_screen,
            radius: self.control_point_radius,
            fill: cursor_fill,
            stroke: left_circle_stroke,
        }));

        shapes.push(Shape::Circle(CircleShape {
            center: right_point_in_screen,
            radius: self.control_point_radius,
            fill: cursor_fill,
            stroke: right_circle_stroke,
        }));

        // draw control points
        painter.extend(shapes);

        let zoom_id = response.id.with(4);
        let zoom_response = ui.interact(response.rect, zoom_id, Sense::hover());

        // scroll through time axis
        if zoom_response.hovered() {
            let raw_scroll_delta = ui.ctx().input(|i| i.smooth_scroll_delta);
            let mut scroll_delta = 0.0;
            if self.horizontal_scroll {
                scroll_delta += raw_scroll_delta.x * self.scroll_factor;
            }
            if self.vertical_scroll {
                scroll_delta += raw_scroll_delta.y * self.scroll_factor;
            }
            let zoom_delta = self.zoom_factor * (ui.ctx().input(|i| i.zoom_delta() - 1.0));

            if self.logarithmic {
                *self.left_slider = self.x_to_val(self.val_to_x(*self.left_slider) + scroll_delta);
                *self.right_slider =
                    self.x_to_val(self.val_to_x(*self.right_slider) + scroll_delta);

                *self.left_slider = self.x_to_val(self.val_to_x(*self.left_slider) + zoom_delta);
                *self.right_slider = self.x_to_val(self.val_to_x(*self.right_slider) - zoom_delta);
            } else {
                *self.left_slider = self.f64_to_val(self.left_slider_f64() + scroll_delta as f64);
                *self.right_slider = self.f64_to_val(self.right_slider_f64() + scroll_delta as f64);

                *self.right_slider = self.f64_to_val(self.right_slider_f64() + zoom_delta as f64);
                *self.left_slider = self.f64_to_val(self.left_slider_f64() - zoom_delta as f64);
            }

            *self.left_slider = self.clamp_to_range(self.left_slider);
            *self.right_slider = self.clamp_to_range(self.right_slider);

            if scroll_delta != 0.0 || zoom_delta != 0.0 {
                response.mark_changed()
            }
        }

        response
    }
}
