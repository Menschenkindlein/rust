struct S;

fn main() {
    let b = [0; S];
    //~^ ERROR mismatched types
    //~| expected usize, found struct `S`
    //~| expected type `usize`
    //~| found struct `S`
}
