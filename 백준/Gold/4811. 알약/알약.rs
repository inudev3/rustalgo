use std::io::Write;
use std::io;
pub struct UnsafeScanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitAsciiWhitespace<'static>,
}
use std::str;
impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    pub fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }
}
fn solve(w:i32,h:i32, memo:&mut Vec<Vec<i64>>)->i64{
    if w<0 || h<0{
        return 0
    }
    if w==0 && h==0{
        return 1
    }

    let (w,h) = (w as usize,h as usize);
    if memo[w][h]!=-1{
        return memo[w][h]
    }
    let (w,h) = (w as i32,h as i32);
    let res =  solve(w-1, h+1,memo)+solve(w,h-1,memo);
    memo[w as  usize ][h as usize] = res;
    res
}
fn main(){
    let (stdin, stdout) = (io::stdin(),io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    let mut dp = vec![vec![-1;31];31];

    loop{
        let n=scan.token::<i32>();
        if n==0{
            break;
        }
       println!("{}",solve(n,0,&mut dp));
    }
    std::process::exit(0);
}