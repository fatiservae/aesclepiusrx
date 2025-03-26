# AsclepiusRx
O prescritor padrão de [Asclepius](https://aesclepius.com.br)!

## Sobre
O projeto AesclepiusRx oferece uma forma de prescrição de medicações segura, eliminando a necssidade de cálculos por parte do médico.

O desenvolvimento se baseia na definição de tipos sólidos em Rust, segurança na conversão de unidades e na comparação de valores. Exemplo simples:
```rust
pub struct Idade {
    tipo: IdadeTipo,
    valor: i32,
}
enum IdadeTipo {
    Meses,
    Anos,
}
```
Exemplo mais complexo:
```rust
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum Volume {
    Ml(Float),
    L(Float),
    Dl(Float),
}
impl Volume {
    /// entrega o valor em `mL`
    fn valor(self) -> f32 {
        match self {
            Volume::Ml(valor) =>  valor.0,
            Volume::L(valor) | Volume::Dl(valor) => valor.0 * 1000.0,
            Volume::DL(valor) => valor.0 * 100.0,
        }
    }
    /// escreve em `&'static str` a unidade do volume
    fn unidade(self) -> &'static str {
        match self {
            Volume::Ml(_) => "mL",
            Volume::L(_) => "L",
            Volume::Dl(_) => "dL",
        }
    }
    /// Devolve o tipo `Volume` em `mL`, independente do seu tipo original
    pub fn normalizar(&self) -> Self {
        match self {
            Volume::Ml(valor) => Volume::Ml(Float(valor.0)),
            Volume::L(valor) => Volume::Ml(Float(valor.0 * 1000.0)),
            Volume::Dl(valor) => Volume::Ml(Float(valor.0 / 100.0)),
        }
    }
}
/// Permite uso idiomático do tipo
impl std::fmt::Display for Volume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Volume::Ml(valor) => write!(f, "{}mL", valor),
            Volume::L(valor) => write!(f, "{}L", valor),
            Volume::Dl(valor) => write!(f, "{}dL", valor),
        }
    }
}
```
Note-se a segurança de tipos, o uso de unidades internacionais e o caráter idiomático.

## Desenvolvimento e contribuição
Se você é desenvolvedor web ou Rust interessado no projeto, entre em contato por [Telegram](https://t.me/fatiservae).

Consulte `LICENCE.md` e `CONTRIBUTING.md` (em breve).

## Aviso legal
**Este software não substitui o julgamento médico sobre a administração, uso, suspensão, recomendação, aplicação, medida, comparação e dispensação de quaisquer drogas ou medicamentos.**
