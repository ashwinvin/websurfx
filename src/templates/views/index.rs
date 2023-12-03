//! A module that handles the view for the index/home/main page in the `anvesh` frontend.

use maud::{html, Markup, PreEscaped};

use crate::templates::partials::{bar::bar, footer::footer, header::header};

/// A function that handles the html code for the index/html/main page view in the search engine frontend.
///
/// # Arguments
///
/// * `colorscheme` - It takes the colorscheme name as an argument.
/// * `theme` - It takes the theme name as an argument.
///
/// # Returns
///
/// It returns the compiled html markup code as a result.
pub fn index(colorscheme: &str, theme: &str) -> Markup {
    html!(
        (header(colorscheme, theme))
        main class="search-container"{
            img src="../images/anvesh_logo.png" alt="Websurfx meta-search engine logo";
            (bar(&String::default()))
            (PreEscaped("</div>"))
        }
        script src="static/index.js"{}
        (footer())
    )
}
