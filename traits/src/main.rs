
mod dynamic;
mod derive;
mod iter;

fn main() {
    dynamic::dynamic_box();
    derive::deriveable();
    iter::call_iter();
}
