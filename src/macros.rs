#[macro_export]
macro_rules! div {
    ( $( $x:expr ),* ) => {
        {
            let mut div = Tag::new("div");
            $(
                div = div.push(&$x.view());
            )*
            div
        }
    };
}

#[macro_export]
macro_rules! p {
    ( $( $x:expr ),* ) => {
        {
            let mut p = Tag::new("p");
            $(
                p = p.push(&$x.view());
            )*
            p
        }
    };
}

#[macro_export]
macro_rules! button {
    ( $( $x:expr ),* ) => {
        {
            let mut button = Tag::new("button");
            $(
                button = button.push(&$x.view());
            )*
            button
        }
    };
}

#[macro_export]
macro_rules! h1 {
    ( $( $x:expr ),* ) => {
        {
            let mut h1 = Tag::new("h1");
            $(
                h1 = h1.push(&$x.view());
            )*
            h1
        }
    };
}

#[macro_export]
macro_rules! h2 {
    ( $( $x:expr ),* ) => {
        {
            let mut h2 = Tag::new("h2");
            $(
                h2 = h2.push(&$x.view());
            )*
            h2
        }
    };
}

#[macro_export]
macro_rules! h3 {
    ( $( $x:expr ),* ) => {
        {
            let mut h3 = Tag::new("h3");
            $(
                h3 = h3.push(&$x.view());
            )*
            h3
        }
    };
}

#[macro_export]
macro_rules! h4 {
    ( $( $x:expr ),* ) => {
        {
            let mut h4 = Tag::new("h4");
            $(
                h4 = h4.push(&$x.view());
            )*
            h4
        }
    };
}

#[macro_export]
macro_rules! h5 {
    ( $( $x:expr ),* ) => {
        {
            let mut h5 = Tag::new("h5");
            $(
                h5 = h5.push(&$x.view());
            )*
            h5
        }
    };
}

#[macro_export]
macro_rules! h6 {
    ( $( $x:expr ),* ) => {
        {
            let mut h6 = Tag::new("h6");
            $(
                h6 = h6.push(&$x.view());
            )*
            h6
        }
    };
}

#[macro_export]
macro_rules! br {
    () => {
        {
            let mut br = Tag::new("br");
            
            br
        }
    };
}

#[macro_export]
macro_rules! span {
    ( $( $x:expr ),* ) => {
        {
            let mut span = Tag::new("span");
            $(
                span = span.push(&$x.view());
            )*
            span
        }
    };
}

#[macro_export]
macro_rules! ul {
    ( $( $x:expr ),* ) => {
        {
            let mut ul = Tag::new("ul");
            $(
                ul = ul.push(&$x.view());
            )*
            ul
        }
    };
}

#[macro_export]
macro_rules! ol {
    ( $( $x:expr ),* ) => {
        {
            let mut ol = Tag::new("ol");
            $(
                ol = ol.push(&$x.view());
            )*
            ol
        }
    };
}

#[macro_export]
macro_rules! li {
    ( $( $x:expr ),* ) => {
        {
            let mut li = Tag::new("li");
            $(
                li = li.push(&$x.view());
            )*
            li
        }
    };
}
