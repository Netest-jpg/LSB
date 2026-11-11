use image::ImageReader;
use std::error::Error;
use std::io;
use std::str::from_utf8;

fn main() -> Result<(), Box<dyn Error>> {
    // encoding part
    let image = ImageReader::open("Saturn.png")?.decode()?;
    let (width, height) = (image.width(), image.height());
    let mut raw = image.to_rgb8().into_raw();

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let message = input.trim().as_bytes();

    let mut payload = (message.len() as u32).to_be_bytes().to_vec();
    payload.extend_from_slice(message);

    if raw.len() < payload.len() * 8 {
        return Err("Message is too big".into());
    }
    let mut i = 0;
    for byte in payload.iter() {
        for bit in 0..8 {
            let msg_bit = (byte >> (7 - bit)) & 1;
            raw[i] = msg_bit;
            i += 1;
        }
    }

    image::save_buffer("out.png", &raw, width, height, image::ColorType::Rgb8)?;

    // decoding part
    let altered = ImageReader::open("out.png")?.decode()?;
    let raw = altered.to_rgb8().into_raw();
    let last_bits: Vec<u8> = raw[..32].iter().map(|&x| x & 1).collect();
    let mut result = [0u8; 4];
    for (chunk_idx, chunk) in last_bits.as_chunks::<8>().0.iter().enumerate() {
        let mut byte = 0u8;
        for (bit_idx, &value) in chunk.iter().enumerate() {
            byte |= (value & 1) << (7 - bit_idx);
        }
        result[chunk_idx] = byte;
    }
    let length = u32::from_be_bytes(result);
    if raw.len() < (32 + (length * 8)) as usize {
        return Err("Index out of bounds".into());
    }
    let lsb: Vec<u8> = raw[32..(32 + (length * 8) as usize)]
        .iter()
        .map(|&x| x & 1)
        .collect();
    let mut result: Vec<u8> = Vec::with_capacity(length as usize);
    for chunk in lsb.as_chunks::<8>().0 {
        let mut byte = 0u8;
        for (bit_idx, &value) in chunk.iter().enumerate() {
            byte |= (value & 1) << (7 - bit_idx);
        }
        result.push(byte);
    }
    let msg = from_utf8(&result)?;
    println!("message: {}", msg);
    Ok(())
}
