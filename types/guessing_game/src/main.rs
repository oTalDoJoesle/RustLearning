use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let num_secreto = rand::thread_rng().gen_range(1..101);
    println!("O número secreto é: {num_secreto}");
    loop {
        let mut escolha = String::new();
        println!("Escolha um número");
        io::stdin()
         .read_line(&mut escolha)
         .expect("Input inválido");
        
        let escolha: u32 = escolha.trim().parse().expect("Erro: Nenhum número detectado");

        println!("Número escolhido: {escolha}\n");
        match escolha.cmp(&num_secreto) {
            Ordering::Less => println!("Número secreto é maior"),
            Ordering::Greater => println!("Número secreto é menor"),
            Ordering::Equal => {
                println!("Número secreto correto!");
                break;
            }
        }
    }
}
