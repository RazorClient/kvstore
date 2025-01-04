// use clap::{App, Arg};
// use my_kv_store::ActionKvStore;  // from our library crate

// fn main() {
//     let matches = App::new("V Store")
//         .version("1.0")
//         .author("Your Name <your_email@example.com>")
//         .about("A simple key-value store CLI")
//         .arg(
//             Arg::with_name("file")
//                 .required(true)
//                 .help("The database file")
//         )
//         .arg(
//             Arg::with_name("action")
//                 .required(true)
//                 .help("Action to perform (get, delete, insert, update)")
//         )
//         .arg(
//             Arg::with_name("key")
//                 .required(true)
//                 .help("Key to perform the action on")
//         )
//         .arg(
//             Arg::with_name("value")
//                 .help("Value for insert or update")
//         )
//         .get_matches();

//     let fname = matches.value_of("file").unwrap();
//     let action = matches.value_of("action").unwrap();
//     let key = matches.value_of("key").unwrap();
//     let maybe_value = matches.value_of("value");

//     // Initialize store
//     let mut store = match ActionKvStore::open(fname) {
//         Ok(store) => store,
//         Err(e) => {
//             eprintln!("Error opening the file: {}", e);
//             std::process::exit(1);
//         }
//     };

//     // Execute action
//     let result = match action {
//         "get" => {
//             match store.get(key) {
//                 Ok(Some(value)) => {
//                     println!("{}", value);
//                     Ok(())
//                 }
//                 Ok(None) => {
//                     eprintln!("Key not found: {}", key);
//                     Ok(())
//                 }
//                 Err(e) => Err(e),
//             }
//         }
//         "delete" => store.delete(key),
//         "insert" => {
//             let value = maybe_value.ok_or("Value is required for insert")?;
//             store.insert(key, value)
//         }
//         "update" => {
//             let value = maybe_value.ok_or("Value is required for update")?;
//             store.update(key, value)
//         }
//         _ => {
//             eprintln!("Unknown action: {}", action);
//             std::process::exit(1);
//         }
//     };

//     // Handle any errors that came from performing the action
//     if let Err(e) = result {
//         eprintln!("Error performing {} on key {}: {}", action, key, e);
//         std::process::exit(1);
//     }
// }
