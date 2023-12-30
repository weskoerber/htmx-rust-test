use maud::{html, Markup};

pub fn page(content: Markup) -> Markup {
    html! {
        html {
            head {
                meta charset="UTF-8" {}
                meta name="viewport" content="width=device-width, initial-scale=1.0" {}
                link rel="stylesheet" href="/css/tailwind.css" {}
                script src="/js/htmx.org/dist/htmx.min.js" {}
            }
            body {
                (content)
            }
        }
    }
}
