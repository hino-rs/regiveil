fn main() {
    let x = regiveil::now("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced", "Hidden");
    println!("{:?}", x);
}