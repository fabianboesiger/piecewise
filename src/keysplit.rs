use dioxus::prelude::*;
use lucide_dioxus::{KeyRound, Puzzle, Split, Merge, Trash};

use super::bytearray::ByteArray;

#[derive(Debug, Copy, Clone)]
enum Mode {
    Split,
    Merge,
}

#[derive(Debug, Copy, Clone)]
enum Error {
    NonMatchingLengths(usize),
    InvalidHex(usize),
    InvalidMergedKey,
    InvalidInputKey,
}

fn compute(mut key: Signal<String>, mut values: Signal<Vec<String>>, mode: Signal<Mode>, mut err: Signal<Option<Error>>) {
    match *mode.read() {
        Mode::Split => {
            let len = values.len();

            if len < 2 {
                return;
            }

            let bytes_size = key.read().as_bytes().len();

            if key.read().as_bytes().iter().any(|&b| b < 32 || b > 126) {
                err.set(Some(Error::InvalidInputKey));
                return;
            }

            let mut output = Vec::new();
            let mut xored = ByteArray::from_string(&*key.read());
            for _ in 0..(len - 1) {
                let random_bytes = ByteArray::random(bytes_size);
                xored = xored.xor(&random_bytes);
                output.push(random_bytes.to_hex());
            }
            output.push(xored.to_hex());
            values.set(output);
        }
        Mode::Merge => {
            if values.len() < 2 {
                return;
            }

            let bytes_size = values.read()[0].as_bytes().len();
            let mut combined = ByteArray::zero(bytes_size);

            for (i, value) in values.read().iter().enumerate() {
                if value.as_bytes().len() % 2 == 1 {
                    err.set(Some(Error::InvalidHex(i)));
                    key.set("".to_string());
                    return;
                }

                if value.as_bytes().len() != bytes_size {
                    err.set(Some(Error::NonMatchingLengths(i)));
                    key.set("".to_string());
                    return;
                }

                let part = ByteArray::from_hex(value);

                if let Ok(part) = part {
                    combined = combined.xor(&part);
                } else {
                    err.set(Some(Error::InvalidHex(i)));
                    key.set("".to_string());
                    return;
                }
            }

            let output_key = combined.to_string();

            if let Ok(output_key) = output_key {
                if output_key.as_bytes().iter().any(|&b| b < 32 || b > 126) {
                    err.set(Some(Error::InvalidMergedKey));
                    key.set("".to_string());
                    return;
                }

                key.set(output_key);
            } else {
                err.set(Some(Error::InvalidMergedKey));
                key.set("".to_string());
                return;
            }
        }
    }
    err.set(None);
}

#[component]
pub fn KeySplit() -> Element {
    let mut values = use_signal(|| vec!["".to_string(), "".to_string()]);
    let mut key = use_signal(|| "".to_string());
    let mut mode: Signal<Mode> = use_signal(|| Mode::Split);
    let err = use_signal(|| None);

    rsx! {
        h1 { "Piecewise" }
        p { "Securely share a key through multiple channels." }

        fieldset { 
            legend { "Key" },
            p { class: "info", "Paste your key below to split it into multiple pieces. Each piece on its own will leak no information about the key." }
            div {
                class: "field key",
                div {
                    class: "row",
                    span {
                        class: "icon input-icon",
                        KeyRound {
                            color: if matches!(*err.read(), Some(Error::InvalidInputKey | Error::InvalidMergedKey)) { "var(--error-color)" } else { "var(--text-color)" },
                            size: 24,
                        }
                    }
                    input {
                        id: "key",
                        value: "{key}",
                        oninput: move |event| {
                            key.set(event.value().to_string());
                            mode.set(Mode::Split);
                            compute(key, values, mode, err);
                        }
                    }
                }
                {match *err.read() {
                    Some(Error::InvalidInputKey) => rsx! {
                        span { class: "error", "This key contains characters that are not supported" }
                    },
                    Some(Error::InvalidMergedKey) => rsx! {
                        span { class: "error", "Could not generate a valid merged key" }
                    },
                    _ => rsx! {}
                }}
               
            }
        }

        div {
            class: "animated-icon-wrapper",
            span {
                class: if let Mode::Split = *mode.read() { "visible animated-icon" } else { "hidden animated-icon" },
                id: "icon-split",
                Split {
                    color: if err.read().is_some() { "var(--error-color)" } else { "var(--text-color)" },
                    size: 24,
                }
            }
            span {
                class: if let Mode::Merge = *mode.read() { "visible animated-icon" } else { "hidden animated-icon" },
                id: "icon-merge",
                Merge {
                    color: if err.read().is_some() { "var(--error-color)" } else { "var(--text-color)" },
                    size: 24,
                }
            }
        }

        fieldset {
            class: "values",
            legend { "Pieces" },
            p { class: "info", "Share the pieces through separate channels. The original key can only be reconstructed if all pieces are known." }
            for (index, value) in values.read().iter().enumerate() {
                div {
                    class: "field value",
                    key: "{index}", 
                    div {
                        class: "row",
                        span {
                            class: "icon input-icon",
                            Puzzle {
                                color: if matches!(*err.read(), Some(Error::InvalidHex(i) |Error::NonMatchingLengths(i)) if i == index) { "var(--error-color)" } else { "var(--text-color)" },
                                size: 24,
                            }
                        }
                        input {
                            id: "value-{index}",
                            value: "{value}",
                            oninput: move |event| {
                                values.write()[index] = event.value().to_string();
                                mode.set(Mode::Merge);
                                compute(key, values, mode, err);
                            }
                        }
                        button {
                            class: "secondary icon",
                            onclick: move |_| {
                                if values.len() <= 2 {
                                    return;
                                }
                                values.write().remove(index);
                                compute(key, values, mode, err);
                            },
                            Trash {
                                color: "var(--background-color)",
                                size: 24,
                            }
                        }
                    }
                    { match *err.read() {
                        Some(Error::InvalidHex(i)) if i == index => rsx! {
                            span { class: "error", "Invalid key piece" }
                        },
                        Some(Error::NonMatchingLengths(i)) if i == index => rsx! {
                            span { class: "error", "This piece does not match the length of the first piece" }
                        },
                        _ => rsx! {}
                    }}
                }
            }
            button {
                class: "secondary",
                onclick: move |_| {
                    values.write().push("".to_string());
                    compute(key, values, mode, err);
                },
                "Add Part"
            }
        }
    }
}
