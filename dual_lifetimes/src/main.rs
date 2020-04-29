enum Answer<'a, 'b> {
    Small(&'a isize),
    Big(&'b isize),
}

fn lesser<'a, 'b>(x: &'a isize, y: &'b isize) -> Answer<'a, 'b> {
    if *x < *y {
        Answer::Small(x)
    } else {
        Answer::Big(y)
    }
}

fn main() {
    let nop = 0 as isize;
    let a = 5 as isize;
    let mut aa: &isize = &nop;

    {
        let b = 7 as isize;

        let s = lesser(&a, &b);

        match s {
            Answer::Small(m) => aa = m,
            Answer::Big(m) => println!("B was smaller {}", *m),
        }
    }
    
    println!("A was smaller {}", *aa);
}
