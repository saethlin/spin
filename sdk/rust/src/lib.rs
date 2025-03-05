pub use spin_macro::*;

pub mod redis {
    wit_bindgen_rust::import!("../../wit/ephemeral/outbound-redis.wit");
}
