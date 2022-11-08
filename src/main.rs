// Using the right values in the toml is important! https://github.com/emilk/egui/blob/eeae485629fca24a81a7251739460b671e1420f7/examples/retained_image/Cargo.toml#L14
// Started with this sample: https://github.com/emilk/egui/blob/eeae485629fca24a81a7251739460b671e1420f7/examples/retained_image/src/main.rs

// This sample seems to move all over in the last few version changes...

use std::{
    thread,
    time::{self, Instant},
};

use eframe::egui;
use egui_extras::RetainedImage;

fn main() {
    let options = eframe::NativeOptions {
        fullscreen: true,
        ..Default::default()
    };
    eframe::run_native(
        "Show an image with eframe/egui",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    image: RetainedImage,
    instant: Instant,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            image: RetainedImage::from_image_bytes(
                "test_image",
                include_bytes!("/Users/jeffb/Desktop/Eastern Canada/Ottawa/IMG_0998.png"),
            )
            .unwrap(),
            instant: Instant::now(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("This is an image:");
            self.image.show_scaled(ui, 0.3);

            ui.heading("This is an image you can click:");
            ui.add(egui::ImageButton::new(
                self.image.texture_id(ctx),
                self.image.size_vec2() * 0.3,
            ));
        });
        // println!("windowinfo {:?}", _frame.info().window_info.size);
        // println!("duration {}", self.instant.elapsed().as_secs_f32());
        if self.instant.elapsed().as_secs() > 4 {
            self.image = RetainedImage::from_image_bytes(
                "test_image",
                include_bytes!("/Users/jeffb/Desktop/Eastern Canada/Ottawa/IMG_0999.png"),
            )
            .unwrap();
        }
    }
}
