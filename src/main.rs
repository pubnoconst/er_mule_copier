mod save_model;
mod cli_app;
mod gui_app;
mod file_io;
mod helpers;
mod testing;

fn main() {
    if std::env::args().len() == 5 {
        cli_app::run() 
    } else {
        gui_app::run()
    }
}
