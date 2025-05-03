
fn main() {
    let mut f: [u128; 181] = [0;181];
    f[1]=1;
    for i in 2..=180{
        f[i]=f[i-1]+f[i-2]
    }
    println!("{:?}",f);
}
