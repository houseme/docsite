#![allow(unused)]
use dioxus::html::input_data::keyboard_types::{Key, Modifiers};
use slab::Slab;
use std::sync::{Arc, Mutex};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

use std::cell::RefCell;
#[cfg(feature = "web")]
thread_local! {
	static LISTENERS: ShortcutHandler = {
		let callbacks: Arc<Mutex<Slab<(Key, Modifiers, Box<dyn FnMut() >)>>> = Arc::new(Mutex::new(Slab::new()));
		let callbacks2 = callbacks.clone();

		let cb: Closure<dyn FnMut(web_sys::Event)> = wasm_bindgen::closure::Closure::new(move |evt: web_sys::Event| {
			let data = dioxus::prelude::KeyboardData::from(evt);
			for (_, (key, modifiers, callback)) in callbacks2.lock().unwrap().iter_mut() {
				if data.key() == *key && data.modifiers() == *modifiers {
					callback();
				}
			}
		});
		let window = web_sys::window().unwrap();
		let document = window.document().unwrap();
		document
		.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref())
		.unwrap();
		cb.forget();

		ShortcutHandler { callbacks }
	};
}

struct ShortcutHandler {
	callbacks: Arc<Mutex<Slab<(Key, Modifiers, Box<dyn FnMut()>)>>>,
}

impl ShortcutHandler {
	fn add(&mut self, key: Key, modifiers: Modifiers, cb: Box<dyn FnMut()>) -> usize {
		self.callbacks.lock().unwrap().insert((key, modifiers, cb))
	}

	fn remove(&mut self, id: usize) {
		self.callbacks.lock().unwrap().remove(id);
	}
}

/// Create a global shortcut that will be removed when the component is unmounted
pub fn use_shortcut(
	cx: &dioxus::prelude::ScopeState,
	key: Key,
	modifiers: crate::Modifiers,
	mut handler: impl FnMut() + 'static,
) {
	#[cfg(feature = "web")]
	{
		cx.use_hook(move || {
			ShortcutHandle(LISTENERS.with(|l| {
				l.callbacks
					.lock()
					.unwrap()
					.insert((key, modifiers, Box::new(handler)))
			}))
		});
	}
}

struct ShortcutHandle(usize);

impl Drop for ShortcutHandle {
	fn drop(&mut self) {
		#[cfg(feature = "web")]
		{
			LISTENERS.with(|l| l.callbacks.lock().unwrap().remove(self.0));
		}
	}
}
