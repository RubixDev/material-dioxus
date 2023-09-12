use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/build/mwc-icon-button.js")]
extern "C" {
    #[derive(Debug)]
    type IconButton;

    #[wasm_bindgen(getter, static_method_of = IconButton)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(IconButton);

/// Props for [`MatIconButton`]
///
/// [MWC Documentation for properties](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/icon-button#propertiesattributes)
#[derive(Props)]
pub struct IconButtonProps<'a> {
    #[props(into)]
    pub label: Option<String>,
    #[props(into)]
    pub icon: Option<String>,
    #[props(default)]
    pub disabled: bool,
    #[props(default)]
    pub children: Element<'a>,
}

fn render<'a>(cx: Scope<'a, IconButtonProps<'a>>) -> Element<'a> {
    match &cx.props.children {
        Some(children) => render! {
            mwc-icon-button {
                "label": optional_string_attr!(cx.props.label),
                "icon": optional_string_attr!(cx.props.icon),
                "disabled": bool_attr!(cx.props.disabled),
                children
            }
        },
        None => render! {
            mwc-icon-button {
                "label": optional_string_attr!(cx.props.label),
                "icon": optional_string_attr!(cx.props.icon),
                "disabled": bool_attr!(cx.props.disabled),
            }
        }
    }
}

component!('a, MatIconButton, IconButtonProps, render, IconButton, "icon-button");
