extern crate futures;
extern crate hyper;
extern crate tokio_core;

use std::fs::*;
use std::io::{Write, Read};
use futures::{Future, Stream};
use hyper::mime::Mime;
use hyper::header::{Headers, ContentType};
use hyper::{Method, Request, Client, Uri};
use tokio_core::reactor::Core;
use std::string::String;

const XLSX_FORMAT: &str = "xlsx";

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        panic!("Please provide directory with excel files and target endpoint.");;
    }

    let mut files: Vec<DirEntry> = Vec::new();
    std::fs::read_dir(args[1].as_str()).unwrap()
        .for_each(|file| {
            files.push(file.unwrap());
        });


    let excel_files: Vec<File> = files.into_iter()
        .filter(|file| {
            return file.file_name().to_str().unwrap().ends_with(XLSX_FORMAT);
        })
        .map(|file| {
            return File::open(file.path().as_path()).unwrap();
        }).collect();

    println!("Found {} excel files in target directory", excel_files.len());


    let mut core: Core = Core::new().unwrap();
    let uri: Uri = args[2].parse().unwrap();

    for file in excel_files {
        upload_file::<String>(file, &mut core, uri.clone());
    }
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
        Ok(_) => {println!("Upload successful")},
        Err(_) => {println!("Upload failed")},
    }
}

