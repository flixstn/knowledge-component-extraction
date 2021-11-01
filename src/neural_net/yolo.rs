use opencv::{
    core::{BORDER_DEFAULT, CV_32F, Point, Range, Rect, Scalar, Size, min_max_loc, no_array, subtract}, 
    dnn::{self, DNN_BACKEND_OPENCV, DNN_TARGET_CPU, Net, nms_boxes, read_net_from_darknet}, 
    highgui, 
    imgproc::{COLOR_BGR2GRAY, LINE_8, THRESH_BINARY, cvt_color, gaussian_blur, rectangle, threshold}, 
    prelude::{Mat, MatTrait, MatTraitManual, NetTrait}, 
    text::{OCRTesseract, OEM_DEFAULT, PSM_SINGLE_BLOCK}, 
    types::{VectorOfMat, VectorOfRect, VectorOfString, VectorOff32, VectorOfi32}, 
    videoio::{self, CAP_PROP_FPS, CAP_PROP_POS_MSEC, VideoCapture, VideoCaptureTrait}
};
use std::{error::Error, sync::mpsc::Sender};
use crate::analyzer::analyzer::Message;

pub struct Yolo;

impl Yolo {
    pub fn run(sender: Sender<(Message, i32)>, file: &str) -> Result<(), Box<dyn Error>>{
        // initialize video capture 
        let mut video_capture = VideoCapture::from_file(
            file, 
            videoio::CAP_ANY
        )?;
        
        // initialize neural network
        let mut net = read_net_from_darknet(
            "./weights/yolov4-obj.cfg", 
            "./weights/yolov4-obj_best.weights"
        )?;
        net.set_preferable_target(DNN_TARGET_CPU)?;
        net.set_preferable_backend(DNN_BACKEND_OPENCV)?;

        // optical character recognition
        let mut ocr = OCRTesseract::create("", "eng", "", OEM_DEFAULT, PSM_SINGLE_BLOCK)?;
        let mut ocr_output = String::new();

        // preallocate image matrices
        let mut img = Mat::default();
        let mut sub_img = Mat::default();
        let mut blob = Mat::default();
        let mut gray = Mat::default();
        let mut _gaussian_img = Mat::default();
        let mut _thresh_img = Mat::default();
        let mut src = Mat::default();

        // set limits for frame analysis
        let delay_seconds = 1;
        let fps = video_capture.get(CAP_PROP_FPS)? as i32;              
        let multiplier = fps * delay_seconds;

        // set config variables for neural net
        let conf_threshold= 0.5_f32;
        let nms_threshold = 0.4_f32;
        let inp_width = 608;
        let inp_height = 608;
        
        // start video processing
        while highgui::wait_key(1)? < 0 {
            // extract each frame from the video along with the frame size
            video_capture.read(&mut img)?;
            let img_width = img.cols();
            let img_height = img.rows();

            // set delay for processing only one frame every second and time stamp
            let frame_id = video_capture.get(1)? as i32;
            let frame_position = video_capture.get(CAP_PROP_POS_MSEC)?.round() as i32 / 1000;

            // break loop if video capture has no more frames
            if !video_capture.grab()? {
                println!("Video processing finished");
                break
            }

            // delay processing and only take one frame every second
            if frame_id % multiplier == 1 {
                // generate a blob from frame
                dnn::blob_from_image_to(
                    &img, &mut blob, 
                    1./255., 
                    Size::new(inp_width, inp_height), 
                    Scalar::new(0.,0.,0., 0.), 
                    false, 
                    false, 
                    CV_32F
                )?;
                
                // get the names of output layer for bbox naming
                let names = get_output_names(&net)?;

                // forward propagation through the network
                let mut net_output = VectorOfMat::new();
                net.set_input(&blob, "", 1.0, Scalar::new(0.,0.,0., 0.))?;
                net.forward(&mut net_output, &names)?;

                // scan through all bounding boxes and keep only the ones with high confidence
                let mut class_ids = VectorOfi32::new();
                let mut confidences = VectorOff32::new();
                let mut boxes = VectorOfRect::new();

                // remove the bounding boxes with low confidence using non-maxima suppression
                for (i, matrix) in net_output.iter().enumerate() {          
                    for j in 0..matrix.rows() {
                        let data = matrix.at_row::<f32>(j as i32)?; 
                        let scores = net_output.get(i)?.row(j)?.col_range(&Range::new(5, net_output.get(i)?.cols())?)?;
                        let mut class_id_point = Point::default();
                        let mut confidence = 0_f64;

                        min_max_loc(
                            &scores, 
                            &mut 0., 
                            &mut confidence, 
                            &mut Point::new(0,0), 
                            &mut class_id_point, 
                            &no_array()?
                        )?;

                        if confidence > conf_threshold as f64 {
                            let center_x = (data[0] *  img_width as f32) as i32;
                            let center_y = (data[1] * img_height as f32) as i32;
                            let width = (data[2] * img_width as f32) as i32;                 // w
                            let height = (data[3] * img_height as f32) as i32;               // h
                            let left = center_x - (width / 2);                               // x
                            let top = center_y - (height / 2);                               // y

                            class_ids.push(class_id_point.x);
                            confidences.push(confidence as f32);
                            boxes.push(Rect::new(left, top, width, height));
                        }
                    }
                }

                // perform non maximum suppression to eliminate redundant overlapping boxes with lower confidences
                let mut indices = VectorOfi32::new();
                nms_boxes(
                    &boxes, 
                    &confidences, 
                    conf_threshold, 
                    nms_threshold, 
                    &mut indices, 
                    1., 
                    0
                )?;

                for num in indices.iter() {
                    let mut bbox = boxes.get(num as usize)?;
                    bbox.x -= 4;
                    bbox.y -= 2;
                    bbox.height += 4;
                    bbox.width += 8;
                    
                    // adjust bounding box if coordinates are < 0
                    if bbox.x < 0 || bbox.y < 0 {
                        bbox.x = 0;
                        bbox.y = 0;
                    }
                    
                    // draw predicted bounding box
                    rectangle(
                        &mut img, 
                        bbox, 
                        Scalar::new(255., 18., 50., 0.0), 
                        2, 
                        LINE_8, 
                        0
                    )?;

                    // convert sub image to grayscale
                    sub_img = Mat::roi(&img, bbox)?;
                    cvt_color(&sub_img, &mut gray, COLOR_BGR2GRAY, 0)?;

                    // convert mean 
                    let mean_value = opencv::core::mean(&gray, &no_array()?)?;
                    // println!("{:?}", mean_value);

                    // TODO: check for zero vector
                    if *mean_value.get(0).unwrap() <= 127.5 {   // mean_value.get(0).is_some() &&
                        subtract(&Scalar::all(255.), &gray, &mut src, &no_array()?, -1)?;
                    }
                }

                // perform OCR on image
                if !gray.empty()? {
                    // TODO: change component level to default: 0?
                    ocr_output = ocr.run(&src, 0, 1)?;
                    sender.send((Message::StreamMessage(ocr_output), frame_position));
                }

                // show frame
                highgui::imshow("image", &src)?;
            }
        }
        sender.send((Message::EndMessage, 0));
        Ok(())
    }
}

fn get_output_names(net: &Net) -> Result<VectorOfString, Box<dyn Error>> {
    let layers = net.get_unconnected_out_layers()?;
    let layer_names = net.get_layer_names()?;
    
    let mut names = VectorOfString::with_capacity(layers.len());

    for (i, _num) in layers.iter().enumerate() {
        let value = layer_names.get((layers.get(i)? - 1) as usize)?;
        names.insert(i, &value)?;
    }

    Ok(names)
}