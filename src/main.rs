extern crate image;

use image::ImageDecoder;
use image::png::PNGDecoder;
use std::env;
use std::fs::File;

fn get_bytes_per_pixel(colortype: image::ColorType) -> usize {
        let bits_per_pixel = match colortype {
        image::RGB(n) => n * 3,
        image::RGBA(n) => n * 4,
        image::Gray(n) => n,
        image::GrayA(n) => n * 2,
        _ => panic!("unknown color type {:?}", colortype),
    };
    
    (bits_per_pixel / 8) as usize
}

fn diff_bw(left: &[u8], right: &[u8]) -> u8 {
    0x00
}

fn diff_avg(left: &[u8], right: &[u8]) -> u8 {
    assert!(left.len() == right.len());
    let mut sum: i32 = 0;
    let mut cnt = 0;

    for (l, r) in left.iter().zip(right.iter()) {
        sum += (*l as i32 - *r as i32).abs();
        cnt += 1;
    }

    (sum / cnt) as u8
}

fn run(path1: &str, path2: &str, out: &str) {
    let f1 = File::open(path1).expect(&format!("error opening file {}", path1));
    let f2 = File::open(path2).expect(&format!("error opening file {}", path2));

    let mut decoder1 = PNGDecoder::new(f1);
    let mut decoder2 = PNGDecoder::new(f2);
    
    let (width1, height1) = decoder1.dimensions().expect("can't get dimensions");
    let (width2, height2) = decoder2.dimensions().expect("can't get dimensions");
    assert!(width1 == width2);
    assert!(height1 == height2);

    let colortype1 = decoder1.colortype().expect("couldn't get color type");
    let colortype2 = decoder2.colortype().expect("couldn't get color type");
    assert!(colortype1 == colortype2);
    
    let bytes_per_pixel = get_bytes_per_pixel(colortype1);

    let rowlen = decoder1.row_len().unwrap();

    let mut line_buf1 = vec![0; rowlen];
    let mut line_buf2 = vec![0; rowlen];
    let mut diff_img = vec![0xff; (width1 * height1) as usize];
    let mut diff_cnt = 0;

    for row in 0..height1 {
        let _ = decoder1.read_scanline(&mut line_buf1).unwrap();
        let _ = decoder2.read_scanline(&mut line_buf2).unwrap();

        for (col, (left, right)) in line_buf1.chunks(bytes_per_pixel).zip(line_buf2.chunks(bytes_per_pixel)).enumerate() {
            if left != right {
                let pos = (width1 * row) as usize + col;
                diff_img[pos] = diff_avg(&left, &right);
                diff_cnt += 1;
            }
        }
    }
    if diff_cnt > 0 {
        println!("diffs: {}", diff_cnt);
        image::save_buffer(out, &diff_img, width1, height1, image::Gray(8)).expect("error storing diff image");
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    assert!(args.len() == 4, "usage: imgdiff left.png right.png out.png");

    run(&args[1], &args[2], &args[3]);
}
