/// Estrutura Ponto
/// x: coordenada x
/// y: coordenada y
#[allow(dead_code)]
struct Ponto {
    x: i64,
    y: i64,
}

#[allow(dead_code)]
struct Retangulo {
    piso_esquerda: Ponto,
    teto_direita: Ponto,
}

#[allow(dead_code)]
impl Retangulo {
    fn area(self) -> i64 {
        let dx = self.piso_esquerda.x - self.teto_direita.x;
        let dy = self.piso_esquerda.y - self.teto_direita.y;

        return dx*dy;
    }
}

fn main() {
    println!("struct Retangulo: teste implementação.");
}

#[cfg(test)]
mod tests {
    use crate::{Ponto, Retangulo};

    #[test]
    fn teste_area() {
        let pe = Ponto{x: 1, y: 2};
        let td = Ponto{x: 3, y: 5};
        let ret = Retangulo {
                    piso_esquerda: pe,
                    teto_direita: td,    
                };
        assert_eq!(ret.area(), 6);
    }
}
