// // Elaborar uma funcão tal que os valores recebidos em cada situação
// // são normalizados para uma única unidade.
// //
// use crate::*;

// pub fn calcular_dose(
//     idade: Idade,
//     massa_paciente: Massa,
//     posologia: &Posologia,
//     apresentacao: &Apresentacao,
// ) -> String {
//     // Massas em mg por padrão
//     massa_paciente.normalizar();
//     // Idade em meses por padrão
//     idade.normalizar();

//     match posologia {
//         Posologia::DoseKg(via, dose_por_kg) => match apresentacao {
//             Apresentacao::DoseVolume(qdte_med, mut vol_med, _, _) => {
//                 dose_por_kg.normalizar();
//                 vol_med.normalizar();
//                 // let vol_med = match vol_med
//                 let dose = (vol_med.valor() / qdte_med.valor())
//                     * dose_por_kg.valor()
//                     * massa_paciente.valor();
//                 format!(
//                     "{}: {:.1}ml agora.",
//                     via.capitalizar(TipoCapitalizacao::Primeira),
//                     dose // correto devolver ml? Tentar devolver na unidade de vol_med
//                 )
//             }
//             Apresentacao::DoseAplicacao(Aplicacao::Gota(valor), _, _) => {
//                 let dose = match dose_por_kg {
//                     Massa::Mg(valor) => Massa::Mg(Float(&massa_paciente.valor() * valor.0)),
//                     _ => todo!(),
//                 };
//                 // let dosagem = match dose {
//                 //     Massa::Mg(mass) => mass.0,
//                 //     _ => todo!(),
//                 // };
//                 let gotas = dose.valor() / valor.valor();
//                 format!("Tomar {} gotas diluídas", gotas)
//             }
//             Apresentacao::DoseAplicacao(Aplicacao::Jato(Massa::Mcg(massa_medicamento)), _, _) => {
//                 let dose_necessaria = match dose_por_kg {
//                     Massa::Mg(valor) => {
//                         Massa::Mg(Float(&massa_paciente.valor() * massa_medicamento.0))
//                     }
//                     _ => todo!(),
//                 };
//                 let jatos = dose_necessaria.valor() / massa_medicamento.0;
//                 format!("Inalar {} jatos.", jatos)
//             }
//             _ => todo!(),
//         },
//         Posologia::DoseKgIntervaloDuracao(via, massa, intervalo, duracao, frequencia) => {
//             let qtde = match massa {
//                 Massa::Mg(quantidade) => Massa::Mg(Float(massa_paciente.valor() * quantidade.0)),
//                 _ => todo!(),
//             };
//             let vol_dose = match apresentacao {
//                 Apresentacao::DoseCompostaVolume(massas, volume, tipo, nomes_comerciais) => {
//                     let vol = match volume {
//                         Volume::Ml(vol) => vol.0,
//                         _ => todo!(),
//                     };
//                     let mass = match massas[0] {
//                         Massa::Mg(mass) => mass.0,
//                         _ => todo!(),
//                     };
//                     // let dosagem = match dose {
//                     //     Massa::Mg(mass) => mass.0,
//                     //     _ => todo!(),
//                     // };
//                     let vol_final = vol * (massa_paciente.valor() * mass) / mass;
//                     format!("{}{}", vol, volume.tipo(),)
//                 }
//                 Apresentacao::DoseVolume(quanto, volume, _, _) => {
//                     format!(
//                         "{:.1}ml",
//                         (qtde.valor() * volume.valor() / quanto.valor())
//                             / (24.00 / frequencia.valor() as f32)
//                     )
//                     // TODO: checkar tipos e unidades
//                 }
//                 Apresentacao::DoseAplicacao(Aplicacao::Gota(massa_por_gota), _, _) => {
//                     let gotas = match massa_por_gota {
//                         Massa::Mcg(m_gota) => qtde.valor() / (m_gota.0) / 1000.0,
//                         Massa::Kg(m_gota) => 0.0,
//                         Massa::Mg(m_gota) => qtde.valor() / m_gota.0,
//                         Massa::G(m_gota) => qtde.valor() / 1000.0 * m_gota.0,
//                     };
//                     format!("{:.0} gota(s)", gotas)
//                 }
//                 _ => todo!(),
//             };
//             format!(
//                 "{}: tomar {} {} por {}.",
//                 via.capitalizar(TipoCapitalizacao::Primeira),
//                 vol_dose,
//                 frequencia,
//                 duracao
//             )
//         }
//         // Posologia::GotaKg(gotas) => {
//         //     let dose = *gotas as f32 * massa_paciente.valor();
//         //     let mut plural = "";
//         //     if dose > 1.0 {
//         //         plural = "s"
//         //     }
//         //     format!("Tomar {} gota{}", dose, plural) // errado...
//         // }
//         Posologia::DoseUnica(dose, via) => match via {
//             Via::Oral => match apresentacao {
//                 _ => format!("Tomar {} uma única vez.", dose),
//             },
//             Via::Intravenosa => format!("Fazer {} uma única vez.", dose),
//             Via::Intramuscular => format!("Injetar {} em músculo vascularizado uma vez.", dose),
//             Via::Inalatoria => match apresentacao {
//                 // Apresentações compostas precisam ser dose-elaboradas de acordo com a principal medicação, listada como a primeira.
//                 Apresentacao::DoseCompostaVolume(massas, volume, tipo, nomes_comerciais) => {
//                     let vol = match volume {
//                         Volume::Ml(vol) => vol.0,
//                         _ => todo!(),
//                     };
//                     let mass = match massas[0] {
//                         Massa::Mg(mass) => mass.0,
//                         _ => todo!(),
//                     };
//                     // let dosagem = match dose {
//                     //     Massa::Mg(mass) => mass.0,
//                     //     _ => todo!(),
//                     // };
//                     let dose_final = vol * (mass * massa_paciente.valor()) / mass;
//                     format!("Preparar {}{} e inalar vez.", volume.tipo(), dose_final)
//                 }
//                 Apresentacao::DoseVolume(massa, volume, tipo, nomes_comerciais) => match volume {
//                     Volume::Ml(vol) => {
//                         let dose_final = vol.0 * massa_paciente.valor() / massa.valor();
//                         // Massa::Mg(mass) => mass.0,
//                         let dosagem = match dose {
//                             Massa::Mg(mass) => mass.0,
//                             _ => todo!(),
//                         };
//                         let dose_final = vol * dosagem / mass;
//                         // _ => todo!(),
//                         format!("Inalar {}{} diluído uma única vez.", , volume.tipo())
//                     }
//                     Apresentacao::DoseAplicacao(aplicacao, _, _) => match aplicacao {
//                         _ => todo!(),
//                     },
//                     Aplicacao::Comprimido(quantidade) => {
//                         format!("Tomar {} comprimidos uma única vez.", quantidade)
//                     }
//                     Aplicacao::Jato(quantidade) => {
//                         let dosagem = match dose {
//                             Massa::Mg(mass) => mass.0,
//                             _ => todo!(),
//                         };
//                         let quant = match quantidade {
//                             Massa::Mg(quant) => quant.0,
//                             Massa::Mcg(quant) => quant.0,
//                             _ => todo!(),
//                         };
//                         let jatos = quant / dosagem;

//                         format!("Fazer {} uma única vez.", quantidade)
//                     }
//                     _ => todo!(),
//                 },
//             },
//             Via::Retal | Via::Topica => format!("Aplicar {} uma única vez.", dose),
//         },
//     }
// }
