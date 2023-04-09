#[macro_use]
extern crate dotenv_codegen;

use std::ops::Deref;
use web_sys::{self, Element, Event, window, Node};
use leptos_reactive::{create_effect, create_runtime, create_scope, Scope};
use wasm_bindgen::{self, JsCast};
use sqlx::postgres::PgPoolOptions;
use sqlx::{self};
use dotenv_codegen::dotenv;

#[derive(sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
}

impl User {
    #[tokio::main]
    pub async fn list() -> Result<Vec<User>, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(dotenv!("DATABASE_URL"))
            .await?;

        let users = sqlx::query_as::<_, User>("Select * FROM users")
            .fetch_all(&pool)
            .await?;

        Ok(users)
    }
}

pub fn mount(f: impl FnOnce(Scope) -> El + 'static) {
    let runtime = create_runtime();
    _ = create_scope(runtime, |cx| {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let root = f(cx);

        body.append_child(&root).unwrap();
    });
}

#[derive(Debug, Clone)]
pub struct El(Element);

impl Deref for El {
    type Target = Element;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl El {
    pub fn new(tag_name: &str) -> Self {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let el = document.create_element(tag_name).unwrap();
        Self(el)
    }

    pub fn on(self, event_name: &str, cb: impl FnMut(Event) + 'static) -> Self {
        use wasm_bindgen::prelude::Closure;

        let cb = Closure::wrap(Box::new(cb) as Box<dyn FnMut(Event)>);
        self.0.add_event_listener_with_callback(
            event_name,
            cb.as_ref().unchecked_ref(),
        ).unwrap();
        cb.forget();
        self
    }

    pub fn attr(self, attr_name: &str, value: &str) -> Self {
        self.0.set_attribute(attr_name, value);
        self
    }

    pub fn text(self, data: &str) -> Self {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let node = document.create_text_node(data);
        self.0.append_child(&node).unwrap();
        self
    }

    pub fn child(self, child: El) -> Self {
        self.0.append_child(&child).unwrap();
        self
    }

    pub fn push_child(&self, child: El) -> Node {
        self.0.append_child(&child).unwrap()
    }

    pub fn dyn_text(self, cx: Scope, f: impl Fn() -> String + 'static) -> Self {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let node = document.create_text_node("");

        self.0.append_child(&node).unwrap();

        create_effect(cx, move |_| {
            let value = f();
            node.set_data(&value);
        });

        self
    }

    pub fn resource(self, list: Result<Vec<User>, sqlx::Error>) -> Self {
        let th_id = Self::new("th").text("Id");
        let th_username = Self::new("th").text("Username");
        let headers = Self::new("tr").child(th_id).child(th_username);
        let rows = Self::new("tr");

        for user in list.unwrap() {
            let td_id = Self::new("td").text(&user.id.to_string());
            let td_username = Self::new("td").text(&user.username.to_string());
            rows.push_child(td_id);
            rows.push_child(td_username);
        }

        self.child(headers).child(rows)
    }
}