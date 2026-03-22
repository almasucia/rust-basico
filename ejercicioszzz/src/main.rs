use std::io;
fn main() {
    println!("     **ejercios zzz**");
    let n:&str  = "lunes";
    let i = 6;
    let mut name = String::new();
    println!("{} es un dia laboral?", n);
    println!(" ->{}",dialaboral(n));
    println!("digito {} de fibonacci es :{}",i,fibonacci(i));
    println!("escriba su nombre:");
    io::stdin()
       .read_line(&mut name)
       .expect("error!! que mrd escribis???");
    println!("hola {}!", name.trim());
}
fn dialaboral(n:&str)-> bool {
    let mut res = false;
    if n == "lunes" || n == "martes" || n == "miercoles" || n == "jueves" || n == "viernes" 
    {
        res = true;
    }
    return res; 
}
fn fibonacci(i:i32)-> i32{
    let mut a = 0;
    let mut b = 1;
    let mut c = 0; 
    for _ in 1..i {
        c = a + b;
        a = b;
        b = c;
        
    }
    return c;  

}    