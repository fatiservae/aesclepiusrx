//use serde::{Deserialize, Serialize};
#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod data;
pub use app::Instancia;
pub use data::BULARIO;

const TOLERANCE: f32 = 1e-6;

#[derive(Debug, Clone)]
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
    DoseVolume(Float, Float, Massa, Volume), // (dose, volume, unidade)
    DoseAplicacao(Float, Aplicacao, &'static str),
}

impl std::fmt::Display for Apresentacao {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Apresentacao::DoseVolume(dose, volume, massa, unidade_volume) => {
                write!(f, "{}{} a cada {}{}", dose, massa, volume, unidade_volume)
            }
            Apresentacao::DoseAplicacao(dose, aplicacao, unidade) => {
                write!(f, "{}{} a cada {}", dose, unidade, aplicacao)
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Volume {
    Ml,
    L,
    Dl,
}

impl std::fmt::Display for Volume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Volume::Ml => write!(f, "mL"),
            Volume::L => write!(f, "L"),
            Volume::Dl => write!(f, "dL"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Massa {
    Mg,
    Kg,
    G,
}

impl std::fmt::Display for Massa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Massa::Mg => write!(f, "mg"),
            Massa::Kg => write!(f, "Kg"),
            Massa::G => write!(f, "g"),
        }
    }
}

#[derive(/*Serialize, Deserialize,*/ Debug, PartialEq, Eq, Clone)]
pub enum Aplicacao {
    Comprimido,
    Jato,
    Gota,
}

impl std::fmt::Display for Aplicacao {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Comprimido => write!(f, "comprimido"),
            Self::Jato => write!(f, "jato"),
            Self::Gota => write!(f, "gota"),
        }
    }
}

#[derive(/*Serialize, Deserialize,*/ Debug, PartialEq, Eq)]
pub enum Posologia {
    DoseKgDuracaoDias(Float, &'static str, usize, Via),
    DoseUnica(Float, &'static str), // TODO: InfusaoContinua,
}

#[derive(/*Serialize, Deserialize,*/ Debug, PartialEq, Eq)]
pub enum Via {
    Intravenosa,
    Oral,
    Retal,
    Topica,
}
