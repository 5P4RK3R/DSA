// // extern crate opencv;

// use opencv::{core, highgui, imgproc, objdetect};

// fn main() {
//     // Load pre-trained face and eye detection cascades
//     let face_cascade = objdetect::CascadeClassifier::new("haarcascade_frontalface_default.xml").unwrap();
//     let eye_cascade = objdetect::CascadeClassifier::new("haarcascade_eye.xml").unwrap();

//     // Load input image
//     let img = core::Mat::from_path("input.jpg", core::CV_LOAD_IMAGE_COLOR).unwrap();
//     let mut gray = core::Mat::default().unwrap();
//     imgproc::cvt_color(&img, &mut gray, imgproc::COLOR_BGR2GRAY, 0).unwrap();

//     // Detect faces in the image
//     let mut faces = core::Vector::<core::Rect>::new();
//     face_cascade.detect_multi_scale(&gray, &mut faces, 1.3, 5, 0, core::Size { width: 0, height: 0 }, core::Size { width: 0, height: 0 }).unwrap();

//     for face in faces.iter() {
//         let roi_gray = gray.region(*face).unwrap();
//         let roi_color = img.region(*face).unwrap();
        
//         // Detect eyes within each face region
//         let mut eyes = core::Vector::<core::Rect>::new();
//         eye_cascade.detect_multi_scale(&roi_gray, &mut eyes, 1.1, 2, 0, core::Size { width: 0, height: 0 }, core::Size { width: 0, height: 0 }).unwrap();
        
//         for eye in eyes.iter() {
//             imgproc::rectangle(&roi_color, core::Rect { x: eye.x, y: eye.y, width: eye.width, height: eye.height }, core::Scalar::new(0.0, 255.0, 0.0, 0.0), 2, 8, 0).unwrap();
//         }
//     }

//     // Display the result
//     highgui::imshow("img", &img).unwrap();
//     highgui::wait_key(0).unwrap();
// }
use opencv::{
    core::{self, Vector},
    dnn,
    highgui,
    imgproc,
    prelude::*,
    videoio,
    Result,
};

fn main() -> Result<()> {
    // Initialize the OpenCV video capture
    let mut camera = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    let mut frame = Mat::default();
    let face_cascade = "haarcascade_frontalface_default.xml";
    let eye_cascade = "haarcascade_eye.xml";

    let mut face_cascade = core::CascadeClassifier::new(face_cascade)?;
    let mut eye_cascade = core::CascadeClassifier::new(eye_cascade)?;

    loop {
        camera.read(&mut frame)?;
        if frame.size()?.width > 0 {
            let mut gray = Mat::default();
            imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

            let mut faces = Vector::new();
            face_cascade.detect_multi_scale(&gray, &mut faces, 1.1, 3, 0, core::Size::new(30, 30), core::Size::new(500, 500))?;

            for face in faces {
                let roi = core::Rect::new(face.x, face.y, face.width, face.height);
                let mut face_img = Mat::roi(&gray, roi)?;
                let mut eyes = Vector::new();
                eye_cascade.detect_multi_scale(&face_img, &mut eyes, 1.1, 2, 0, core::Size::new(20, 20), core::Size::new(80, 80))?;

                for eye in eyes {
                    let eye_roi = core::Rect::new(eye.x + face.x, eye.y + face.y, eye.width, eye.height);
                    imgproc::rectangle(&mut frame, eye_roi, core::Scalar::new(0.0, 255.0, 0.0, 0.0), 2, 8, 0)?;
                }
                imgproc::rectangle(&mut frame, roi, core::Scalar::new(255.0, 0.0, 0.0, 0.0), 2, 8, 0)?;
            }

            highgui::imshow("Eye Blink Detection", &frame)?;

            if highgui::wait_key(10)? > 0 {
                break;
            }
        }
    }

    Ok(())
}
