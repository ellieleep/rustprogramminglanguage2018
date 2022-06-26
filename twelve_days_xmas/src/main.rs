fn main() {

    let gifts = ["a partridge in a pear tree", "turtle doves",
        "french hens", "calling birds", "golden rings", "geese-a-laying", "swans-a-swimming",
        "maids-a-milking", "ladies dancing", "lords-a-leaping", "pipers piping", "drummers drumming"];

    for day in 1..13 {
        println!("On the {} day of xmas I was given {} {}", day, day, gifts[day-1]);
    }
}