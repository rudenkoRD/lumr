use lumr::graphics::window::GraphicsWindow;

fn main() {
    lumr::logger::init();
    
    let window = GraphicsWindow::new();
    window.run_event_loop();
}
