use std::io::{self, Write};
fn main() {
    nuevo_num();
    let num_inicial: u64 = loop { 
        match leer_numero() {
            Some(num) => break num,
            None    => println!("Número inválido. Intenta de nuevo:"),
        }
    };  
    let mut n =  Numero::new(num_inicial);

    loop { 
        mostrar_menu(&n);
        let opcion = leer_linea();
                match opcion.as_str() {  
            "1" => println!("  ¿Es par?          → {}", n.es_par()),
            "2" => println!("  ¿Es primo?        → {}", n.es_primo()),
            "3" => println!("  Cantidad Digitos: → {}", n.cantidad_digitos()),
            "4" => println!("  Invertir:         → {}", n.invertir()),
            "5" => println!("  ¿Es capicúa?      → {}", n.es_capicua()),
            "6" => println!("  ¿Es Armstrong?    → {}", n.es_armstrong()),
            "0" => {
                println!("  Ingresa el nuevo número:");
                match leer_numero() {
                    Some(nume) => { n.resetear(nume); println!("  ✓ Nuevo número: {}", n.num); }
                    None    => println!("  Número inválido, se mantiene {}", n.num),
                }
            }
            "q" | "Q" => { println!("\n  Hasta luego.\n"); break; } 
            _ => println!("  Opción no válida."),
        }
    }
}
struct Numero{
    num: u64
} 

impl Numero {
    fn new(num:u64) -> Self {
        Numero {num}
    }
    fn es_par(&self) -> bool {
        self.num % 2 == 0         
    }
    fn es_primo(&self) -> bool{
        let mut cont:u64 = 0;
        let aux:u64 = self.num;
        for mut i  in 1..aux {
            if aux%i==0 {
                cont= cont+1;                
            }
            if cont==2 {
                i=cont; 
            }
        }
        cont==2
    }
    fn cantidad_digitos(&self) -> u16 {
        let mut cont:u16 = 0;
        let mut aux = self.num;
        while aux > 0 {
            cont=cont+1;
            aux= aux/10;            
        }
        cont
    }
    fn invertir(&self) -> u64 {
        let mut num: u64 = self.num;
        let mut invertido: u64 = 0;

        while num > 0 {
            let digito: u64 = num % 10;
            invertido = invertido * 10 + digito;
            num = num/10;
        }

        invertido
    }
    fn es_capicua(&self) -> bool {
        self.num == self.invertir()
    }
    fn elevado(&self, base: u64, exp: u64) -> u64 {
        let mut conta = 1;
        for _ in 0..exp {
            conta *= base;
        }
        conta
    }
    fn es_armstrong(&self) -> bool {
        let mut num = self.num;
        let digitos = self.cantidad_digitos();
        let mut suma = 0;

        while num > 0 {
            let digito = num % 10;
            suma += self.elevado(digito, digitos as u64);
            num /= 10;
        }
        suma  == self.num
    }    
    fn resetear(&mut self, nuevo: u64) {
        self.num = nuevo;
    } 
}
fn leer_linea() -> String {
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Error al leer");
    entrada.trim().to_string()
}
fn leer_numero() -> Option<u64> {
    leer_linea().parse::<u64>().ok()
}
fn mostrar_menu(n: &Numero){
    println!("╔═════════▣◎▣═════════╗");
    println!("┃ NUMERO ACTUAL:      ┃");
    println!("╠═════════▣◎▣═════════╣");
    println!("┃                     ┃");
    println!("┃                     ┃");
    println!("┃                     ┃");
    println!("┃                     ┃");
    println!("╚═════════▣◎▣═════════╝");
}
fn nuevo_num() {
    println!("╔════════▣◎▣════════╗");
    println!("┃ *DIGITE UN NUMERO ┃");
    println!("╚════════▣◎▣════════╝");
} 
