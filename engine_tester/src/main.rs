use lumr::core::application::Application;

fn main() {
    lumr::logger::init();
    
    let application = Application::new();
    application.window.run_event_loop();
}
