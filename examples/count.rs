use std::rc::Rc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

#[tokio::main]
async fn main() -> Result<(), &'static str> {
	pretty_env_logger::init();
	let counter = Rc::new(AtomicUsize::new(0));
	retrier::retry(move || {
		let counter = counter.clone();
		async move {
			let num = counter.load(Ordering::Relaxed);
			if num > 1 {
				Ok(true)
			} else {
				counter.store(num + 1, Ordering::Relaxed);
				Err("nope")
			}
		}
	})
	.await?;
	Ok(())
}
