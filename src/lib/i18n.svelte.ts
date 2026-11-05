// Two languages: Brazilian Portuguese and English, picked by the user or, by
// default, from the system — any `pt` locale gets Portuguese. The panel is small
// enough that a dictionary is all it needs — no library, no message format.

const dictionaries = {
  en: {
    bridgeEnabled: "Bridge enabled",
    selectWindows: "Select the Ganymède and Dofus windows below.",
    sendingTo: "Sending to {title}",
    grantPermission: "Grant permission…",
    ganymedeWindow: "Ganymède window",
    notSelected: "Not selected",
    notRunning: "{name} (not running)",
    dofusWindow: "Dofus window",
    commands: "Commands",
    items: "Items",
    itemsHint:
      "An item name clicked in Ganymède is pasted into Dofus as is — into the search field you have open, without opening the chat",
    chatKey: "Dofus chat key",
    chatKeyHint:
      "The key that opens the Dofus chat. Zaapy presses it before pasting the command, so it has to be the one bound in the game's controls — Enter unless you moved it.",
    pressKey: "Press a key…",
    unsupportedKey: "Unsupported key",
    unsupportedKeyHint:
      "Zaapy can send Enter, Escape, Tab, Space, Backspace, Delete, and any letter or digit. It has no key code for the rest on both Windows and macOS.",
    escapeToCancel: "Escape to cancel",
    afterSending: "After sending",
    afterSendingHint:
      "Dofus keeps the chat focused after a command, so your next keystrokes land in it. Pressing Escape hands the keyboard back to the game.",
    pressEscape: "Press Escape",
    accounts: "Accounts",
    accountsHint:
      "Send /travel and /zaap to every open Dofus window, your selected character last. Item names still go to the selected character only.",
    multiAccount: "Multi-account",
    sendTo: "Send to",
    noOtherClients: "No other Dofus window is open",
    language: "Language",
    languageHint: "The language of this window and of the tray menu.",
    languageAuto: "System",
    openLogFolder: "Open log folder",
  },
  pt: {
    bridgeEnabled: "Ponte ativada",
    selectWindows: "Selecione as janelas do Ganymède e do Dofus abaixo.",
    sendingTo: "Enviando para {title}",
    grantPermission: "Conceder permissão…",
    ganymedeWindow: "Janela do Ganymède",
    notSelected: "Não selecionada",
    notRunning: "{name} (não está aberto)",
    dofusWindow: "Janela do Dofus",
    commands: "Comandos",
    items: "Itens",
    itemsHint:
      "O nome de um item clicado no Ganymède é colado no Dofus como está — no campo de busca que estiver aberto, sem abrir o chat",
    chatKey: "Tecla do chat",
    chatKeyHint:
      "A tecla que abre o chat do Dofus. O Zaapy a aperta antes de colar o comando, então precisa ser a que está configurada nos atalhos do jogo — Enter, a menos que você tenha trocado.",
    pressKey: "Aperte uma tecla…",
    unsupportedKey: "Tecla não suportada",
    unsupportedKeyHint:
      "O Zaapy envia Enter, Escape, Tab, Espaço, Backspace, Delete e qualquer letra ou número. Não há código para as demais teclas no Windows e no macOS ao mesmo tempo.",
    escapeToCancel: "Esc para cancelar",
    afterSending: "Depois de enviar",
    afterSendingHint:
      "O Dofus mantém o chat focado depois de um comando, então as próximas teclas caem nele. O Escape devolve o teclado ao jogo.",
    pressEscape: "Apertar Escape",
    accounts: "Contas",
    accountsHint:
      "Envia /travel e /zaap para todas as janelas do Dofus abertas, com o personagem selecionado por último. Nomes de itens vão só para o personagem selecionado.",
    multiAccount: "Multi-conta",
    sendTo: "Enviar para",
    noOtherClients: "Nenhuma outra janela do Dofus está aberta",
    language: "Idioma",
    languageHint: "O idioma desta janela e do menu da bandeja.",
    languageAuto: "Sistema",
    openLogFolder: "Abrir pasta de logs",
  },
} as const;

export type MessageKey = keyof (typeof dictionaries)["en"];

export type Language = "auto" | "en" | "pt";

const system: keyof typeof dictionaries = navigator.language.toLowerCase().startsWith("pt")
  ? "pt"
  : "en";

// Reactive: a component that calls `t` re-renders when the language changes.
const state = $state<{ choice: Language }>({ choice: "auto" });

export function setLanguage(choice: string) {
  state.choice = choice === "en" || choice === "pt" ? choice : "auto";
}

/// A message in the chosen language, with `{name}` placeholders filled in.
export function t(key: MessageKey, values: Record<string, string> = {}): string {
  const language = state.choice === "auto" ? system : state.choice;
  return dictionaries[language][key].replace(/\{(\w+)\}/g, (_, name) => values[name] ?? "");
}
