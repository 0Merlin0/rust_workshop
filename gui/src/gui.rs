use crate::{grid, GuiController};
use crate::simple_display::Board;
use eframe::egui;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

enum PlayState {
    Paused,
    Playing(Instant),
}

pub enum DisplayRequest {
    Display(Board),
    Full(Board),
}

pub enum DisplayResult {
    Display(bool),
    Full(Board),
}


struct MyApp {
    state: DisplayRequest,
    playing: PlayState,
    speed: u64,
    sender: mpsc::Sender<DisplayResult>,
    receiver: mpsc::Receiver<DisplayRequest>,
}

impl MyApp {
    fn new(
        sender: mpsc::Sender<DisplayResult>,
        receiver: mpsc::Receiver<DisplayRequest>,
        width: usize,
        height: usize,
    ) -> Self {
        Self {
            state: DisplayRequest::Display(Board::new(width, height)),
            playing: PlayState::Paused,
            speed: 1,
            sender,
            receiver,
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
    //loop {
    //    let board = match logic_receive.recv() {
    //        Ok(board) => func(board),
    //        _ => return,
    //    };
    //    match logic_send.send(board) {
    //        Ok(()) => (),
    //        _ => return,
    //    };
    //});

    let options = eframe::NativeOptions {
        //initial_window_size: Some(egui::vec2(640.0, 640.0)),
        ..Default::default()
    };
    let res = eframe::run_native(
        "Game of Life",
        options,
        Box::new(|_cc| Box::<MyApp>::new(MyApp::new(gui_send, gui_receive, 10, 10))),
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
                    ui.add(grid::grid(board, true));
                    if needs_update {
                        //let board = std::mem::take(&mut self.board);
                        self.sender.send(DisplayResult::Full(board.clone())).unwrap();
                        self.playing = match self.playing {
                            PlayState::Playing(_) => PlayState::Playing(Instant::now()),
                            PlayState::Paused => PlayState::Paused,
                        }
                    }
                    ctx.request_repaint_after(play_duration / 4);

                },
                DisplayRequest::Display(board) => {
                    ui.add(grid::grid(board, false));
                    if ui.button("Continue").clicked() {
                        self.sender.send(DisplayResult::Display(true)).unwrap();
                    }
                }
            }

        });
    }
}
