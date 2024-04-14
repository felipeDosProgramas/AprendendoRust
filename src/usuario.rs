pub struct UsuarioLegal {
    id: u32,
    nome: String
}
pub fn criar_usuario (id: u32, nome: &str) -> UsuarioLegal {
    UsuarioLegal{
        id,
        nome: String::from(nome)
    }
}
pub fn printa_usuario (usuario_legal: &UsuarioLegal) {
    println!("Hello, {}. Você é o número {}",
             usuario_legal.nome, usuario_legal.id);
}
pub fn salvar_usuario (usuario_legal: UsuarioLegal, lista_usuario: &mut Vec<UsuarioLegal>) {
    lista_usuario.push(usuario_legal)
}