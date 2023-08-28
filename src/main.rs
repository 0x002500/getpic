use captrs::*;
use std::fs;
use shuteye::sleep;
use std::time::Duration;
use winapi::um::wingdi::{BITMAPFILEHEADER, BITMAPINFOHEADER};

pub fn serialize_row<T: Sized>(src: &T) -> &[u8] {
    unsafe {
        ::std::slice::from_raw_parts((src as *const T) as *const u8, ::std::mem::size_of::<T>())
    }
}

fn main() {
    //创建存放图片的位置
    let nway = fs::create_dir("./pic");
    let way = "./pic";
    
    let mut capturer = Capturer::new(0).unwrap();

    let (w, h) = capturer.geometry();

    for i in 1..10 {
        let ps = capturer.capture_frame();
        if ps.is_err() {
            println!("{:?}", ps.err());
            continue;
        }
        let ps = ps.unwrap();

        let mut buf = vec![];

        // 因为图片是倒着的，要水平翻转一下
        for y in (0..h).rev() {
            for x in 0..w {
                let Bgr8 { r, g, b, a } = ps[(y * w + x) as usize];
                buf.push(b);
                buf.push(g);
                buf.push(r);
                // buf.push(a);
            }
        }

        save_to_file(
            format!("./pic/{}.bmp", i).as_str(),
            &buf[..],
            (w) as i32,
            (h) as i32,
        );

        sleep(Duration::from_millis(1000 / 60));
    }
}

fn save_to_file(file: &str, rgba: &[u8], w: i32, h: i32) {
    let mut data: Vec<u8> = vec![];

    let bif = BITMAPFILEHEADER {
        bfType: ('B' as u16) | (('M' as u16) << 8),
        bfOffBits: 54,
        bfReserved1: 0,
        bfReserved2: 0,
        bfSize: (w * h * 3 + 54) as u32,
    };

    for v in serialize_row(&bif) {
        data.push(*v);
    }
    let bii = BITMAPINFOHEADER {
        biBitCount: 24,
        biSize: 40,
        biWidth: w,
        biHeight: h,
        biPlanes: 1,
        biCompression: 0,
        biSizeImage: (w * h * 3) as u32,
        biClrImportant: 0,
        biClrUsed: 0,
        biXPelsPerMeter: 0,
        biYPelsPerMeter: 0,
    };

    for v in serialize_row(&bii) {
        data.push(*v);
    }

    for v in rgba {
        data.push(*v);
    }

    use std::fs::File;
    use std::io::Write;

    let mut file = File::create(file).expect("create failed");
    file.write_all(&data[..]).expect("write failed");
}
