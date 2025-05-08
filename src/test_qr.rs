use qrcode::QrCode;
use image::Luma;

fn main() {
    // 要编码的文本
    let text = "Hello, QR Code!";

    // 生成二维码
    let code = QrCode::new(text.as_bytes()).unwrap();

    // 将二维码转换为图像
    let image = code.render::<Luma<u8>>().build();

    // 保存图像到文件
    image.save("qrcode.png").unwrap();

    println!("QR code saved as qrcode.png");
}