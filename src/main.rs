// #![allow(unused)]
mod prelude {
    pub use crate::{
        analyzer::analyzer::*,
        classifier::classifier::*,
        neural_net::yolo::Yolo, 
        parser::{Parser, ProtoParser, knowledge_component::*, cjparser::CJParser, pyparser::PyParser},
        lexer::pylexer::*,
        lexer::cjlexer::*,
    };

    pub use clap::{App, Arg};
    pub use indexmap::IndexSet;
    pub use logos::{Logos, Lexer};
    pub use opencv::{core::{BORDER_DEFAULT, CV_32F, Point, Range, Rect, Rect2i, Rect_, Scalar, Size, create_continuous, min_max_loc, no_array, subtract}, dnn::{self, DNN_BACKEND_OPENCV, DNN_TARGET_CPU, Net, nms_boxes, read_net_from_darknet}, highgui, imgproc::{COLOR_BGR2GRAY, COLOR_BGR5552GRAY, COLOR_BGRA2GRAY, COLOR_RGB2GRAY, LINE_8, THRESH_BINARY, cvt_color, gaussian_blur, rectangle, threshold}, prelude::{Mat, MatTrait, MatTraitManual, NetTrait}, text::{OCRTesseract, OEM_DEFAULT, PSM_SINGLE_BLOCK}, types::{VectorOfMat, VectorOfRect, VectorOfString, VectorOff32, VectorOfi32}, videoio::{self, CAP_PROP_FPS, CAP_PROP_POS_MSEC, VideoCapture, VideoCaptureTrait}};
    pub use serde::{Serialize, Deserialize};
    pub use std::{env::current_dir, error::Error, fs::write, fs::create_dir, hash::Hash, hash::Hasher, process::Command, str::from_utf8, sync::{Arc, Mutex, mpsc::{self, Sender, Receiver}}, thread, time::Duration};
}
mod analyzer;
mod classifier;
mod lexer;
mod neural_net;
mod parser;

use crate::prelude::*;



fn main() {
    if let Err(err) = try_main() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn try_main() -> Result<(), Box<dyn Error>> {
    // TODO: implement playlist parameter
    // argument parsing
    let matches = App::new("Knowledge Component Extraction")
        .version("0.1.0")
        .about("Analyzes programming videos and extracts knowledge components")
        .arg(Arg::with_name("url")
                .short("url")
                .long("url")
                .takes_value(true)
                .help("URL as input"))
        .arg(Arg::with_name("path")
                .short("p")
                .long("path")
                .takes_value(true)
                .help("Path as input"))
        .arg(Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("File as input"))
        .get_matches();


    if let Some(url) = matches.value_of("url") {
        let mut analyzer = VideoAnalyzer::new(url);
        analyzer.run()?;
        analyzer.save_result()?;
        // analyzer.download_video()?;
    }

    if let Some(path) = matches.value_of("path") {
        // TODO: implement
    } 

    if let Some(file) = matches.value_of("file") {
        // TODO: implement
    }

    Ok(())
}