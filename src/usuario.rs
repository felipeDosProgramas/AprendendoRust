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

impl UsuarioLegal {
    fn printa_usuario (&self) -> &UsuarioLegal {
        println!("Hello, {}. Você é o número {}",
            self.nome, self.id);
        self
    }
    fn altera_nome (&self, novo_nome: &str) -> Self {
        Self {
            id: self.id,
            nome: novo_nome.to_string()
        }
    }
    fn salvar_usuario (&self , lista_usuario: &mut Vec<UsuarioLegal>) {
        lista_usuario.push(Self{
            id: self.id,
            nome: self.nome.clone()
        })
    }

}
pub fn criar_todos_usuarios (mut indice: u32, nomes: Vec<&str>, mut usuarios: &mut Vec<UsuarioLegal>){
    for nome in nomes {
        let usuario: UsuarioLegal = criar_usuario(indice, nome);
        usuario
            .printa_usuario()
            .altera_nome("maria")
            .altera_nome("sergio")
            .printa_usuario()
            .salvar_usuario(&mut usuarios);
        indice += 1;
    }
}