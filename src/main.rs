// Using the right values in the toml is important! https://github.com/emilk/egui/blob/eeae485629fca24a81a7251739460b671e1420f7/examples/retained_image/Cargo.toml#L14
// Started with this sample: https://github.com/emilk/egui/blob/eeae485629fca24a81a7251739460b671e1420f7/examples/retained_image/src/main.rs

// This sample seems to move all over in the last few version changes...

use std::fs;
use std::time::Instant;

use eframe::egui;
use egui_extras::RetainedImage;
use image_play::{get_albums, get_randoms_from_album, Album};

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
    images: Vec<RetainedImage>,
    albums: Vec<Album>,
    current_album: Album,
    instant: Instant,
}

impl MyApp {
    fn load_pics(&mut self) {
        for pic_path in get_randoms_from_album(&self.current_album, 6) {
            self.images.clear();
            let pic =
                RetainedImage::from_image_bytes("test", &fs::read(pic_path).unwrap()[..]).unwrap();
            self.images.push(pic);
        }
    }
}

impl Default for MyApp {
    fn default() -> Self {
        let mut albums = get_albums(
            "/Users/jeffb/Library/Mobile Documents/com~apple~CloudDocs/rust_basic/image_play/test_items/pics"
        ).unwrap();

        let mut images = Vec::new();

        let curr = albums.pop().unwrap();

        for pic_path in get_randoms_from_album(&curr, 6) {
            println!("pic path: {}", pic_path);
            let pic =
                RetainedImage::from_image_bytes("test", &fs::read(pic_path).unwrap()[..]).unwrap();
            images.push(pic);
        }

        Self {
            images: images,
            albums: albums,
            instant: Instant::now(),
            current_album: curr,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.instant.elapsed().as_secs() > 5 {
                self.load_pics();
                self.instant = Instant::now();
            }

            ui.heading(self.current_album.name.to_owned());

            egui::Grid::new("some_unique_id").show(ui, |ui| {
                for (pos, i) in self.images.iter().enumerate() {
                    ui.add(egui::Image::new(i.texture_id(ctx), i.size_vec2() * 0.5));

                    if (pos + 1) % 3 == 0 {
                        ui.end_row();
                    }
                }

                // ui.add(egui::Image::new(
                //     self.image.texture_id(ctx),
                //     self.image.size_vec2() * 0.3,
                // ));
                // ui.label("First row, second column");
                // ui.end_row();

                // ui.label("Second row, first column");
                // ui.label("Second row, second column");
                // ui.label("Second row, third column");
                // ui.end_row();

                // ui.horizontal(|ui| {
                //     ui.label("Same");
                //     ui.label("cell");
                // });
                // ui.label("Third row, second column");
                // ui.end_row();
            });

            // self.image.show_scaled(ui, 0.3);

            // ui.heading("This is an image you can click:");
            // ui.add(egui::ImageButton::new(
            //     self.image.texture_id(ctx),
            //     self.image.size_vec2() * 0.3,
            // ));
        });
        // println!("windowinfo {:?}", _frame.info().window_info.size);
        // println!("duration {}", self.instant.elapsed().as_secs_f32());

        // can update images

        // if self.instant.elapsed().as_secs() > 4 {
        //     self.image = RetainedImage::from_image_bytes(
        //         "test_image",
        //         include_bytes!("/Users/jeffb/Desktop/Eastern Canada/Ottawa/IMG_0999.png"),
        //     )
        //     .unwrap();
        // }
    }
}
