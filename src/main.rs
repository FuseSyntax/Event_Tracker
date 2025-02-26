use std::fs::{File, OpenOptions};
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use std::time::UNIX_EPOCH;

use csv::WriterBuilder;
use eframe::egui;
use rdev::{Event, EventType, listen};
use eframe::epi::{App, Frame};

fn main() {
    // Create an mpsc channel to send event descriptions to the UI thread
    let (tx, rx) = mpsc::channel::<String>();

    // Start a background thread to track events and write them to CSV
    thread::spawn(move || {
        // Open or create the CSV file
        let path = Path::new("events.csv");
        let file = if path.exists() {
            OpenOptions::new().append(true).open(path).unwrap()
        } else {
            let file = File::create(path).unwrap();
            let mut writer = WriterBuilder::new().from_writer(file);
            writer.write_record(&["timestamp", "event_type", "key", "button", "x", "y"]).unwrap();
            writer.flush().unwrap();
            OpenOptions::new().append(true).open(path).unwrap()
        };
        let mut writer = WriterBuilder::new().has_headers(false).from_writer(file);

        // Store the current mouse position
        thread_local! {
            static CURRENT_POS: std::cell::RefCell<(f64, f64)> = std::cell::RefCell::new((0.0, 0.0));
        }

        // Define how to handle each event
        let callback = move |event: Event| {
            let timestamp = event.time.duration_since(UNIX_EPOCH).unwrap().as_millis();
            let mut message = String::new();
            match event.event_type {
                EventType::KeyPress(key) => {
                    message = format!("{}: Key Press: {:?}", timestamp, key);
                    let row = vec![
                        timestamp.to_string(),
                        "key_press".to_string(),
                        format!("{:?}", key),
                        "".to_string(),
                        "".to_string(),
                        "".to_string(),
                    ];
                    writer.write_record(&row).unwrap();
                }
                EventType::KeyRelease(key) => {
                    message = format!("{}: Key Release: {:?}", timestamp, key);
                    let row = vec![
                        timestamp.to_string(),
                        "key_release".to_string(),
                        format!("{:?}", key),
                        "".to_string(),
                        "".to_string(),
                        "".to_string(),
                    ];
                    writer.write_record(&row).unwrap();
                }
                EventType::MouseMove { x, y } => {
                    CURRENT_POS.with(|pos| *pos.borrow_mut() = (x, y));
                    message = format!("{}: Mouse Move: x={}, y={}", timestamp, x, y);
                    let row = vec![
                        timestamp.to_string(),
                        "mouse_move".to_string(),
                        "".to_string(),
                        "".to_string(),
                        x.to_string(),
                        y.to_string(),
                    ];
                    writer.write_record(&row).unwrap();
                }
                EventType::ButtonPress(button) => {
                    CURRENT_POS.with(|pos| {
                        let (x, y) = *pos.borrow();
                        message = format!("{}: Button Press: {:?} at x={}, y={}", timestamp, button, x, y);
                        let row = vec![
                            timestamp.to_string(),
                            "button_press".to_string(),
                            "".to_string(),
                            format!("{:?}", button),
                            x.to_string(),
                            y.to_string(),
                        ];
                        writer.write_record(&row).unwrap();
                    });
                }
                EventType::ButtonRelease(button) => {
                    CURRENT_POS.with(|pos| {
                        let (x, y) = *pos.borrow();
                        message = format!("{}: Button Release: {:?} at x={}, y={}", timestamp, button, x, y);
                        let row = vec![
                            timestamp.to_string(),
                            "button_release".to_string(),
                            "".to_string(),
                            format!("{:?}", button),
                            x.to_string(),
                            y.to_string(),
                        ];
                        writer.write_record(&row).unwrap();
                    });
                }
                _ => {}
            }
            writer.flush().unwrap();

            // Send the event message to the UI
            if !message.is_empty() {
                // Ignore error if the receiver has been dropped
                let _ = tx.send(message);
            }
        };

        // Start listening to events
        if let Err(error) = listen(callback) {
            println!("Error: {:?}", error);
        }
    });

    // Set up and run the UI, passing the receiver
    let app = MyApp {
        rx,
        events: Vec::new(),
    };
    eframe::run_native(Box::new(app), eframe::NativeOptions::default());
}

// Define the UI structure
struct MyApp {
    rx: mpsc::Receiver<String>,
    events: Vec<String>,
}

impl App for MyApp {
    fn name(&self) -> &str {
        "Event Tracker"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut Frame) {
        ctx.set_visuals(egui::Visuals::dark());
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Tracking active");
            ui.separator();
            ui.heading("Recent Events:");

            // Drain new events from the channel (non-blocking)
            while let Ok(event) = self.rx.try_recv() {
                self.events.push(event);
            }

            // Display only the last 10 events
            let start = if self.events.len() > 10 { self.events.len() - 10 } else { 0 };
            for event in &self.events[start..] {
                ui.label(event);
            }
        });
    }
}
