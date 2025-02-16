// Função para inverter uma String

fn reverse(palavra: &str) -> String { // Essa linha está declarando que o tipo da função vai receber o dado do tipo String
    palavra.chars().rev().collect() // Essa linha está invertendo o dado recebido pela função "palavra"
}


fn main(){
    let palavra = "Giovanna Andrade"; // Essa linha define a palavra que será invertida (no caso a frase) 
    let palavra_invertida = reverse(palavra); // Aqui chamamos a função "palavra" e armazenamos na variavel "palavra_invertida"

    println!("Palavra(as) Invertida(as): {}", palavra_invertida);
}