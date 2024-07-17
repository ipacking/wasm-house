//! wasm-pack build --dev --target web -d ../vue/src/wasm/slint

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn add(left: isize, right: isize) -> isize {
    left + right
}

#[wasm_bindgen(start)]
pub fn start() {
    let win = MainWindow::new().unwrap();
    win.run().unwrap();
}

slint::slint! {
    import { VerticalBox, TextEdit, Button } from "std-widgets.slint";
    export component MainWindow inherits Window {
        title: "slint";
        width: 500px;
        height: 400px;

        VerticalBox {
            spacing: 20px;
            Text {
                text: "input text ...";
            }

            te := TextEdit {

            }

            tx := Text{

            }

            Button {
                text: "execute";
                clicked => {
                    tx.text = te.text;
                }
            }
        }
    }
}
