mod Usuario;

struct UsuarioLegal {
    id: u32,
    nome: String
}
fn criar_usuario (id: u32, nome: &str) -> UsuarioLegal {
    UsuarioLegal{
        id,
        nome: String::from(nome)
    }
}
fn printa_usuario (usuario_legal: &UsuarioLegal) {
    println!("Hello, {}. Você é o número {}",
         usuario_legal.nome, usuario_legal.id);
}
fn salvar_usuario (usuario_legal: UsuarioLegal, lista_usuario: &mut Vec<UsuarioLegal>) {
    lista_usuario.push(usuario_legal)
}
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
