use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};

#[derive(Default)]
struct InputState {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

struct Player {
    x: f64,
    y: f64,
    size: f64,
    speed: f64,
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document");

    // Ensure canvas exists
    let canvas = match document.get_element_by_id("game-canvas") {
        Some(el) => el.dyn_into::<HtmlCanvasElement>()?,
        None => {
            let c = document.create_element("canvas")?;
            c.set_attribute("id", "game-canvas")?;
            document.body().unwrap().append_child(&c)?;
            c.dyn_into::<HtmlCanvasElement>()?
        }
    };

    let width = 800.0;
    let height = 600.0;
    canvas.set_width(width as u32);
    canvas.set_height(height as u32);
    canvas.style().set_property("width", &format!("{}px", width))?;
    canvas.style().set_property("height", &format!("{}px", height))?;

    let ctx = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    // Shared state
    let input = Rc::new(RefCell::new(InputState::default()));

    // Keydown
    {
        let input = input.clone();
        let on_keydown = Closure::wrap(Box::new(move |ev: KeyboardEvent| {
            match ev.key().as_str() {
                "ArrowUp" | "w" | "W" => input.borrow_mut().up = true,
                "ArrowDown" | "s" | "S" => input.borrow_mut().down = true,
                "ArrowLeft" | "a" | "A" => input.borrow_mut().left = true,
                "ArrowRight" | "d" | "D" => input.borrow_mut().right = true,
                _ => {}
            }
        }) as Box<dyn FnMut(_)>);
        window
            .add_event_listener_with_callback("keydown", on_keydown.as_ref().unchecked_ref())?;
        on_keydown.forget();
    }

    // Keyup
    {
        let input = input.clone();
        let on_keyup = Closure::wrap(Box::new(move |ev: KeyboardEvent| {
            match ev.key().as_str() {
                "ArrowUp" | "w" | "W" => input.borrow_mut().up = false,
                "ArrowDown" | "s" | "S" => input.borrow_mut().down = false,
                "ArrowLeft" | "a" | "A" => input.borrow_mut().left = false,
                "ArrowRight" | "d" | "D" => input.borrow_mut().right = false,
                _ => {}
            }
        }) as Box<dyn FnMut(_)>);
        window
            .add_event_listener_with_callback("keyup", on_keyup.as_ref().unchecked_ref())?;
        on_keyup.forget();
    }

    let player = Rc::new(RefCell::new(Player { x: width / 2.0, y: height / 2.0, size: 28.0, speed: 220.0 }));

    start_game_loop(ctx, input, player, width, height);

    Ok(())
}

fn start_game_loop(
    ctx: CanvasRenderingContext2d,
    input: Rc<RefCell<InputState>>,
    player: Rc<RefCell<Player>>,
    width: f64,
    height: f64,
) {
    let window = web_sys::window().unwrap();
    let ctx = Rc::new(ctx);

    // Animation closure holder
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let last_time = Rc::new(RefCell::new(0.0));

    let input2 = input.clone();
    let player2 = player.clone();
    let ctx2 = ctx.clone();
    let last_time2 = last_time.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |time: f64| {
        let dt = {
            let mut lt = last_time2.borrow_mut();
            let delta = if *lt == 0.0 { 0.0 } else { (time - *lt) / 1000.0 };
            *lt = time;
            delta
        };

        // Update player
        {
            let mut p = player2.borrow_mut();
            let inp = input2.borrow();
            let mut dx = 0.0;
            let mut dy = 0.0;
            if inp.left { dx -= 1.0; }
            if inp.right { dx += 1.0; }
            if inp.up { dy -= 1.0; }
            if inp.down { dy += 1.0; }
            let len = (dx*dx + dy*dy).sqrt();
            if len > 0.0 {
                dx /= len; dy /= len;
                p.x += dx * p.speed * dt;
                p.y += dy * p.speed * dt;
            }
            // clamp
            let half = p.size / 2.0;
            if p.x < half { p.x = half; }
            if p.y < half { p.y = half; }
            if p.x > width - half { p.x = width - half; }
            if p.y > height - half { p.y = height - half; }
        }

        // Render
        ctx2.set_fill_style(&JsValue::from_str("#0b0b0f"));
        ctx2.fill_rect(0.0, 0.0, width, height);

        // draw player as circle
        let p = player2.borrow();
        ctx2.begin_path();
        ctx2.set_fill_style(&JsValue::from_str("#ffd166"));
        let _ = ctx2.arc(p.x, p.y, p.size / 2.0, 0.0, std::f64::consts::PI * 2.0);
        ctx2.fill();

        // next frame
        let _ = window.request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref());
    }) as Box<dyn FnMut(f64)>));

    // start
    window.request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref()).unwrap();
}

