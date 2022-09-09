extern crate image;
extern crate base64;

use std::io::{Cursor, Error, ErrorKind};
use base64::{decode, encode};
use image::{DynamicImage, GenericImageView, ImageError, ImageFormat, ImageResult};
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, HtmlImageElement, window};
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub fn convert(data: &str) {
    let window = window().expect("expected a window");
    let document = window.document().expect("expected a document");

    let fmt = get_image_format(data);
    let img = read_img(get_image_data(data), fmt).unwrap();
    let element = append_image_element(get_mime(fmt), img, document).unwrap();
    let alt = format!("Hello World!");
    element.set_alt(&alt);
}

fn get_image_format(data: &str) -> ImageFormat {
    if data.starts_with("data:") {
        let parts: Vec<&str> = data.split(";").collect();
        let fmt = parts[0].replace("data:", "");
        return match fmt.as_str() {
            "image/jpeg" => ImageFormat::Jpeg,
            "image/png" => ImageFormat::Png,
            _ => panic!("unsupported image format")
        };
    }
    return ImageFormat::Png;
}

fn get_mime(format: ImageFormat) -> &'static str {
    return match format {
        ImageFormat::Jpeg => "image/jpeg",
        _ => "image/png"
    }
}

fn get_image_data(data: &str) -> &str {
    if data.starts_with("data:") {
        let parts: Vec<&str> = data.split(",").collect();
        return parts[1];
    }
    return data;
}

fn read_img(data: &str, format: ImageFormat) -> ImageResult<DynamicImage> {
    //use matching to either return image or an ImageError
    match decode(data) {
        Ok(v) =>
            return image::load_from_memory_with_format(v.as_slice(), format),
        Err(e) =>
            //create new ImageError where the source comes from the decoding
            return Err(ImageError::IoError(Error::new(ErrorKind::Other, e))),
    };
}

fn append_image_element(mime: &str, img: DynamicImage, document: Document) -> Result<HtmlImageElement, Element>{
    let target = document.get_element_by_id("target").expect("document should have a target element");
    let img_element = document.create_element("img")?.dyn_into::<HtmlImageElement>()?;
    target.append_child(&img_element)?;
    img_element.set_name("output");

    let dim = img.dimensions();
    img_element.set_width(dim.0);
    img_element.set_height(dim.1);

    let gray = img.grayscale();
    match to_base64(gray) {
        Ok(encoded) => {
            img_element.set_src(&to_source(mime, &encoded));
        }
        _ => {}
    }

    return Ok(img_element);
}

fn to_base64(img: DynamicImage) -> Result<String, ImageError> {
    let mut bytes: Vec<u8> = vec![];
    img.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)?;
    return Ok(encode(&bytes));
}

fn to_source(mime: &str, encoded: &str) -> String {
    return format!("data:{};base64,{}", mime, encoded);
}


#[test]
fn test_read_img_err() {
    match read_img("xyz", ImageFormat::Png) {
        Ok(_) => panic!("unexpected image"),
        Err(e) => assert_eq!("Invalid last symbol 122, offset 2.", e.to_string())
    };
}

#[test]
fn test_get_image_format() {
    assert_eq!(ImageFormat::Png, get_image_format("data:image/png"));
    assert_eq!(ImageFormat::Jpeg, get_image_format("data:image/jpeg"));
}

#[test]
fn test_get_mime() {
    assert_eq!("image/png", get_mime(ImageFormat::Png));
    assert_eq!("image/jpeg", get_mime(ImageFormat::Jpeg));
}

#[test]
fn test_to_base64() {
    const IMG: &str = "iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAYAAADgdz34AAAABHNCSVQICAgIfAhkiAAAAAlwSFlzAAAApgAAAKYB3X3/OAAAABl0RVh0U29mdHdhcmUAd3d3Lmlua3NjYXBlLm9yZ5vuPBoAAANCSURBVEiJtZZPbBtFFMZ/M7ubXdtdb1xSFyeilBapySVU8h8OoFaooFSqiihIVIpQBKci6KEg9Q6H9kovIHoCIVQJJCKE1ENFjnAgcaSGC6rEnxBwA04Tx43t2FnvDAfjkNibxgHxnWb2e/u992bee7tCa00YFsffekFY+nUzFtjW0LrvjRXrCDIAaPLlW0nHL0SsZtVoaF98mLrx3pdhOqLtYPHChahZcYYO7KvPFxvRl5XPp1sN3adWiD1ZAqD6XYK1b/dvE5IWryTt2udLFedwc1+9kLp+vbbpoDh+6TklxBeAi9TL0taeWpdmZzQDry0AcO+jQ12RyohqqoYoo8RDwJrU+qXkjWtfi8Xxt58BdQuwQs9qC/afLwCw8tnQbqYAPsgxE1S6F3EAIXux2oQFKm0ihMsOF71dHYx+f3NND68ghCu1YIoePPQN1pGRABkJ6Bus96CutRZMydTl+TvuiRW1m3n0eDl0vRPcEysqdXn+jsQPsrHMquGeXEaY4Yk4wxWcY5V/9scqOMOVUFthatyTy8QyqwZ+kDURKoMWxNKr2EeqVKcTNOajqKoBgOE28U4tdQl5p5bwCw7BWquaZSzAPlwjlithJtp3pTImSqQRrb2Z8PHGigD4RZuNX6JYj6wj7O4TFLbCO/Mn/m8R+h6rYSUb3ekokRY6f/YukArN979jcW+V/S8g0eT/N3VN3kTqWbQ428m9/8k0P/1aIhF36PccEl6EhOcAUCrXKZXXWS3XKd2vc/TRBG9O5ELC17MmWubD2nKhUKZa26Ba2+D3P+4/MNCFwg59oWVeYhkzgN/JDR8deKBoD7Y+ljEjGZ0sosXVTvbc6RHirr2reNy1OXd6pJsQ+gqjk8VWFYmHrwBzW/n+uMPFiRwHB2I7ih8ciHFxIkd/3Omk5tCDV1t+2nNu5sxxpDFNx+huNhVT3/zMDz8usXC3ddaHBj1GHj/As08fwTS7Kt1HBTmyN29vdwAw+/wbwLVOJ3uAD1wi/dUH7Qei66PfyuRj4Ik9is+hglfbkbfR3cnZm7chlUWLdwmprtCohX4HUtlOcQjLYCu+fzGJH2QRKvP3UNz8bWk1qMxjGTOMThZ3kvgLI5AzFfo379UAAAAASUVORK5CYII=";
    let img = read_img(IMG, ImageFormat::Png).unwrap();
    let dim = img.dimensions();
    assert_eq!(24, dim.0);
    assert_eq!(24, dim.1);
    let encoded = to_base64(img);
    assert!(!encoded.unwrap().is_empty());
}

#[test]
fn test_to_source() {
    let src = to_source("image/png", "xyz");
    assert_eq!("data:image/png;base64,xyz", src);
}