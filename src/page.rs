use maud::{Markup, html};
use rocket::{Route, response::content};

use crate::test_table;

pub fn page(markup: Markup) -> Markup {
    html! {
       html color-mode="user" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                meta name="description" content="FreeLunch";
                ({frontend::resources()})
                ({title("Free Lunch")})
            }

            body {
                (markup)
        }
       }
    }
}

fn title(title: impl Into<String>) -> Markup {
    html! {
    title { ({title.into()}) }
    }
}

pub mod frontend {
    use maud::{Markup, PreEscaped, html};

    const HTMX: &str = r#"<script src="/_assets/htmx.js"></script>"#;
    const ACTIVE_TABLE: &str = r#"<script type="module" src="/_assets/active-table.js"></script>"#;
    const FAVICON: &str = r#"<link rel="icon" href="/_assets/favicon.ico" type="image/x-icon">"#;
    const CSS: &str = r#"<link rel="stylesheet" href="/_assets/mvp.css">"#;

    pub fn resources() -> Markup {
        html! {
        (PreEscaped(HTMX))
        (PreEscaped(ACTIVE_TABLE))
        (PreEscaped(FAVICON))
        (PreEscaped(CSS))
           }
    }
}

#[get("/")]
pub fn body() -> content::RawHtml<String> {
    content::RawHtml(page(body_m()).into_string())
}

fn body_m() -> Markup {
    html! {
    body {
         header {
             (navigation())
         }
         main id="main_target"{
             section id="tools"{
                 header{
                        h1{"TEST"}
                        (test_table::table())

                 }
             }

         }

        }
    }
}

pub fn navigation() -> Markup {
    html! {

        nav {
                a href="/" {
                    img src="./_assets/planet.svg" height="150";
                }
                ul {
                    li {
                            a href="/" {
                                button{
                                     "Dashboard"
                                }
                            }

                    }
                    li {
                        a href="about" {
                            button{
                               "About"
                            }
                        }

                    }

                }
            }

    }
}

pub fn api() -> Vec<Route> {
    routes![body]
}
