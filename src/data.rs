use crate::{
    Aplicacao, Apresentacao, Dose, Duracao, Float, Frequencia, Intervalo, Massa, Medicamento,
    NomesComerciais, Posologia, TipoApresentacao, Uso, Via, Volume, UI,
};

pub const BULARIO: &'static [Medicamento] = &[
    Medicamento {
        nome: "Dipirona",
        apresentacoes: &[Apresentacao::DoseAplicacao(
            Aplicacao::Gota(Massa::Mg(Float(25.0))),
            TipoApresentacao::Ampola,
            NomesComerciais(&["Novalgina", "dipimax"]),
        ), Apresentacao::DoseVolume(Massa::Mg(Float(500.0)), Volume::Ml(Float(1.0)), TipoApresentacao::Gotejador, NomesComerciais(&["Novalgina"]))],
        posologias: &[
            Posologia::MgKgIntervaloDuracao(
                Uso{main: "para febre ou dor", alts: None},
                Via::Oral,
                Massa::Mg(Float(25.0)), // rever essa dose, esta é a MgKg... a diária é bem maior!
                Intervalo::Dia,
                Duracao::Dia(7),
                Frequencia::Horas(6),
            ),
            Posologia::MgKg(Uso{main: "para febre ou dor", alts: None},Via::Oral, Massa::Mg(Float(25.0))),
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
            Apresentacao::DoseAplicacao(
                Aplicacao::Comprimido(Massa::Mg(Float(500.0))),
                TipoApresentacao::Comprimido,
                NomesComerciais(&[]),
            ),
        ],
        posologias: &[
            Posologia::MgKgIntervaloDuracao(
                Uso{main: "dose pediátrica padrão", alts: None},
                Via::Oral,
                Massa::Mg(Float(50.0)),
                Intervalo::Dia,
                Duracao::Dia(7),
                Frequencia::Horas(8),
            ),
            Posologia::DoseDiaria(
                Uso{main: "dose padrão diária para tratamento de mastite na lactante", alts: None}                ,Via::Oral, Massa::Mg(Float(500.0)), 2),
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
                Uso{main: "dose pediátrica padrão", alts: None},
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
        apresentacoes: &[
            Apresentacao::DoseVolume(
                Massa::Mg(Float(200.0)),
                Volume::Ml(Float(5.0)),
                TipoApresentacao::Frasco,
                NomesComerciais(&["Astro"]),
            ),
            Apresentacao::DoseAplicacao(
                Aplicacao::Comprimido(Massa::Mg(Float(500.0))),
                TipoApresentacao::Comprimido,
                NomesComerciais(&[]),
            ),
        ],
        posologias: &[
            Posologia::MgKgIntervaloDuracao(
                Uso{main: "dose pediátrica padrão mínima", alts: None},
                Via::Oral,
                Massa::Mg(Float(10.0)),
                Intervalo::Dia,
                Duracao::Dia(5),
                Frequencia::Horas(24),
            ),
            Posologia::MgKgIntervaloDuracao(
                Uso{main: "dose pediátrica padrão máxima", alts: None},
                Via::Oral,
                Massa::Mg(Float(20.0)),
                Intervalo::Dia,
                Duracao::Dia(3),
                Frequencia::Horas(24),
            ),
            Posologia::MgKg(
                Uso{main: "dose pediátrica padrão", alts: None},
                Via::Oral, Massa::Mg(Float(20.0))),
        ],
        advertencias: Some(&[
            "Dose diária máxima de 500mg em crianças.",
            "Em dose única pediátrica, não ultrapassar 1g/paciente.",
        ]),
    },
    Medicamento {
        nome: "Claritromicina",
        apresentacoes: &[
            Apresentacao::DoseVolume(
                Massa::Mg(Float(25.0)),
                Volume::Ml(Float(1.0)),
                TipoApresentacao::Suspensao,
                NomesComerciais(&["Klaricid"]),
            ),
            Apresentacao::DoseVolume(
                Massa::Mg(Float(50.0)),
                Volume::Ml(Float(1.0)),
                TipoApresentacao::Suspensao,
                NomesComerciais(&["Klaricid"]),
            ),
            Apresentacao::DoseAplicacao(
                Aplicacao::Comprimido(Massa::Mg(Float(500.0))),
                TipoApresentacao::Comprimido, // comp  de liberacao prolongada, criat tipo
                NomesComerciais(&["Klaricid UD"]),
            ),
        ],
        posologias: &[Posologia::MgKgIntervaloDuracao(
                Uso{main: "dose pediátrica padrão", alts: None},
            Via::Oral,
            Massa::Mg(Float(15.0)),
            Intervalo::Dia,
            Duracao::Dia(10),
            Frequencia::Horas(12),
        )],
        advertencias: None,
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
        posologias: &[Posologia::MgKg(
                Uso{main: "dose pediátrica padrão para abordagem da convulsão", alts: None},

            Via::Intramuscular, Massa::Mg(Float(0.2)))],
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
        posologias: &[Posologia::MgKg(
                Uso{main: "dose pediátrica padrão para abordagem da convulsão", alts: None},
            Via::Intramuscular, Massa::Mg(Float(0.5)))],
        advertencias: None,
    },
    Medicamento {
        nome: "Ibuprofeno",
        apresentacoes: &[
            Apresentacao::DoseAplicacao(
                Aplicacao::Comprimido(Massa::Mg(Float(600.0))),
                TipoApresentacao::Comprimido,
                NomesComerciais(&[]),
            ),
            Apresentacao::DoseAplicacao(
                Aplicacao::Gota(Massa::Mg(Float(10.0))),
                TipoApresentacao::Frasco,
                NomesComerciais(&["Alivium gota concentrada (GC) 200mg/ml"]),
            ),
            Apresentacao::DoseVolume(
                Massa::Mg(Float(30.0)),
                Volume::Ml(Float(1.0)),
                TipoApresentacao::Frasco,
                NomesComerciais(&["Alivium suspensão"]),
            ),
        ],
        posologias: &[
            Posologia::MgKg(
                Uso{main: "dose pediátrica padrão antitérmica", alts: None},
                Via::Oral, Massa::Mg(Float(20.0))),
            Posologia::MgKgIntervaloDuracao(
                Uso{main: "dose pediátrica padrão antitérmica de horário", alts: None},
                Via::Oral,
                Massa::Mg(Float(20.0)),
                Intervalo::Dia,
                Duracao::Dia(3),
                Frequencia::Horas(6),
            ),
        ],
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
        posologias: &[Posologia::MgKg(
                Uso{main: "dose sedativa pediátrica padrão", alts: None},
            Via::Intravenosa, Massa::Mg(Float(0.3)))],
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
        posologias: &[Posologia::MgKg(
                Uso{main: "dose sedativa pediátrica padrão", alts: None},
            Via::Intravenosa, Massa::Mg(Float(1.5)))],
        advertencias: Some(&[
            "Devido a manutenção do tônus, usar sempre um bloqueador neuromuscular.",
        ]),
    },
    Medicamento {
        nome: "Levodropropizina",
        apresentacoes: &[Apresentacao::DoseVolume(
            Massa::Mg(Float(6.0)),
            Volume::Ml(Float(1.0)),
            TipoApresentacao::Xarope,
            NomesComerciais(&["Percof"]),
        )],
        posologias: &[Posologia::MgKg(
                Uso{main: "dose antitussígena pediátrica", alts: None},
            Via::Oral, Massa::Mg(Float(1.0)))],
        advertencias: Some(&["Uso em crianças acima de 2 anos."]),
    },
    Medicamento {
        nome: "Cefalexina",
        apresentacoes: &[
            Apresentacao::DoseAplicacao(
                Aplicacao::Comprimido(Massa::Mg(Float(500.0))),
                TipoApresentacao::Comprimido,
                NomesComerciais(&[]),
            ),
            Apresentacao::DoseVolume(
                Massa::Mg(Float(250.0)),
                Volume::Ml(Float(5.0)),
                TipoApresentacao::Suspensao,
                NomesComerciais(&[]),
            ), // Apresentacao::
        ],
        posologias: &[
            Posologia::MgKg(
                Uso{main: "dose padrão pediátrica para tratamento de infecção urinária", alts: None},
                Via::Oral, Massa::Mg(Float(100.0))),
            Posologia::MgKg(
                Uso{main: "dose padrão pediátrica profilática de infecção urinária", alts: None},
                Via::Oral, Massa::Mg(Float(25.0))),
            Posologia::MgKgIntervaloDuracao(
                Uso {
                    main: "Infecção urinária",
                    alts: None
                },
                Via::Oral,
                Massa::Mg(Float(70.0)),
                Intervalo::Dia,
                Duracao::Dia(14),
                Frequencia::Horas(6),
            ),
            Posologia::DoseDiaria(
                Uso{main: "dose padrão de tratamento de infecções de tecido mole, inclusive em pediatria", alts: None},
                Via::Oral, Massa::Mg(Float(500.0)), 1),
        ],
        advertencias: None,
    }, Medicamento {
        nome: "Penicilina benzatina",
        apresentacoes: &[Apresentacao::DoseUI(UI(600000), TipoApresentacao::Ampola, NomesComerciais(&["Benzetacil"]))],
        posologias: &[Posologia::DoseUnica(
            Uso {
                main: "Faringoamigdalite em pacientes menores de 45kg.",
                alts: None
            }, Dose::UI(UI(600000)), Via::Intramuscular)],
        advertencias: None
    }
    // SBP - ITU 2021
    //
    // Febril
    // -- IV
    // cefuroxime 150mg/kg/dia 8/8h
    // genta 5-7,5mg/kgdia 1x ao dia pode ser IM
    // amicacina 15mg/kgdia 1x ao dia
    // cefotaxime 150-200mg/kg/dia 8/8h
    // pipetazo 300mg/kgdia 6/6 ou 8/8
    // -- VO
    // Cefuroxime 30mg/kgdia 12/12h
    // cefaclor 40mg/kgdia 8/8h
    //
    // Afebril
    // Nitrufurantoina 5-7mg/kgdia 6/6h
    // Cefalexina 50mg/kgdia 6 ou 8h
    // bactrim 8-12mg tmp kg/dia 12/12h
    //
    //
    // fexofenadina 6mg/ml allegra
    // levodropropizina 6mg/ml percof
    // zina odt levocetirizina 5mg dispersível/comp mastigável? ver tipo
    //
    // sucralfato Sucrafilm 200mg/ml tratar refluxos
    // 2.5ml 6/6h menores de 6a
    // 5ml maiores...
    //
    // foroato de mometasona 50mcg/jato a partir de 2 anos, dose terapeutica 200mcg
];

// AesclepiusRx  Copyright (C) 2025  Jefferson T.
// Under Gnu General Licence 3.0 for ever.
// Any part of this program is and always have to be under the conditions of the LICENCE.txt
// file under the same repository.
// This program comes with ABSOLUTELY NO WARRANTY; for details type `show w'.
// This is free software, and you are welcome to redistribute it
// under certain conditions; type `show c' for details.
