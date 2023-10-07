use std::io::Cursor;

use rayon::prelude::*;
use image::DynamicImage;

use lopdf::dictionary;
use lopdf::{Document, Object, Stream};
use lopdf::content::{Content, Operation};

pub struct Pdf {
    imgs: Vec<DynamicImage>,
}

impl Pdf {
    pub fn new() -> Self {
        Self {
            imgs: vec![],
        }
    }

    pub fn read_imgs(&mut self, paths: &Vec<String>) {
        self.imgs = paths.par_iter()
            .map(|path| image::open(path))
            .filter(|img| img.is_ok())
            .map(|img| img.unwrap())
            .collect();
    }

    fn compress(&self, quality: u8) -> Vec<(Vec<u8>, u32, u32)>{
        // Get widths
        let mut widths: Vec<u32> = self.imgs.par_iter()
            .map(|img| img.width()).collect();

        // Calculate median of widths
        let median = match widths.len() {
            0 => 0,
            1 => widths[0],
            2 => (widths[0] + widths[1]) / 2,
            _ => {
                widths.sort();
                widths[widths.len() / 2]
            }
        };

        // Resize images, convert to jpeg and return bytes
        let img_bytes: Vec<(Vec<u8>, u32, u32)> = self.imgs.par_iter().map(|img| {
            let nwidth = median;
            let nheight = (img.height() as f32 * median as f32 / img.width() as f32) as u32;
            let img = img.resize_exact(nwidth, nheight, image::imageops::FilterType::Nearest).to_rgb8();
            let mut img_bytes = Cursor::new(vec![]);
            img.write_to(&mut img_bytes, image::ImageOutputFormat::Jpeg(quality)).unwrap();
            (img_bytes.into_inner(), nwidth, nheight)
        }).collect();

        img_bytes
    }

    pub fn create(&self, output: String, quality: u8) -> Result<(), Box<dyn std::error::Error>> {
        // Compress image
        let img_data = self.compress(quality); // Compression quality of 90, change as needed
    
        // Header
        let mut doc = Document::with_version("1.7");
        let pages_id = doc.new_object_id();
        let mut num_pages = 0;
        let mut kids = vec![];
    
        // Page and image
        for (data, width, height) in img_data.iter() {
            let x_cm = (*width as f64 * 2.54 / 300.0 * 100.0) as u32;
            let y_cm = (*height as f64 * 2.54 / 300.0 * 100.0) as u32;
            let resources = dictionary! {
                "XObject" => dictionary! {
                    "Image1" => doc.add_object(Stream::new(dictionary! {
                        "Type" => "XObject",
                        "Subtype" => "Image",
                        "Width" => (*width),
                        "Height" => (*height),
                        "ColorSpace" => "DeviceRGB",
                        "BitsPerComponent" => 8,
                        "Filter" => "DCTDecode",
                        "Interpolate" => true,
                        "DpiX" => 300,
                    }, data.clone()))
                }
            };
    
            let content = Content {
                operations: vec![
                    Operation::new("q", vec![]), 
                    Operation::new("cm", vec![Object::Integer(x_cm as i64), Object::Integer(0), Object::Integer(0), Object::Integer(y_cm as i64), Object::Integer(0), Object::Integer(0)]),
                    Operation::new("Do", vec![Object::Name("Image1".as_bytes().to_vec())]), 
                    Operation::new("Q", vec![]),
                ],
            };

            let content_id = doc.add_object(Stream::new(dictionary! {}, content.encode().unwrap()));
            let page_id = doc.add_object(dictionary! {
                "Type" => "Page",
                "Parent" => pages_id,
                "Contents" => content_id,
                "Resources" => resources,
                "MediaBox" => vec![0.into(), 0.into(), x_cm.into(), y_cm.into()],
            });
            num_pages += 1;
            kids.push(page_id);
        }

        // Pages
        let pages = dictionary! {
            "Type" => "Pages",
            "Kids" => kids.into_iter()
                .map(|object_id| Object::Reference(object_id))
                .collect::<Vec<_>>(),
            "Count" => num_pages,
        };
        doc.objects.insert(pages_id, Object::Dictionary(pages));

        // Catalog
        let catalog_id = doc.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        });

        // Trailer
        doc.trailer.set("Root", catalog_id);
        doc.compress();
    
        // Save the PDF
        doc.save(&output)?;
        Ok(())
    }

}