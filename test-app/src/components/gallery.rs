use enum_iterator::IntoEnumIterator;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::window;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Properties, PartialEq)]
pub struct GalleryProps {
    pub query: String,
}

#[function_component]
pub fn Gallery(props: &GalleryProps) -> Html {
    let initial_icons = use_memo(|_| IconId::into_enum_iter().collect::<Vec<IconId>>(), ());
    let icons = use_memo(
        |query| {
            initial_icons
                .iter()
                .filter(|icon_id| {
                    let title = format!("{:?}", icon_id);

                    query.to_ascii_lowercase().split(' ').all(|word| {
                        title
                            .to_ascii_lowercase()
                            .contains(&word.to_ascii_lowercase())
                    })
                })
                .cloned()
                .collect::<Vec<IconId>>()
        },
        props.query.clone(),
    );

    if icons.is_empty() {
        return html! {
            <div class="no-icons">{"No Icons Found"}</div>
        };
    }

    html! {
        <div class="gallery">
            <>
                {icons.iter().cloned().map(|icon_id| {
                html_nested! {
                    <GalleryItem {icon_id}/>
                }
            }).collect::<Html>()}
            </>
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct GalleryItemProps {
    icon_id: IconId,
}

#[function_component]
fn GalleryItem(props: &GalleryItemProps) -> Html {
    let icon_id = props.icon_id;
    let title = format!("{:?}", icon_id);
    let icon_name = title.clone();
    let timeout_ref = use_mut_ref(|| None);
    let show_copied = use_state(|| false);

    let onclick = {
        let show_copied = show_copied.clone();
        let window = window().unwrap();
        window.navigator().clipboard().map(|clipboard| {
            Callback::from(move |_: MouseEvent| {
                log::info!("clicked {:?}", icon_id);
                let _ = clipboard.write_text(&format!("{:?}", icon_id));
                show_copied.set(true);
            })
        })
    };

    use_effect_with_deps(
        {
            let show_copied = show_copied.clone();
            move |show| {
                let window = window().unwrap();
                if *show {
                    let closure = Closure::<dyn FnMut()>::new(move || {
                        show_copied.set(false);
                    });

                    let id = window
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            closure.as_ref().unchecked_ref(),
                            1000, // disappear after 1s
                        )
                        .unwrap();

                    *timeout_ref.borrow_mut() = Some(id);
                    closure.forget();
                }

                move || {
                    if let Some(id) = timeout_ref.borrow_mut().take() {
                        window.clear_timeout_with_handle(id);
                    }
                }
            }
        },
        *show_copied,
    );

    html! {
        <div class="icon">
            <Icon
            {title}
            {icon_id}
            width={"24"}
            height={"24"}
            onclick={onclick}
            />
            <p class="icon-name">{icon_name}</p>

            if *show_copied {
                <div class="copied-tooltip">
                    {"Copied"}
                </div>
            }
        </div>
    }
}
