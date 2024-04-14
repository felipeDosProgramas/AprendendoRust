pub struct UsuarioLegal {
    id: u32,
    pub nome: String
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

pub fn criar_todos_usuarios (mut indice: u32, nomes: Vec<&str>, mut usuarios: &mut Vec<UsuarioLegal>){
    for nome in nomes {
        let usuario: UsuarioLegal = criar_usuario(indice, nome);
        printa_usuario(&usuario);
        salvar_usuario(usuario, &mut usuarios);
        indice += 1;
    }
}