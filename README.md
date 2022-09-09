<p align="center">
<img src="https://app.ipbase.com/img/logo/ipbase.png" width="300"/>
</p>

# ipbase-rs: Rust geolocation service via ipbase.com

This package is a Rust wrapper for [ipbase.com] that aims to make the usage of the API as easy as possible in your project.


## Installation

This crate is under development. Especially the response parsing needs some more testing. However, if you still want to use it, you can install it by adding this to your `Cargo.toml`:

```toml
[dependencies]
ipbase = "0.1.0"
```

## Requirements

1. API Key for [ipbase.com](https://ipbase.com/)
2. Async runtime like [tokio](https://crates.io/crates/tokio)

## Quickstart

```rust
use ipbase::Ipbase;
use ipbase::models;

async fn request_latest() -> Result<models::DetailsResponse, ipbase::Error> {
    let ipbase_api = Ipbase::new("<your-api-key>")?;
    let details = ipbase_api.info("1.1.1.1").await?;
     Ok(details)
}
```

Find out more about our endpoints, parameters and response data structure in the [docs]

## License

The MIT License (MIT). Please see [License File](LICENSE.md) for more information.

[docs]: https://ipbase.com/docs
[ipbase.com]: https://ipbase.com