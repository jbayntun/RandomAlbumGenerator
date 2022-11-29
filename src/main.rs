// Using the right values in the toml is important! https://github.com/emilk/egui/blob/eeae485629fca24a81a7251739460b671e1420f7/examples/retained_image/Cargo.toml#L14
// Started with this sample: https://github.com/emilk/egui/blob/eeae485629fca24a81a7251739460b671e1420f7/examples/retained_image/src/main.rs

// This sample seems to move all over in the last few version changes...

use log::debug;
use rand::seq::IteratorRandom;
use std::fs;
use std::time::{Duration, Instant};

use eframe::egui;
use egui::vec2;
use egui_extras::RetainedImage;
use image_play::{get_albums, get_randoms_from_album, Album};

/// Seconds that an album will be on screen.
const ROTATION_DURATION: u64 = 10;

fn main() {
    let options = eframe::NativeOptions {
        fullscreen: true,
        ..Default::default()
    };

    eframe::run_native(
        "Random Album Generator",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct AppImage {
    image: RetainedImage,
    name: String,
}

struct MyApp {
    root_dir: String,
    images: Vec<AppImage>,
    albums: Vec<Album>,
    current_album: Option<Album>,
    instant: Instant,
}

impl MyApp {
    /// Loads random images as RetainedImage from the album.
    fn load_pics(&mut self) {
        self.images.clear();
        match &self.current_album {
            None => {
                debug!("Bad things happened, somehow there's no current image.");
            }
            Some(album) => {
                for image in get_randoms_from_album(&album, 6) {
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
    }

    fn get_random_album(&mut self) {
        //if our albums are empty, reload from disc
        if 0 == self.albums.len() {
            self.albums = get_albums(&self.root_dir).unwrap();
        }

        // select a random album and set the current to point at it
        let i = (0..self.albums.len())
            .choose(&mut rand::thread_rng())
            .unwrap();
        self.current_album = Some(self.albums.swap_remove(i));

        self.load_pics();
    }
}

impl Default for MyApp {
    fn default() -> Self {
        let mut app = Self {
            // TODO make the root dir a parameter or environment variable.
            root_dir: "test_items/final_test".to_string(),
            images: Vec::new(),
            albums: Vec::new(),
            instant: Instant::now(),
            current_album: None,
        };

        app.get_random_album();
        app
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // TODO magic numbers, at least make a refresh time constant and subtract from
            // it for the if check.
            ctx.request_repaint_after(Duration::new(ROTATION_DURATION, 0));
            if self.instant.elapsed().as_millis() > (1000 * ROTATION_DURATION as u128) - 20 {
                println!("getting new images!");
                self.get_random_album();
                self.instant = Instant::now();
            }

            // TODO need to scale this automatically based on number of images.
            let max_height = ctx.available_rect().height() / 2.0;
            let max_width = ctx.available_rect().width() / 3.0;
            let max_size = vec2(max_width, max_height);

            let mut x_pos: f32 = 0.0;
            // This barely lets the album name be shown, but means there is no margin at the bottom.
            let mut y_pos: f32 = 30.0;

            let album_name = match &self.current_album {
                None => {
                    debug!(
                        "Bad things happened, somehow there's no current image in the main loop"
                    );
                    "!!! ERROR fake name".to_owned()
                }
                Some(album) => album.name.to_owned(),
            };

            ui.heading(album_name);

            egui::Grid::new("some_unique_id").show(ui, |ui| {
                for (pos, i) in self.images.iter().enumerate() {
                    egui::Window::new(i.name.to_owned())
                        .fixed_size(max_size)
                        .fixed_pos((x_pos, y_pos))
                        .show(ctx, |ui| {
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
