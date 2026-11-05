//! The few strings Rust shows the user: the tray menu and notifications. The
//! settings page has its own dictionary (`src/lib/i18n.svelte.ts`); both follow
//! the language setting — by default the system's: any `pt` or `fr` locale gets
//! its own language, everything else English.

use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::OnceLock;

use zaapy_core::bridge::BridgeError;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Lang {
    English,
    Portuguese,
    French,
}

const AUTO: u8 = 0;

/// The language the user picked in the settings; `AUTO` follows the system.
static CHOICE: AtomicU8 = AtomicU8::new(AUTO);

/// Apply the `language` setting (`auto`, `en`, `pt` or `fr`).
pub fn set_language(code: &str) {
    let choice = match code {
        "en" => 1,
        "pt" => 2,
        "fr" => 3,
        _ => AUTO,
    };
    CHOICE.store(choice, Ordering::Relaxed);
}

fn lang() -> Lang {
    match CHOICE.load(Ordering::Relaxed) {
        1 => Lang::English,
        2 => Lang::Portuguese,
        3 => Lang::French,
        _ => {
            static SYSTEM: OnceLock<Lang> = OnceLock::new();
            *SYSTEM.get_or_init(|| {
                let locale = sys_locale::get_locale().unwrap_or_default().to_lowercase();
                if locale.starts_with("pt") {
                    Lang::Portuguese
                } else if locale.starts_with("fr") {
                    Lang::French
                } else {
                    Lang::English
                }
            })
        }
    }
}

fn pick(en: &str, pt: &str, fr: &str) -> String {
    match lang() {
        Lang::English => en,
        Lang::Portuguese => pt,
        Lang::French => fr,
    }
    .to_string()
}

pub fn settings() -> String {
    pick("Settings…", "Configurações…", "Paramètres…")
}

pub fn bridge_enabled() -> String {
    pick("Bridge enabled", "Ponte ativada", "Pont activé")
}

pub fn multi_account() -> String {
    pick("Multi-account", "Multi-conta", "Multi-compte")
}

pub fn quit() -> String {
    pick("Quit Zaapy", "Sair do Zaapy", "Quitter Zaapy")
}

pub fn update_to(version: &str) -> String {
    pick(
        &format!("Update to {version}…"),
        &format!("Atualizar para {version}…"),
        &format!("Mettre à jour vers {version}…"),
    )
}

pub fn update_available(version: &str) -> String {
    pick(
        &format!("Zaapy {version} is available. Install it from the tray menu."),
        &format!("O Zaapy {version} está disponível. Instale pelo menu da bandeja."),
        &format!(
            "Zaapy {version} est disponible. Installez-le depuis le menu de la zone de notification."
        ),
    )
}

pub fn update_failed(error: &dyn std::fmt::Display) -> String {
    pick(
        &format!("The update failed: {error}"),
        &format!("A atualização falhou: {error}"),
        &format!("La mise à jour a échoué : {error}"),
    )
}

/// The core's errors in the user's language. English is the core's own text,
/// which is also what the log keeps.
pub fn bridge_error(error: &BridgeError) -> String {
    match (lang(), error) {
        (Lang::English, _) => error.to_string(),

        (Lang::Portuguese, BridgeError::TargetNotFound) => {
            "nenhuma janela do cliente Dofus selecionado está aberta".to_string()
        }
        (Lang::Portuguese, BridgeError::TargetAmbiguous { count }) => format!(
            "{count} janelas correspondem ao cliente selecionado; restrinja o filtro de título"
        ),
        (Lang::Portuguese, BridgeError::FocusRejected { detail }) => {
            format!("o sistema recusou trazer a janela do Dofus para a frente: {detail}")
        }
        (Lang::Portuguese, BridgeError::FocusFailed { waited_ms }) => {
            format!("a janela do Dofus não veio para a frente em {waited_ms} ms")
        }
        (Lang::Portuguese, BridgeError::InputFailed { detail }) => {
            format!("o sistema recusou as teclas: {detail}")
        }

        (Lang::French, BridgeError::TargetNotFound) => {
            "aucune fenêtre du client Dofus sélectionné n'est ouverte".to_string()
        }
        (Lang::French, BridgeError::TargetAmbiguous { count }) => format!(
            "{count} fenêtres correspondent au client sélectionné ; affinez le filtre de titre"
        ),
        (Lang::French, BridgeError::FocusRejected { detail }) => format!(
            "le système a refusé de mettre la fenêtre Dofus au premier plan : {detail}"
        ),
        (Lang::French, BridgeError::FocusFailed { waited_ms }) => {
            format!("la fenêtre Dofus n'est pas passée au premier plan en {waited_ms} ms")
        }
        (Lang::French, BridgeError::InputFailed { detail }) => {
            format!("le système a refusé les touches : {detail}")
        }
    }
}
