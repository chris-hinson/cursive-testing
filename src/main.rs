extern crate cursive;
extern crate cursive_hexview;
extern crate sdl2;

mod cpu;
use cpu::Cpu;

mod my_views;
use my_views::BufferView;
use my_views::UltraHexaView;

use cursive::theme::{BaseColor::*, BorderStyle, Color::*, Palette, Theme};
use cursive::traits::Nameable;
use cursive::view::SizeConstraint;
use cursive::views::{Dialog, DummyView, LinearLayout, ResizedView};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use rand::Rng;

struct AppState {
    is_running: bool,
}

fn main() {
    //main structs
    let mut cur = cursive::default();
    cur.set_fps(15);
    let mut our_runner = cur.into_runner();

    //app state and our cpu
    let app_state = AppState { is_running: true };
    let mut our_cpu = Cpu::new();

    //our TUI needs an app state so we can update our cpu accordingly
    our_runner.set_user_data(app_state);

    /*
        Background => Dark(Blue)
        Shadow => Dark(Black)
        View => Dark(White)
        Primary => Dark(Black)
        Secondary => Dark(Blue)
        Tertiary => Light(White)
        TitlePrimary => Dark(Red)
        TitleSecondary => Dark(Yellow)
        Highlight => Dark(Red)
        HighlightInactive => Dark(Blue)
        HighlightText => Dark(White)
    */

    let mut our_palette = Palette::default();
    our_palette.set_color("Background", Dark(Yellow));
    our_palette.set_color("View", Dark(Black));
    our_palette.set_color("Primary", Dark(White));
    our_palette.set_color("TitlePrimary", Dark(Cyan));

    let our_theme = Theme {
        shadow: false,
        borders: BorderStyle::Simple,
        palette: our_palette,
    };
    our_runner.set_theme(our_theme);

    ///////////////////////////////////////////////////////////////////////////////////////////////
    let log_view = ResizedView::new(
        SizeConstraint::Full,
        SizeConstraint::Full,
        BufferView::new(75).with_name("buf"),
    );

    let cpu_state = ResizedView::new(SizeConstraint::Full, SizeConstraint::Full, DummyView);

    let ppu_view = ResizedView::new(SizeConstraint::Full, SizeConstraint::Full, DummyView);
    let apu_view = ResizedView::new(SizeConstraint::Full, SizeConstraint::Fixed(7), DummyView);

    /*let vram_view = ResizedView::new(
        SizeConstraint::Full,
        SizeConstraint::Fixed(15),
        ScrollView::new(
            HexView::new_from_iter(our_cpu.get_data().iter())
                .display_state(DisplayState::Enabled)
                .with_name("vram_memory"),
        ),
    );
    let rom_view = ResizedView::new(
        SizeConstraint::Full,
        SizeConstraint::Fixed(15),
        ScrollView::new(
            HexView::new_from_iter(our_cpu.get_data().iter())
                .display_state(DisplayState::Enabled)
                .with_name("rom_memory"),
        ),
    );*/

    let ram_view = ResizedView::new(
        SizeConstraint::Full,
        SizeConstraint::Fixed(15),
        UltraHexaView::new_from_iter_with_watch(&our_cpu.data, vec![1, 2, 3, 4])
            .with_name("ram_view"),
    );
    let rom_view = ResizedView::new(
        SizeConstraint::Full,
        SizeConstraint::Fixed(15),
        UltraHexaView::new_from_iter_with_watch(&our_cpu.rom, vec![1, 2, 3, 4])
            .with_name("rom_view"),
    );
    let chr_view = ResizedView::new(
        SizeConstraint::Full,
        SizeConstraint::Fixed(15),
        UltraHexaView::new_from_iter(&our_cpu.rom).with_name("chr_vieww"),
    );

    //add views to layer and add layer to screen
    let top_level = LinearLayout::horizontal()
        .child(Dialog::around(log_view).title("LOGS"))
        .child(
            LinearLayout::vertical()
                .child(
                    LinearLayout::horizontal()
                        .child(Dialog::around(cpu_state).title("CPU"))
                        .child(Dialog::around(ppu_view).title("PPU")),
                )
                .child(Dialog::around(apu_view).title("APU")),
        );

    let bottom_level = LinearLayout::horizontal()
        .child(Dialog::around(ram_view).title("VRAM"))
        .child(Dialog::around(rom_view).title("ROM"))
        .child(Dialog::around(chr_view).title("CHR"));

    our_runner.add_layer(
        LinearLayout::vertical()
            .child(top_level)
            .child(bottom_level),
    );
    ///////////////////////////////////////////////////////////////////////////////////////////////

    //add global keybinds

    //TODO: im like 99% sure this is leaking memory, but calling cur.quit() and or our_runner.quit()
    //just doesnt do anything lmfao
    our_runner.add_global_callback('q', |_cur| {
        panic!("panicked out");
    });

    //global callback to toggle appState's running variable
    our_runner.add_global_callback('w', |cur| {
        cur.with_user_data(|data: &mut AppState| {
            data.is_running = !data.is_running;
        });
    });

    //sdl2 stuff now
    ///////////////////////////////////////////////////////////////////////////////////////////////
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    ///////////////////////////////////////////////////////////////////////////////////////////////

    //manual event loop
    loop {
        //really we want to seperate our main loop into two things.
        //check our sdl event pump
        //check our app state through the tui's transient state
        //THEN if we are running, update our data (tui views and sdl data)
        //always update our tui and canvas, even if no new data was written

        let mut step_running = false;
        //update our cpu state if running
        our_runner.with_user_data(|s: &mut AppState| {
            if s.is_running {
                step_running = true;
            }
        });

        //only update views if our cpu is running rn, otherwise just do our base cursive stuff
        if step_running {
            //step our cpu
            our_cpu.step();

            //generate some test data
            let mut r = rand::thread_rng();
            let mut test_data: Vec<(usize, u8)> = Vec::new();

            for _i in 0..10 {
                let index = r.gen_range(0..our_cpu.data.len());
                let val = r.gen_range(0..u8::MAX);
                test_data.push((index, val));
            }

            our_runner.call_on_name("rom_view", |view: &mut UltraHexaView| {
                view.update_data(&test_data);
            });

            our_runner.call_on_name("ram_view", |view: &mut UltraHexaView| {
                view.update_data(&test_data);
            });

            let log_line = format!("{:075X}", our_cpu.pc).to_owned();
            our_runner.call_on_name("buf", |v: &mut BufferView| v.update(&mut vec![log_line]));
        }

        //refresh our tui
        //NOTE: set_autorefresh() might do the same thing?
        let _event_received = our_runner.step();
        our_runner.refresh();

        //sdl2 drawing
        if step_running {
            i = (i + 1) % 255;
            canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
            canvas.clear();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => panic!("lol"),
                    _ => {}
                }
            }
            // The rest of the game loop goes here...

            canvas.present();
        }
    }
}
