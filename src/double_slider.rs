use eframe::emath::{Pos2, Rect, Vec2};
use eframe::epaint::{CircleShape, Color32, PathShape, Shape, Stroke};
use egui::epaint::RectShape;
use egui::{Sense, Ui, Widget};
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
///     let mut my_f32: f32 = 0.0;
///     let mut my_other_f32: f32 = 0.0;
///         ui.add(DoubleSlider::new(&mut my_f32,&mut my_other_f32, 0.0..=100.0));
/// });
/// ```
///
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct DoubleSlider<'a> {
    left_slider: &'a mut f32,
    right_slider: &'a mut f32,
    separation_distance: f32,
    control_point_radius: f32,
    inverted_highlighting: bool,
    scroll_factor: f32,
    zoom_factor: f32,
    width: f32,
    color: Color32,
    cursor_fill: Color32,
    stroke: Stroke,
    range: RangeInclusive<f32>,
}

impl<'a> DoubleSlider<'a> {
    pub fn new(
        lower_value: &'a mut f32,
        upper_value: &'a mut f32,
        range: RangeInclusive<f32>,
    ) -> Self {
        DoubleSlider {
            left_slider: lower_value,
            right_slider: upper_value,
            separation_distance: 75.0,
            control_point_radius: 7.0,
            inverted_highlighting: false,
            scroll_factor: 10.0,
            zoom_factor: 10.0,
            width: 100.0,
            cursor_fill: Color32::DARK_GRAY,
            color: Color32::DARK_GRAY,
            stroke: Stroke::new(7.0, Color32::RED.linear_multiply(0.5)),
            range,
        }
    }

    /// Set the primary width for the slider.
    #[inline]
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// set the zoom factor. This depends on the responsiveness that you would like to have for zooming
    #[inline]
    pub fn zoom_factor(mut self, zoom_factor: f32) -> Self {
        self.zoom_factor = zoom_factor;
        self
    }

    /// set the scroll factor. This depends on the responsiveness that you would like to have for scrolling
    #[inline]
    pub fn scroll_factor(mut self, scroll_factor: f32) -> Self {
        self.scroll_factor = scroll_factor;
        self
    }

    /// invert the highlighted part.
    #[inline]
    pub fn invert_highlighting(mut self, inverted_highlighting: bool) -> Self {
        self.inverted_highlighting = inverted_highlighting;
        self
    }

    /// Set the separation distance for the two sliders.
    #[inline]
    pub fn separation_distance(mut self, separation_distance: f32) -> Self {
        self.separation_distance = separation_distance;
        self
    }

    /// Set the primary color for the slider.
    #[inline]
    pub fn color(mut self, color: Color32) -> Self {
        self.color = color;
        self
    }

    /// Set the stroke for the main line.
    #[inline]
    pub fn stroke(mut self, stroke: Stroke) -> Self {
        self.stroke = stroke;
        self
    }

    /// Set the color fill for the slider cursor.
    #[inline]
    pub fn cursor_fill(mut self, cursor_fill: Color32) -> Self {
        self.cursor_fill = cursor_fill;
        self
    }

    /// Set the auxiliary stroke.
    #[inline]
    pub fn aux_stroke(mut self, aux_stroke: Stroke) -> Self {
        self.stroke = aux_stroke;
        self
    }

    /// Set the control point radius
    #[inline]
    pub fn control_point_radius(mut self, control_point_radius: f32) -> Self {
        self.control_point_radius = control_point_radius;
        self
    }

    fn val_to_x(&self, val: f32) -> f32 {
        (self.width - 2.0 * self.control_point_radius - 2.0 * OFFSET)
            / (self.range.end() - self.range.start())
            * (val - self.range.start())
            + self.control_point_radius
            + OFFSET
    }

    fn x_to_val(&self, x: f32) -> f32 {
        (self.range.end() - self.range.start())
            / (self.width - 2.0 * self.control_point_radius - 2.0 * OFFSET)
            * x
    }
}

impl<'a> Widget for DoubleSlider<'a> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        // calculate height
        let height = 2.0 * self.control_point_radius + 2.0 * OFFSET;

        let (mut response, painter) =
            ui.allocate_painter(Vec2::new(self.width, height), Sense::drag());
        let mut left_edge = response.rect.left_center();
        left_edge.x += self.control_point_radius;
        let mut right_edge = response.rect.right_center();
        right_edge.x -= self.control_point_radius;

        // draw the line
        painter.add(PathShape::line(
            vec![left_edge, right_edge],
            Stroke::new(self.stroke.width, self.color),
        ));

        let to_screen = eframe::emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.size()),
            response.rect,
        );
        let mut shapes = vec![];
        let stroke = if !self.inverted_highlighting {
            let in_between_rect = Rect::from_min_max(
                to_screen.transform_pos(Pos2 {
                    x: self.val_to_x(*self.left_slider),
                    y: height / 2.0 - self.stroke.width / 2.0,
                }),
                to_screen.transform_pos(Pos2 {
                    x: self.val_to_x(*self.right_slider),
                    y: height / 2.0 + self.stroke.width / 2.0,
                }),
            );
            let in_between_id = response.id.with(2);
            let in_between_response = ui.interact(in_between_rect, in_between_id, Sense::drag());

            // drag both sliders by dragging the highlighted part (only when not highlighting is not inverted)
            if in_between_response.dragged() {
                *self.right_slider += self.x_to_val(in_between_response.drag_delta().x);
                *self.left_slider += self.x_to_val(in_between_response.drag_delta().x);
                response.mark_changed();
            }

            if in_between_response.hovered() {
                let mut stroke = ui.style().interact(&in_between_response).fg_stroke;
                stroke.width /= 2.0;
                stroke
            } else {
                Stroke::new(1.0, self.stroke.color)
            }
        } else {
            Stroke::new(0.0, self.stroke.color)
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
        let point_response = ui.interact(point_rect, point_id, Sense::drag());

        if point_response.dragged() {
            response.mark_changed();
        }

        // handle logic
        *self.left_slider += self.x_to_val(point_response.drag_delta().x);
        if *self.right_slider < *self.left_slider + self.separation_distance {
            *self.right_slider = *self.left_slider + self.separation_distance;
        }
        *self.right_slider = self
            .right_slider
            .clamp(*self.range.start(), *self.range.end());
        *self.left_slider = self
            .left_slider
            .clamp(*self.range.start(), *self.range.end());

        let left_circle_stroke = ui.style().interact(&point_response).fg_stroke;

        // handle upper bound
        // get the control point
        let right_point_in_screen = to_screen.transform_pos(Pos2 {
            x: self.val_to_x(*self.right_slider),
            y: self.control_point_radius + OFFSET,
        });
        let point_rect = Rect::from_center_size(right_point_in_screen, size);
        let point_id = response.id.with(1);
        let point_response = ui.interact(point_rect, point_id, Sense::drag());

        if point_response.dragged() {
            response.mark_changed();
        }

        // handle logic
        *self.right_slider += self.x_to_val(point_response.drag_delta().x);
        if *self.left_slider > *self.right_slider - self.separation_distance {
            *self.left_slider = *self.right_slider - self.separation_distance;
        }
        *self.right_slider = self
            .right_slider
            .clamp(*self.range.start(), *self.range.end());
        *self.left_slider = self
            .left_slider
            .clamp(*self.range.start(), *self.range.end());

        let right_circle_stroke = ui.style().interact(&point_response).fg_stroke;

        // override all shapes before drawing, due to logic limits (calculated above)
        if !self.inverted_highlighting {
            let in_between_rect = Rect::from_min_max(
                to_screen.transform_pos(Pos2 {
                    x: self.val_to_x(*self.left_slider),
                    y: height / 2.0 - self.stroke.width / 2.0 + OFFSET / 2.0,
                }),
                to_screen.transform_pos(Pos2 {
                    x: self.val_to_x(*self.right_slider),
                    y: height / 2.0 + self.stroke.width / 2.0 - OFFSET / 2.0,
                }),
            );
            shapes.push(Shape::Rect(RectShape::new(
                in_between_rect,
                0.0,
                self.stroke.color,
                stroke,
            )));
        } else {
            let left_rect = Rect::from_min_max(
                to_screen.transform_pos(Pos2 {
                    x: self.control_point_radius,
                    y: height / 2.0 - self.stroke.width / 2.0,
                }),
                to_screen.transform_pos(Pos2 {
                    x: self.val_to_x(*self.left_slider),
                    y: height / 2.0 + self.stroke.width / 2.0,
                }),
            );

            let right_rect = Rect::from_min_max(
                to_screen.transform_pos(Pos2 {
                    x: self.val_to_x(*self.right_slider),
                    y: height / 2.0 - self.stroke.width / 2.0,
                }),
                to_screen.transform_pos(Pos2 {
                    x: self.width - self.control_point_radius,
                    y: height / 2.0 + self.stroke.width / 2.0,
                }),
            );
            shapes.push(Shape::Rect(RectShape::new(
                left_rect,
                0.0,
                self.stroke.color,
                Stroke::new(0.0, self.stroke.color),
            )));
            shapes.push(Shape::Rect(RectShape::new(
                right_rect,
                0.0,
                self.stroke.color,
                Stroke::new(0.0, self.stroke.color),
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
            fill: self.cursor_fill,
            stroke: left_circle_stroke,
        }));

        shapes.push(Shape::Circle(CircleShape {
            center: right_point_in_screen,
            radius: self.control_point_radius,
            fill: self.cursor_fill,
            stroke: right_circle_stroke,
        }));

        // draw control points
        painter.extend(shapes);

        let zoom_id = response.id.with(4);
        let zoom_response = ui.interact(response.rect, zoom_id, Sense::hover());

        // scroll through time axis
        if zoom_response.hovered() {
            let scroll_delta = ui.ctx().input(|i| i.smooth_scroll_delta);
            *self.left_slider += scroll_delta.x / self.scroll_factor;
            *self.right_slider += scroll_delta.x / self.scroll_factor;

            *self.left_slider += scroll_delta.y / self.scroll_factor;
            *self.right_slider += scroll_delta.y / self.scroll_factor;
            let zoom_delta = ui.ctx().input(|i| i.zoom_delta() - 1.0);

            *self.right_slider += zoom_delta * self.zoom_factor;
            *self.left_slider -= zoom_delta * self.zoom_factor;
        }

        response
    }
}
