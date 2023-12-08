use image::GenericImageView;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;
pub mod walk_dirs;

fn main() {
    let pic_path = "/media/pipi/USB01/Master_Master/".to_string();
    let imglist = walk_dirs::walk_dir(pic_path);

    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();
    for jpg in imglist {
        println!("jpg {}", jpg);
        let tx = tx.clone();
        pool.execute(move || {
            let resizeimage = resize_image(jpg.clone());
            tx.send(resizeimage).unwrap();
        });
    }
    drop(tx);

    for t in rx.iter() {
        let info = t;
        println!("Info: {:?}", info.clone());
    }
}

fn image_meta(apath: String) -> (f64, f64, f64, &'static str) {
    let img = image::open(apath.clone()).expect(&apath);
    let (width, height) = img.dimensions();
    let oldwidth = width.clone() as f64;
    let oldheight = height.clone() as f64;
    let aspect_ratio = oldwidth / oldheight;
    let mut orient = "";
    if oldwidth > oldheight {
        println!("landscape");
        orient = "landscape";
    } else if oldwidth < oldheight {
        print!("portrait");
        orient = "portrait";
    } else if oldwidth == oldheight {
        println!("square");
        orient = "square";
    };

    let res = (oldwidth, oldheight, aspect_ratio, orient);

    res
}

pub fn resize_image(jpgpath: String) -> String {
    let jpg_meta = image_meta(jpgpath.clone());
    let width = jpg_meta.0;
    let aspect_ratio = jpg_meta.2;
    let orient = jpg_meta.3;
    println!("width: {}", width);
    println!("aspect_ratio: {}", aspect_ratio);
    println!("orient: {}", orient);
    // if orient == "landscape" {
    //     println!("landscape");
    //     if width > 1200 {
    //         let newwidth = 1200;
    //         let newheight = newwidth * aspect_ratio.clone();
    //         println!("newwidth: {}\nnewheight: {}", newwidth, newheight);
    //         let img = image::open(jpgpath.clone()).expect(&jpgpath);
    //         let resized = img.resize(newwidth, newheight, image::imageops::FilterType::Lanczos3);
    //         let fn_split = jpgpath.split("/").collect::<Vec<&str>>();
    //         let filename_last = fn_split.last().unwrap();
    //         let new_fn = "/media/pipi/USB01/Master_Master_Resize/".to_string() + &filename_last;
    //         // resized.save(new_fn.clone()).unwrap();
    //         println!("new_fn: {}", new_fn);
    //         return new_fn.clone();
    //     }
    // } else if orient == "portrait" {
    //     if width > 900 {
    //         let newwidth = 900;
    //         let newheight = newwidth / aspect_ratio.clone();
    //         println!("newwidth: {}\nnewheight: {}", newwidth, newheight);
    //         let img = image::open(jpgpath.clone()).expect(&jpgpath);
    //         let resized = img.resize(newwidth, newheight, image::imageops::FilterType::Lanczos3);
    //         let fn_split = jpgpath.split("/").collect::<Vec<&str>>();
    //         let filename_last = fn_split.last().unwrap();
    //         let new_fn = "/media/pipi/USB01/Master_Master_Resize/".to_string() + &filename_last;
    //         // resized.save(new_fn.clone()).unwrap();
    //         println!("new_fn: {}", new_fn);
    //         return new_fn.clone();
    //     }
    // } else if orient == "square" {
    //     let img = image::open(jpgpath.clone()).expect(&jpgpath);
    //     let fn_split = jpgpath.split("/").collect::<Vec<&str>>();
    //     let filename_last = fn_split.last().unwrap();
    //     let new_fn = "/media/pipi/USB01/Master_Master_Resize/".to_string() + &filename_last;
    //     // img.save(new_fn.clone()).unwrap();
    //     println!("new_fn: {}", new_fn);
    //     return new_fn.clone();
    // } else {
    //     return "No Match".to_string();
    // }

    jpgpath.clone()
}
