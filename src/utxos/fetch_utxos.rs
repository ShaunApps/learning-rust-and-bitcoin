extern crate hyper;

use std::env;
use std::io::{self, Write};

use hyper::Client;
use hyper::rt::{self, Future, Stream};


// fn get_address_and_fetch() -> String {

//     // Some simple CLI args requirements...
//     let address = match env::args().nth(1) {
//         Some(address) => address[1],
//         None => {
//             println!("Usage: address <address>");
//             return;
//         }
//     };

//     let mut url: String = "http://blockchain.info/rawaddr/".to_owned();

//     url.push_str(address);
//     println!("{}", url);
    


// }

    
//     // HTTPS requires picking a TLS implementation, so give a better
//     // warning if the user tries to request an 'https' URL.
//     let url = url.parse::<hyper::Uri>().unwrap();
//     if url.scheme_part().map(|s| s.as_ref()) != Some("http") {
//         println!("This example only works with 'http' URLs.");
//         return;
//     }

//     // Run the runtime with the future trying to fetch and print this URL.
//     //
//     // Note that in more complicated use cases, the runtime should probably
//     // run on its own, and futures should just be spawned into it.
//     rt::run(fetch_url(url));
// }

// fn fetch_url(url: hyper::Uri) -> impl Future<Item=(), Error=()> {
//     let client = Client::new();

//     client
//         // Fetch the url...
//         .get(url)
//         // And then, if we get a response back...
//         .and_then(|res| {
//             println!("Response: {}", res.status());
//             println!("Headers: {:#?}", res.headers());

//             // The body is a stream, and for_each returns a new Future
//             // when the stream is finished, and calls the closure on
//             // each chunk of the body...
//             res.into_body().for_each(|chunk| {
//                 io::stdout().write_all(&chunk)
//                     .map_err(|e| panic!("example expects stdout is open, error={}", e))
//             })
//         })
//         // If all good, just tell the user...
//         .map(|_| {
//             println!("\n\nDone.");
//         })
//         // If there was an error, let the user know...
//         .map_err(|err| {
//             println!("Error {}", err);
//         })
// }