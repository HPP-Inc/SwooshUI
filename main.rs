use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{window, Document, Element, HtmlElement};

// SwooshUI: A minimal MacOS-like file bar UI for browsers via WASM

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let document = window().unwrap().document().unwrap();
    inject_swooshui_styles(&document)?;
    let bar = create_swooshui_bar(&document)?;
    document.body().unwrap().append_child(&bar)?;
    Ok(())
}

fn inject_swooshui_styles(document: &Document) -> Result<(), JsValue> {
    let style = document.create_element("style")?;
    style.set_inner_html(r#"
        .swooshui-bar {
            display: flex;
            align-items: center;
            background: rgba(245,245,247,0.95);
            box-shadow: 0 2px 8px rgba(0,0,0,0.07);
            height: 38px;
            padding: 0 18px;
            border-radius: 0 0 12px 12px;
            font-family: 'San Francisco', 'Segoe UI', Arial, sans-serif;
            font-size: 15px;
            user-select: none;
            position: fixed;
            top: 0; left: 0; right: 0;
            z-index: 1000;
            backdrop-filter: blur(8px);
        }
        .swooshui-logo {
            width: 22px; height: 22px;
            margin-right: 16px;
            border-radius: 6px;
            background: linear-gradient(135deg, #5e5ce6 60%, #a5aaff 100%);
            display: flex; align-items: center; justify-content: center;
            color: white; font-weight: bold; font-size: 17px;
        }
        .swooshui-menu {
            display: flex; gap: 18px;
        }
        .swooshui-menu-item {
            padding: 4px 10px;
            border-radius: 6px;
            cursor: pointer;
            transition: background 0.15s;
        }
        .swooshui-menu-item:hover,
        .swooshui-menu-item.active {
            background: rgba(120,120,130,0.13);
        }
        .swooshui-clock {
            margin-left: auto;
            font-variant-numeric: tabular-nums;
            color: #444;
            font-size: 15px;
            letter-spacing: 0.5px;
        }
    "#);
    document.head().unwrap().append_child(&style)?;
    Ok(())
}

fn create_swooshui_bar(document: &Document) -> Result<Element, JsValue> {
    let bar = document.create_element("nav")?;
    bar.set_class_name("swooshui-bar");

    let logo = document.create_element("div")?;
    logo.set_class_name("swooshui-logo");
    logo.set_inner_html("🌀");

    let menu = document.create_element("div")?;
    menu.set_class_name("swooshui-menu");

    let items = ["File", "Edit", "View", "Go", "Window", "Help"];
    for &item in &items {
        let menu_item = document.create_element("div")?;
        menu_item.set_class_name("swooshui-menu-item");
        menu_item.set_inner_html(item);

        // Add click event for highlighting and logging
        {
            let menu_item = menu_item.clone();
            let menu = menu.clone();
            let item_str = item.to_string();
            let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
                // Remove 'active' from all menu items
                let children = menu.children();
                for i in 0..children.length() {
                    if let Some(child) = children.item(i) {
                        child.set_class_name("swooshui-menu-item");
                    }
                }
                // Set 'active' on clicked item
                menu_item.set_class_name("swooshui-menu-item active");
                // Log to console
                web_sys::console::log_1(&format!("Clicked: {}", item_str).into());
            }) as Box<dyn FnMut(_)>);
            menu_item
                .dyn_ref::<HtmlElement>()
                .unwrap()
                .set_onclick(Some(closure.as_ref().unchecked_ref()));
            closure.forget();
        }

        menu.append_child(&menu_item)?;
    }

    // Add a clock to the right
    let clock = document.create_element("div")?;
    clock.set_class_name("swooshui-clock");
    clock.set_inner_html("--:--:--");
    bar.append_child(&logo)?;
    bar.append_child(&menu)?;
    bar.append_child(&clock)?;

    // Start clock update interval
    {
        let clock = clock.dyn_into::<web_sys::HtmlElement>()?;
        let closure = Closure::wrap(Box::new(move || {
            let now = js_sys::Date::new_0();
            let h = now.get_hours();
            let m = now.get_minutes();
            let s = now.get_seconds();
            let time = format!("{:02}:{:02}:{:02}", h, m, s);
            clock.set_inner_text(&time);
        }) as Box<dyn FnMut()>);

        window().unwrap().set_interval_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            1000,
        )?;
        closure.forget();
    }

    Ok(bar)
}