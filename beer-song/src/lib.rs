#![allow(unused_variables)]

pub fn verse(n: i32) -> String {
    if n == 0 {
        return "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store \
                and buy some more, 99 bottles of beer on the wall.\n"
                   .to_string();
    } else if n == 1 {
        return "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, \
                no more bottles of beer on the wall.\n"
                   .to_string();
    } else {
        return format!("{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and \
                        pass it around, {1} bottle{2} of beer on the wall.\n",
                       n,
                       n - 1,
                       match n - 1 {
                           1 => "",
                           _ => "s",
                       });
    }
}

pub fn sing(m: i32, n: i32) -> String {
    let mut result = String::new();

    for (i, val) in (n..m + 1).rev().enumerate() {
        result = format!("{0}{1}{2}",
                         result,
                         match i {
                             0 => "",
                             _ => "\n",
                         },
                         verse(val));
    }

    result
}
