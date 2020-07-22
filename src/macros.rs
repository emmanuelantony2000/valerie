/// `div` element
#[macro_export]
macro_rules! div {
    ( $( $x:expr ),* ) => {
        {
            let mut div = $crate::Tag::<$crate::html::elements::Div>::new();
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
            let mut p = $crate::Tag::<$crate::html::elements::P>::new();
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
            let mut button = $crate::Tag::<$crate::html::elements::Button>::new();
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
            let mut h1 = $crate::Tag::<$crate::html::elements::H1>::new();
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
            let mut h2 = $crate::Tag::<$crate::html::elements::H2>::new();
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
            let mut h3 = $crate::Tag::<$crate::html::elements::H3>::new();
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
            let mut h4 = $crate::Tag::<$crate::html::elements::H4>::new();
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
            let mut h5 = $crate::Tag::<$crate::html::elements::H5>::new();
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
            let mut h6 = $crate::Tag::<$crate::html::elements::H6>::new();
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
        $crate::Tag::<$crate::html::elements::Br>::new()
    }};
}

/// `span` element
#[macro_export]
macro_rules! span {
    ( $( $x:expr ),* ) => {
        {
            let mut span = $crate::Tag::<$crate::html::elements::Span>::new();
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
            let mut ul = $crate::Tag::<$crate::html::elements::Ul>::new();
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
            let mut ol = $crate::Tag::<$crate::html::elements::Ol>::new();
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
            let mut li = $crate::Tag::<$crate::html::elements::Li>::new();
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
            $crate::Tag::<$crate::html::elements::Input>::new().set_type($type)
        }
    };
    ( $type:expr, $( $x:expr ),* ) => {
        {
            let mut input = $crate::Tag::<$crate::html::elements::Input>::new().set_type($type);
            $(
                input = input.push($x);
            )*
            input
        }
    };
}

/// `img` element
#[macro_export]
macro_rules! img {
    ( $src:expr ) => {{
        $crate::Tag::<$crate::html::elements::Img>::new().src($src)
    }};
    ( $src:expr, $alt:expr ) => {{
        $crate::Tag::<$crate::html::elements::Img>::new()
            .src($src)
            .alt($alt)
    }};
}
