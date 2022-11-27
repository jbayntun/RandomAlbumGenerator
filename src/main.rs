// Using the right values in the toml is important! https://github.com/emilk/egui/blob/eeae485629fca24a81a7251739460b671e1420f7/examples/retained_image/Cargo.toml#L14
// Started with this sample: https://github.com/emilk/egui/blob/eeae485629fca24a81a7251739460b671e1420f7/examples/retained_image/src/main.rs

// This sample seems to move all over in the last few version changes...

use std::fs;
use std::time::{Duration, Instant};

use eframe::egui;
use egui::vec2;
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

struct AppImage {
    image: RetainedImage,
    name: String,
}

struct MyApp {
    images: Vec<AppImage>,
    albums: Vec<Album>,
    current_album: Album,
    instant: Instant,
}

impl MyApp {
    fn load_pics(&mut self) {
        self.images.clear();
        for image in get_randoms_from_album(&self.current_album, 6) {
            let pic = RetainedImage::from_image_bytes(
                "test",
                &fs::read(image.path.to_owned()).unwrap()[..],
            )
            .unwrap();
            self.images.push(AppImage {
                image: pic,
                name: image.name.to_owned(),
            });
        }
    }
}

impl Default for MyApp {
    fn default() -> Self {
        let mut albums = get_albums(
            "/Users/jeffb/Library/Mobile Documents/com~apple~CloudDocs/rust_basic/image_play/test_items/pics"
        ).unwrap();

        let curr = albums.pop().unwrap();

        let mut app = Self {
            images: Vec::new(),
            albums: albums,
            instant: Instant::now(),
            current_album: curr,
        };

        app.load_pics();
        app
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // TODO magic numbers, at least make a refresh time constant and subtract from
            // it for the if check.
            ctx.request_repaint_after(Duration::new(3, 0));
            if self.instant.elapsed().as_millis() > 2990 {
                println!("getting new images!");
                self.load_pics();
                self.instant = Instant::now();
            }

            // TODO need to scale this automatically based on number of images.
            let max_height = ctx.available_rect().height() / 2.0;
            let max_width = ctx.available_rect().width() / 3.0;
            let max_size = vec2(max_width, max_height);

            let mut x_pos: f32 = 0.0;
            let mut y_pos: f32 = 0.0;

            ui.heading(self.current_album.name.to_owned());

            egui::Grid::new("some_unique_id").show(ui, |ui| {
                for (pos, i) in self.images.iter().enumerate() {
                    let w = egui::Window::new(i.name.to_owned())
                        .fixed_size(max_size)
                        .fixed_pos((x_pos, y_pos))
                        .show(ctx, |ui| {
                            //ui.label("Hello World!");
                            ui.add(egui::Image::new(
                                i.image.texture_id(ctx),
                                // TODO do something better to scale the images
                                i.image.size_vec2() * 0.3,
                            ));
                        });

                    x_pos += max_width;
                    if (pos + 1) % 3 == 0 {
                        ui.end_row();
                        x_pos = 0.0;
                        y_pos += max_height;
                    }
                }
            });
        });
    }
}
