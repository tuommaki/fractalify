extern crate image;
extern crate num_complex;

use image::io::Reader as ImageReader;

use std::env;
use std::fs;
use std::fs::{DirEntry,OpenOptions};
use std::io;
use std::io::ErrorKind;
use std::path::Path;
use std::process;
use std::time::{Duration, Instant};
use std::thread::sleep;
use std::vec::Vec;


const PROGRAM_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("fractalify v{}", PROGRAM_VERSION);

    if args.len() < 3 {
        eprintln!("error: parameters missing");
        eprintln!("usage: {} <input dir> <output dir>", &args[0]);

        process::exit(1);
    }
    
    do_work(&args[1], &args[2]).unwrap()
}

fn do_work(input_dir: &str, output_dir: &str) -> io::Result<()> {
    println!("processing images from {} to {}", input_dir, output_dir);

    let refresh_interval = Duration::new(5, 0);
    let cb = |de: &DirEntry| process_file(de, Path::new(output_dir));
    loop {
        let beginning = Instant::now();

        match visit_dirs(Path::new(input_dir), &cb) {
            Err(e) => return Err(e),
            Ok(_) => println!("directory processed."),
        }

        // Is there's need to wait for next round?
        let elapsed = Instant::now().duration_since(beginning);
        if elapsed < refresh_interval {
            sleep(refresh_interval-elapsed)
        }
    }
}


// Simple directory tree visitor that calls `cb` on file entries.
fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry) -> io::Result<()>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                match cb(&entry) {
                    Err(e) => return Err(e),
                    Ok(_) => continue,
                }
            }
        }
    }
    Ok(())
}

fn process_file(de: &DirEntry, output_dir: &Path) -> io::Result<()> {
    println!("processing file {:?}", de.path());

    let src = de.path();
    let dst = output_dir.join(de.file_name());
    
    match OpenOptions::new().write(true)
        .create_new(true)
        .open(dst.clone()) {
            Ok(_) => (),
            Err(e) => match e.kind() {
                ErrorKind::AlreadyExists => {
                    println!("...file already processed");
                    return Ok(())
                },
                _ => return Err(e),
            },
        };

    fractalify(&dst, &src)
}

fn fractalify(dst: &Path, src: &Path) -> io::Result<()> {
    println!("fractalifying {:?} -> {:?}", src, dst);
    
    let mut imgbuf = match ImageReader::open(src)?.decode(){
        Ok(i) => i.to_rgb8(),
        Err(e) => return Err(io::Error::new(ErrorKind::Other, e)),
    };

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", imgbuf.dimensions());

    let (width, height) = imgbuf.dimensions();
    let scalex = 3.0 / width as f32;
    let scaley = 3.0 / height as f32;

    for x in 0..width {
        for y in 0..height {
            let cx = y as f32 * scalex - 1.5;
            let cy = x as f32 * scaley - 1.5;

            let c = num_complex::Complex::new(-0.4, 0.6);
            let mut z = num_complex::Complex::new(cx, cy);

            let mut i = 0;
            while i < 255 && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }

            let pixel = imgbuf.get_pixel_mut(x, y);
            let image::Rgb(data) = *pixel;
            *pixel = image::Rgb([data[0], i as u8, data[2]]);
        }
    }

    match imgbuf.save(dst) {
        Ok(_) => Ok(()),
        Err(e) => Err(io::Error::new(ErrorKind::Other, e)),
    }
}
