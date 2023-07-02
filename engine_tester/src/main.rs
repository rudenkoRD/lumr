use lumr::errors::Errors;
use std::env;

fn main() {
    lumr::logger::init();
    let res = error_test(1);
}

fn error_test(num:i32) -> Result<(),Errors> {
    lumr::logger::info!("Error");
    if num == 1 {
        return Err(Errors::TestError.into());
    }
    Ok(())
}
