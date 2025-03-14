//use serde::{Deserialize, Serialize};
#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod calc;
mod data;
pub use data::BULARIO;

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
            ),
            medicamento_selecionado: Medicamento {
                nome: "medicamento",
                nome_comercial: None,
                apresentacoes: &[Apresentacao::DoseVolume(
                    Massa::Mg(Float(0.0)),
                    Volume::Ml(Float(0.0)),
                )],
                posologias: &[Posologia::DoseUnica(Massa::Mg(Float(0.0)))],
                advertencias: None,
            },
            posologia_selecionada: Posologia::DoseUnica(Massa::Mg(Float(0.0))),
            prescricao: "Medicamento 0g/ml ..... 0ml".to_string(),
        }
    }
}

const TOLERANCE: f32 = 1e-6;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Idade {
    tipo: IdadeTipo,
    valor: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    // fn valor(&self) -> i32 {
    //     format!("{}", self.idade.valor)
    // }
    fn tipo(&self) -> &'static str {
        match self.tipo {
            IdadeTipo::Meses => "meses",
            IdadeTipo::Anos => "anos",
        }
    }
}
impl std::fmt::Display for Idade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.valor, self.tipo())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Float(pub f32);

impl std::fmt::Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

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
    pub nome_comercial: Option<&'static str>,
    pub apresentacoes: &'static [Apresentacao],
    pub posologias: &'static [Posologia],
    pub advertencias: Option<&'static [&'static str]>,
}

impl std::fmt::Display for Medicamento {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.nome_comercial {
            Some(nome_comercial) => write!(
                f,
                "{}: {}",
                nome_comercial.to_string(),
                self.nome.to_string()
            ),
            None => write!(f, "{}", self.nome.to_string()),
        }
    }
}
//#[derive(Serialize, Deserialize)]
#[derive(/*Serialize, Deserialize,*/ Debug, PartialEq, Eq, Clone)]
pub enum Apresentacao {
    DoseVolume(Massa, Volume), // (dose, volume, unidade)
    DoseAplicacao(Aplicacao),
}

impl std::fmt::Display for Apresentacao {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Apresentacao::DoseVolume(massa, volume) => {
                write!(f, "{} a cada {}", massa, volume)
            }
            Apresentacao::DoseAplicacao(aplicacao) => match aplicacao {
                Aplicacao::Comprimido(valor) => write!(f, "{} a cada comprimido", valor),
                Aplicacao::Jato(valor) => write!(f, "{} a cada jato", valor),
                Aplicacao::Gota(valor) => write!(f, "{} a cada gota", valor),
                Aplicacao::Microgota(valor) => write!(f, "{} a cada microgota", valor),
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Volume {
    Ml(Float),
    L(Float),
    Dl(Float),
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

#[derive(/*Serialize, Deserialize,*/ Debug, PartialEq, Eq, Clone)]
pub enum Intervalo {
    Minuto,
    Hora,
    Dia,
    // Semanas,
}
impl std::fmt::Display for Intervalo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Minuto => write!(f, "min"),
            Self::Hora => write!(f, "h"),
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
            Self::Minuto(valor) => write!(f, "por {} minutos(s)", valor),
            Self::Hora(valor) => write!(f, "por {} hora(s)", valor),
            Self::Dia(valor) => write!(f, "por {} dia(s)", valor),
        }
    }
}

#[derive(/*Serialize, Deserialize,*/ Debug, PartialEq, Eq, Clone)]
pub enum Posologia {
    DoseKgIntervaloDuracao(Massa, Intervalo, Duracao, Via), // ex. 25mg/kg*dia por 5 dias via oral
    GotaKg(i32),
    DoseUnica(Massa), // TODO: InfusaoContinua,
}

impl std::fmt::Display for Posologia {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Posologia::DoseKgIntervaloDuracao(massa, intervalo, duracao, via) => {
                write!(
                    f,
                    "{}/Kg{} por {} {}", // Via:: já completa "via"
                    massa, intervalo, duracao, via
                )
            }
            Posologia::DoseUnica(dose) => write!(f, "{}", dose),
            Posologia::GotaKg(gotas) => {
                let gota_gotas = if *gotas > 1 { "gotas" } else { "gota" };
                write!(f, "{} {} por Kg", gotas, gota_gotas)
            }
        }
    }
}

#[derive(/*Serialize, Deserialize,*/ Debug, PartialEq, Eq, Clone)]
pub enum Via {
    Intravenosa,
    Oral,
    Retal,
    Topica,
}
impl std::fmt::Display for Via {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Via::Intravenosa => write!(f, "via intravenosa"),
            Via::Oral => write!(f, "via oral"),
            Via::Retal => write!(f, "via retal"),
            Via::Topica => write!(f, "via tópica"),
        }
    }
}
