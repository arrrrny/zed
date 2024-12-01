use gpui::{
    canvas, div, point, prelude::*, px, size, App, AppContext, Bounds, MouseDownEvent, Path,
    Pixels, Point, WindowOptions,
};
use gpui3::{self as gpui, ModelContext, Window};
struct PaintingViewer {
    default_lines: Vec<Path<Pixels>>,
    lines: Vec<Vec<Point<Pixels>>>,
    start: Point<Pixels>,
    _painting: bool,
}

impl PaintingViewer {
    fn new() -> Self {
        let mut lines = vec![];

        // draw a line
        let mut path = Path::new(point(px(50.), px(180.)));
        path.line_to(point(px(100.), px(120.)));
        // go back to close the path
        path.line_to(point(px(100.), px(121.)));
        path.line_to(point(px(50.), px(181.)));
        lines.push(path);

        // draw a lightening bolt ⚡
        let mut path = Path::new(point(px(150.), px(200.)));
        path.line_to(point(px(200.), px(125.)));
        path.line_to(point(px(200.), px(175.)));
        path.line_to(point(px(250.), px(100.)));
        lines.push(path);

        // draw a ⭐
        let mut path = Path::new(point(px(350.), px(100.)));
        path.line_to(point(px(370.), px(160.)));
        path.line_to(point(px(430.), px(160.)));
        path.line_to(point(px(380.), px(200.)));
        path.line_to(point(px(400.), px(260.)));
        path.line_to(point(px(350.), px(220.)));
        path.line_to(point(px(300.), px(260.)));
        path.line_to(point(px(320.), px(200.)));
        path.line_to(point(px(270.), px(160.)));
        path.line_to(point(px(330.), px(160.)));
        path.line_to(point(px(350.), px(100.)));
        lines.push(path);

        let square_bounds = Bounds {
            origin: point(px(450.), px(100.)),
            size: size(px(200.), px(80.)),
        };
        let height = square_bounds.size.height;
        let horizontal_offset = height;
        let vertical_offset = px(30.);
        let mut path = Path::new(square_bounds.lower_left());
        path.curve_to(
            square_bounds.origin + point(horizontal_offset, vertical_offset),
            square_bounds.origin + point(px(0.0), vertical_offset),
        );
        path.line_to(square_bounds.upper_right() + point(-horizontal_offset, vertical_offset));
        path.curve_to(
            square_bounds.lower_right(),
            square_bounds.upper_right() + point(px(0.0), vertical_offset),
        );
        path.line_to(square_bounds.lower_left());
        lines.push(path);

        Self {
            default_lines: lines.clone(),
            lines: vec![],
            start: point(px(0.), px(0.)),
            _painting: false,
        }
    }

    fn clear(&mut self, cx: &mut ModelContext<Self>) {
        self.lines.clear();
        cx.notify();
    }

    fn render(&mut self, _window: &mut Window, cx: &mut ModelContext<Self>) -> impl IntoElement {
        let default_lines = self.default_lines.clone();
        let lines = self.lines.clone();
        div()
            .font_family(".SystemUIFont")
            .bg(gpui::white())
            .size_full()
            .p_4()
            .flex()
            .flex_col()
            .child(
                div()
                    .flex()
                    .gap_2()
                    .justify_between()
                    .items_center()
                    .child("Mouse down any point and drag to draw lines (Hold on shift key to draw straight lines)")
                    .child(
                        div()
                            .id("clear")
                            .child("Clean up")
                            .bg(gpui::black())
                            .text_color(gpui::white())
                            .active(|this| this.opacity(0.8))
                            .flex()
                            .px_3()
                            .py_1()
                            .on_click(
                                cx.listener(|viewer, _, _, cx| {
                                    viewer.clear(cx);
                                })
                            ),
                    ),
            )
            .child(
                div()
                    .size_full()
                    .child(
                        canvas(
                            move |_,_, _| {},
                            move |_,_, window, _cx| {
                                const STROKE_WIDTH: Pixels = px(2.0);
                                for path in &default_lines {
                                    window.paint_path(path.clone(), gpui::black());
                                }
                                for points in &lines {
                                    let mut path = Path::new(points[0]);
                                    for p in points.iter().skip(1) {
                                        path.line_to(*p);
                                    }

                                    let mut last = points.last().unwrap();
                                    for p in points.iter().rev() {
                                        let mut offset_x = px(0.);
                                        if last.x == p.x {
                                            offset_x = STROKE_WIDTH;
                                        }
                                        path.line_to(point(p.x + offset_x, p.y + STROKE_WIDTH));
                                        last = p;
                                    }

                                    window.paint_path(path, gpui::black());
                                }
                            },
                        )
                        .size_full(),
                    )
                    .on_mouse_down(
                        gpui::MouseButton::Left,
                        cx.listener(|viewer, ev: &MouseDownEvent, _window, cx| {
                            viewer._painting = true;
                            viewer.start = ev.position;
                            let path = vec![ev.position];
                            viewer.lines.push(path);
                            cx.notify();
                        }),
                    )
                    .on_mouse_move({
                        cx.listener(|viewer, ev: &gpui::MouseMoveEvent, _window, cx| {
                            if !viewer._painting {
                                return;
                            }

                            let is_shifted = ev.modifiers.shift;
                            let mut pos = ev.position;
                            // When holding shift, draw a straight line
                            if is_shifted {
                                let dx = pos.x - viewer.start.x;
                                let dy = pos.y - viewer.start.y;
                                if dx.abs() > dy.abs() {
                                    pos.y = viewer.start.y;
                                } else {
                                    pos.x = viewer.start.x;
                                }
                            }

                            if let Some(path) = viewer.lines.last_mut() {
                                path.push(pos);
                            }
                            cx.notify();
                        })
                    })
                    .on_mouse_up(
                        gpui::MouseButton::Left,
                        cx.listener(|viewer, _, _window, cx| {
                            viewer._painting = false;
                            cx.notify();
                        }),
                    ),
            )
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(
            WindowOptions {
                focus: true,
                ..Default::default()
            },
            |_, _| (PaintingViewer::new(), PaintingViewer::render),
        )
        .unwrap();
        cx.activate(true);
    });
}