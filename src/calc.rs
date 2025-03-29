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
    // Massas em mg por padrão
    massa_paciente.normalizar();
    // Idade em meses por padrão
    idade.normalizar();

    match posologia {
        Posologia::DoseKg(_via, dose_por_kg) => {
            dose_por_kg.normalizar();
            let via = _via.capitalizar(TipoCapitalizacao::Primeira);
            match apresentacao {
                Apresentacao::DoseVolume(qdte_med, mut vol_med, tipo, nomes_comerciais) => {
                    vol_med.normalizar();
                    let dose = (vol_med / *qdte_med) * *dose_por_kg * massa_paciente;
                    // TODO:correto devolver ml? Tentar devolver na unidade de vol_med
                    format!("{}: tomar {:.1}ml.", via, dose)
                }
                Apresentacao::DoseCompostaVolume(massas, volume_apres, _, _) => {
                    let massa_princ = massas[0].normalizar();
                    let vol = (*volume_apres / massa_princ) * *dose_por_kg * massa_paciente;
                    // TODO:correto devolver ml? Tentar devolver na unidade de vol_med
                    format!("{}: tomar {:.1}ml.", via, vol)
                }
                Apresentacao::DoseAplicacao(aplicacao, _, _) => match aplicacao {
                    Aplicacao::Gota(massa_por_gota) => {
                        massa_por_gota.normalizar();
                        let gotas = *massa_por_gota / (*dose_por_kg * massa_paciente);
                        format!("{}: Tomar {} diluídas", via, gotas)
                    }
                    Aplicacao::Microgota(qtde_por_ugota) => {
                        qtde_por_ugota.normalizar();
                        let ugotas = *qtde_por_ugota / (massa_paciente * *dose_por_kg);
                        format!("{}: Tomar {} diluídas", via, ugotas)
                    }
                    Aplicacao::Jato(massa_por_jato) => {
                        massa_por_jato.normalizar();
                        let jatos = *massa_por_jato / (*dose_por_kg * massa_paciente);
                        format!("{}: Inalar {} jatos.", via, jatos)
                    }
                    Aplicacao::Comprimido(qtde_por_cp) => {
                        qtde_por_cp.normalizar();
                        let cps = *qtde_por_cp / (massa_paciente * *dose_por_kg);
                        format!("{}: Tomar {} comprimidos.", via, cps)
                    }
                },
            }
        }

        Posologia::DoseKgIntervaloDuracao(via, qtde_dose, intervalo, duracao, frequencia) => {
            qtde_dose.normalizar();
            let no_doses_interv = 24.00 / frequencia.valor() as f32;

            // TODO: Considerar se qtde_dose deve mesmo ser em ml.
            let qte_dose = match apresentacao {
                Apresentacao::DoseCompostaVolume(massas, volume, _, _) => {
                    volume.normalizar();
                    let dose = (massa_paciente * *volume / massas[0]) / no_doses_interv;
                    format!("{}", dose)
                }
                Apresentacao::DoseVolume(qtde_por_vol, volume, _, _) => {
                    format!(
                        "{:.1}ml",
                        ((*volume / *qtde_por_vol) * massa_paciente) / no_doses_interv
                    )
                }
                Apresentacao::DoseAplicacao(aplicacao, _, _) => match aplicacao {
                    Aplicacao::Gota(massa_por_gota) => {
                        massa_por_gota.normalizar();
                        let gotas = massa_paciente / *massa_por_gota;
                        format!("{:.0} gota(s)", gotas)
                    }
                    Aplicacao::Microgota(massa_ugota) => {
                        massa_ugota.normalizar();
                        let ugotas = massa_paciente / *massa_ugota;
                        format!("{:.0} microgota(s)", ugotas)
                    }
                    Aplicacao::Comprimido(massa_cp) => {
                        massa_cp.normalizar();
                        let cps = massa_paciente / *massa_cp;
                        format!("{:.0} comprimido(s)", cps)
                    }
                    Aplicacao::Jato(qtd_por_jato) => {
                        qtd_por_jato.normalizar();
                        let jatos = massa_paciente / *qtd_por_jato;
                        format!("{:.0} jato(s)", jatos)
                    }
                },
            };
            format!(
                "{}: administrar {} {} por {}.",
                via.capitalizar(TipoCapitalizacao::Primeira),
                qtde_dose,
                frequencia,
                duracao
            )
        }
        Posologia::DoseUnica(qtde, via) => {
            qtde.normalizar();
            match via {
                Via::Oral => match apresentacao {
                    _ => format!("Tomar {} uma única vez.", qtde),
                },
                Via::Intravenosa => format!("Fazer {} uma única vez.", qtde),
                Via::Intramuscular => format!("Injetar {} em músculo vascularizado uma vez.", qtde),
                Via::Inalatoria => match apresentacao {
                    // Apresentações compostas precisam ser dose-elaboradas de acordo com a principal medicação, listada como a primeira.
                    //     Apresentacao::DoseCompostaVolume(massas, volume, tipo, nomes_comerciais) => {
                    //         let vol = match volume {
                    //             Volume::Ml(vol) => vol.0,
                    //             _ => todo!(),
                    //         };
                    //         let mass = match massas[0] {
                    //             Massa::Mg(mass) => mass.0,
                    //             _ => todo!(),
                    //         };
                    //         // let dosagem = match dose {
                    //         //     Massa::Mg(mass) => mass.0,
                    //         //     _ => todo!(),
                    //         // };
                    //         let dose_final = vol * (mass * massa_paciente.valor()) / mass;
                    //         format!("Preparar {}{} e inalar vez.", volume.tipo(), dose_final)
                    //     }
                    //     Apresentacao::DoseVolume(massa, volume, tipo, nomes_comerciais) => match volume
                    //     {
                    //         Volume::Ml(vol) => {
                    //             let dose_final = vol.0 * massa_paciente.valor() / massa.valor();
                    //             // Massa::Mg(mass) => mass.0,
                    //             let dosagem = match qtde {
                    //                 Massa::Mg(mass) => mass.0,
                    //                 _ => todo!(),
                    //             };
                    //             let dose_final = vol * dosagem / massa;
                    //             // _ => todo!(),
                    //             format!(
                    //                 "Inalar {}{} diluído uma única vez.",
                    //                 dose_final,
                    //                 volume.tipo()
                    //             )
                    //         }
                    //         Apresentacao::DoseAplicacao(aplicacao, _, _) => match aplicacao {
                    //             _ => todo!(),
                    //         },
                    //         Aplicacao::Comprimido(quantidade) => {
                    //             format!("Tomar {} comprimidos uma única vez.", quantidade)
                    //         }
                    //         Aplicacao::Jato(quantidade) => {
                    //             let dosagem = match dose {
                    //                 Massa::Mg(mass) => mass.0,
                    //                 _ => todo!(),
                    //             };
                    //             let quant = match quantidade {
                    //                 Massa::Mg(quant) => quant.0,
                    //                 Massa::Mcg(quant) => quant.0,
                    //                 _ => todo!(),
                    //             };
                    //             let jatos = quant / dosagem;

                    //             format!("Fazer {} uma única vez.", quantidade)
                    //         }
                    //         _ => todo!(),
                    //     },
                    _ => todo!(),
                },
                _ => todo!(),
                // Via::Retal | Via::Topica => format!("Aplicar {} uma única vez.", dose),
            }
        }
    }
}
