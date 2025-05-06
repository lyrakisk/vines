use vines::cpu::mappers::basic_mapper::*;
use vines::ppu::PPU;
use vines::rom::*;
use vines::{controller::*, cpu::*};


use std::cell::RefCell;
use std::env;
use std::fs::read;
use std::path::PathBuf;
use std::rc::Rc;

use std::time::Instant;

fn main() {
    env::set_var("RUST_BACKTRACE", "full");

    let rom_bytes = read(PathBuf::from("examples/pacman/pacman.nes")).unwrap();
    let rom = Rom::try_from(&rom_bytes).unwrap();
    println!("prg rom len: {}", rom.prg_rom.len());
    println!("chr rom len: {}", rom.chr_rom.len());
    let ppu = Rc::new(RefCell::new(PPU::new(rom.chr_rom.clone())));

    let controller = Rc::new(RefCell::new(Controller::new()));
    let cpu_mapper = BasicMapper::new(rom, ppu.clone(), controller.clone());
    let mut cpu = CPU::new(cpu_mapper);

    cpu.reset();

    let mut total_cycles: usize = 0;
    let mut dur_sum = 0;
    let mut dur_count = 0;

    loop {
        // let start = Instant::now();

        if total_cycles % 6820 == 0 {
            handle_user_input();
            render();
        }

        let instruction_result = cpu.execute_next_instruction();

        for _ in 0..instruction_result.executed_cycles {
            ppu.borrow_mut().tick();
        }

        total_cycles += instruction_result.executed_cycles as usize;

    }
}

fn render() {
    todo!()
}

fn handle_user_input() {
    todo!()
}
// fn handle_user_input(controller: Rc<RefCell<Controller>>, event_pump: &mut EventPump) {
//     let mut controller = controller.borrow_mut();
//     for event in event_pump.poll_iter() {
//         if !event.is_keyboard() {
//             continue;
//         }

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
//                 controller.press_button(Controller::DOWN);
//             }
//             Event::KeyUp {
//                 keycode: Some(Keycode::Down),
//                 ..
//             } => {
//                 controller.release_button(Controller::DOWN);
//             }
//             Event::KeyDown {
//                 keycode: Some(Keycode::Up),
//                 ..
//             } => {
//                 controller.press_button(Controller::UP);
//             }
//             Event::KeyUp {
//                 keycode: Some(Keycode::Up),
//                 ..
//             } => {
//                 controller.release_button(Controller::UP);
//             }
//             Event::KeyDown {
//                 keycode: Some(Keycode::Left),
//                 ..
//             } => {
//                 controller.press_button(Controller::LEFT);
//             }
//             Event::KeyUp {
//                 keycode: Some(Keycode::Left),
//                 ..
//             } => {
//                 controller.release_button(Controller::LEFT);
//             }
//             Event::KeyDown {
//                 keycode: Some(Keycode::Right),
//                 ..
//             } => {
//                 controller.press_button(Controller::RIGHT);
//             }
//             Event::KeyUp {
//                 keycode: Some(Keycode::Right),
//                 ..
//             } => {
//                 controller.release_button(Controller::RIGHT);
//             }
//             Event::KeyDown {
//                 keycode: Some(Keycode::A),
//                 ..
//             } => {
//                 controller.press_button(Controller::BUTTON_A);
//             }
//             Event::KeyUp {
//                 keycode: Some(Keycode::A),
//                 ..
//             } => {
//                 controller.release_button(Controller::BUTTON_A);
//             }
//             Event::KeyDown {
//                 keycode: Some(Keycode::S),
//                 ..
//             } => {
//                 controller.press_button(Controller::BUTTON_B);
//             }
//             Event::KeyUp {
//                 keycode: Some(Keycode::S),
//                 ..
//             } => {
//                 controller.release_button(Controller::BUTTON_B);
//             }
//             Event::KeyDown {
//                 keycode: Some(Keycode::Z),
//                 ..
//             } => {
//                 controller.press_button(Controller::SELECT);
//             }
//             Event::KeyUp {
//                 keycode: Some(Keycode::Z),
//                 ..
//             } => {
//                 controller.release_button(Controller::SELECT);
//             }
//             Event::KeyDown {
//                 keycode: Some(Keycode::X),
//                 ..
//             } => {
//                 controller.press_button(Controller::START);
//             }
//             Event::KeyUp {
//                 keycode: Some(Keycode::X),
//                 ..
//             } => {
//                 controller.release_button(Controller::START);
//             }
//             _ => { /* do nothing */ }
//         }
//     }
// }
