// Licence at the end.
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
    // idade.em_meses();

    match posologia {
        Posologia::DoseUIKg(_, qte, _, _) => {
            format!("{}UI", massa_paciente * *qte)
        }
        Posologia::MgKg(_, _via, dose_por_kg) => {
            dose_por_kg.em_mg();
            let via = _via.capitalizar(TipoCapitalizacao::Primeira);
            match apresentacao {
                Apresentacao::DoseUI(_, _, _) => {
                    format!("Apresentações em Unidades Internacionais não se aplicam a doses de miligrama por quilograma de massa corporal.")
                }
                Apresentacao::DoseVolume(qdte_med, vol_med, tipo_apres, nomes_comerciais) => {
                    qdte_med.em_mg();
                    vol_med.em_ml();
                    let mut dose =
                        dose_por_kg.em_mg() * massa_paciente * vol_med.em_ml() / qdte_med.em_mg();
                    // TODO:correto devolver ml? Tentar devolver na unidade de vol_med

                    match tipo_apres {
                        TipoApresentacao::Gotejador => {
                            dose = dose * 20.0; // de ml paga gotas
                            format!("{}: fazer {:.0}gotas.", via, dose)
                        }
                        _ => {
                            format!("{}: fazer {:.1}ml.", via, dose)
                        }
                    }
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
                        let ugotas = qtde_por_ugota.em_mg() * massa_paciente / dose_por_kg.em_mg();
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
                Apresentacao::DoseUI(qte, tipo_apres, _) => {
                    format!("Apresentações em Unidades Internacionais não se aplicam a doses de miligrama por quilograma de massa corporal.")
                }
                Apresentacao::DoseCompostaVolume(massas, volume, _, _) => {
                    let dose =
                        qtde_dose.em_mg() * massa_paciente * (volume.em_ml() / massas[0].em_mg())
                            / no_doses_interv;
                    format!("{:.1}ml", dose)
                }
                Apresentacao::DoseVolume(qtde_por_vol, volume, _, _) => {
                    let dose =
                        *qtde_dose * massa_paciente * (volume.em_ml() / qtde_por_vol.em_mg())
                            / no_doses_interv;
                    format!("{:.1}ml", dose)
                }
                Apresentacao::DoseAplicacao(aplicacao, _, _) => {
                    match aplicacao {
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
                        Aplicacao::Comprimido(qtde_por_cp) => {
                            let cps = (massa_paciente * qtde_dose.em_mg() / qtde_por_cp.em_mg())
                                / no_doses_interv;
                            format!("{:.1} comprimidos.", cps)
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
                    }
                }
            };
            format!(
                "{}: administrar {} {} por {}.",
                via.capitalizar(TipoCapitalizacao::Primeira),
                dose,
                frequencia,
                duracao
            )
        }
        Posologia::DoseUnica(_, qte, via) => match qte {
            Dose::UI(ui) => {
                format!("{}", ui)
            }
            Dose::Massa(qte) => {
                //ERROR: será que tá certo fazer cálculos aqui?! a dose é unica!
                qte.em_mg();
                let dose = match apresentacao {
                    Apresentacao::DoseUI(ui, _, _) => {
                        format!("Posologia em miligramas não se aplica apresentação em Unidades Internacionais")
                    }
                    Apresentacao::DoseVolume(massa_por_vol, volume, _, _) => {
                        format!("{}", *qte * volume.em_ml() / massa_por_vol.em_mg())
                    }
                    Apresentacao::DoseCompostaVolume(qtdes, volume, _, _) => {
                        format!("{}", *qte * volume.em_ml() / qtdes[0].em_mg())
                    }
                    Apresentacao::DoseAplicacao(aplcacao, _, _) => match aplcacao {
                        Aplicacao::Comprimido(qtde_por_cap) => {
                            format!("{} comprimidos", *qte / qtde_por_cap.em_mg())
                        }
                        Aplicacao::Capsula(qtde_por_cp) => {
                            format!("{} cápsulas", *qte / qtde_por_cp.em_mg())
                        }
                        Aplicacao::Jato(qtde_por_jato) => {
                            format!("{} jatos", *qte / qtde_por_jato.em_mg())
                        }
                        Aplicacao::Gota(qtde_por_gota) => {
                            format!("{} gotas", *qte / qtde_por_gota.em_mg())
                        }
                        Aplicacao::Microgota(qtde_por_ugota) => {
                            format!("{} microgotas", *qte / qtde_por_ugota.em_mg())
                        }
                    },
                };
                format!("{} uma única vez {}.", dose, via)
            }
        },
        Posologia::DoseDiaria(_, via, dose_diaria, no_doses) => {
            let tempo = 24 / no_doses;

            match apresentacao {
                Apresentacao::DoseUI(qte, _, _) => {
                    format!("{} uma vez ao dia", qte)
                }
                Apresentacao::DoseVolume(massa_dose, volume_dose, _, _) => {
                    let vol_dose = dose_diaria.em_mg() * volume_dose.em_ml() / massa_dose.em_mg();
                    let dose_div = vol_dose / *no_doses as f32;
                    if *no_doses == 1 {
                        format!("Administrar {:.1}ml uma vez ao dia", vol_dose)
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
                            format!("tomar {:.1} comprimido(s) por dia", comps)
                        } else {
                            format!("tomar {:.1} comprimidos a cada {}h", comps_div, tempo)
                        }
                    }
                    Aplicacao::Capsula(massa_cp) => {
                        let caps = dose_diaria.em_mg() / massa_cp.em_mg();
                        let caps_div = caps / *no_doses as f32;
                        if *no_doses == 1 {
                            format!("tomar {:.1} cápsula(s) por dia", caps)
                        } else {
                            format!("tomar {:.1} cápsula(s) a cada {}h", caps_div, tempo)
                        }
                    }
                    Aplicacao::Jato(massa_por_jato) => {
                        let jatos = dose_diaria.em_mg() / massa_por_jato.em_mg();
                        let jatos_div = jatos / *no_doses as f32;
                        if *no_doses == 1 {
                            format!("inalar {:.1} jato(s) a por dia", jatos)
                        } else {
                            format!("inalar {:.1} jato(s) a cada {}h", jatos_div, tempo)
                        }
                    }

                    Aplicacao::Gota(massa_por_gota) => {
                        let gotas = dose_diaria.em_mg() / massa_por_gota.em_mg();
                        let gotas_div = gotas / *no_doses as f32;
                        if *no_doses == 1 {
                            format!("tomar {:.1} jato(s) a por dia", gotas)
                        } else {
                            format!("tomar {:.1} jato(s) a cada {}h", gotas_div, tempo)
                        }
                    }

                    Aplicacao::Microgota(massa_por_microgota) => {
                        let microgotas = dose_diaria.em_mg() / massa_por_microgota.em_mg();
                        let microgotas_div = microgotas / *no_doses as f32;
                        if *no_doses == 1 {
                            format!("tomar {:.1} jato(s) a por dia", microgotas)
                        } else {
                            format!("tomar {:.1} jato(s) a cada {}h", microgotas_div, tempo)
                        }
                    }
                },
            }
        }
    }
}

// AesclepiusRx  Copyright (C) 2025  Jefferson T.
// Under Gnu General Licence 3.0 for ever.
// Any part of this program is and always have to be under the conditions of the LICENCE.txt
// file under the same repository.
// This program comes with ABSOLUTELY NO WARRANTY; for details type `show w'.
// This is free software, and you are welcome to redistribute it
// under certain conditions; type `show c' for details.
