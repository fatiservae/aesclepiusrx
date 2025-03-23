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
    // let mut massa_paciente = match massa_paciente.normalizar() {
    //     Massa::Kg(valor) => valor.0 * 1000000.0,
    //     Massa::G(valor) => valor.0 / 1000.0,
    //     Massa::Mg(valor) => valor.0,
    //     Massa::Mcg(valor) => valor.0 / 1000.0,
    // };

    match posologia {
        Posologia::DoseKg(via, dose_por_kg) => match apresentacao {
            Apresentacao::DoseVolume(qdte_med, mut vol_med) => {
                dose_por_kg.normalizar();
                vol_med.normalizar();
                // let vol_med = match vol_med
                let dose = (vol_med.valor() / qdte_med.valor())
                    * dose_por_kg.valor()
                    * massa_paciente.valor();
                format!(
                    "{}: {:.}ml agora.",
                    via.capitalizar(TipoCapitalizacao::Primeira),
                    dose // correto devolver ml? Tentar devolver na unidade de vol_med
                )
            }
            Apresentacao::DoseAplicacao(Aplicacao::Gota(valor)) => {
                let dose = match dose_por_kg {
                    Massa::Mg(valor) => Massa::Mg(Float(&massa_paciente.valor() * valor.0)),
                    _ => todo!(),
                };
                let dosagem = match dose {
                    Massa::Mg(mass) => mass.0,
                    _ => todo!(),
                };
                let gotas = valor.valor() / dosagem;
                format!("Tomar {} gotas diluídas", gotas)
            }
            Apresentacao::DoseAplicacao(Aplicacao::Jato(Massa::Mcg(massa_medicamento))) => {
                let dose_necessaria = match dose_por_kg {
                    Massa::Mg(valor) => {
                        Massa::Mg(Float(&massa_paciente.valor() * massa_medicamento.0))
                    }
                    _ => todo!(),
                };
                let jatos = dose_necessaria.valor() / massa_medicamento.0;
                format!("Inalar {} jatos.", jatos)
            }
            _ => todo!(),
        },
        Posologia::DoseKgIntervaloDuracao(via, massa, intervalo, duracao, frequencia) => {
            let qtde = match massa {
                Massa::Mg(quantidade) => Massa::Mg(Float(massa_paciente.valor() * quantidade.0)),
                _ => todo!(),
            };
            let vol_dose = match apresentacao {
                Apresentacao::DoseVolume(quanto, volume) => {
                    format!(
                        "{:.1}ml",
                        (qtde.valor() * volume.valor() / quanto.valor())
                            / (24.00 / frequencia.valor() as f32)
                    )
                    // TODO: checkar tipos e unidades
                }
                _ => todo!(),
            };
            format!(
                "{}: tomar {} {} por {}.",
                via.capitalizar(TipoCapitalizacao::Primeira),
                vol_dose,
                frequencia,
                duracao
            )
        }
        Posologia::GotaKg(gotas) => {
            let dose = *gotas as f32 * massa_paciente.valor();
            let mut plural = "";
            if dose > 1.0 {
                plural = "s"
            }
            format!("Tomar {} gota{}", dose, plural) // errado...
        }
        Posologia::DoseUnica(dose, via) => match via {
            Via::Oral => match apresentacao {
                _ => format!("Tomar {} uma única vez.", dose),
            },
            Via::Intravenosa => format!("Fazer {} uma única vez.", dose),
            Via::Inalatoria => match apresentacao {
                Apresentacao::DoseVolume(massa, volume) => {
                    let vol = match volume {
                        Volume::Ml(vol) => vol.0,
                        _ => todo!(),
                    };
                    let mass = match massa {
                        Massa::Mg(mass) => mass.0,
                        _ => todo!(),
                    };
                    let dosagem = match dose {
                        Massa::Mg(mass) => mass.0,
                        _ => todo!(),
                    };
                    let dose_final = vol * dosagem / mass;
                    format!("Inalar {} uma única vez.", dose_final)
                }
                Apresentacao::DoseAplicacao(aplicacao) => match aplicacao {
                    Aplicacao::Comprimido(quantidade) => {
                        format!("Tomar {} comprimidos uma única vez.", quantidade)
                    }
                    Aplicacao::Jato(quantidade) => {
                        let dosagem = match dose {
                            Massa::Mg(mass) => mass.0,
                            _ => todo!(),
                        };
                        let quant = match quantidade {
                            Massa::Mg(quant) => quant.0,
                            Massa::Mcg(quant) => quant.0,
                            _ => todo!(),
                        };
                        let jatos = quant / dosagem;

                        format!("Fazer {} uma única vez.", quantidade)
                    }
                    _ => todo!(),
                },
            },
            Via::Retal | Via::Topica => format!("Aplicar {} uma única vez.", dose),
        },
    }
}
