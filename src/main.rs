mod usuario;

use usuario::{salvar_usuario, UsuarioLegal, criar_usuario, printa_usuario};

fn main() {
    let nomes: Vec<&str> = vec!["sergio", "joão","maria","josé","onça"];
    let mut usuarios: Vec<UsuarioLegal> = Vec::new();
    let mut indice: u32 = 0;
    for nome in nomes {
        let usuario: UsuarioLegal = criar_usuario(indice, nome);
        printa_usuario(&usuario);
        salvar_usuario(usuario, &mut usuarios);
        indice += 1;
    }
}
