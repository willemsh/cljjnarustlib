#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use std::sync::{Arc, Mutex};

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::render::Canvas;
use sdl2::EventPump;
use sdl2::Sdl;


use sdl2::keyboard::{Keycode, Scancode};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};
use sdl2::ttf::{Font, Sdl2TtfContext};

use sdl2::video::{Window, WindowContext};

use std::collections::HashSet;

use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

use std::marker::Send;
use std::mem;
use std::path::Path;
use std::rc::Rc;
use std::time::Duration;

//use serde_json::{Result, Value};
use serde::{Deserialize, Serialize};
use serde_json::map::Map;
use serde_json::Value;
use std::error::Error;
use std::fs;
//use serde_json::Result;
use std::ffi::CStr;
use std::os::raw::c_char;

//https://www.libsdl.org/download-2.0.php
//Windows: SDL2-devel-2.0.12-VC.zip (Visual C++ 32/64-bit)

#[derive(Debug)]
pub struct GameObject {
    x: i32,
    y: i32,
    texture: String, // texture
}

#[repr(C)]
pub struct TextObject {
    x: i32,
    y: i32,
    font: i32,    // font
    text: String, // text
}

#[repr(C)]
pub struct App {
    pub sdl_context: Sdl,
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    pub input_state: InputState,
    pub texture_creator: TextureCreator<WindowContext>,
    pub world: World,
}

#[repr(C)]
pub struct World {
    pub objects: Vec<GameObject>,
}

#[repr(C)]
pub struct InputState {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    space: bool,
    m_left: bool,
    m_right: bool,
    m_x: i32,
    m_y: i32,
}

pub struct ConfigSerde;

impl ConfigSerde {
    pub fn read_config(path: &str) -> Result<Map<String, Value>, Box<dyn Error>> {
        let config = fs::read_to_string(path)?;
        let parsed: Value = serde_json::from_str(&config)?;
        let obj: Map<String, Value> = parsed.as_object().unwrap().clone();
        Ok(obj)
    }
}

pub fn check_exit<'a>(event_pump: &'a mut sdl2::EventPump) -> Result<bool, std::string::String> {
    //let mut prev_keys: Vec<sdl2::keyboard::Keycode> = HashSet::new();

    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return Ok(true),
            _ => {}
        }
    }

    Ok(false)
}
pub fn get_context() -> Sdl {
    let sdl_context = sdl2::init().unwrap();
    sdl_context
}

pub fn init_window(sdl_context: &Sdl) -> Window {
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 cljjnarustlib", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    window
}

pub fn render (
    app: &mut App,
) -> Result<(), String> {
    println!("rendering...");
    /*
    let texture1 = app.texture_creator.load_texture("resources/test2.png")?;
    let texture2 = app.texture_creator.load_texture("resources/test.png")?;
    let texture3 = app.texture_creator.load_texture("resources/pikachu.png")?;
    */
    //let font = font_manager.load(&details)?;
    app.canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));

    app.canvas.clear();

    //app.canvas.set_draw_color(Color::RGBA(0, 255, 0, 0));


    for (i, p) in app.world.objects.iter().enumerate() {
        //println!("{i}, {:?}", p);
        if p.texture.eq("") {
            continue;
        }

        let _r = app.canvas.draw_rect(Rect::new(p.x, p.y - 15, 5, 20));

        let texture = app.texture_creator.load_texture((&p.texture).as_str())?;
        let _a = app.canvas.copy(&texture, None, Rect::new(p.x, p.y, 20, 20));
    }

    app.canvas.present();
    //println!("OK");
    Ok(())
}

pub fn init () -> Arc<Mutex<App>> {
    let sdl_context = sdl2::init().unwrap();

    let window = init_window(&sdl_context);

    let canvas = window.into_canvas().build().unwrap();
    let event_pump: sdl2::EventPump = sdl_context.event_pump().unwrap();

    let texture_creator = canvas.texture_creator();


    let arc = Arc::new(Mutex::new(App {
        sdl_context: sdl_context,
        canvas: canvas,
        event_pump: event_pump,
        input_state: InputState {
            up: false,
            down: false,
            left: false,
            right: false,
            space: false,
            m_left: false,
            m_right: false,
            m_x: 0,
            m_y: 0,
        },
        texture_creator: texture_creator,
        world: World { objects: Vec::<GameObject>::new() }
    }));

    arc
}


// ---------
pub fn update_input_state(app: &mut App) {
    let ks = app.event_pump.keyboard_state();
    let ms = app.event_pump.mouse_state();
    // Create a set of pressed Keys.
    app.input_state.up = ks.is_scancode_pressed(Scancode::Up);
    app.input_state.down = ks.is_scancode_pressed(Scancode::Down);
    app.input_state.left = ks.is_scancode_pressed(Scancode::Left);
    app.input_state.right = ks.is_scancode_pressed(Scancode::Right);
    app.input_state.space = ks.is_scancode_pressed(Scancode::Space);

    app.input_state.m_x = ms.x();
    app.input_state.m_y = ms.y();
    app.input_state.m_left = ms.left();
    app.input_state.m_right = ms.right();
}

pub fn add_gameobject(world: &mut World, s: &str, x: i32, y: i32) {
    world.objects.push(GameObject { texture: s.to_string(), x:x, y:y});
}

pub fn move_gameobject(world: &mut World, idx: usize, x: i32, y: i32) {
    let p = world.objects.get_mut(idx).unwrap();
    p.x = x;
    p.y = y;
}
