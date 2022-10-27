use std::fs;
use bytebuffer::ByteBuffer;
use huffman;

fn main() {

    let input = "book.txt".to_string();
    let output = "compressed.txt".to_string();

    let contents: String = fs::read_to_string(input).expect("File not found!");
    let byte_buffer: ByteBuffer = huffman::encode(&contents);

    fs::write(&output, byte_buffer.to_bytes()).expect("Unable to write file");

    let bytes = fs::read(&output).expect("Failed to open file!");
    let byte_buffer = ByteBuffer::from_bytes(&bytes);
    let decoded = huffman::decode(byte_buffer);

    fs::write("decompressed.txt", decoded).expect("Unable to write data");

}