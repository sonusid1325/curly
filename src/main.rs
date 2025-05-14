use clap::{Arg, Command};

fn main() {
    let matches = Command::new("rust-curl")
        .version("1.0")
        .author("sonusid1325")
        .about("Stylish HTTP client like curl")
        .arg(Arg::new("url").required(true).help("The URL to request"))
        .arg(
            Arg::new("method")
                .short('X')
                .long("request")
                .default_value("GET")
                .help("HTTP method to use"),
        )
        .get_matches();

    let url = matches.get_one::<String>("url").unwrap();
    let method = matches.get_one::<String>("method").unwrap();

    println!("Method: {}, URL: {}", method, url);
}
