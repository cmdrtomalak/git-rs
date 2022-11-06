use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};
#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::Path;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let args: Vec<String> = env::args().collect();
    match &args[1][..] {
        "init" => {
            fs::create_dir(".git").unwrap();
            fs::create_dir(".git/objects").unwrap();
            fs::create_dir(".git/refs").unwrap();
            fs::write(".git/HEAD", "ref: refs/heads/master\n").unwrap();
            println!("Initialized git directory");
        }
        "cat-file" => {
            let param = &args[2][..];
            let object = &args[3][..];
            // pretty-print
            if param == "-p" {
                let _ = cat_file_p(object);
            }
        }
        "hash-object" => {
            let param = &args[2][..];
            let filename = &args[3][..];
            if param == "-w" {
                let data = serial_object(filename).unwrap();
                print!("{}", std::str::from_utf8(&data).unwrap());
                let hash = create_hash(&data);
                let _ = write_object(&hash, &data);
            }
        }
        _ => {
            println!("unknown command: {}", args[1]);
        }
    }
}

fn cat_file_p(object: &str) -> io::Result<()> {
    let floc = format!(".git/objects/{}/{}", &object[..2], &object[2..]);
    let file = fs::File::open(floc)?;
    let mut z = ZlibDecoder::new(file);
    let mut s = String::new();
    z.read_to_string(&mut s)?;

    if s.starts_with("blob") {
        let separator = s.find("\u{00}");
        let (_header, body) = s.split_at(separator.unwrap());
        print!("{}", &body[1..]);
    } else {
        println!("Unsupported format");
    }
    Ok(())
}

fn serial_object(filename: &str) -> Result<Vec<u8>, io::Error> {
    let mut fhandle = fs::File::open(filename)?;
    let mut content = Vec::new();
    let default = 0;
    let amt = fhandle.read_to_end(&mut content).unwrap_or(default) as u8;

    let mut object_data: Vec<u8> = b"blob".to_vec();
    object_data.append(&mut "\x20".as_bytes().to_vec());
    object_data.append(&mut amt.to_string().as_bytes().to_vec());
    object_data.append(&mut "\x00".as_bytes().to_vec());
    object_data.append(&mut content.to_vec());

    Ok(object_data)
}

fn create_hash(data: &[u8]) -> String {
    let mut hasher = Sha1::new();

    hasher.update(&data);

    let hash = hasher.finalize();
    format!("{:x}", hash)
}

fn write_object(hash: &str, data: &[u8]) -> io::Result<()> {
    let dir = format!(".git/objects/{}", &hash[..2]);
    let loc = format!(".git/objects/{}/{}", &hash[..2], &hash[2..]);
    if !Path::new(&dir).exists() {
        fs::create_dir(&dir).unwrap();
    }
    // compress with Zlib
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(&data)?;
    let compressed = e.finish()?;
    fs::write(&loc, &compressed).expect("Unable to write file");

    println!("{} file written", loc);
    Ok(())
}
