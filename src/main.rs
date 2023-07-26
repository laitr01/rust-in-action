use regex::Regex;
use clap::{App, Arg}; //Brings clap::App and clap::Arg objects into local scope

fn main() {
    array();
    vector();
    regex();
}

fn array() {
    let search_term = "picture";
    let quote ="\
    Every face, every shop, bedroom window, public-house, and
    dark square is a picture feverishly turned--in search of what?
    It is the same with books. What do we seek through millions of pages?";
    // mutable
    let mut line_num: usize = 1;

    for line in quote.lines() {
        if line.contains(search_term) {
            println!("{}: {}", line_num, line);
        }
        line_num+=1;
    }
    //enumerate()
    for(i, line) in quote.lines().enumerate() {
        if line.contains(search_term) {
            println!("{}: {}", i +1, line);
        }
    }

    // Making lists of things with arrays, slices, and vectors
    //array : fixed-width/ lightweight

    let one = [1,2,3];
    let two: [u8;3] = [1,2,3];
    let blank1 = [0;3];
    let blank2: [u8;3] = [0;3];

    let arrays = [one, two, blank1, blank2];

    for a in &arrays {
        print!("{:?}: ", a);
        for n in a.iter() {
            print!("\t{} + 10 = {}", n, n+10)
        }
        let mut sum = 0;
        for i in 0..a.len() {
            sum+=a[i];
        }
        println!("\t((Z{:?} = {})",a,sum)
    }
    //slices
    //Slices are dynamically sized array-like objects. The term dynamically sized means that
    //their size is not known at compile time.
    //Another important use for slices is their ability to act as a view on arrays (and other
    //slices). The term view here is taken from database technology and means that slices
    //can gain fast, read-only access to data without needing to copy anything around

}

fn vector() {
    //vector: growable, small runtime penalty
    //Vec<T> performs best when you can provide it with a size hint via Vec::with_
    //capacity(). Providing an estimate minimizes the number of times memory will need
    //to be allocated from the OS.
    let ctx_lines = 2;
    let needle = "oo";
    let haystack = "\
     Every face, every shop,
     bedroom window, public-house, and
     dark square is a picture
     feverishly turned--in search of what?
     It is the same with books.
     What do we seek
     through millions of pages?";

    let mut tags: Vec<usize> = vec![];
    let mut ctx: Vec<Vec<(usize, String)>> = vec![];

    for (i, line) in haystack.lines().enumerate() {
        if line.contains(needle) {
            tags.push(i);
            let v = Vec::with_capacity(2*ctx_lines+1);
            ctx.push(v);
        }
    }
    if tags.is_empty() {
        return;
    }
    for (i, line) in haystack.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(ctx_lines);
            let upper_bound = tag + ctx_lines;
            if (i >= lower_bound) && (i <= upper_bound) {
                let line_as_string = String::from(line);
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx.iter() {
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }
}

fn regex() {
    let args = App::new("grep-lite")
     .version("0.1")
     .about("searches for patterns")
     .arg(Arg::with_name("pattern")
     .help("The pattern to search for")
     .takes_value(true)
     .required(true))
     .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap(); //unwrap() unwraps a Result, crashing if an error occurs.
    let quote = "Every face, every shop, bedroom window, public-house, and
    dark square is a picture feverishly turned--in search of what?
    It is the same with books. What do we seek through millions of pages?";
    for line in quote.lines() {
        let contains_substring = re.find(line);

        match contains_substring {
            Some(_) => println!("{}", line), //Some(T) is the positive case of an Option, meaning that re.find() was successful: it matches all values.
            None => (), //None is the negative case of an Option; () can be thought of as a null placeholder value here.
        }
    }
}