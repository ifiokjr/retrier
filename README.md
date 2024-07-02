<h1 align="center">
  retrier ♻️
</h1>

<p align="center">
   A wasm-compatible retry library for futures
</p>

<div align="center">
  <a href="https://github.com/ifiokjr/retrier/actions">
		<img src="https://github.com/ifiokjr/retrier/workflows/ci/badge.svg"/>
	</a>
  <a href="https://crates.io/crates/retrier">
		<img src="https://img.shields.io/crates/v/retrier"/>
	</a>
  <a href="http://docs.rs/retrier">
		<img src="https://docs.rs/retrier/badge.svg"/>
	</a>  
  <a href="https://ifiokjr.github.io/retrier">
		<img src="https://img.shields.io/badge/docs-master-green.svg"/>
	</a>
</div>

<br />

A goal of any operation should be a successful outcome. This crate gives operations a better chance at achieving that.

## 📦 install

In your Cargo.toml file, add the following under the `[dependencies]` heading

```toml
retrier = "0.1"
```

## 📝note

This is fork of the [`again`](https://github.com/softprops/again) library. This fork brings it up to date and adds to automation features which should make maintainance simpler.

## 🤸usage

For very simple cases you can use the module level `retry` function
to retry a potentially fallible operation.

```rust
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    retrier::retry(|| reqwest::get("https://api.you.com")).await?;
    Ok(())
}
```

You may not want to retry _every_ kind of error. For preciseness you can be more explicit in which kinds of errors should be retried with the module level `retry_if` function.

```rust
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    retrier::retry_if(
      || reqwest::get("https://api.you.com")
      reqwest::Error::is_status
    ).await?;
    Ok(())
}
```

You can also customize retry behavior to suit your applications needs
with a configurable and reusable `RetryPolicy`.

```rust
use std::error::Error;
use std::time::Duration;
use retrier::RetryPolicy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    let policy = RetryPolicy::exponential(Duration::from_millis(200))
      .with_max_retries(10)
      .with_jitter(true);
    policy.retry(|| reqwest::get("https://api.you.com")).await?;
    Ok(())
}
```

See the [docs](http://docs.rs/retrier) for more examples.

