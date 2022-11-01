mod templates;

use perseus::{Html, PerseusApp};
use templates::index;

#[perseus::main(perseus_warp::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new().template(index::get_template)
}