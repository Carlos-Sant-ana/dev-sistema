/// Estrutura Ponto
/// x: coordenada x
/// y: coordenada y
pub struct Ponto {
    x: i64,
    y: i64,
}

/// Cria a estrutura ponto.
impl Ponto {
    pub fn new(x: i64, y: i64) -> Self {
        Ponto {
            x,
            y,
        }
    }
}

fn main() {

    //    println!("{:?}", tipo);
}

#[cfg(test)]
mod tests {
    //    use self::Ponto;
    #[test]
    fn eita_funciona() {
        let p = Ponto.new(2, 3);

        assert_eq!(p.x, 2);
    }
}
