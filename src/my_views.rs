use cursive::Printer;
use cursive::Vec2;
use cursive::View;

///////////////////////////////////////////////////////////////////////////////////////////////////
// Let's define a buffer view, that shows the last lines from a stream.
//NOTE: this was stolen from the cursive logs.rs example, but i made it not async bc i dont like async
pub struct BufferView {
    // We'll use a ring buffer
    buffer: Vec<String>,
}

impl BufferView {
    // Creates a new view with the given buffer size
    pub fn new(size: usize) -> Self {
        let mut buffer = Vec::new();
        buffer.resize(size, String::new());
        BufferView { buffer }
    }

    //appends a series of lines to the buffer
    pub fn update(&mut self, v: &mut Vec<String>) {
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
