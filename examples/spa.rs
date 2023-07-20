use dioxus_docs_site::app;

fn main() {
	wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
	#[cfg(feature = "web")]
	dioxus_web::launch(app)
}
