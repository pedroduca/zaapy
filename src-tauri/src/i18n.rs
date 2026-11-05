//! The few strings Rust shows the user: the tray menu and notifications. The
//! settings page has its own dictionary (`src/lib/i18n.ts`); both follow the
//! language setting — by default the system's: Brazilian Portuguese for any `pt`
//! locale, English otherwise.

use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::OnceLock;

use zaapy_core::bridge::BridgeError;

const AUTO: u8 = 0;
const ENGLISH: u8 = 1;
const PORTUGUESE: u8 = 2;

/// The language the user picked in the settings; `AUTO` follows the system.
static CHOICE: AtomicU8 = AtomicU8::new(AUTO);

/// Apply the `language` setting (`auto`, `en` or `pt`).
pub fn set_language(code: &str) {
    let choice = match code {
        "en" => ENGLISH,
        "pt" => PORTUGUESE,
        _ => AUTO,
    };
    CHOICE.store(choice, Ordering::Relaxed);
}

fn portuguese() -> bool {
    match CHOICE.load(Ordering::Relaxed) {
        ENGLISH => false,
        PORTUGUESE => true,
        _ => {
            static SYSTEM_PT: OnceLock<bool> = OnceLock::new();
            *SYSTEM_PT.get_or_init(|| {
                sys_locale::get_locale()
                    .is_some_and(|locale| locale.to_lowercase().starts_with("pt"))
            })
        }
    }
}

fn pick(en: &str, pt: &str) -> String {
    if portuguese() { pt } else { en }.to_string()
}

pub fn settings() -> String {
    pick("Settings…", "Configurações…")
}

pub fn bridge_enabled() -> String {
    pick("Bridge enabled", "Ponte ativada")
}

pub fn multi_account() -> String {
    pick("Multi-account", "Multi-conta")
}

pub fn quit() -> String {
    pick("Quit Zaapy", "Sair do Zaapy")
}

pub fn update_to(version: &str) -> String {
    if portuguese() {
        format!("Atualizar para {version}…")
    } else {
        format!("Update to {version}…")
    }
}

pub fn update_available(version: &str) -> String {
    if portuguese() {
        format!("O Zaapy {version} está disponível. Instale pelo menu da bandeja.")
    } else {
        format!("Zaapy {version} is available. Install it from the tray menu.")
    }
}

pub fn update_failed(error: &dyn std::fmt::Display) -> String {
    if portuguese() {
        format!("A atualização falhou: {error}")
    } else {
        format!("The update failed: {error}")
    }
}

/// The core's errors in the user's language. Falls back to the core's own
/// English text, which is what the log keeps either way.
pub fn bridge_error(error: &BridgeError) -> String {
    if !portuguese() {
        return error.to_string();
    }
    match error {
        BridgeError::TargetNotFound => {
            "nenhuma janela do cliente Dofus selecionado está aberta".to_string()
        }
        BridgeError::TargetAmbiguous { count } => format!(
            "{count} janelas correspondem ao cliente selecionado; restrinja o filtro de título"
        ),
        BridgeError::FocusRejected { detail } => {
            format!("o sistema recusou trazer a janela do Dofus para a frente: {detail}")
        }
        BridgeError::FocusFailed { waited_ms } => format!(
            "a janela do Dofus não veio para a frente em {waited_ms} ms"
        ),
        BridgeError::InputFailed { detail } => {
            format!("o sistema recusou as teclas: {detail}")
        }
    }
}
