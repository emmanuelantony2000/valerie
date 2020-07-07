/// `div` element
#[macro_export]
macro_rules! div {
    ( $( $x:expr ),* ) => {
        {
            let mut div = $crate::Tag::<web_sys::Element>::new("div");
            $(
                div = div.push($x);
            )*
            div
        }
    };
}

/// `p` element
#[macro_export]
macro_rules! p {
    ( $( $x:expr ),* ) => {
        {
            let mut p = $crate::Tag::<web_sys::Element>::new("p");
            $(
                p = p.push($x);
            )*
            p
        }
    };
}

/// `button` element
#[macro_export]
macro_rules! button {
    ( $( $x:expr ),* ) => {
        {
            let mut button = $crate::Tag::<web_sys::Element>::new("button");
            $(
                button = button.push($x);
            )*
            button
        }
    };
}

/// `h1` element
#[macro_export]
macro_rules! h1 {
    ( $( $x:expr ),* ) => {
        {
            let mut h1 = $crate::Tag::<web_sys::Element>::new("h1");
            $(
                h1 = h1.push($x);
            )*
            h1
        }
    };
}

/// `h2` element
#[macro_export]
macro_rules! h2 {
    ( $( $x:expr ),* ) => {
        {
            let mut h2 = $crate::Tag::<web_sys::Element>::new("h2");
            $(
                h2 = h2.push($x);
            )*
            h2
        }
    };
}

/// `h3` element
#[macro_export]
macro_rules! h3 {
    ( $( $x:expr ),* ) => {
        {
            let mut h3 = $crate::Tag::<web_sys::Element>::new("h3");
            $(
                h3 = h3.push($x);
            )*
            h3
        }
    };
}

/// `h4` element
#[macro_export]
macro_rules! h4 {
    ( $( $x:expr ),* ) => {
        {
            let mut h4 = $crate::Tag::<web_sys::Element>::new("h4");
            $(
                h4 = h4.push($x);
            )*
            h4
        }
    };
}

/// `h5` element
#[macro_export]
macro_rules! h5 {
    ( $( $x:expr ),* ) => {
        {
            let mut h5 = $crate::Tag::<web_sys::Element>::new("h5");
            $(
                h5 = h5.push($x);
            )*
            h5
        }
    };
}

/// `h6` element
#[macro_export]
macro_rules! h6 {
    ( $( $x:expr ),* ) => {
        {
            let mut h6 = $crate::Tag::<web_sys::Element>::new("h6");
            $(
                h6 = h6.push($x);
            )*
            h6
        }
    };
}

/// `br` element
#[macro_export]
macro_rules! br {
    () => {{
        let mut br = $crate::Tag::<web_sys::Element>::new("br");

        br
    }};
}

/// `span` element
#[macro_export]
macro_rules! span {
    ( $( $x:expr ),* ) => {
        {
            let mut span = $crate::Tag::<web_sys::Element>::new("span");
            $(
                span = span.push($x);
            )*
            span
        }
    };
}

/// `ul` element
#[macro_export]
macro_rules! ul {
    ( $( $x:expr ),* ) => {
        {
            let mut ul = $crate::Tag::<web_sys::Element>::new("ul");
            $(
                ul = ul.push($x);
            )*
            ul
        }
    };
}

/// `ol` element
#[macro_export]
macro_rules! ol {
    ( $( $x:expr ),* ) => {
        {
            let mut ol = $crate::Tag::<web_sys::Element>::new("ol");
            $(
                ol = ol.push($x);
            )*
            ol
        }
    };
}

/// `li` element
#[macro_export]
macro_rules! li {
    ( $( $x:expr ),* ) => {
        {
            let mut li = $crate::Tag::<web_sys::Element>::new("li");
            $(
                li = li.push($x);
            )*
            li
        }
    };
}

/// `input` element
#[macro_export]
macro_rules! input {
    ( $type:expr ) => {
        {
            let mut input = $crate::Tag::<web_sys::HtmlInputElement>::new("input").attr("type", $type);
            input
        }
    };
    ( $type:expr, $( $x:expr ),* ) => {
        {
            let mut input = $crate::Tag::<web_sys::HtmlInputElement>::new("input").attr("type", $type);
            $(
                input = input.push($x);
            )*
            input
        }
    };
}
