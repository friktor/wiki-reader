extern crate gtk;
mod app;

fn main() {
  use app::{ Application };

  if gtk::init().is_err() {
    println!("Failed to initialize GTK.");
    return;
  } else {
    let app = Application::new();
    app.prepare_and_run();
  }
}