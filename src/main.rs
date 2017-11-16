extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate clap;

use std::fs::*;
use std::io::Read;
use futures::{Future, Stream};
use hyper::header::ContentType;
use hyper::{Method, Request, Client, Uri};
use tokio_core::reactor::Core;
use std::string::String;
use clap::{Arg, App};

const XLSX_FORMAT: &str = "xlsx";

fn main() {
    let matches = App::new("Krmítko")
        .version("0.2.0")
        .author("Pavel Pscheidl <pavel.junior@pscheidl.cz>")
        .about("Uploads Microsoft Excel files via HTTP POST as an octet stream to an endpoint of choice.")
        .arg(Arg::with_name("DIRECTORY")
            .short("d")
            .long("directory")
            .value_name("DIRECTORY PATH")
            .help("Directory to scan for excel files")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("ENDPOINT")
            .short("e")
            .long("endpoint")
            .value_name("ENDPOINT URL")
            .help("Endpoint to HTTP POST files to")
            .required(true)
            .takes_value(true))
        .get_matches();

    let directory: &str = matches.value_of("DIRECTORY").unwrap_or_else(|| {
        panic!("Please set target directory");
    });

    let endpoint: &str = matches.value_of("ENDPOINT").unwrap_or_else(|| {
        panic!("Please specify target endpoint");
    });

    let directory_entries: Vec<DirEntry> = find_files(directory);

    let mut core: Core = Core::new().unwrap();
    let uri: Uri = endpoint.parse().unwrap();

    for dir_entry in directory_entries {
        let file = File::open(dir_entry.path().as_path()).unwrap();
        println!("Beginning upload of {:?}", dir_entry.file_name());
        upload_file::<String>(file, &mut core, uri.clone());
    }
}

fn find_files(directory: &str) -> Vec<DirEntry> {
    let files: Vec<DirEntry> = std::fs::read_dir(directory).unwrap()
        .map(|file| {
            file.unwrap()
        }).filter(|file| {
        return file.file_name().to_str().unwrap().ends_with(XLSX_FORMAT);
    })
        .collect();

    println!("Found {} excel files in target directory \'{}\' \n", files.len(), directory);

    return files;
}

fn upload_file<T>(mut file: File, core: &mut Core, uri: Uri) {
    let mut file_content: Vec<u8> = Vec::new();
    let num: usize = file.read_to_end(&mut file_content).unwrap();

    println!("Uploading {} bytes to {}", num, uri);

    let mut request: Request = Request::new(Method::Post, uri);
    request.headers_mut().set(ContentType::octet_stream());
    request.set_body(file_content);
    let client = Client::new(&core.handle());
    let post = client.request(request)
        .and_then(|res| {
            println!("Server responded with {}", res.status());

            res.body().concat2()
        });

    let status = core.run(post);
    match status {
        Ok(_) => { println!("Upload successful \n") }
        Err(_) => { println!("Upload failed \n") }
    }
}

