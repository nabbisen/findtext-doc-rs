mod core;

use core::search::handle;

/// entry point
pub fn search(keyword: &str, filepath: &str) -> Result<bool, Box<dyn std::error::Error>> {
    handle(keyword, filepath)
}
