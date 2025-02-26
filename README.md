
# Event Tracker

Event Tracker is a desktop application built in Rust that monitors mouse and keyboard events and logs them into a CSV file. The project leverages Rust libraries such as [rdev](https://crates.io/crates/rdev) for capturing input events, [csv](https://crates.io/crates/csv) for handling CSV file operations, and [eframe/egui](https://crates.io/crates/eframe) for the graphical user interface.


## Demo Video 
![Demo GIF](./demo.gif)



## Features

- **Event Tracking:** Captures keyboard and mouse events in real-time.
- **CSV Logging:** Writes event data along with timestamps to a CSV file (`events.csv`).
- **Desktop UI:** A simple UI displaying a status message ("Tracking active") using eframe/egui.
- **Background Processing:** Uses a background thread to ensure UI responsiveness while tracking events.


## How it works

- **Event Capturing:** The application spawns a background thread that listens to keyboard and mouse events using the rdev crate. Depending on the event type (e.g., key press, mouse move), the event is logged with a timestamp.


- **CSV File Logging:** If the events.csv file does not exist, it is created and initialized with appropriate headers. Otherwise, new event data is appended to the file.


- **User Interface:** A minimal GUI built with eframe and egui displays a simple message indicating that the event tracking is active.


## Prerequisites

Before building the project, ensure that you have the following installed:

- **Rust Toolchain:** You can install it via [rustup](https://rustup.rs/).
- **Linux Dependencies:** On Debian-based distributions (like Kali Linux), install the XCB development libraries:
  
  ```bash
  sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev
```
