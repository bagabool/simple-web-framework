use leptos_reactive::{create_signal, SignalGet, SignalUpdate};
use rust_web_framework::{El, mount, User};


fn main() {
    mount(|cx| {
        let (count, set_count) = create_signal(cx, 0);

        El::new("div")
            .child(
                El::new("button")
                    .on("click", move |_| set_count.update(|x| *x += 1))
                    .attr("id", "my-button")
                    .text("Coucou")
            )
            .child(
                El::new("p")
                    .dyn_text(cx, move || count.get().to_string())
            )
            .child(
                El::new("table")
                    .resource(User::list())
            )
    }
    )
}
