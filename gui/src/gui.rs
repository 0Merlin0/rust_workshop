use crate::{grid, GuiController};
use crate::board::Board;
use eframe::egui;
use std::sync::mpsc;
use std::thread;
use std::path::PathBuf;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use egui::Pos2;

enum PlayState {
    Paused,
    Playing(Instant),
}

pub enum DisplayRequest {
    Text(String),
    DisplayClickable(Board),
    DisplayOnce(Board),
    Full(Board),
}

pub enum DisplayResult {
    Full(Board),
}


struct Loader {
    path: PathBuf,
    boards: HashMap<String, Board>,
}

impl Loader {
    fn new() -> Self {
        let mut loader = Loader { path: PathBuf::from("./saved_boards.json"), boards: HashMap::new() };
        loader.refresh();
        loader
    }

    fn refresh(&mut self) {
        if let Ok(bytes) = std::fs::read(&self.path) {
            let json = String::from_utf8_lossy(&bytes).to_string();
            if let Ok(loaded) = serde_json::from_str(&json) {
                self.boards = loaded;
            }
        }
    }

    fn save(&mut self, name: &str, board: &Board) {
        self.boards.entry(name.to_string()).and_modify(|current| *current = board.clone()).or_insert(board.clone());
         if let Ok(json) = serde_json::to_string(&self.boards) {
            let _ = std::fs::write(&self.path, json.into_bytes());
        }
    }
    fn load(&mut self, name: &str, board: &mut Board) {
        self.refresh();
        if let Some(b) = self.boards.get(name) {
            *board = b.clone();
        }
    }
}

enum WindowState {
    None,
    Load(Pos2),
    Save(String, Pos2),

}

struct MyApp {
    state: DisplayRequest,
    playing: PlayState,
    speed: u64,
    sender: mpsc::Sender<DisplayResult>,
    receiver: mpsc::Receiver<DisplayRequest>,
    loader: Loader,
    window: WindowState,
}

impl MyApp {
    fn new(
        sender: mpsc::Sender<DisplayResult>,
        receiver: mpsc::Receiver<DisplayRequest>,
    ) -> Self {
        Self {
            state: DisplayRequest::Text("".to_string()),
            playing: PlayState::Paused,
            speed: 1,
            sender,
            receiver,
            loader: Loader::new(),
            window: WindowState::None,
        }
    }
}

pub fn init<F>(func: F) -> Result<(), eframe::Error>
where
    F: Fn(GuiController),
    F: Send + 'static,
{
    let (logic_send, gui_receive) = mpsc::channel::<DisplayRequest>();
    let (gui_send, logic_receive) = mpsc::channel::<DisplayResult>();

    let thread_join_handle = thread::spawn(move || {
        let gui_controller = GuiController { sender: logic_send, receiver: logic_receive };
        func(gui_controller);
    });

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(640.0, 640.0)),
        ..Default::default()
    };
    let res = eframe::run_native(
        "Game of Life",
        options,
        Box::new(|_cc| Box::<MyApp>::new(MyApp::new(gui_send, gui_receive))),
    );
    thread_join_handle.join().unwrap();
    res
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {


            if let Ok(s) = self.receiver.try_recv() { self.state = s }

            match &mut self.state {
                DisplayRequest::Full(board) => {
                    ui.heading("Game of Life");
                    ui.add(egui::Slider::new(&mut board.width, 3..=50).text("width"));
                    ui.add(egui::Slider::new(&mut board.height, 3..=50).text("height"));
                    board.adjust_vector();
                    let elapsed_duration = match self.playing {
                        PlayState::Paused => Duration::from_millis(0),
                        PlayState::Playing(last) => last.elapsed(),
                    };

                    ui.add(egui::Slider::new(&mut self.speed, 1..=100).text("speed"));

                    let play_duration = Duration::from_millis(2500 / self.speed);

                    let needs_update = ui.button("Step").clicked() || elapsed_duration >= play_duration;
                    let play_button = match self.playing {
                        PlayState::Paused => ui.button("Play"),
                        PlayState::Playing(_) => ui.button("Pause"),
                    };
                    if play_button.clicked() {
                        self.playing = match self.playing {
                            PlayState::Paused => PlayState::Playing(Instant::now()),
                            PlayState::Playing(_) => PlayState::Paused,
                        };
                    }

                    match &mut self.window {
                        WindowState::Load(pos) => {
                            let mut open = true;
                            egui::Window::new("Load").default_pos(*pos).collapsible(false).open(&mut open).show(ctx, |ui| {
                                let mut keys: Vec<String> = self.loader.boards.iter().map(|(k, _v)|k.clone()).collect();
                                keys.sort();
                                for name in keys {
                                   if ui.button(&name).clicked() {
                                       self.window = WindowState::None;
                                       self.loader.load(&name, board);
                                   }
                                }
                            });
                            if !open {
                                self.window = WindowState::None;
                            }
                        },
                        WindowState::Save(text, pos) => {
                            let mut open = true;
                            let mut saved = false;
                            egui::Window::new("Save").default_pos(*pos).collapsible(false).open(&mut open).show(ctx, |ui| {

                                ui.label("Save as:");
                                ui.text_edit_singleline(text);
                                if ui.button("Save").clicked() {
                                    saved = true;
                                    self.loader.save(&text, board);
                                }
                            });
                            if !open || saved {
                                self.window = WindowState::None;
                            }
                        },
                        WindowState::None => (),
                    };

                    egui::Grid::new("Loader Buttons")
                    .num_columns(2)
                    .show(ui, |ui| {
                        let save_button = ui.button("Save");
                        if save_button.clicked() {
                            let pos = save_button.interact_pointer_pos().unwrap_or(Pos2::ZERO);
                            self.window = WindowState::Save(String::new(), pos);
                        }
                        let load_button = ui.button("Load");
                        if load_button.clicked() {
                            let pos = load_button.interact_pointer_pos().unwrap_or(Pos2::ZERO);
                            self.window = WindowState::Load(pos);
                        }
                    });
                    ui.add(grid::grid(board, true));
                    if needs_update {
                        //let board = std::mem::take(&mut self.board);
                        self.sender.send(DisplayResult::Full(board.clone())).unwrap_or_else(|_| {
                            eprintln!("Logic code has exitted before GUI");
                            std::process::exit(1);
                        });
                        self.playing = match self.playing {
                            PlayState::Playing(_) => PlayState::Playing(Instant::now()),
                            PlayState::Paused => PlayState::Paused,
                        }
                    }
                    ctx.request_repaint_after(play_duration / 4);

                },
                DisplayRequest::DisplayClickable(board) => {
                    ui.heading("Game of Life");
                    let elapsed_duration = match self.playing {
                        PlayState::Paused => Duration::from_millis(0),
                        PlayState::Playing(last) => last.elapsed(),
                    };

                    ui.add(egui::Slider::new(&mut self.speed, 1..=100).text("speed"));

                    let play_duration = Duration::from_millis(2500 / self.speed);

                    let needs_update = ui.button("Step").clicked() || elapsed_duration >= play_duration;
                    let play_button = match self.playing {
                        PlayState::Paused => ui.button("Play"),
                        PlayState::Playing(_) => ui.button("Pause"),
                    };
                    if play_button.clicked() {
                        self.playing = match self.playing {
                            PlayState::Paused => PlayState::Playing(Instant::now()),
                            PlayState::Playing(_) => PlayState::Paused,
                        };
                    }
                    ui.add(grid::grid(board, true));
                    if needs_update {
                        //let board = std::mem::take(&mut self.board);
                        self.sender.send(DisplayResult::Full(board.clone())).unwrap_or_else(|_| { 
                            eprintln!("Logic code has exitted before GUI");
                            std::process::exit(1);
                        });
                        self.playing = match self.playing {
                            PlayState::Playing(_) => PlayState::Playing(Instant::now()),
                            PlayState::Paused => PlayState::Paused,
                        }
                    }
                    ctx.request_repaint_after(play_duration / 4);
                },
                DisplayRequest::DisplayOnce(board) => {
                    ui.add(grid::grid(board, false));
                    if ui.button("Exit").clicked() {
                        std::process::exit(0);
                    }
                },
                DisplayRequest::Text(s) => {
                    ui.heading(s);
                    if ui.button("Exit").clicked() {
                        std::process::exit(0);
                    }
                }
            }

        });
    }
}
