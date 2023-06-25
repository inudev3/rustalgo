//use std::io::Write;
use std::io;
use std::str;
use std::collections::HashSet;
pub struct UnsafeScanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitAsciiWhitespace<'static>,
}

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
fn main(){
    let mut scan = UnsafeScanner::new(io::stdin().lock());
    let mut out = io::BufWriter::new(io::stdout().lock());
    let mut dp = vec![vec![-1;101];101];
    let (n,m) = (scan.token::<usize>(),scan.token::<usize>());
    let mut bongs = vec![0;n];
    for i in 0..n{
        bongs[i] = scan.token::<i32>().abs();
    }
    let mut month:HashSet<_> = bongs.iter().cloned().collect();
    for i in 0..m-1{
        let mut new_month = HashSet::new();
        for &bong in &bongs{

            for &month_item in &month{
                new_month.insert(bong^month_item);
            }
        }
        month = new_month;
    }
    println!("{}", month.iter().max().unwrap());

    std::process::exit(0);
}
