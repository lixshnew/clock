use console_engine::pixel;
use console_engine::pixel::Pixel;
use console_engine::Color;
use console_engine::KeyCode;
use console_engine::ConsoleEngine;
use chrono::Local;

#[derive(PartialEq)]
enum Mode {
    DOT,
    SEC,
}
struct Digit {
    x: i32,
    y: i32,
    d: char,
}

impl Digit {
    fn new(x:i32,y:i32)-> Digit {
        Digit{
            x,
            y,
            d: '0' 
        }
    }
}

struct Colon {
    x: i32,
    y: i32,
}

impl Colon {
    fn new(x:i32,y:i32)-> Colon {
        Colon{
            x,
            y,
        }
    }
}

struct TimeStr{
    width: i32,
    height: i32,
    mode: Mode,
    dot: bool,
    h1: Digit,
    h2: Digit,
    c1: Colon,
    m1: Digit,
    m2: Digit,
    c2: Colon,
    s1: Digit,
    s2: Digit,
}

impl TimeStr{
    fn new()-> TimeStr {
        TimeStr { 
            width: 50, 
            height: 14, 
            mode: Mode::SEC,
            dot: true,
            h1: Digit::new(9,3),
            h2: Digit::new(13,3),
            c1: Colon::new(17,3),
            m1: Digit::new(19,3),
            m2: Digit::new(23,3),
            c2: Colon::new(27,3),
            s1: Digit::new(29,3),
            s2: Digit::new(33,3),
        }
    }

    fn draw(&self, eng: &mut ConsoleEngine){
        if self.mode == Mode::SEC {
            draw_digit(eng, &self.h1);
            draw_digit(eng, &self.h2);
            draw_colon(eng, self.c1.x, self.c1.y);
            draw_digit(eng, &self.m1);
            draw_digit(eng, &self.m2);
            draw_colon(eng, self.c2.x, self.c2.y);
            draw_digit(eng, &self.s1);
            draw_digit(eng, &self.s2);
        }else{
            draw_digit(eng, &self.h1);
            draw_digit(eng, &self.h2);
            if self.dot {
                draw_colon(eng, self.c1.x, self.c1.y);
            }else{
                clear_colon(eng, self.c1.x, self.c1.y);
            }
            draw_digit(eng, &self.m1);
            draw_digit(eng, &self.m2);
            clear_colon(eng,self.c2.x,self.c2.y);
            clear_digit(eng, &self.s1);
            clear_digit(eng, &self.s2);
        }
        
    }
}

fn clear_digit(eng: &mut ConsoleEngine,digit: &Digit){
    for i in 0..4 {
        for j in 0..8{
            eng.set_pxl(digit.x + i, digit.y + j, pixel::pxl(' '));
        }
    }
}
fn draw_digit(eng: &mut ConsoleEngine,digit: &Digit){

    let num = match digit.d {
        '1' => &ONE,
        '2' => &TWO,
        '3' => &THREE,
        '4' => &FOUR,
        '5' => &FIVE,
        '6' => &SIX,
        '7' => &SEVEN,
        '8' => &EIGHT,
        '9' => &NINE,
        _ => &ZERO,
    };

    for i in 0..4 {
        for j in 0..8{
            let index = (i+j*4) as usize;
            eng.set_pxl(digit.x + i, digit.y + j, pixel::pxl(num[index]));
        }
    }
}

fn clear_colon(eng: &mut ConsoleEngine, x: i32, y:i32){
    for i in 0..2 {
        for j in 0..8{
            eng.set_pxl(x + i, y + j, pixel::pxl(' '));
        }
    }
}

fn draw_colon(eng: &mut ConsoleEngine, x: i32, y:i32){
    for i in 0..2 {
        for j in 0..8{
            let index = (i+j*2) as usize;
            eng.set_pxl(x + i, y + j, pixel::pxl(COLON[index]));
        }
    }
}
fn main() {

    let mut timestr = TimeStr::new();
    let mut engine = ConsoleEngine::init(timestr.width as u32, timestr.height as u32, 4).unwrap();
    engine.clear_screen(); 
    //engine.fill_rect(0, 0, 49, 13, pixel::pxl('+'));
    
    let mut n = 0;

    loop {
        
        let t = Local::now().format("%H:%M:%S").to_string();
        // 20:13:45
        timestr.h1.d = t.chars().nth(0).unwrap();
        timestr.h2.d = t.chars().nth(1).unwrap();
        timestr.m1.d = t.chars().nth(3).unwrap();
        timestr.m2.d = t.chars().nth(4).unwrap();
        timestr.s1.d = t.chars().nth(6).unwrap();
        timestr.s2.d = t.chars().nth(7).unwrap();

        // if n==0 || n == 1 {
        //     timestr.dot = true;
        // }else{
        //     timestr.dot = false;
        // }

        if n == 0 {
            timestr.dot = ! timestr.dot;
        }

        timestr.draw(&mut engine);

        if engine.is_key_pressed(KeyCode::Char('q')) { // if the user presses 'q' :
            break; // exits app
        }

        if engine.is_key_pressed(KeyCode::Char('s')) { // if the user presses 'q' :
            timestr.mode = Mode::SEC;
        }
        if engine.is_key_pressed(KeyCode::Char('d')) { // if the user presses 'q' :
            timestr.mode = Mode::DOT;
        }

        n += 1;
        n %= 4;
    
        engine.draw(); // draw the screen
        engine.wait_frame(); // wait for next frame + capture inputs
    }


}

//                        0   1   2   3   4   5   6
const CHARS: [char;7] = [' ','━','┃','┏','┓','┗','┛'];
const ZERO:[char;32]=[
    CHARS[3],CHARS[1],CHARS[1],CHARS[4],
    CHARS[2],CHARS[3],CHARS[4],CHARS[2],
    CHARS[2],CHARS[2],CHARS[2],CHARS[2],
    CHARS[2],CHARS[2],CHARS[2],CHARS[2],
    CHARS[2],CHARS[2],CHARS[2],CHARS[2],
    CHARS[2],CHARS[2],CHARS[2],CHARS[2],
    CHARS[2],CHARS[5],CHARS[6],CHARS[2],
    CHARS[5],CHARS[1],CHARS[1],CHARS[6],
    ];
//                          0   1   2   3   4   5   6
//const CHARS: [char;7] = [' ','━','┃','┏','┓','┗','┛'];
// const ONE:[char;32]=[
//     CHARS[0],CHARS[0],CHARS[3],CHARS[4],
//     CHARS[0],CHARS[0],CHARS[2],CHARS[2],
//     CHARS[0],CHARS[0],CHARS[2],CHARS[2],
//     CHARS[0],CHARS[0],CHARS[2],CHARS[2],
//     CHARS[0],CHARS[0],CHARS[2],CHARS[2],
//     CHARS[0],CHARS[0],CHARS[2],CHARS[2],
//     CHARS[0],CHARS[0],CHARS[2],CHARS[2],
//     CHARS[0],CHARS[0],CHARS[5],CHARS[6],
//     ];

// Digit 1 << 1 pixel, looks better
const ONE:[char;32]=[
        CHARS[0],CHARS[3],CHARS[4],CHARS[0],
        CHARS[0],CHARS[2],CHARS[2],CHARS[0],
        CHARS[0],CHARS[2],CHARS[2],CHARS[0],
        CHARS[0],CHARS[2],CHARS[2],CHARS[0],
        CHARS[0],CHARS[2],CHARS[2],CHARS[0],
        CHARS[0],CHARS[2],CHARS[2],CHARS[0],
        CHARS[0],CHARS[2],CHARS[2],CHARS[0],
        CHARS[0],CHARS[5],CHARS[6],CHARS[0],
        ];
//                          0   1   2   3   4   5   6
//const CHARS: [char;7] = [' ','━','┃','┏','┓','┗','┛'];
const TWO:[char;32]=[
    CHARS[3],CHARS[1],CHARS[1],CHARS[4],
    CHARS[5],CHARS[1],CHARS[4],CHARS[2],
    CHARS[0],CHARS[0],CHARS[2],CHARS[2],
    CHARS[3],CHARS[1],CHARS[6],CHARS[2],
    CHARS[2],CHARS[3],CHARS[1],CHARS[6],
    CHARS[2],CHARS[2],CHARS[0],CHARS[0],
    CHARS[2],CHARS[5],CHARS[1],CHARS[4],
    CHARS[5],CHARS[1],CHARS[1],CHARS[6],
    ];
//                          0   1   2   3   4   5   6
//const CHARS: [char;7] = [' ','━','┃','┏','┓','┗','┛'];
const THREE:[char;32]=[
    CHARS[3],CHARS[1],CHARS[1],CHARS[4],
    CHARS[5],CHARS[1],CHARS[4],CHARS[2],
    CHARS[0],CHARS[0],CHARS[2],CHARS[2],
    CHARS[3],CHARS[1],CHARS[6],CHARS[2],
    CHARS[5],CHARS[1],CHARS[4],CHARS[2],
    CHARS[0],CHARS[0],CHARS[2],CHARS[2],
    CHARS[3],CHARS[1],CHARS[6],CHARS[2],
    CHARS[5],CHARS[1],CHARS[1],CHARS[6],
    ];

//                          0   1   2   3   4   5   6
//const CHARS: [char;7] = [' ','━','┃','┏','┓','┗','┛'];
const FOUR:[char;32]=[
    CHARS[3],CHARS[4],CHARS[3],CHARS[4],
    CHARS[2],CHARS[2],CHARS[2],CHARS[2],
    CHARS[2],CHARS[2],CHARS[2],CHARS[2],
    CHARS[2],CHARS[5],CHARS[6],CHARS[2],
    CHARS[5],CHARS[1],CHARS[4],CHARS[2],
    CHARS[0],CHARS[0],CHARS[2],CHARS[2],
    CHARS[0],CHARS[0],CHARS[2],CHARS[2],
    CHARS[0],CHARS[0],CHARS[5],CHARS[6],
    ];

//                          0   1   2   3   4   5   6
//const CHARS: [char;7] = [' ','━','┃','┏','┓','┗','┛'];
const FIVE:[char;32]=[
    CHARS[3],CHARS[1],CHARS[1],CHARS[4],
    CHARS[2],CHARS[3],CHARS[1],CHARS[6],
    CHARS[2],CHARS[2],CHARS[0],CHARS[0],
    CHARS[2],CHARS[5],CHARS[1],CHARS[4],
    CHARS[5],CHARS[1],CHARS[4],CHARS[2],
    CHARS[0],CHARS[0],CHARS[2],CHARS[2],
    CHARS[3],CHARS[1],CHARS[6],CHARS[2],
    CHARS[5],CHARS[1],CHARS[1],CHARS[6],
    ];

//                          0   1   2   3   4   5   6
//const CHARS: [char;7] = [' ','━','┃','┏','┓','┗','┛'];
const SIX:[char;32]=[
    CHARS[3],CHARS[1],CHARS[1],CHARS[4],
    CHARS[2],CHARS[3],CHARS[1],CHARS[6],
    CHARS[2],CHARS[2],CHARS[0],CHARS[0],
    CHARS[2],CHARS[5],CHARS[1],CHARS[4],
    CHARS[2],CHARS[3],CHARS[4],CHARS[2],
    CHARS[2],CHARS[2],CHARS[2],CHARS[2],
    CHARS[2],CHARS[5],CHARS[6],CHARS[2],
    CHARS[5],CHARS[1],CHARS[1],CHARS[6],
    ];

//                          0   1   2   3   4   5   6
//const CHARS: [char;7] = [' ','━','┃','┏','┓','┗','┛'];
const SEVEN:[char;32]=[
    CHARS[3],CHARS[1],CHARS[1],CHARS[4],
    CHARS[5],CHARS[1],CHARS[4],CHARS[2],
    CHARS[0],CHARS[0],CHARS[2],CHARS[2],
    CHARS[0],CHARS[0],CHARS[2],CHARS[2],
    CHARS[0],CHARS[0],CHARS[2],CHARS[2],
    CHARS[0],CHARS[0],CHARS[2],CHARS[2],
    CHARS[0],CHARS[0],CHARS[2],CHARS[2],
    CHARS[0],CHARS[0],CHARS[5],CHARS[6],
    ];
//                          0   1   2   3   4   5   6
//const CHARS: [char;7] = [' ','━','┃','┏','┓','┗','┛'];
const EIGHT:[char;32]=[
    CHARS[3],CHARS[1],CHARS[1],CHARS[4],
    CHARS[2],CHARS[3],CHARS[4],CHARS[2],
    CHARS[2],CHARS[2],CHARS[2],CHARS[2],
    CHARS[2],CHARS[5],CHARS[6],CHARS[2],
    CHARS[2],CHARS[3],CHARS[4],CHARS[2],
    CHARS[2],CHARS[2],CHARS[2],CHARS[2],
    CHARS[2],CHARS[5],CHARS[6],CHARS[2],
    CHARS[5],CHARS[1],CHARS[1],CHARS[6],
    ];
//                          0   1   2   3   4   5   6
//const CHARS: [char;7] = [' ','━','┃','┏','┓','┗','┛'];
const NINE:[char;32]=[
    CHARS[3],CHARS[1],CHARS[1],CHARS[4],
    CHARS[2],CHARS[3],CHARS[4],CHARS[2],
    CHARS[2],CHARS[2],CHARS[2],CHARS[2],
    CHARS[2],CHARS[5],CHARS[6],CHARS[2],
    CHARS[5],CHARS[1],CHARS[4],CHARS[2],
    CHARS[0],CHARS[0],CHARS[2],CHARS[2],
    CHARS[3],CHARS[1],CHARS[6],CHARS[2],
    CHARS[5],CHARS[1],CHARS[1],CHARS[6],
    ];
//                         0   1   2   3   4   5   6
//onst CHARS: [char;7] = [' ','━','┃','┏','┓','┗','┛'];
const COLON:[char;16]=[
    CHARS[0],CHARS[0],
    CHARS[0],CHARS[0],
    CHARS[3],CHARS[4],
    CHARS[5],CHARS[6],
    CHARS[0],CHARS[0],
    CHARS[3],CHARS[4],
    CHARS[5],CHARS[6],
    CHARS[0],CHARS[0],
    ];
