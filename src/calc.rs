use crate::*;

pub fn calcular_dose(massa_paciente: Massa, posologia: &Posologia, apresentacao: &Apresentacao) -> String {
    let massa_paciente_final = match massa_paciente {
        Massa::Kg(valor) => valor.0,
        Massa::G(valor) => valor.0/1000.0,
        Massa::Mg(_) | Massa::Mcg(_) => 0.0,
    };
    match posologia {
        Posologia::DoseKgIntervaloDuracao(massa, intervalo, duracao, via) => {
            let dose = massa_paciente.valor() * massa.valor();
            format!("{}: tomar {} a cada {} por {}.", via, dose, intervalo, duracao)
        },
        Posologia::GotaKg(gotas) => {
            let dose = *gotas as f32;
            format!("Tomar {}", dose*massa_paciente_final) // errado...
        },
        Posologia::DoseUnica(dose) => {
            format!("Tomar {} uma única vez.", dose) //melhorar tipo com via
        }
    }
}
