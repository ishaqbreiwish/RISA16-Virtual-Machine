use minifb::{Key, Window, WindowOptions};
use risa16::assembler::assemble;
use risa16::vm::{State, VM};

const WIDTH: usize = 32;
const HEIGHT: usize = 32;
const SCALE: usize = 12;

fn main() {
    let src = r#"
        // Draw a white border around the 32x32 framebuffer
        movimm r2 0   // x
        movimm r3 0   // y

        // top row (y=0, x=0..31)
        movimm r3 0
    top:
        draw r2 r3 255
        movimm r1 1
        add r2 r1
        movimm r1 31
        cmp r2 r1
        jmpnz top

        // bottom row (y=31, x=0..31)
        movimm r2 0
        movimm r3 31
    bottom:
        draw r2 r3 255
        movimm r1 1
        add r2 r1
        movimm r1 32
        cmp r2 r1
        jmpnz bottom

        // left column (x=0, y=0..31)
        movimm r2 0
        movimm r3 0
    left:
        draw r2 r3 255
        movimm r1 1
        add r3 r1
        movimm r1 32
        cmp r3 r1
        jmpnz left

        // right column (x=31, y=0..31)
        movimm r2 31
        movimm r3 0
    right:
        draw r2 r3 255
        movimm r1 1
        add r3 r1
        movimm r1 32
        cmp r3 r1
        jmpnz right

        halt
    "#;

    let bytes = assemble(src).expect("assembly failed");

    let mut vm = VM::new();
    vm.memory.data[..bytes.len()].copy_from_slice(&bytes);
    while vm.state == State::RUNNING {
        vm.step();
    }

    // convert grayscale framebuffer to u32 ARGB for minifb
    let buffer: Vec<u32> = vm.framebuffer.pixels.iter()
        .map(|&p| {
            let v = p as u32;
            0xFF000000 | (v << 16) | (v << 8) | v
        })
        .collect();

    let mut window = Window::new(
        "RISA-16 Framebuffer",
        WIDTH * SCALE,
        HEIGHT * SCALE,
        WindowOptions::default(),
    ).expect("failed to create window");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
