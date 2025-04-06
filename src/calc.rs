// Elaborar uma funcão tal que os valores recebidos em cada situação
// são normalizados para uma única unidade.
//
use crate::*;

pub fn calcular_dose(
    idade: Idade,
    massa_paciente: Massa,
    posologia: &Posologia,
    apresentacao: &Apresentacao,
) -> String {
    // Massas de pacientes em Kg por padrão.
    // Sempre converta na dúvida.

    // Idade em meses por padrão
    idade.em_meses();

    match posologia {
        Posologia::MgKg(_, _via, dose_por_kg) => {
            dose_por_kg.em_mg();
            let via = _via.capitalizar(TipoCapitalizacao::Primeira);
            match apresentacao {
                Apresentacao::DoseVolume(qdte_med, vol_med, tipo, nomes_comerciais) => {
                    qdte_med.em_mg();
                    vol_med.em_ml();
                    let dose =
                        dose_por_kg.em_mg() * massa_paciente * vol_med.em_ml() / qdte_med.em_mg();
                    // TODO:correto devolver ml? Tentar devolver na unidade de vol_med
                    format!("{}: fazer {:.1}ml.", via, dose)
                }
                Apresentacao::DoseCompostaVolume(massas, volume_apres, _, _) => {
                    let massa_princ = massas[0].em_mg();
                    let vol =
                        ((*volume_apres / massa_princ) * *dose_por_kg * massa_paciente) / 1000.0;
                    // TODO:correto devolver ml? Tentar devolver na unidade de vol_med
                    format!("{}: fazer {:.1}ml.", via, vol)
                }
                Apresentacao::DoseAplicacao(aplicacao, _, _) => match aplicacao {
                    Aplicacao::Gota(massa_por_gota) => {
                        let gotas = dose_por_kg.em_mg() * massa_paciente / massa_por_gota.em_mg();
                        format!("{}: tomar {:.0} gotas diluídas", via, gotas)
                    }
                    Aplicacao::Microgota(qtde_por_ugota) => {
                        let ugotas =
                            qtde_por_ugota.em_mg() * massa_paciente.em_mg() / dose_por_kg.em_mg();
                        format!("{}: tomar {:.0} microgotas diluídas", via, ugotas)
                    }
                    Aplicacao::Jato(massa_por_jato) => {
                        let jatos = dose_por_kg.em_mg() * massa_paciente / massa_por_jato.em_mg();
                        format!("{}: inalar {:.0} jatos.", via, jatos)
                    }
                    Aplicacao::Comprimido(qtde_por_cp) => {
                        let cps = massa_paciente * dose_por_kg.em_mg() / qtde_por_cp.em_mg();
                        format!("{}: tomar {:.1} comprimidos.", via, cps)
                    }
                    Aplicacao::Capsula(qtde_por_cp) => {
                        let cps = massa_paciente * dose_por_kg.em_mg() / qtde_por_cp.em_mg();
                        format!("{}: tomar {:.0} cápsulas.", via, cps)
                    }
                },
            }
        }

        Posologia::MgKgIntervaloDuracao(_, via, qtde_dose, intervalo, duracao, frequencia) => {
            qtde_dose.em_mg();
            // TODO: e se o intervalo não é dia?
            let freq_dia = frequencia.valor() as f32;
            let no_doses_interv = 24.00 / freq_dia;

            // TODO: Considerar se qtde_dose deve mesmo ser em ml.
            let dose = match apresentacao {
                Apresentacao::DoseCompostaVolume(massas, volume, _, _) => {
                    let dose = *qtde_dose * massa_paciente * (volume.em_ml() / massas[0].em_mg())
                        / no_doses_interv;
                    format!("{:.1}ml", dose)
                }
                Apresentacao::DoseVolume(qtde_por_vol, volume, _, _) => {
                    let dose =
                        *qtde_dose * massa_paciente * (volume.em_ml() / qtde_por_vol.em_mg())
                            / no_doses_interv;
                    format!("{:.1}ml", dose)
                }
                Apresentacao::DoseAplicacao(aplicacao, _, _) => match aplicacao {
                    Aplicacao::Gota(massa_por_gota) => {
                        // massa_por_gota.normalizar();
                        let gotas = (massa_paciente * *qtde_dose / massa_por_gota.em_mg())
                            / no_doses_interv;
                        format!("{:.0} gota(s)", gotas)
                    }
                    Aplicacao::Microgota(massa_ugota) => {
                        massa_ugota.em_mg();
                        let ugotas = massa_paciente / *massa_ugota;
                        format!("{:.0} microgota(s)", ugotas)
                    }
                    Aplicacao::Comprimido(massa_cp) => {
                        massa_cp.em_mg();
                        let cps = massa_paciente / *massa_cp;
                        format!("{:.0} comprimido(s)", cps)
                    }
                    Aplicacao::Capsula(massa_cp) => {
                        massa_cp.em_mg();
                        let cps = massa_paciente / *massa_cp;
                        format!("{:.0} cápsula(s)", cps)
                    }
                    Aplicacao::Jato(qtd_por_jato) => {
                        qtd_por_jato.em_mg();
                        let jatos = massa_paciente / *qtd_por_jato;
                        format!("{:.0} jato(s)", jatos)
                    }
                },
            };
            format!(
                "{}: administrar {} {} por {}.",
                via.capitalizar(TipoCapitalizacao::Primeira),
                dose,
                frequencia,
                duracao
            )
        }
        Posologia::DoseUnica(_, qtde, via) => {
            qtde.em_mg();
            let dose = match apresentacao {
                Apresentacao::DoseVolume(massa_por_vol, volume, _, _) => {
                    format!("{}", massa_paciente * *volume / *massa_por_vol)
                }
                Apresentacao::DoseCompostaVolume(qtdes, volume, _, _) => {
                    format!("{}", massa_paciente * *volume / qtdes[0])
                }
                Apresentacao::DoseAplicacao(aplcacao, _, _) => match aplcacao {
                    Aplicacao::Comprimido(qtde_por_cp) | Aplicacao::Capsula(qtde_por_cp) => {
                        format!("{}", massa_paciente / *qtde_por_cp)
                    }
                    Aplicacao::Jato(qtde_por_jato) => {
                        format!("{}", massa_paciente / *qtde_por_jato)
                    }
                    Aplicacao::Gota(qtde_por_gota) => {
                        format!("{}", massa_paciente / *qtde_por_gota)
                    }
                    Aplicacao::Microgota(qtde_por_ugota) => {
                        format!("{}", massa_paciente / *qtde_por_ugota)
                    }
                },
            };
            format!("{} uma única vez {}.", dose, via)
        }
        Posologia::DoseDiaria(_, via, dose_diaria, no_doses) => {
            let tempo = 24 / no_doses;

            match apresentacao {
                Apresentacao::DoseVolume(massa_dose, volume_dose, _, _) => {
                    let vol_dose = dose_diaria.em_mg() * volume_dose.em_ml() / massa_dose.em_mg();
                    let dose_div = vol_dose / *no_doses as f32;
                    if *no_doses == 1 {
                        format!("Administrar {:.1}ml uma vez ao dia.", vol_dose)
                    } else {
                        format!("Adminisrtar {:.1}ml a cada {}h", dose_div, tempo)
                    }
                }
                Apresentacao::DoseCompostaVolume(massas_dose, volume_dose, _, _) => {
                    todo!()
                }
                Apresentacao::DoseAplicacao(aplicacao, _, _) => match aplicacao {
                    Aplicacao::Comprimido(massa_cp) => {
                        let comps = dose_diaria.em_mg() / massa_cp.em_mg();
                        let comps_div = comps / *no_doses as f32;
                        if *no_doses == 1 {
                            format!("tomar {:.1} comprimido(s) por dia.", comps)
                        } else {
                            format!("tomar {:.1} comprimidos a cada {}h.", comps_div, tempo)
                        }
                    }
                    Aplicacao::Jato(massa_por_jato) => {
                        let jatos = dose_diaria.em_mg() / massa_por_jato.em_mg();
                        let jatos_div = jatos / *no_doses as f32;
                        if *no_doses == 1 {
                            format!("inalar {:.1} jato(s) a por dia.", jatos)
                        } else {
                            format!("inalar {:.1} jato(s) a cada {}h.", jatos_div, tempo)
                        }
                    }
                    _ => todo!(),
                },
            }
        }
    }
}
// match via {
//     Via::Oral => match apresentacao {
//         _ => format!("Tomar {} uma única vez.", qtde),
//     },
//     Via::Intravenosa => format!("Fazer {} uma única vez.", qtde),
//     Via::Intramuscular => format!("Injetar {} em músculo vascularizado uma vez.", qtde),
//     Via::Inalatoria => match apresentacao {
//         // Apresentações compostas precisam ser dose-elaboradas de acordo com a principal medicação, listada como a primeira.
//         //     Apresentacao::DoseCompostaVolume(massas, volume, tipo, nomes_comerciais) => {
//         //         let vol = match volume {
//         //             Volume::Ml(vol) => vol.0,
//         //             _ => todo!(),
//         //         };
//         //         let mass = match massas[0] {
//         //             Massa::Mg(mass) => mass.0,
//         //             _ => todo!(),
//         //         };
//         //         // let dosagem = match dose {
//         //         //     Massa::Mg(mass) => mass.0,
//         //         //     _ => todo!(),
//         //         // };
//         //         let dose_final = vol * (mass * massa_paciente.valor()) / mass;
//         //         format!("Preparar {}{} e inalar vez.", volume.tipo(), dose_final)
//         //     }
//         //     Apresentacao::DoseVolume(massa, volume, tipo, nomes_comerciais) => match volume
//         //     {
//         //         Volume::Ml(vol) => {
//         //             let dose_final = vol.0 * massa_paciente.valor() / massa.valor();
//         //             // Massa::Mg(mass) => mass.0,
//         //             let dosagem = match qtde {
//         //                 Massa::Mg(mass) => mass.0,
//         //                 _ => todo!(),
//         //             };
//         //             let dose_final = vol * dosagem / massa;
//         //             // _ => todo!(),
//         //             format!(
//         //                 "Inalar {}{} diluído uma única vez.",
//         //                 dose_final,
//         //                 volume.tipo()
//         //             )
//         //         }
//         //         Apresentacao::DoseAplicacao(aplicacao, _, _) => match aplicacao {
//         //             _ => todo!(),
//         //         },
//         //         Aplicacao::Comprimido(quantidade) => {
//         //             format!("Tomar {} comprimidos uma única vez.", quantidade)
//         //         }
//         //         Aplicacao::Jato(quantidade) => {
//         //             let dosagem = match dose {
//         //                 Massa::Mg(mass) => mass.0,
//         //                 _ => todo!(),
//         //             };
//         //             let quant = match quantidade {
//         //                 Massa::Mg(quant) => quant.0,
//         //                 Massa::Mcg(quant) => quant.0,
//         //                 _ => todo!(),
//         //             };
//         //             let jatos = quant / dosagem;

//         //             format!("Fazer {} uma única vez.", quantidade)
//         //         }
//         //         _ => todo!(),
//         //     },
//         _ => todo!(),
//     },
// _ => todo!(),
// Via::Retal | Via::Topica => format!("Aplicar {} uma única vez.", dose),
// }
