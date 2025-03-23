use crate::{
    Aplicacao, Apresentacao, Duracao, Float, Frequencia, Intervalo, Massa, Medicamento, Posologia,
    Via, Volume,
};

pub const BULARIO: &'static [Medicamento] = &[
    Medicamento {
        nome: "Dipirona",
        nome_comercial: Some("Novalgina"),
        apresentacoes: &[Apresentacao::DoseAplicacao(Aplicacao::Gota(Massa::Mg(
            Float(25.0),
        )))],
        posologias: &[
            Posologia::DoseKgIntervaloDuracao(
                Via::Oral,
                Massa::Mg(Float(25.0)),
                Intervalo::Hora(6),
                Duracao::Dia(7),
                Frequencia::Horas(8),
            ),
            Posologia::GotaKg(1), // existe?
            Posologia::DoseKg(Via::Oral, Massa::Mg(Float(25.0))),
        ],
        advertencias: None,
    },
    Medicamento {
        nome: "Amoxicilina",
        nome_comercial: Some("AMCLAVU BD"),
        apresentacoes: &[Apresentacao::DoseVolume(
            Massa::Mg(Float(250.0)),
            Volume::Ml(Float(5.0)),
        )],
        posologias: &[Posologia::DoseKgIntervaloDuracao(
            Via::Oral,
            Massa::Mg(Float(50.0)),
            Intervalo::Dia,
            Duracao::Dia(7),
            Frequencia::Horas(8),
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
            Via::Oral,
            Massa::Mg(Float(32.0)),
            Intervalo::Dia,
            Duracao::Dia(5),
            Frequencia::Horas(6),
        )],
        advertencias: None,
    },
    Medicamento {
        nome: "Salbutamol",
        nome_comercial: Some("Aerolin"),
        apresentacoes: &[Apresentacao::DoseAplicacao(Aplicacao::Jato(Massa::Mcg(
            Float(100.0),
        )))],
        posologias: &[Posologia::DoseUnica(
            Massa::Mg(Float(800.0)),
            Via::Inalatoria,
        )],
        advertencias: None,
    },
    Medicamento {
        nome: "Azitromicina",
        nome_comercial: Some("Astro"),
        apresentacoes: &[Apresentacao::DoseVolume(
            Massa::Mg(Float(200.0)),
            Volume::Ml(Float(5.0)),
        )],
        posologias: &[Posologia::DoseKgIntervaloDuracao(
            Via::Oral,
            Massa::Mg(Float(20.0)),
            Intervalo::Dia,
            Duracao::Dia(5),
            Frequencia::Horas(24),
        )],
        advertencias: Some(&["Dose diária máxima de 500mg em crianças."]),
    },
];
