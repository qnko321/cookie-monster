extern crate core;

use inputbot::{KeybdKey::*, MouseButton::*, *};
use std::{thread::sleep, time::Duration};
use std::borrow::Borrow;
use std::io::ErrorKind::WouldBlock;
use enigo::{Enigo, MouseControllable, MouseButton};
use std::thread;
use std::time::{Instant, SystemTime};
use screenshot::{get_screenshot, Screenshot};
use anyhow::Result;
use chrono::Utc;

static mut RUNNING: bool = true;
static mut CURSOR_ON_COOKIE: bool = false;

fn main() {
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
        loop {
            let screenshot = get_screenshot(0).unwrap();
            check_buildings(&screenshot, &mut enigo, 0);
            sleep(Duration::from_secs(10 * 60));
        }

    });

    loop {
        unsafe { if RUNNING == true {
            let screenshot = get_screenshot(0).unwrap();
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
                //1547391 + 53524
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
    if compare_colors(check_pixel_color(&screenshot, 1692, 215), (255, 255, 255), 10) {
        enigo.mouse_move_to(1692, 215);
        enigo.mouse_click(MouseButton::Left);
        can_buy = true;
        println!("bought (\"cursor\")");
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

    /*if compare_colors(check_pixel_color(&screenshot, 1621, 94), (12, 26, 36), 10) {
        enigo.mouse_move_to(1685, 659);
        enigo.mouse_click(MouseButton::Left);
        can_buy = true;
        println!("affordable upgrade 1");
    }
    TODO: Following need testing
    */

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