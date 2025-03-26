//use serde::{Deserialize, Serialize};
#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod calc;
mod data;
pub use {
    data::BULARIO,
    std::ops::{Add, Div, Mul, Sub},
};

//#[derive(Serialize, Deserialize)]
pub struct Instancia {
    // Example stuff:
    search: String,
    options: Vec<&'static str>,
    visible: bool,
    nome: String,
    massa: Massa,
    idade: Idade,
    medicamento_selecionado: Medicamento,
    apresentacao_selecionada: Apresentacao, //#[serde(skip)] // This how you opt-out of serialization of a field
    posologia_selecionada: Posologia,
    prescricao: String,
}

impl Default for Instancia {
    fn default() -> Self {
        Self {
            // Example stuff:
            search: String::new(),
            options: vec!["Pequi", "maça", "outra"],
            visible: true,
            nome: "Algum medicamento".to_owned(),
            massa: Massa::Kg(Float(0.0)),
            idade: Idade {
                tipo: IdadeTipo::Meses,
                valor: 2,
            },
            apresentacao_selecionada: Apresentacao::DoseVolume(
                Massa::Mg(Float(0.0)),
                Volume::Ml(Float(0.0)),
                TipoApresentacao::Ampola,
                NomesComerciais(&["Marca do medicamento"]),
            ),
            medicamento_selecionado: Medicamento {
                nome: "medicamento",
                apresentacoes: &[Apresentacao::DoseVolume(
                    Massa::Mg(Float(0.0)),
                    Volume::Ml(Float(0.0)),
                    TipoApresentacao::Ampola,
                    NomesComerciais(&["Marca do medicamento"]),
                )],
                posologias: &[Posologia::DoseUnica(Massa::Mg(Float(0.0)), Via::Oral)],
                advertencias: None,
            },
            posologia_selecionada: Posologia::DoseUnica(Massa::Mg(Float(0.0)), Via::Oral),
            prescricao: "Medicamento X a 0g/ml ..... 0ml".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Idade {
    tipo: IdadeTipo,
    valor: i32,
}
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum IdadeTipo {
    Meses,
    Anos,
}
impl std::fmt::Display for IdadeTipo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IdadeTipo::Meses => write!(f, "meses"),
            IdadeTipo::Anos => write!(f, "anos"),
        }
    }
}
impl Idade {
    fn normalizar(&self) -> Self {
        match self.tipo {
            IdadeTipo::Meses => Idade {
                tipo: IdadeTipo::Meses,
                valor: self.valor,
            },
            IdadeTipo::Anos => Idade {
                tipo: IdadeTipo::Meses,
                valor: self.valor * 12,
            },
        }
    }
    fn meses(&self) -> i32 {
        self.normalizar().valor
    }
    fn anos(&self) -> i32 {
        match self.tipo {
            IdadeTipo::Meses => self.valor / 12,
            IdadeTipo::Anos => self.valor,
        }
    }
}
impl std::fmt::Display for Idade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.valor, self.tipo)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Float(pub f32);
impl std::fmt::Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
const TOLERANCE: f32 = 1e-6;
/// O tipo fundamental `f32` foi insuficiente para lidar com as diferentes
/// formas de cálculo. Por isso, `PartialEq` e `Eq` precisaram ser derivados
/// para Float(), uma vez que `egui` necessita fazer comparações quando
/// chama `update()`.
impl std::cmp::PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < TOLERANCE
    }
    fn ne(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() >= TOLERANCE
    }
}
// Implementação de Eq (vazia, pois depende de PartialEq)
impl std::cmp::Eq for Float {}

#[derive(/*Serialize, Deserialize,*/ Debug, PartialEq, Eq, Clone)]
pub struct Medicamento {
    pub nome: &'static str,
    pub apresentacoes: &'static [Apresentacao],
    pub posologias: &'static [Posologia],
    pub advertencias: Option<&'static [&'static str]>,
}
impl std::fmt::Display for Medicamento {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.nome.to_string())
    }
}

#[derive(/*Serialize, Deserialize,*/ Debug, PartialEq, Eq, Clone)]
pub enum Apresentacao {
    DoseVolume(Massa, Volume, TipoApresentacao, NomesComerciais), // (dose, volume, unidade)
    DoseCompostaVolume(&'static [Massa], Volume, TipoApresentacao, NomesComerciais),
    DoseAplicacao(Aplicacao, TipoApresentacao, NomesComerciais),
}
impl std::fmt::Display for Apresentacao {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Apresentacao::DoseVolume(massa, volume, tipo, nomes_comerciais) => {
                write!(
                    f,
                    "{} com {} a cada {}",
                    tipo.capitalizar(TipoCapitalizacao::Primeira),
                    massa,
                    volume
                )
            }
            Apresentacao::DoseCompostaVolume(quantidades, volume, tipo, nomes_comerciais) => {
                let mut dose_apres: String = String::new();
                for (index, quantidade) in quantidades.into_iter().enumerate() {
                    if index > 0 {
                        dose_apres.push_str(&format!("+{}", quantidade));
                    } else {
                        dose_apres = format!("{}", quantidade)
                    }
                }
                write!(
                    f,
                    "{} com {} a cada {}",
                    tipo.capitalizar(TipoCapitalizacao::Primeira),
                    dose_apres,
                    volume
                )
            }
            Apresentacao::DoseAplicacao(aplicacao, tipo, nomes_comerciais) => match aplicacao {
                Aplicacao::Comprimido(valor) => {
                    write!(
                        f,
                        "{} com {} a cada comprimido",
                        tipo.capitalizar(TipoCapitalizacao::Primeira),
                        valor
                    )
                }
                Aplicacao::Jato(valor) => write!(
                    f,
                    "{} com {} a cada jato",
                    tipo.capitalizar(TipoCapitalizacao::Primeira),
                    valor
                ),
                Aplicacao::Gota(valor) => write!(
                    f,
                    "{} com {} a cada gota",
                    tipo.capitalizar(TipoCapitalizacao::Primeira),
                    valor
                ),
                Aplicacao::Microgota(valor) => write!(
                    f,
                    "{} com {} a cada microgota",
                    tipo.capitalizar(TipoCapitalizacao::Primeira),
                    valor
                ),
            },
        }
    }
}

#[derive(/*Serialize, Deserialize,*/ Debug, PartialEq, Eq, Clone)]
pub struct NomesComerciais(&'static [&'static str]);

impl std::fmt::Display for NomesComerciais {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0.iter().fold(String::new(), |acc, &s| acc + s)
        )
    }
}

#[derive(/*Serialize, Deserialize,*/ Debug, PartialEq, Eq, Clone)]
pub enum TipoApresentacao {
    Ampola,
    Frasco,
    Comprimido,
    Spray,
    Capsula,
    Draguea,
    Bisnaga,
    Gel,
    PoReconstituivel,
    Pastilha,
    ComprimidoMastigavel,
    ComprimidoSublingual,
    Patch,
    Shampoo,
    Pomada,
    Gas,
    Solucao,
}

impl Capitalizar for TipoApresentacao {
    fn capitalizar(&self, tipo: TipoCapitalizacao) -> String {
        match tipo {
            TipoCapitalizacao::Primeira => {
                let mut chars = format!("{}", self);
                match chars.chars().next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str()[1..],
                }
            }
            TipoCapitalizacao::Todas => todo!(),
        }
    }
}

impl std::fmt::Display for TipoApresentacao {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TipoApresentacao::Ampola => write!(f, "ampolas"),
            TipoApresentacao::Frasco => write!(f, "frascos"),
            TipoApresentacao::Comprimido => write!(f, "comprimidos"),
            TipoApresentacao::Draguea => write!(f, "drágueas"),
            TipoApresentacao::Spray => write!(f, "spray"),
            _ => todo!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum Volume {
    Ml(Float),
    L(Float),
    Dl(Float),
}

impl Volume {
    fn valor(self) -> f32 {
        match self {
            Volume::Ml(valor) | Volume::L(valor) | Volume::Dl(valor) => valor.0,
        }
    }
    fn tipo(self) -> &'static str {
        match self {
            Volume::Ml(_) => "mL",
            Volume::L(_) => "L",
            Volume::Dl(_) => "dL",
        }
    }
    pub fn normalizar(&self) -> Self {
        match self {
            Volume::Ml(valor) => Volume::Ml(Float(valor.0)),
            Volume::L(valor) => Volume::Ml(Float(valor.0 * 1000.0)),
            Volume::Dl(valor) => Volume::Ml(Float(valor.0 / 100.0)),
        }
    }
}

impl std::fmt::Display for Volume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Volume::Ml(valor) => write!(f, "{}mL", valor),
            Volume::L(valor) => write!(f, "{}L", valor),
            Volume::Dl(valor) => write!(f, "{}dL", valor),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum Massa {
    Mcg(Float),
    Mg(Float),
    Kg(Float),
    G(Float),
}

impl Massa {
    pub fn update(self, valor_novo: f32) -> Self {
        match self {
            Self::Mcg(_) => Self::Mcg(Float(valor_novo)),
            Self::Mg(_) => Self::Mg(Float(valor_novo)),
            Self::Kg(_) => Self::Kg(Float(valor_novo)),
            Self::G(_) => Self::G(Float(valor_novo)),
        }
    }
    pub fn incrementar(&self, incremento: f32) -> Self {
        match self {
            Self::Mcg(valor) => Self::Mcg(Float(valor.0 + incremento)),
            Self::Mg(valor) => Self::Mg(Float(valor.0 + incremento)),
            Self::Kg(valor) => Self::Kg(Float(valor.0 + incremento)),
            Self::G(valor) => Self::G(Float(valor.0 + incremento)),
        }
    }

    pub fn valor(&self) -> f32 {
        match self {
            Massa::Mcg(v) | Massa::Mg(v) | Massa::Kg(v) | Massa::G(v) => v.0.clone(),
        }
    }

    pub fn normalizar(&self) -> Self {
        match self {
            Massa::Mg(valor) => Massa::Mg(Float(valor.0)),
            Massa::Mcg(valor) => Massa::Mg(Float(valor.0 / 1000.0)),
            Massa::Kg(valor) => Massa::Mg(Float(valor.0 * 1000000.0)),
            Massa::G(valor) => Massa::Mg(Float(valor.0 * 1000.0)),
        }
    }
}

impl std::fmt::Display for Massa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Massa::Mcg(valor) => write!(f, "{}mcg", valor),
            Massa::Mg(valor) => write!(f, "{}mg", valor),
            Massa::Kg(valor) => write!(f, "{}Kg", valor),
            Massa::G(valor) => write!(f, "{}g", valor),
        }
    }
}

/// Tempo normalmente descrito entre doses da apresentação padrão. Ex.: Amoxicilina 50mg/Kgdia significa que o intervalo da medicação é `dia` e a dose de 50mg/Kg. Porém, este remédio pode ser administrado de 8/8h, tornando a frequência de administrações muito maior que o intervalo.
#[derive(/*Serialize, Deserialize,*/ Debug, PartialEq, Eq, Clone)]
pub enum Intervalo {
    Minuto,
    Hora(i32),
    Dia,
    // Semanas,
}
impl std::fmt::Display for Intervalo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Minuto => write!(f, "min"),
            Self::Hora(tempo) => write!(f, "{}h", tempo),
            Self::Dia => write!(f, "dia"),
        }
    }
}

#[derive(/*Serialize, Deserialize,*/ Debug, PartialEq, Eq, Clone)]
pub enum Aplicacao {
    Comprimido(Massa),
    Jato(Massa),
    Gota(Massa),
    Microgota(Massa),
}
impl std::fmt::Display for Aplicacao {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // TODO: implementar matches em valor para plurais
            // evitando espaço quando for unitário.
            // Ex.: por jato vs. por cada 3 jatos.
            Self::Comprimido(valor) => write!(f, "{} por comprimido", valor),
            Self::Jato(valor) => write!(f, "{} a cada jato", valor),
            Self::Gota(valor) => write!(f, "{} por gota", valor),
            Self::Microgota(valor) => write!(f, "{} por microgota", valor),
        }
    }
}

#[derive(/*Serialize, Deserialize,*/ Debug, PartialEq, Eq, Clone)]
pub enum Duracao {
    Minuto(i32),
    Hora(i32),
    Dia(i32),
}
impl Duracao {
    fn valor(&self) -> i32 {
        match self {
            Duracao::Minuto(valor) => *valor,
            Duracao::Hora(valor) => *valor,
            Duracao::Dia(valor) => *valor,
        }
    }
}
impl std::fmt::Display for Duracao {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // TODO: plurais...
            Self::Minuto(valor) => write!(f, "{} minutos(s)", valor),
            Self::Hora(valor) => write!(f, "{} hora(s)", valor),
            Self::Dia(valor) => write!(f, "{} dia(s)", valor),
        }
    }
}

/// A frequência descreve o tempo, número ou fração de administração da medicação dentro do `Intervalo`.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Frequencia {
    Horas(i32),
    Min(i32),
}

impl Frequencia {
    fn valor(self) -> i32 {
        match self {
            Frequencia::Horas(valor) | Frequencia::Min(valor) => valor,
        }
    }
}
impl std::fmt::Display for Frequencia {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Frequencia::Horas(hrs) => write!(f, "a cada {}h", hrs), //TODO: plurais
            Frequencia::Min(min) => write!(f, "a cada {}min", min), //TODO: plurais
        }
    }
}

#[derive(/*Serialize, Deserialize,*/ Debug, PartialEq, Eq, Clone)]
pub enum Posologia {
    DoseKg(Via, Massa),
    DoseKgIntervaloDuracao(Via, Massa, Intervalo, Duracao, Frequencia), // ex. 25mg/kg*dia por 5 dias via oral a cada 8h
    DoseUnica(Massa, Via),
    // TODO: InfusaoContinua,
}

impl std::fmt::Display for Posologia {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Posologia::DoseKg(via, massa) => {
                write!(f, "{}/Kg {}", massa, via)
            }
            Posologia::DoseKgIntervaloDuracao(via, massa, intervalo, duracao, frequencia) => {
                write!(
                    f,
                    "{}/Kg{} por {} {} {}", // Via:: já completa "via"
                    massa, intervalo, duracao, frequencia, via
                )
            }
            Posologia::DoseUnica(dose, via) => write!(f, "{} {} uma única vez.", dose, via),
        }
    }
}

#[derive(/*Serialize, Deserialize,*/ Debug, PartialEq, Eq, Clone, Copy)]
pub enum Via {
    Intravenosa,
    Intramuscular,
    Oral,
    Retal,
    Topica,
    Inalatoria,
}
impl Capitalizar for Via {
    fn capitalizar(&self, tipo: TipoCapitalizacao) -> String {
        match tipo {
            TipoCapitalizacao::Primeira => {
                let mut chars = format!("{}", self);
                match chars.chars().next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str()[1..],
                }
            }
            TipoCapitalizacao::Todas => todo!(),
        }
    }
}
impl std::fmt::Display for Via {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Via::Intravenosa => write!(f, "via intravenosa"),
            Via::Oral => write!(f, "via oral"),
            Via::Retal => write!(f, "via retal"),
            Via::Topica => write!(f, "via tópica"),
            Via::Inalatoria => write!(f, "via inalatória"),
            Via::Intramuscular => write!(f, "via intramuscular"),
        }
    }
}

pub enum TipoCapitalizacao {
    Primeira,
    Todas,
}

/// Improviso? Criar uma macro derive para cobrir tipos futuros? Não sei...
pub trait Capitalizar {
    fn capitalizar(&self, tipo: TipoCapitalizacao) -> String;
}
