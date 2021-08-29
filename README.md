# surf_bucket_select

A temporary project to use bucket select with surf. Actualy, Rusoto is under migration to a new SDK.

## Installation

 ```Toml
[dependencies]
surf_bucket_select = "0.2.2"
# select your client: `curl-client` or `h1-client` or etc ... ( <https://github.com/http-rs/surf> )
surf = { version = "2.3", default-features = false, features = ["h1-client", "middleware-logger", "encoding"] }
```

## Usage

See the examples 
* `./examples/read_csv_file.rs` 
* `./examples/read_json_file.rs` 
  
how to use this library. 

If you want to run this example :

 ```sh
cp .env.dist .env
make start
cargo run --example read_csv_file
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[Apache](https://choosealicense.com/licenses/apache-2.0/)
[MIT](https://choosealicense.com/licenses/mit/)
