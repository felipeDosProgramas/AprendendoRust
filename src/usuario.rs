pub struct UsuarioLegal {
    id: u32,
    pub nome: String,
    pub e_adm: bool
}
fn usuario_e_adm(nome: &str) -> bool{
    let administradores: Vec<&str> = vec!["sergio","maria","jose"];
    let mut e_adm: bool = false;
    for adm in administradores{
        if adm == nome{
            e_adm = true;
        }
    }
    e_adm
}/*
fn criar_usuario (id: u32, nome: &str) -> UsuarioLegal {
    UsuarioLegal{
        id,
        nome: String::from(nome),
        e_adm: usuario_e_adm(nome)
    }
}*/

impl UsuarioLegal {
    fn new (id: u32, nome: &str) -> UsuarioLegal {
        UsuarioLegal{
            id,
            nome: String::from(nome),
            e_adm: usuario_e_adm(nome)
        }
    }
    fn printa_usuario (&self) -> &UsuarioLegal {
        println!("Hello, {}. Você é o número {} e {}",
            self.nome, self.id,  if self.e_adm { "felizmente é adm" } else { "infelizmente não é adm" });
        self
    }
    fn altera_nome (&self, novo_nome: &str) -> Self {
        Self {
            id: self.id,
            nome: novo_nome.to_string(),
            e_adm: usuario_e_adm(novo_nome)
        }
    }
    fn salvar_usuario (&self , lista_usuario: &mut Vec<UsuarioLegal>) {
        lista_usuario.push(Self{
            id: self.id,
            nome: self.nome.clone(),
            e_adm: self.e_adm
        })
    }

}
pub fn criar_todos_usuarios (mut indice: u32, nomes: Vec<&str>, mut usuarios: &mut Vec<UsuarioLegal>){
    for nome in nomes {
        UsuarioLegal::new(indice, nome)
            .printa_usuario()
            .salvar_usuario(&mut usuarios);
        indice += 1;
    }
}