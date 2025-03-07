use crate::{Aplicacao, Apresentacao, Float, Massa, Medicamento, Posologia, Via, Volume};

pub const BULARIO: &'static [Medicamento] = &[
    Medicamento {
        nome: "Amoxicilina",
        nome_comercial: Some("AMCLAVU BD"),
        apresentacoes: &[Apresentacao::DoseVolume(
            Float(5.0),
            Float(1.0),
            Massa::Mg,
            Volume::Ml,
        )],
        posologias: &[Posologia::DoseKgDuracaoDias(
            Float(50.0),
            "mg",
            7,
            Via::Oral,
        )],
        advertencias: None,
    },
    Medicamento {
        nome: "Penicilina",
        nome_comercial: Some("Benzetacil"),
        apresentacoes: &[Apresentacao::DoseVolume(
            Float(100.0),
            Float(1.0),
            Massa::Mg,
            Volume::Ml,
        )],
        posologias: &[Posologia::DoseKgDuracaoDias(
            Float(32.0),
            "mg",
            5,
            Via::Oral,
        )],
        advertencias: None,
    },
    Medicamento {
        nome: "Salbutamol",
        nome_comercial: Some("Aerolin"),
        apresentacoes: &[Apresentacao::DoseAplicacao(
            Float(100.0),
            Aplicacao::Jato,
            "mcg",
        )],
        posologias: &[Posologia::DoseUnica(Float(800.0), "dia")],
        advertencias: None,
    },
];
