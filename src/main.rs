fn gave_or_send(index: usize) -> String {
    if index == 11 {
        String::from("gave")
    } else {
        String::from("sent")
    }
}

fn main() {
    let presents = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five golden rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    let days = [
        String::from("first"),
        String::from("second"),
        String::from("third"),
        String::from("fourth"),
        String::from("fifth"),
        String::from("sixth"),
        String::from("seventh"),
        String::from("eight"),
        String::from("ninth"),
        String::from("tenth"),
        String::from("eleventh"),
        String::from("twelfth"),
    ];

    for i in 0..12 {
        let mut giving_presents = String::new();
        for n in (0..(i + 1)).rev() {
            giving_presents.push_str(&presents[n]);
            giving_presents.push('\n');
        }
        let lyric = format!(
            "On the {} day of Christmas, \nmy true love {} to me, \n{}",
            days[i],
            gave_or_send(i),
            giving_presents
        );

        print!("{}\n", lyric);
    }

    // プレゼントの順番が逆であること、一日目にプレゼントがないことがおかしいので、修正する
    // println!("{:?}", &days[0..1]);
}

// 1. On the first day of Christmas,
// my true love sent to me
// A partridge in a pear tree.

// 2. On the second day of Christmas,
// my true love sent to me
// Two turtle doves
// and a partridge in a pear tree.

// 3. On the third day of Christmas,
// my true love sent to me
// Three French hens,
// two turtle doves
// And a partridge in a pear tree.

// 4~12番まで新しいプレゼントを追加し、歌う

// 12. On the twelfth day of Christmas,
// my true love gave to me
// Twelve drummers drumming,
// eleven pipers piping,
// ten lords a-leaping,
// nine ladies dancing,
// eight maids a-milking,
// seven swans a-swimming,
// six geese a-laying,
// Five golden rings.
// Four calling birds,
// three French hens,
// two turtle doves
// And a partridge in a pear tree.
