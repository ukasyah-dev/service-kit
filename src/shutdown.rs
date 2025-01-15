use signal_hook::{
    consts::{SIGINT, SIGTERM},
    iterator::Signals,
};

pub async fn wait_for_signal() {
    let mut signals = Signals::new(&[SIGINT, SIGTERM]).unwrap();

    for _ in signals.forever() {
        log::info!("Received signal, shutting down");
        break;
    }
}
