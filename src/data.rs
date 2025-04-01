use crate::{
    Aplicacao, Apresentacao, Duracao, Float, Frequencia, Intervalo, Massa, Medicamento,
    NomesComerciais, Posologia, TipoApresentacao, Via, Volume,
};

pub const BULARIO: &'static [Medicamento] = &[
    Medicamento {
        nome: "Dipirona",
        apresentacoes: &[Apresentacao::DoseAplicacao(
            Aplicacao::Gota(Massa::Mg(Float(25.0))),
            TipoApresentacao::Ampola,
            NomesComerciais(&["novalgina", "dipimax"]),
        )],
        posologias: &[
            Posologia::MgKgIntervaloDuracao(
                Via::Oral,
                Massa::Mg(Float(25.0)), // rever essa dose, esta é a MgKg... a diária é bem maior!
                Intervalo::Dia,
                Duracao::Dia(7),
                Frequencia::Horas(6),
            ),
            Posologia::MgKg(Via::Oral, Massa::Mg(Float(25.0))),
        ],
        advertencias: None,
    },
    Medicamento {
        nome: "Amoxicilina",
        apresentacoes: &[
            Apresentacao::DoseVolume(
                Massa::Mg(Float(250.0)),
                Volume::Ml(Float(5.0)),
                TipoApresentacao::PoReconstituivel,
                NomesComerciais(&["AMCLAVU BD"]),
            ),
            Apresentacao::DoseVolume(
                Massa::Mg(Float(500.0)),
                Volume::Ml(Float(5.0)),
                TipoApresentacao::PoReconstituivel,
                NomesComerciais(&[]),
            ),
            Apresentacao::DoseAplicacao(
                Aplicacao::Comprimido(Massa::Mg(Float(875.0))),
                TipoApresentacao::Comprimido,
                NomesComerciais(&[]),
            ),
            Apresentacao::DoseAplicacao(Aplicacao::Comprimido(Massa::Mg(Float(500.0))), TipoApresentacao::Comprimido, NomesComerciais(&[]))
        ],
        posologias: &[
            Posologia::MgKgIntervaloDuracao(
                Via::Oral,
                Massa::Mg(Float(50.0)),
                Intervalo::Dia,
                Duracao::Dia(7),
                Frequencia::Horas(8),
            ),
            Posologia::DoseDiaria(Via::Oral, Massa::Mg(Float(500.0)), 2),
        ],
        advertencias: None,
    },
    Medicamento {
        nome: "Amoxicilina com clavulanato",
        apresentacoes: &[Apresentacao::DoseCompostaVolume(
            &[Massa::Mg(Float(250.0)), Massa::Mg(Float(62.50))],
            Volume::Ml(Float(5.0)),
            TipoApresentacao::Frasco,
            NomesComerciais(&["AMCLAVU BD"]),
        )],
        posologias: &[Posologia::MgKgIntervaloDuracao(
            Via::Oral,
            Massa::Mg(Float(50.0)),
            Intervalo::Dia,
            Duracao::Dia(7),
            Frequencia::Horas(8),
        )],
        advertencias: None,
    },
    // Medicamento {
    //     nome: "Penicilina",
    //     apresentacoes: &[Apresentacao::DoseVolume(
    //         Massa::Mg(Float(100.0)),
    //         Volume::Ml(Float(1.0)),
    //         TipoApresentacao::Ampola,
    //         NomesComerciais(&["Benzetacil"]),
    //     )],
    //     posologias: &[Posologia::MgKgIntervaloDuracao(
    //         Via::Oral,
    //         Massa::Mg(Float(32.0)),
    //         Intervalo::Dia,
    //         Duracao::Dia(5),
    //         Frequencia::Horas(6),
    //     )],
    //     advertencias: None,
    // },
    // Medicamento {
    //     nome: "Salbutamol",
    //     apresentacoes: &[Apresentacao::DoseAplicacao(
    //         Aplicacao::Jato(Massa::Mcg(Float(100.0))),
    //         TipoApresentacao::Spray,
    //         NomesComerciais(&["Aerolin"]),
    //     )],
    //     posologias: &[Posologia::DoseUnica(
    //         Massa::Mg(Float(800.0)),
    //         Via::Inalatoria,
    //     )],
    //     advertencias: None,
    // },
    Medicamento {
        nome: "Azitromicina",
        apresentacoes: &[Apresentacao::DoseVolume(
            Massa::Mg(Float(200.0)),
            Volume::Ml(Float(5.0)),
            TipoApresentacao::Frasco,
            NomesComerciais(&["Astro"]),
        )],
        posologias: &[
            Posologia::MgKgIntervaloDuracao(
                Via::Oral,
                Massa::Mg(Float(10.0)),
                Intervalo::Dia,
                Duracao::Dia(5),
                Frequencia::Horas(24),
            ),
            Posologia::MgKgIntervaloDuracao(
                Via::Oral,
                Massa::Mg(Float(20.0)),
                Intervalo::Dia,
                Duracao::Dia(3),
                Frequencia::Horas(24),
            ),
        ],
        advertencias: Some(&["Dose diária máxima de 500mg em crianças."]),
    },
    Medicamento {
        nome: "Midazolam",
        apresentacoes: &[
            Apresentacao::DoseVolume(
                Massa::Mg(Float(15.0)),
                Volume::Ml(Float(5.0)),
                TipoApresentacao::Ampola,
                NomesComerciais(&["Dormonid"]),
            ),
            Apresentacao::DoseVolume(
                Massa::Mg(Float(5.0)),
                Volume::Ml(Float(1.0)),
                TipoApresentacao::Ampola,
                NomesComerciais(&["Dormonid"]),
            ),
            Apresentacao::DoseVolume(
                Massa::Mg(Float(1.0)),
                Volume::Ml(Float(1.0)),
                TipoApresentacao::Ampola,
                NomesComerciais(&["Dormonid"]),
            ),
        ],
        posologias: &[Posologia::MgKg(Via::Intramuscular, Massa::Mg(Float(0.2)))],
        advertencias: None,
    },
    Medicamento {
        nome: "Diazepam",
        apresentacoes: &[Apresentacao::DoseVolume(
            Massa::Mg(Float(10.0)),
            Volume::Ml(Float(2.0)),
            TipoApresentacao::Ampola,
            NomesComerciais(&[]),
        )],
        posologias: &[Posologia::MgKg(Via::Intramuscular, Massa::Mg(Float(0.5)))],
        advertencias: None,
    },
    Medicamento {
        nome: "Ibuprofeno",
        apresentacoes: &[Apresentacao::DoseAplicacao(
            Aplicacao::Gota(Massa::Mg(Float(2.5))),
            TipoApresentacao::Frasco,
            NomesComerciais(&["Alivium"]),
        )],
        posologias: &[Posologia::MgKg(Via::Oral, Massa::Mg(Float(5.0)))],
        advertencias: None,
    },
    Medicamento {
        nome: "Etomidato",
        apresentacoes: &[Apresentacao::DoseVolume(
            Massa::Mg(Float(2.0)),
            Volume::Ml(Float(10.0)),
            TipoApresentacao::Ampola,
            NomesComerciais(&[]),
        )],
        posologias: &[Posologia::MgKg(Via::Intravenosa, Massa::Mg(Float(0.3)))],
        advertencias: Some(&[
            "Sempre diluir e bolus em velocidade estável, evitar lentamente ou muito rapidamente.",
        ]),
    },
    Medicamento {
        nome: "Cetamina",
        apresentacoes: &[
            Apresentacao::DoseVolume(
                Massa::Mg(Float(50.0)),
                Volume::Ml(Float(2.0)),
                TipoApresentacao::Ampola,
                NomesComerciais(&[]),
            ),
            Apresentacao::DoseVolume(
                Massa::Mg(Float(50.0)),
                Volume::Ml(Float(5.0)),
                TipoApresentacao::Ampola,
                NomesComerciais(&[]),
            ),
            Apresentacao::DoseVolume(
                Massa::Mg(Float(50.0)),
                Volume::Ml(Float(10.0)),
                TipoApresentacao::Ampola,
                NomesComerciais(&[]),
            ),
        ],
        posologias: &[Posologia::MgKg(Via::Intravenosa, Massa::Mg(Float(1.5)))],
        advertencias: Some(&[
            "Devido a manutenção do tônus, usar sempre um bloqueador neuromuscular.",
        ]),
    },
];
