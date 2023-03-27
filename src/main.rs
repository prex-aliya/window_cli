use std::process;

use ncurses::{*, ll::printw};

const BOARDER: i32 = 3;


struct Window {
    x: i32,
    y: i32,
    width: i32,
    length: i32,
    content: String
}

impl Window {
    fn print(&mut self) {
        let width = self.width;
        mvprintw(self.x, self.y, "X");
        for i in 1..=self.width { mvprintw(self.x, self.y+i, "#"); }

        for i in 1..self.length {
            mvprintw(i+self.x, self.y, "#");
            for _ in 0..width {
                addstr(" ");
            }
            mvprintw(i+self.x, self.y+self.width, "#");
        }

        for i in 0..=self.width { mvprintw(self.x+self.length, self.y+i, "#"); }

        mvprintw(self.x+1, self.y+2, &self.content);
    }
    fn location(&mut self, x: i32, y: i32) {
        if !((self.x + x) < BOARDER) {
            self.x += x;
        }
        if !((self.y + y) < BOARDER) {
            self.y += y; 
        }
    }
}

fn keys(window: &mut Window, key: char) {
    match key as u8 as char {
        'w' => window.location(-1,0),
        'r' => window.location(1,0),
        'a' => window.location(0,-1),
        's' => window.location(0,1),

        '-' => window.width -= 1,
        '=' => window.width += 1,
        '_' => window.length -= 1,
        '+' => window.length += 1,

        'i' => {
            let mut input = getch();
            let mut output = String::new();
            while input as u8 as char != '\n' {
                output.push(input as u8 as char);
                input = getch();
            }
            window.content = output;
        }


        _ => {}
    }
}

fn print(window: &mut Window) {
    window.print();
}

fn main() {
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    // Enables keypad mode. This makes (at least for me) mouse events getting
    // reported as KEY_MOUSE, instead as of random letters.
    keypad(stdscr(), true);
    // Don't mask any mouse events
    //mousemask(ALL_MOUSE_EVENTS.try_into().unwrap(), 0);

    print!("\033[?1003h\n");


    let mut windows: Vec<Window> = 
        vec![Window { x: 4, y: 5, width: 15, length: 10, content: "First".to_string() }, Window { x: 15, y: 5, width: 15, length: 10, content: "Secondary".to_string() }];
    let mut focus: i32 = 0;

    let mut quit = false;
    while !quit {
        clear();

        for x in (0..windows.len()).rev() {
            if x != focus.try_into().unwrap() {
                print(&mut windows[x]);
            }
        }

        print(&mut windows[focus as usize]);

        let max: i32 = windows.len() as i32;
        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            '\t' => {
                if focus < max-1 {
                    focus += 1;
                } else {
                    focus = 0;
                }
            },
            '\n' => windows.push(Window {x: 1, y:1, width: 15, length: 10, content: "initial".to_string()}),
            _ => keys(&mut windows[focus as usize], key as u8 as char)
        }

         
    }


    endwin();
}

// TODO: print method when going off screen on bottom and right wraps around the white space and
// the text but not the boarder, should be fixed by stoping the overwrap

// TODO: within boarder wrap, for multiline.
