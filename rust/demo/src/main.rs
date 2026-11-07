mod platform;
use eframe::egui;
use tinyeasings::{ease, Curve, ALL_CURVES};

struct App {
    curve: usize,
    playing: bool,
    duration: f32,
    t: f64,
    ping_pong: bool,
    direction: f64,
    gallery: bool,
    last: f64,
}
const TEAL: egui::Color32 = egui::Color32::from_rgb(102, 219, 198);
const ORANGE: egui::Color32 = egui::Color32::from_rgb(245, 174, 92);
fn graph(ui: &mut egui::Ui, curve: Curve, t: f64, size: egui::Vec2) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 8., egui::Color32::from_rgb(12, 20, 31));
    let plot = rect.shrink2(egui::vec2(26., 22.));
    let samples: Vec<_> = (0..=200)
        .map(|i| {
            let x = i as f64 / 200.;
            (x, ease(curve, x))
        })
        .collect();
    let min = samples.iter().map(|p| p.1).fold(0_f64, f64::min) - 0.08;
    let max = samples.iter().map(|p| p.1).fold(1_f64, f64::max) + 0.08;
    let at = |x: f64, y: f64| {
        egui::pos2(
            plot.left() + plot.width() * x as f32,
            plot.bottom() - plot.height() * ((y - min) / (max - min)) as f32,
        )
    };
    for i in 0..=4 {
        let x = i as f64 / 4.;
        painter.line_segment(
            [at(x, min), at(x, max)],
            egui::Stroke::new(1., egui::Color32::from_rgb(26, 39, 53)),
        );
        painter.line_segment(
            [at(0., x), at(1., x)],
            egui::Stroke::new(1., egui::Color32::from_rgb(26, 39, 53)),
        );
    }
    painter.line_segment(
        [at(0., 0.), at(1., 1.)],
        egui::Stroke::new(1., egui::Color32::from_gray(74)),
    );
    painter.add(egui::Shape::line(
        samples.into_iter().map(|(x, y)| at(x, y)).collect(),
        egui::Stroke::new(2.5, TEAL),
    ));
    let y = ease(curve, t);
    let p = at(t, y);
    painter.line_segment(
        [at(t, min), p],
        egui::Stroke::new(1., ORANGE.gamma_multiply(0.5)),
    );
    painter.circle_filled(p, 5., ORANGE);
    if size.y > 200. {
        for (label, p) in [("0", at(0., 0.)), ("1", at(1., 1.))] {
            painter.text(
                p + egui::vec2(-10., 10.),
                egui::Align2::CENTER_CENTER,
                label,
                egui::FontId::monospace(11.),
                egui::Color32::GRAY,
            );
        }
    }
    response
}
fn track(ui: &mut egui::Ui, label: &str, value: f64, color: egui::Color32) {
    ui.label(label);
    let (rect, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 38.), egui::Sense::hover());
    let a = rect.left() + rect.width() * 0.13;
    let b = rect.right() - rect.width() * 0.13;
    ui.painter().line_segment(
        [
            egui::pos2(a, rect.center().y),
            egui::pos2(b, rect.center().y),
        ],
        egui::Stroke::new(3., egui::Color32::from_rgb(34, 49, 64)),
    );
    for x in [a, b] {
        ui.painter().line_segment(
            [
                egui::pos2(x, rect.top() + 7.),
                egui::pos2(x, rect.bottom() - 7.),
            ],
            egui::Stroke::new(1., egui::Color32::GRAY),
        );
    }
    ui.painter().circle_filled(
        egui::pos2(a + (b - a) * value as f32, rect.center().y),
        9.,
        color,
    );
}
impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        platform::theme(&cc.egui_ctx);
        Self {
            curve: platform::option("curve")
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(28)
                .min(29),
            playing: platform::option("paused").as_deref() != Some("1"),
            duration: 2.,
            t: 0.,
            ping_pong: false,
            direction: 1.,
            gallery: platform::option("gallery").as_deref() == Some("1"),
            last: platform::now(),
        }
    }
}
impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _: &mut eframe::Frame) {
        let now = platform::now();
        let dt = ((now - self.last) / 1000.).min(0.05);
        self.last = now;
        if self.playing {
            self.t += dt / self.duration as f64 * self.direction;
            if self.t >= 1. {
                if self.ping_pong {
                    self.t = 1.;
                    self.direction = -1.;
                } else {
                    self.t = 0.;
                }
            } else if self.t <= 0. {
                self.t = 0.;
                self.direction = 1.;
            }
            ui.ctx().request_repaint();
        }
        platform::controls(ui, |ui| {
            platform::heading(ui, "tinyeasings", "Explore all 30 animation curves");
            ui.label("CURVE");
            egui::ComboBox::from_id_salt("curve")
                .selected_text(ALL_CURVES[self.curve].name())
                .show_ui(ui, |ui| {
                    for (i, c) in ALL_CURVES.iter().enumerate() {
                        ui.selectable_value(&mut self.curve, i, c.name());
                    }
                });
            ui.horizontal(|ui| {
                if ui.button("Previous").clicked() {
                    self.curve = (self.curve + 29) % 30;
                }
                if ui.button("Next").clicked() {
                    self.curve = (self.curve + 1) % 30;
                }
            });
            ui.checkbox(&mut self.gallery, "Show all curves");
            ui.separator();
            ui.label("PLAYBACK");
            ui.horizontal(|ui| {
                if ui
                    .button(if self.playing { "Pause" } else { "Play" })
                    .clicked()
                {
                    self.playing = !self.playing;
                }
                if ui.button("Restart").clicked() {
                    self.t = 0.;
                    self.direction = 1.;
                }
            });
            ui.add(egui::Slider::new(&mut self.duration, 0.5..=6.).text("Seconds"));
            ui.checkbox(&mut self.ping_pong, "Ping-pong");
            if ui
                .add(egui::Slider::new(&mut self.t, 0.0..=1.).text("Input t"))
                .changed()
            {
                self.playing = false;
            }
            ui.separator();
            ui.monospace(format!(
                "t = {:.4}\ny = {:.4}",
                self.t,
                ease(ALL_CURVES[self.curve], self.t)
            ));
            ui.small("Teal: easing curve\nOrange: animated sample\nGrey: linear reference");
            ui.separator();
            ui.small("Back and elastic curves deliberately overshoot. The graph keeps their full range visible.");
            ui.hyperlink_to(
                "Library & source",
                "https://github.com/varnholt/tinyeasings/tree/main/rust",
            );
        });
        egui::CentralPanel::default().show(ui, |ui| {
            if self.gallery {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.heading("All curves");
                    ui.small("Click a card to inspect its motion.");
                    let columns = if ui.available_width() > 760. {
                        3
                    } else if ui.available_width() > 440. {
                        2
                    } else {
                        1
                    };
                    let width =
                        (ui.available_width() - 12. * (columns - 1) as f32) / columns as f32;
                    for row in ALL_CURVES.chunks(columns) {
                        ui.horizontal(|ui| {
                            for curve in row {
                                ui.vertical(|ui| {
                                    ui.set_width(width);
                                    ui.label(curve.name());
                                    if graph(ui, *curve, self.t, egui::vec2(width, 145.)).clicked()
                                    {
                                        self.curve = *curve as usize;
                                        self.gallery = false;
                                    }
                                });
                            }
                        });
                    }
                });
            } else {
                let curve = ALL_CURVES[self.curve];
                ui.add_space(10.);
                ui.heading(curve.name());
                ui.label("Input progress -> output progress");
                let height = (ui.available_height() - 190.).max(150.);
                graph(ui, curve, self.t, egui::vec2(ui.available_width(), height));
                ui.add_space(8.);
                track(ui, "Linear", self.t, egui::Color32::GRAY);
                track(ui, curve.name(), ease(curve, self.t), ORANGE);
            }
        });
    }
}
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    platform::run(
        "tinyeasings  |  Rust demo",
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}
#[cfg(target_arch = "wasm32")]
fn main() {
    platform::run("tinyeasings", Box::new(|cc| Ok(Box::new(App::new(cc)))));
}
