extern crate cursive;
extern crate cursive_hexview;

mod cpu;
use cpu::Cpu;

use cursive::theme::{BaseColor::*, BorderStyle, Color::*, Palette, PaletteColor::*, Theme};
use cursive::traits::Nameable;
use cursive::view::SizeConstraint;
use cursive::views::{
    DebugView, Dialog, DummyView, LinearLayout, ResizedView, ScrollView, TextContent, TextView,
};
use cursive::{Printer, Vec2, View};
use cursive_hexview::{DisplayState, HexView};

///////////////////////////////////////////////////////////////////////////////////////////////////
// Let's define a buffer view, that shows the last lines from a stream.
//NOTE: this was stolen from the cursive logs.rs example, but i made it not async bc i dont like async
struct BufferView {
    // We'll use a ring buffer
    buffer: Vec<String>,
}

impl BufferView {
    // Creates a new view with the given buffer size
    fn new(size: usize) -> Self {
        let mut buffer = Vec::new();
        buffer.resize(size, String::new());
        BufferView { buffer }
    }

    //appends a series of lines to the buffer
    fn update(&mut self, v: &mut Vec<String>) {
        self.buffer.append(v);
    }
}

impl View for BufferView {
    //i literally do not know what this function does and im too afraid to ask
    fn layout(&mut self, _: Vec2) {
        // Before drawing, we'll want to update the buffer
        //self.update();
    }

    fn draw(&self, printer: &Printer) {
        // Print the latest up to (size) lines of the buffer
        for (i, line) in self.buffer.iter().rev().take(printer.size.y).enumerate() {
            printer.print((0, printer.size.y - 1 - i), line);
        }
    }
}
///////////////////////////////////////////////////////////////////////////////////////////////////

struct AppState {
    is_running: bool,
}

fn main() {
    //main structs
    let mut cur = cursive::default();
    cur.set_fps(15);
    let mut our_runner = cur.into_runner();
    //TODO: im pretty sure this is doing literally nothing lmfao

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
    let apu_view = ResizedView::new(SizeConstraint::Full, SizeConstraint::Full, DummyView);

    let vram_view = ResizedView::new(
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
        .child(Dialog::around(vram_view).title("VRAM"))
        .child(Dialog::around(rom_view).title("ROM"));

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

    //manual event loop
    loop {
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

            //update all our views
            our_runner.call_on_name("vram_memory", |view: &mut cursive_hexview::HexView| {
                view.set_data(our_cpu.get_data().iter());
            });

            our_runner.call_on_name("rom_memory", |view: &mut cursive_hexview::HexView| {
                view.set_data(our_cpu.rom.iter());
            });

            let log_line = format!("{:075X}", our_cpu.PC).to_owned();
            our_runner.call_on_name("buf", |v: &mut BufferView| v.update(&mut vec![log_line]));
        }

        //refresh our gui
        //NOTE: set_autorefresh() might do the same thing?
        let _event_received = our_runner.step();
        our_runner.refresh();
    }
}
