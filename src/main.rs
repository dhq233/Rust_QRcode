use qrcode::QrCode;
use image::Luma;

fn main() {
    let html_content = "https://www.bilibili.com";

    let code = QrCode::new(html_content.as_bytes()).unwrap();
    let image = code.render::<Luma<u8>>().build();
    
    image.save("qrcode.png").unwrap();
}
