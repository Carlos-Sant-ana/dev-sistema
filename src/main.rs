/// Representa a figura geométrica ponto onde
/// x: coordenada x
/// y: coordenada y
#[allow(dead_code)]
struct Ponto {
    x: i64,
    y: i64,
}

/// Representa a figura geométrica retângulo onde
/// piso_esquerda: é o ponto mais a esquerda e próximo do piso,
/// teto_direita: é o ponto mais a direito e próximo do teto (topo).
#[allow(dead_code)]
struct Retangulo {
    piso_esquerda: Ponto,
    teto_direita: Ponto,
}

// Implementações para o Retangulo.
// FIXME: insert assert para as coordenadas.
#[allow(dead_code)]
impl Retangulo {
    /// Calcula a área do retangulo pela multiplicação 
    /// das diferenças entre as coordenadas x e y.
    /// Retorna um inteiro como resultado do cálculo da área.
    fn area(self) -> i64 {
        let dx = self.teto_direita.x - self.piso_esquerda.x;
        let dy = self.teto_direita.y - self.piso_esquerda.y;

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
    fn teste_retangulo_area() {
        let pe = Ponto{x: 1, y: 2};
        let td = Ponto{x: 3, y: 5};
        let ret = Retangulo {
                    piso_esquerda: pe,
                    teto_direita: td,    
                };
        let area = ret.area();
        assert_eq!(area, 6);
    }
}
