#![feature(core_intrinsics)]

extern crate core;
extern crate image;

pub mod my_screenshot;

use inputbot::{KeybdKey::*, MouseButton::*, *};
use std::{thread::sleep, time::Duration};
use std::backtrace::BacktraceStatus::Captured;
use std::borrow::Borrow;
use std::io::ErrorKind::WouldBlock;
use std::path::Path;
use enigo::{Enigo, MouseControllable, MouseButton};
use std::thread;
use std::time::{Instant, SystemTime};
use anyhow::Result;
use chrono::Utc;
use image::{DynamicImage, GenericImageView};
use crate::my_screenshot::{Screenshot};
use crate::my_screenshot::ffi::Capturer;
use image::io::Reader as ImageReader;

static mut RUNNING: bool = true;
static mut CURSOR_ON_COOKIE: bool = false;

fn find_all(path: &'static str) -> Vec<(i32, i32)> {
    let line_y = 31;

    let holder = ImageReader::open("test_holder.png").unwrap().decode().unwrap();

    /*for x in 0..to_find.width() {
        if to_find.get_pixel(x, to_find.height() - 1).0 == [0, 0, 0, 255] {
            let anchor: (i32, i32) = (x as i32, (to_find.height() - 1) as i32);
            for x_holder in 0..holder.width() {
                let pixel_color = holder.get_pixel(x_holder, line_height).0;
                if pixel_color[0] == 255 && pixel_color[1] == 255 && pixel_color[2] == 255 {

                }
            }

            is_first_black = false;
        }
    }*/

    let mut numbers: Vec<DynamicImage> = vec![];

    for i in 0..10 {
        numbers.push(
            ImageReader::open(path.to_owned() + format!("{}.png", i).as_str())
            .unwrap()
            .decode()
            .unwrap()
        );
    }

    'line: for x in 0..holder.width() {
        if holder.get_pixel(x, line_y).0 == [255, 255, 255, 255] {
            let anchor = (x as i32, line_y as i32);
            'number: for number in 0..10 {
                let to_find = &numbers[number];
                let mut matches = true;
                for y in (0..to_find.height()).rev() {
                    for x in 0..to_find.width() {
                        if to_find.get_pixel(x, y).0 == [0, 0, 0, 255] {
                            let (x_holder_check, y_holder_check) = (anchor.0 as u32 + x, anchor.1 as u32 - (to_find.height() as u32 - y as u32 - 1u32));
                            if holder.get_pixel(x_holder_check as u32, y_holder_check as u32).0 != [255, 255, 255, 255] {
                                matches = false;
                                continue 'number;
                            }
                        }
                    }
                }
                println!("{} {} {:?}", matches, number, anchor);
                continue 'line;
            }
            //println!("{}, {:?}", matches, anchor);
        }
    }


    vec![]
}

fn main() {
    //find_all("cookie_numbers/");
    let mut capturer = Capturer::new(0);
    let mut enigo = Enigo::new();

    bind_keybinds();
    thread::spawn(|| {
        handle_input_events();
    });

    enigo.mouse_move_to(287, 417);
    unsafe { CURSOR_ON_COOKIE = true; }
    thread::spawn(|| {
        auto_click();
    });

    thread::spawn(|| {
        let mut enigo = Enigo::new();
        let mut building_capturer = Capturer::new(0);
        loop {
            let screenshot = building_capturer.get_screenshot().unwrap();
            check_buildings(&screenshot, &mut enigo, 0);
            sleep(Duration::from_secs(10 * 60));
        }
    });

    loop {
        unsafe { if RUNNING == true {
            let screenshot = capturer.get_screenshot().unwrap();
            if !RUNNING { continue; }
            check_upgrades(&screenshot, &mut enigo);
        } else {
            sleep(Duration::from_millis(100));
        }}
    }
}

fn auto_click() {
    let mut enigo = Enigo::new();
    let mut click_counter: u128 = 0;
    let mut send_message = false;
    loop {
        unsafe {
            if CURSOR_ON_COOKIE && RUNNING {
                enigo.mouse_click(MouseButton::Left);
                if click_counter + 100 > u128::MAX && !send_message {
                    println!("reached max value at {}", Utc::now());
                    send_message = true;
                } else {
                    click_counter += 1;
                }
                sleep(Duration::from_micros(1000));
            } else if !RUNNING {
                println!("click count: {}", click_counter);
                //1547391 + 53524 + 1587876 + 1588045
                sleep(Duration::from_millis(1000));
            }
        }
    }
}

fn check_buildings(screenshot: &Screenshot, enigo: &mut Enigo, mut counter: u32) {
    if counter > 20 {
        print!("counter reached 20!");
        return;
    }
    counter += 1;
    println!("check buildings");
    unsafe { CURSOR_ON_COOKIE = false; }
    let mut can_buy = false;
    if compare_colors(check_pixel_color(&screenshot, 1692, 786), (255, 255, 255), 2) {
        enigo.mouse_move_to(1692, 786);
        enigo.mouse_click(MouseButton::Left);
        can_buy = true;
        println!("bought (\"alchemy lab\")");
    }
    if compare_colors(check_pixel_color(screenshot, 1691, 725), (255, 255, 255), 2) {
        enigo.mouse_move_to(1691, 725);
        enigo.mouse_click(MouseButton::Left);
        can_buy = true;
        println!("bought (\"shipment\")");
    }
    if compare_colors(check_pixel_color(screenshot, 1685, 659), (255, 255, 255), 2) {
        enigo.mouse_move_to(1685, 659);
        enigo.mouse_click(MouseButton::Left);
        can_buy = true;
        println!("bought (\"wizard tower\")");
    }
    if compare_colors(check_pixel_color(&screenshot, 1681, 595), (255, 255, 255), 2) {
        enigo.mouse_move_to(1681, 595);
        enigo.mouse_click(MouseButton::Left);
        can_buy = true;
        println!("bought (\"temple\")");
    }
    if compare_colors(check_pixel_color(&screenshot, 1677, 534), (255, 255, 255), 2) {
        enigo.mouse_move_to(1677, 534);
        enigo.mouse_click(MouseButton::Left);
        can_buy = true;
        println!("bought (\"bank\")");
    }
    if compare_colors(check_pixel_color(&screenshot, 1677, 470), (255, 255, 255), 2) {
        enigo.mouse_move_to(1677, 470);
        enigo.mouse_click(MouseButton::Left);
        can_buy = true;
        println!("bought (\"factory\")");
    }
    if compare_colors(check_pixel_color(&screenshot, 1694, 398), (255, 255, 255), 2) {
        enigo.mouse_move_to(1694, 398);
        enigo.mouse_click(MouseButton::Left);
        can_buy = true;
        println!("bought (\"mine\")");
    }
    if compare_colors(check_pixel_color(&screenshot, 1677, 344), (255, 255, 255), 2) {
        enigo.mouse_move_to(1677, 344);
        enigo.mouse_click(MouseButton::Left);
        can_buy = true;
        println!("bought (\"farm\")");
    }
    if compare_colors(check_pixel_color(&screenshot, 1723, 279), (255, 255, 255), 2) {
        enigo.mouse_move_to(1723, 279);
        enigo.mouse_click(MouseButton::Left);
        can_buy = true;
        println!("bought (\"grandma\")");
    }
    println!("cursor color: {:?}", check_pixel_color(&screenshot, 1692, 215));
    if compare_colors(check_pixel_color(&screenshot, 1692, 215), (255, 255, 255), 10) {
        enigo.mouse_move_to(1692, 215);
        enigo.mouse_click(MouseButton::Left);
        can_buy = true;
        println!("bought (\"cursor\")");
    }

    enigo.mouse_move_to(287, 417);
    unsafe { CURSOR_ON_COOKIE = true; }

    if can_buy {
        check_buildings(screenshot, enigo, counter);
    }
}

fn check_upgrades(screenshot: &Screenshot, enigo: &mut Enigo) {
    unsafe { CURSOR_ON_COOKIE = false; }
    if compare_colors(check_pixel_color(screenshot, 1845, 85), (230, 190, 148), 1) {
        enigo.mouse_move_to(1845 + 10, 85 + 10);
        enigo.mouse_click(MouseButton::Left);
        println!("bought (\"fifth upgrade\")");
    }
    if compare_colors(check_pixel_color(screenshot, 1785, 85), (230, 190, 148), 1) {
        enigo.mouse_move_to(1785 + 10, 85 + 10);
        enigo.mouse_click(MouseButton::Left);
        println!("bought (\"forth upgrade\")");
    }
    if compare_colors(check_pixel_color(screenshot, 1725, 85), (230, 190, 148), 1) {
        enigo.mouse_move_to(1725 + 10, 85 + 10);
        enigo.mouse_click(MouseButton::Left);
        println!("bought (\"third upgrade\")");
    }
    if compare_colors(check_pixel_color(screenshot, 1665, 85), (230, 190, 148), 1) {
        enigo.mouse_move_to(1665 + 10, 85 + 10);
        enigo.mouse_click(MouseButton::Left);
        println!("bought (\"second upgrade\")");
    }
    if compare_colors(check_pixel_color(screenshot, 1605, 85), (230, 190, 148), 1) {
        enigo.mouse_move_to(1605 + 10, 85 + 10);
        enigo.mouse_click(MouseButton::Left);
        println!("bought (\"first upgrade\")");
    }
    enigo.mouse_move_to(287, 417);
    unsafe { CURSOR_ON_COOKIE = true; }
}

fn compare_colors((r1, g1, b1): (u8, u8, u8), (r2, g2, b2): (u8, u8, u8), tolerance: u8) -> bool {
    (r1 as i16 - r2 as i16).abs() <= tolerance as i16
    && (b1 as i16 - b2 as i16).abs() <= tolerance as i16
    && (g1 as i16 - g2 as i16).abs() <= tolerance as i16
}

fn check_pixel_color(screenshot: &Screenshot, x: i32, y: i32) -> (u8, u8, u8) {
    if x < 0 || y < 0 {
        return (0, 0, 0)
    }
    let pixel = screenshot.get_pixel(y as usize, x as usize);
    (pixel.r, pixel.g, pixel.b)
}

fn bind_keybinds() {
    YKey.bind(|| {
        unsafe {
            RUNNING = !RUNNING;
            println!("{}", RUNNING);
        }
    })
}