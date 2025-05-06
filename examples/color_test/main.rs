use vines::controller::Controller;
use vines::cpu::mappers::basic_mapper::*;
use vines::cpu::*;
use vines::ppu::PPU;
use vines::rom::*;


use std::cell::RefCell;
use std::env;
use std::fs::read;
use std::path::PathBuf;
use std::rc::Rc;
fn main() {
    // env::set_var("RUST_BACKTRACE", "1");

    // let mut rng = rand::thread_rng();

    // let rom_bytes = read(PathBuf::from("examples/color_test/color_test.nes")).unwrap();
    // let rom = Rom::try_from(&rom_bytes).unwrap();
    // println!("prg rom len: {}", rom.prg_rom.len());
    // println!("chr rom len: {}", rom.chr_rom.len());

    // let ppu = Rc::new(RefCell::new(PPU::new(rom.chr_rom.clone())));
    // let controller = Rc::new(RefCell::new(Controller::new()));
    // let cpu_mapper = BasicMapper::new(rom, ppu.clone(), controller.clone());
    // let mut cpu = CPU::new(cpu_mapper);

    // cpu.reset();

    // let mut total_cycles: usize = 0;
    // loop {
    //     handle_user_input(controller.clone(), &mut event_pump);

    //     if total_cycles % 341 == 0 {
    //         texture
    //             .update(None, &ppu.borrow().frame.bytes, 256 * 3)
    //             .unwrap();
    //         canvas.copy(&texture, None, None).unwrap();
    //         canvas.present();
    //     }

    //     let instruction_result = cpu.execute_next_instruction();

    //     for _ in 0..instruction_result.executed_cycles {
    //         ppu.borrow_mut().tick();
    //     }

    //     total_cycles += instruction_result.executed_cycles as usize;
    // }
}

// fn handle_user_input(controller: Rc<RefCell<Controller>>, event_pump: &mut EventPump) {
//     for event in event_pump.poll_iter() {
//         match event {
//             Event::Quit { .. }
//             | Event::KeyDown {
//                 keycode: Some(Keycode::Escape),
//                 ..
//             } => std::process::exit(0),
//             Event::KeyDown {
//                 keycode: Some(Keycode::Down),
//                 ..
//             } => {
//                 controller.borrow_mut().press_button(Controller::DOWN);
//             }
//             Event::KeyUp {
//                 keycode: Some(Keycode::Down),
//                 ..
//             } => {
//                 controller.borrow_mut().release_button(Controller::DOWN);
//             }
//             Event::KeyDown {
//                 keycode: Some(Keycode::Up),
//                 ..
//             } => {
//                 controller.borrow_mut().press_button(Controller::UP);
//             }
//             Event::KeyUp {
//                 keycode: Some(Keycode::Up),
//                 ..
//             } => {
//                 controller.borrow_mut().release_button(Controller::UP);
//             }
//             Event::KeyDown {
//                 keycode: Some(Keycode::Left),
//                 ..
//             } => {
//                 controller.borrow_mut().press_button(Controller::LEFT);
//             }
//             Event::KeyUp {
//                 keycode: Some(Keycode::Left),
//                 ..
//             } => {
//                 controller.borrow_mut().release_button(Controller::LEFT);
//             }
//             Event::KeyDown {
//                 keycode: Some(Keycode::Right),
//                 ..
//             } => {
//                 controller.borrow_mut().press_button(Controller::RIGHT);
//             }
//             Event::KeyUp {
//                 keycode: Some(Keycode::Right),
//                 ..
//             } => {
//                 controller.borrow_mut().release_button(Controller::RIGHT);
//             }
//             Event::KeyDown {
//                 keycode: Some(Keycode::A),
//                 ..
//             } => {
//                 controller.borrow_mut().press_button(Controller::BUTTON_A);
//             }
//             Event::KeyUp {
//                 keycode: Some(Keycode::A),
//                 ..
//             } => {
//                 controller.borrow_mut().release_button(Controller::BUTTON_A);
//             }
//             Event::KeyDown {
//                 keycode: Some(Keycode::S),
//                 ..
//             } => {
//                 controller.borrow_mut().press_button(Controller::BUTTON_B);
//             }
//             Event::KeyUp {
//                 keycode: Some(Keycode::S),
//                 ..
//             } => {
//                 controller.borrow_mut().release_button(Controller::BUTTON_B);
//             }
//             Event::KeyDown {
//                 keycode: Some(Keycode::Z),
//                 ..
//             } => {
//                 controller.borrow_mut().press_button(Controller::SELECT);
//             }
//             Event::KeyUp {
//                 keycode: Some(Keycode::Z),
//                 ..
//             } => {
//                 controller.borrow_mut().release_button(Controller::SELECT);
//             }
//             Event::KeyDown {
//                 keycode: Some(Keycode::X),
//                 ..
//             } => {
//                 controller.borrow_mut().press_button(Controller::START);
//             }
//             Event::KeyUp {
//                 keycode: Some(Keycode::X),
//                 ..
//             } => {
//                 controller.borrow_mut().release_button(Controller::START);
//             }
//             _ => { /* do nothing */ }
//         }
//     }
// }
