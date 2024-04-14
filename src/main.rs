mod usuario;

use usuario::{UsuarioLegal, criar_todos_usuarios};

fn main() {
    let nomes: Vec<&str> = vec!["sergio", "joão","maria","josé","onça"];
    let mut usuarios: Vec<UsuarioLegal> = Vec::new();
    criar_todos_usuarios(
        0,
        nomes,
        &mut usuarios
    );
    if let Some(usuario) = usuarios.get(10){
        println!("{}", usuario.nome);
    } else {
        println!("nem achamo");
    }
}
