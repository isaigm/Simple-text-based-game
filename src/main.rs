use ncurses::*;
use rand::distributions::{Distribution, Uniform};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const SIZE: usize= 23;
#[derive(Default, Copy, Clone)]
struct Character{
    x: i32,
    y: i32,
    ch: u32,
    color: i16,
    msg: usize,
}
impl Character{
    fn new(x: i32, y: i32, ch: u32, color: i16, msg: usize) -> Self{
        Self {x, y, ch, color, msg}
    }
}
fn to_u32(ch : char) -> u32{
    ch as u32
}
fn is_colliding_with(win : &WINDOW, x: i32, y: i32) -> bool{
    mvwinch(*win, y, x) != to_u32(' ')
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
fn collide(x: i32, y: i32, population: &[Character; SIZE]) -> Option<(usize, usize)>{
    for i in 0..SIZE{
        if population[i].x == x && population[i].y == y{
            return Some((population[i].msg, i));
        }
    }
    None
}
fn main() {
    let mut messages : Vec<String> = vec!();
    if let Ok(lines) = read_lines("../../vanilla.nki") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                messages.push(ip);
            }
        }
    }
    let mut player = Character::new(2, 2, to_u32('@'), 0, 0);
    let mut population: [Character; 23] = [Default::default(); SIZE];
    let chars: [u32; SIZE] = [to_u32('3'), to_u32('4'), to_u32('7'), to_u32('8'), to_u32('V'), to_u32('H'), to_u32('N'), to_u32('Y'), to_u32('S'), to_u32('k'), to_u32('x'), to_u32('e'), to_u32('q'), to_u32('m'),to_u32('o'),to_u32('u'), to_u32('*'), to_u32('.'), to_u32(';'), to_u32('-'), to_u32('/'), to_u32('!'), to_u32(']')];
    initscr();
    noecho();   
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    start_color();
    init_pair(0, COLOR_WHITE, COLOR_BLACK);
    init_pair(1, COLOR_BLUE, COLOR_BLACK);
    init_pair(2, COLOR_RED, COLOR_BLACK);
    init_pair(3, COLOR_YELLOW, COLOR_BLACK);
    init_pair(4, COLOR_GREEN, COLOR_BLACK);
    init_pair(5, COLOR_CYAN, COLOR_BLACK);
    init_pair(6, COLOR_MAGENTA, COLOR_BLACK);
    let mut rows: i32 = 0;
    let mut cols: i32 = 0;
    let mut rng = rand::thread_rng();
    getmaxyx(stdscr(), &mut rows, &mut cols);
    let win : WINDOW = newwin(rows - 3, cols, 3, 0);
    let message : WINDOW = newwin(3, cols, 0, 0);
    let kitten = Uniform::from(0..SIZE).sample(&mut rng);
    keypad(win, true);
    for i in 0..SIZE{
        let n1 = Uniform::from(1..cols-1);
        let n2 = Uniform::from(1..rows-4);
        let mut x: i32 = n1.sample(&mut rng);
        let mut y: i32 = n2.sample(&mut rng);
        while mvwinch(win, y, x) != to_u32(' '){
            x = n1.sample(&mut rng);
            y = n2.sample(&mut rng) ;

        }
        let color = Uniform::from(1..7);
        let msg = Uniform::from(0..messages.len());
        population[i] = Character::new(x, y, chars[i], color.sample(&mut rng), msg.sample(&mut rng));
    }
    let mut curr_msg : String = String::from("The most bored game");
    mvwaddstr(message, 1, 0, curr_msg.as_str());
    wrefresh(message);
    loop{
        wclear(win);
        wclear(message);
        wborder(win, to_u32('|') , to_u32('|'), to_u32('-'), to_u32('-'), to_u32('+'), to_u32('+'), to_u32('+'), to_u32('+'));
        for i in 0..SIZE{
            wattr_on(win, COLOR_PAIR(population[i].color));
            mvwaddch(win, population[i].y, population[i].x, population[i].ch);
            wattr_off(win, COLOR_PAIR(population[i].color));
        }
        wattr_on(win, COLOR_PAIR(player.color));
        mvwaddch(win, player.y, player.x, player.ch);
        wattr_off(win, COLOR_PAIR(player.color));
        let ch = wgetch(win);
        match ch{
            27 => break,
            KEY_LEFT => {
                if is_colliding_with(&win, player.x - 1, player.y){
                    match collide(player.x - 1, player.y, &population){
                        Some((msg, i)) => {
                            if i == kitten{
                                curr_msg = String::from("You win");
                            }else{
                                curr_msg = messages[msg].clone()
                            }
                        },
                        None => ()
                    }
                }else{
                    player.x -= 1;
                }
            },
            KEY_RIGHT => {
                if is_colliding_with(&win, player.x + 1, player.y){
                    match collide(player.x + 1, player.y, &population){
                        Some((msg, i)) => {
                            if i == kitten{
                                curr_msg = String::from("You win");
                            }else{
                                curr_msg = messages[msg].clone()
                            }
                        },
                        None => ()
                    }
                }else{
                    player.x += 1;
                }
            },
            KEY_UP => {
                if is_colliding_with(&win, player.x, player.y - 1){
                    match collide(player.x, player.y - 1, &population){
                        Some((msg, i)) => {
                            if i == kitten{
                                curr_msg = String::from("You win");
                            }else{
                                curr_msg = messages[msg].clone()
                            }
                        },
                        None => ()
                    }
                }else{
                    player.y -= 1;
                }
            },
            KEY_DOWN => {
                if is_colliding_with(&win, player.x, player.y + 1){
                    match collide(player.x, player.y + 1, &population){
                        Some((msg, i)) => {
                            if i == kitten{
                                curr_msg = String::from("You win");
                            }else{
                                curr_msg = messages[msg].clone()
                            }
                        },
                        None => ()
                    }
                }else{
                    player.y += 1;
                }
            },
            _ => () 
        }
        mvwaddstr(message, 1, 0, curr_msg.as_str());
        wrefresh(win);
        wrefresh(message);
    }
    delwin(win);
    delwin(message);
    endwin();
}
