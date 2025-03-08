use crate::{
    Aplicacao, Apresentacao, Duracao, Float, Intervalo, Massa, Medicamento, Posologia, Via, Volume,
};

pub const BULARIO: &'static [Medicamento] = &[
    Medicamento {
        nome: "Dipirona",
        nome_comercial: Some("Novalgina"),
        apresentacoes: &[Apresentacao::DoseVolume(
            Massa::Mg(Float(500.0)),
            Volume::Ml(Float(1.0)),
        )],
        posologias: &[
            Posologia::DoseKgIntervaloDuracao(
                Massa::Mg(Float(25.0)),
                Intervalo::Hora,
                Duracao::Dia(6),
                Via::Oral,
            ),
            Posologia::GotaKg(1),
        ],
        advertencias: None,
    },
    Medicamento {
        nome: "Amoxicilina",
        nome_comercial: Some("AMCLAVU BD"),
        apresentacoes: &[Apresentacao::DoseVolume(
            Massa::Mg(Float(5.0)),
            Volume::Ml(Float(1.0)),
        )],
        posologias: &[Posologia::DoseKgIntervaloDuracao(
            Massa::Mg(Float(50.0)),
            Intervalo::Dia,
            Duracao::Dia(7),
            Via::Oral,
        )],
        advertencias: None,
    },
    Medicamento {
        nome: "Penicilina",
        nome_comercial: Some("Benzetacil"),
        apresentacoes: &[Apresentacao::DoseVolume(
            Massa::Mg(Float(100.0)),
            Volume::Ml(Float(1.0)),
        )],
        posologias: &[Posologia::DoseKgIntervaloDuracao(
            Massa::Mg(Float(32.0)),
            Intervalo::Dia,
            Duracao::Dia(5),
            Via::Oral,
        )],
        advertencias: None,
    },
    Medicamento {
        nome: "Salbutamol",
        nome_comercial: Some("Aerolin"),
        apresentacoes: &[Apresentacao::DoseAplicacao(Aplicacao::Jato(Massa::Mcg(
            Float(100.0),
        )))],
        posologias: &[Posologia::DoseUnica(Massa::Mg(Float(800.0)))],
        advertencias: None,
    },
];
