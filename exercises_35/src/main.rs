use std::iter::Inspect;


enum WebEvent{
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click{x: i64, y: i64}
}

fn inspect(event: WebEvent){
    match event {
        WebEvent::PageLoad => println!("page load"),
        WebEvent::PageUnload => println!("page unload"),

        WebEvent::KeyPress(c) => println!("presione {}", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),

        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y)
        }
    }

}

fn main() {
    let presion = WebEvent::KeyPress('F');

    let load = WebEvent::PageLoad;

    let clock = WebEvent::Click { x: 2, y: 4 };

    inspect(presion);
    inspect(load);
    inspect(clock);
}
