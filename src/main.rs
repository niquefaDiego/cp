use {
    std::{cell::RefCell, fmt::Debug, io::{self}, str::{FromStr, SplitAsciiWhitespace}}
};

fn solve()
{
}

fn main() {
    let ntc: usize = read_line();
    for _ in 0..ntc { solve(); }
}

// --------------- Boilerplate code ---------------
// Input 
thread_local! {
    pub static LINE: RefCell<String> = RefCell::new(String::new());
    pub static TOKENS: RefCell<SplitAsciiWhitespace<'static>> = RefCell::new("".split_ascii_whitespace());
}

#[allow(dead_code)]
fn next<T: FromStr>() -> T where T::Err: Debug {
    TOKENS.with_borrow_mut(|tokens| {
        tokens.next().unwrap().parse().unwrap()
    })
}

#[allow(dead_code)]
fn read_to_buffer() {
    LINE.with_borrow_mut(|line| {
        line.clear();
        io::stdin().read_line(line)
            .expect("Failed to read line for tokenization");
        TOKENS.with_borrow_mut(|tokens| {
            *tokens = unsafe { std::mem::transmute::<_, SplitAsciiWhitespace<'static>>(line.split_ascii_whitespace()) };
        });
    });
}

#[allow(dead_code)]
fn read_line<T: FromStr>() -> T where T::Err: Debug {
    let mut line: String = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string().parse().unwrap()
}

#[allow(dead_code)]
fn read_vec<T: FromStr>() -> Vec<T> where T::Err: Debug {
    let mut line: String = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.split_ascii_whitespace()
        .map(|x| x.parse().unwrap()).collect()
}
