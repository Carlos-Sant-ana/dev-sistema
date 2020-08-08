
pub struct Tipo {
  var1: TipoVar1,
  var2: TipoVar2,
  ...
  varn: TipoVarN
}

impl Tipo {
  pub fn new(valor1: TipoVar1, valor2, TipoVar2, ..., valorn: TipoVarN) -> Self {
    Tipo {
      var1: valor1,
      var2: valor2,
      ...
      varn: valorn,
    }
  }
}

fn main() {
    let tipo = Tipo::new(valor1, valor2, ..., valorn);
    println!("{:?}", tipo);
}

#[cfg(test)]
mod tests {
    #[test]
    fn eita_funciona() {
      // Implementar um teste usando o tipo Tipo
        assert_eq!(2 + 2, 4);
    }
}