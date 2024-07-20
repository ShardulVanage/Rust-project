//  Binary Quantization
fn cosine_similarity_binary(vec1: &[f32], vec2: &[f32]) -> f32 {
    assert_eq!(vec1.len(), vec2.len());

    let mut vec1_bits = vec![];
    let mut vec2_bits = vec![];

    for chunk in vec1.chunks(32) {
        let mut bits = 0u32;
        for (i, &val) in chunk.iter().enumerate() {
            if val >= 0.5 {
                bits |= 1 << i;
            }
        }
        vec1_bits.push(bits);
    }

    for chunk in vec2.chunks(32) {
        let mut bits = 0u32;
        for (i, &val) in chunk.iter().enumerate() {
            if val >= 0.5 {
                bits |= 1 << i;
            }
        }
        vec2_bits.push(bits);
    }

    let mut dot_product = 0u32;
    let mut magnitude1 = 0u32;
    let mut magnitude2 = 0u32;

    for (&bits1, &bits2) in vec1_bits.iter().zip(vec2_bits.iter()) {
        dot_product += (bits1 & bits2).count_ones();
        magnitude1 += bits1.count_ones();
        magnitude2 += bits2.count_ones();
    }

    let magnitude = (magnitude1 as f32).sqrt() * (magnitude2 as f32).sqrt();
    if magnitude == 0.0 {
        0.0
    } else {
        dot_product as f32 / magnitude
    }
}
//  Octal Quantization
fn cosine_similarity_octal(vec1: &[f32], vec2: &[f32]) -> f32 {
    assert_eq!(vec1.len(), vec2.len());

    let quantize = |val: f32| -> u8 {
        let mut result = (val * 15.0).round() as u8;
        if result > 15 {
            result = 15;
        }
        result
    };

    let mut vec1_octal = vec![];
    let mut vec2_octal = vec![];

    for chunk in vec1.chunks(2) {
        let mut octal = 0u8;
        if let Some(&val) = chunk.get(0) {
            octal |= quantize(val) << 4;
        }
        if let Some(&val) = chunk.get(1) {
            octal |= quantize(val);
        }
        vec1_octal.push(octal);
    }

    for chunk in vec2.chunks(2) {
        let mut octal = 0u8;
        if let Some(&val) = chunk.get(0) {
            octal |= quantize(val) << 4;
        }
        if let Some(&val) = chunk.get(1) {
            octal |= quantize(val);
        }
        vec2_octal.push(octal);
    }

    let mut dot_product = 0u32;
    let mut magnitude1 = 0u32;
    let mut magnitude2 = 0u32;

    for (&oct1, &oct2) in vec1_octal.iter().zip(vec2_octal.iter()) {
        let val1_0 = (oct1 >> 4) & 0xF;
        let val1_1 = oct1 & 0xF;
        let val2_0 = (oct2 >> 4) & 0xF;
        let val2_1 = oct2 & 0xF;

        dot_product += (val1_0 * val2_0) as u32 + (val1_1 * val2_1) as u32;
        magnitude1 += (val1_0 * val1_0) as u32 + (val1_1 * val1_1) as u32;
        magnitude2 += (val2_0 * val2_0) as u32 + (val2_1 * val2_1) as u32;
    }

    let magnitude = (magnitude1 as f32).sqrt() * (magnitude2 as f32).sqrt();
    if magnitude == 0.0 {
        0.0
    } else {
        dot_product as f32 / magnitude
    }
}
// Simple u8 Quantization

fn cosine_similarity_u8(vec1: &[f32], vec2: &[f32]) -> f32 {
    assert_eq!(vec1.len(), vec2.len());

    let quantize = |val: f32| -> u8 {
        let mut result = (val * 255.0).round() as u8;
        if result > 255 {
            result = 255;
        }
        result
    };

    let vec1_u8: Vec<u8> = vec1.iter().map(|&val| quantize(val)).collect();
    let vec2_u8: Vec<u8> = vec2.iter().map(|&val| quantize(val)).collect();

    let mut dot_product = 0u32;
    let mut magnitude1 = 0u32;
    let mut magnitude2 = 0u32;

    for (&val1, &val2) in vec1_u8.iter().zip(vec2_u8.iter()) {
        dot_product += (val1 as u32 * val2 as u32);
        magnitude1 += (val1 as u32 * val1 as u32);
        magnitude2 += (val2 as u32 * val2 as u32);
    }

    let magnitude = (magnitude1 as f32).sqrt() * (magnitude2 as f32).sqrt();
    if magnitude == 0.0 {
        0.0
    } else {
        dot_product as f32 / magnitude
    }
}
fn main() {
    let vec1: Vec<f32> = vec![0.1, 0.2, 0.7, 0.9, 0.5, 0.3, 0.8, 0.4, 0.6, 0.1, 0.3, 0.2, 0.5, 0.7, 0.8, 0.9];
    let vec2: Vec<f32> = vec![0.3, 0.5, 0.6, 0.7, 0.4, 0.2, 0.8, 0.9, 0.1, 0.6, 0.7, 0.4, 0.3, 0.5, 0.6, 0.7];

    let similarity_binary = cosine_similarity_binary(&vec1, &vec2);
    let similarity_octal = cosine_similarity_octal(&vec1, &vec2);
    let similarity_u8 = cosine_similarity_u8(&vec1, &vec2);

    println!("Cosine Similarity (Binary): {}", similarity_binary);
    println!("Cosine Similarity (Octal): {}", similarity_octal);
    println!("Cosine Similarity (u8): {}", similarity_u8);
}
