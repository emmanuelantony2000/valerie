/// Trait for HTML Elements
pub trait HtmlElement {
    /// The tag of the Element
    fn tag() -> &'static str;
}

// Main root

/// HTML <html> element
pub struct Html;

impl HtmlElement for Html {
    fn tag() -> &'static str {
        "html"
    }
}

// Document metadata

/// HTML <base> element
pub struct Base;

impl HtmlElement for Base {
    fn tag() -> &'static str {
        "base"
    }
}

/// HTML <head> element
pub struct Head;

impl HtmlElement for Head {
    fn tag() -> &'static str {
        "head"
    }
}

/// HTML <link> element
pub struct Link;

impl HtmlElement for Link {
    fn tag() -> &'static str {
        "link"
    }
}

/// HTML <meta> element
pub struct Meta;

impl HtmlElement for Meta {
    fn tag() -> &'static str {
        "meta"
    }
}

/// HTML <style> element
pub struct Style;

impl HtmlElement for Style {
    fn tag() -> &'static str {
        "style"
    }
}

/// HTML <title> element
pub struct Title;

impl HtmlElement for Title {
    fn tag() -> &'static str {
        "title"
    }
}

// Sectioning root

/// HTML <body> element
pub struct Body;

impl HtmlElement for Body {
    fn tag() -> &'static str {
        "body"
    }
}

// Content sectioning

/// HTML <address> element
pub struct Address;

impl HtmlElement for Address {
    fn tag() -> &'static str {
        "address"
    }
}

/// HTML <article> element
pub struct Article;

impl HtmlElement for Article {
    fn tag() -> &'static str {
        "article"
    }
}

/// HTML <aside> element
pub struct Aside;

impl HtmlElement for Aside {
    fn tag() -> &'static str {
        "aside"
    }
}

/// HTML <footer> element
pub struct Footer;

impl HtmlElement for Footer {
    fn tag() -> &'static str {
        "footer"
    }
}

/// HTML <header> element
pub struct Header;

impl HtmlElement for Header {
    fn tag() -> &'static str {
        "header"
    }
}

/// HTML <h1> element
pub struct H1;

impl HtmlElement for H1 {
    fn tag() -> &'static str {
        "h1"
    }
}

/// HTML <h2> element
pub struct H2;

impl HtmlElement for H2 {
    fn tag() -> &'static str {
        "h2"
    }
}

/// HTML <h3> element
pub struct H3;

impl HtmlElement for H3 {
    fn tag() -> &'static str {
        "h3"
    }
}

/// HTML <h4> element
pub struct H4;

impl HtmlElement for H4 {
    fn tag() -> &'static str {
        "h4"
    }
}

/// HTML <h5> element
pub struct H5;

impl HtmlElement for H5 {
    fn tag() -> &'static str {
        "h5"
    }
}

/// HTML <h6> element
pub struct H6;

impl HtmlElement for H6 {
    fn tag() -> &'static str {
        "h6"
    }
}

/// HTML <hgroup> element
pub struct Hgroup;

impl HtmlElement for Hgroup {
    fn tag() -> &'static str {
        "hgroup"
    }
}

/// HTML <main> element
pub struct Main;

impl HtmlElement for Main {
    fn tag() -> &'static str {
        "main"
    }
}

/// HTML <nav> element
pub struct Nav;

impl HtmlElement for Nav {
    fn tag() -> &'static str {
        "nav"
    }
}

/// HTML <section> element
pub struct Section;

impl HtmlElement for Section {
    fn tag() -> &'static str {
        "section"
    }
}

// Text content

/// HTML <blockquote> element
pub struct Blockquote;

impl HtmlElement for Blockquote {
    fn tag() -> &'static str {
        "blockquote"
    }
}

/// HTML <dd> element
pub struct Dd;

impl HtmlElement for Dd {
    fn tag() -> &'static str {
        "dd"
    }
}

/// HTML <div> element
pub struct Div;

impl HtmlElement for Div {
    fn tag() -> &'static str {
        "div"
    }
}

/// HTML <dl> element
pub struct Dl;

impl HtmlElement for Dl {
    fn tag() -> &'static str {
        "dl"
    }
}

/// HTML <dt> element
pub struct Dt;

impl HtmlElement for Dt {
    fn tag() -> &'static str {
        "dt"
    }
}

/// HTML <figcaption> element
pub struct Figcaption;

impl HtmlElement for Figcaption {
    fn tag() -> &'static str {
        "figcaption"
    }
}

/// HTML <figure> element
pub struct Figure;

impl HtmlElement for Figure {
    fn tag() -> &'static str {
        "figure"
    }
}

/// HTML <hr> element
pub struct Hr;

impl HtmlElement for Hr {
    fn tag() -> &'static str {
        "hr"
    }
}

/// HTML <li> element
pub struct Li;

impl HtmlElement for Li {
    fn tag() -> &'static str {
        "li"
    }
}

/// HTML <ol> element
pub struct Ol;

impl HtmlElement for Ol {
    fn tag() -> &'static str {
        "ol"
    }
}

/// HTML <p> element
pub struct P;

impl HtmlElement for P {
    fn tag() -> &'static str {
        "p"
    }
}

/// HTML <pre> element
pub struct Pre;

impl HtmlElement for Pre {
    fn tag() -> &'static str {
        "pre"
    }
}

/// HTML <ul> element
pub struct Ul;

impl HtmlElement for Ul {
    fn tag() -> &'static str {
        "ul"
    }
}

// Inline text semantics

/// HTML <a> element
pub struct A;

impl HtmlElement for A {
    fn tag() -> &'static str {
        "a"
    }
}

/// HTML <abbr> element
pub struct Abbr;

impl HtmlElement for Abbr {
    fn tag() -> &'static str {
        "abbr"
    }
}

/// HTML <b> element
pub struct B;

impl HtmlElement for B {
    fn tag() -> &'static str {
        "b"
    }
}

/// HTML <bdi> element
pub struct Bdi;

impl HtmlElement for Bdi {
    fn tag() -> &'static str {
        "bdi"
    }
}

/// HTML <bdo> element
pub struct Bdo;

impl HtmlElement for Bdo {
    fn tag() -> &'static str {
        "bdo"
    }
}

/// HTML <br> element
pub struct Br;

impl HtmlElement for Br {
    fn tag() -> &'static str {
        "br"
    }
}

/// HTML <cite> element
pub struct Cite;

impl HtmlElement for Cite {
    fn tag() -> &'static str {
        "cite"
    }
}

/// HTML <code> element
pub struct Code;

impl HtmlElement for Code {
    fn tag() -> &'static str {
        "code"
    }
}

/// HTML <data> element
pub struct Data;

impl HtmlElement for Data {
    fn tag() -> &'static str {
        "data"
    }
}

/// HTML <dfn> element
pub struct Dfn;

impl HtmlElement for Dfn {
    fn tag() -> &'static str {
        "dfn"
    }
}

/// HTML <em> element
pub struct Em;

impl HtmlElement for Em {
    fn tag() -> &'static str {
        "em"
    }
}

/// HTML <i> element
pub struct I;

impl HtmlElement for I {
    fn tag() -> &'static str {
        "i"
    }
}

/// HTML <kbd> element
pub struct Kbd;

impl HtmlElement for Kbd {
    fn tag() -> &'static str {
        "kbd"
    }
}

/// HTML <mark> element
pub struct Mark;

impl HtmlElement for Mark {
    fn tag() -> &'static str {
        "mark"
    }
}

/// HTML <q> element
pub struct Q;

impl HtmlElement for Q {
    fn tag() -> &'static str {
        "q"
    }
}

/// HTML <rb> element
pub struct Rb;

impl HtmlElement for Rb {
    fn tag() -> &'static str {
        "rb"
    }
}

/// HTML <rp> element
pub struct Rp;

impl HtmlElement for Rp {
    fn tag() -> &'static str {
        "rp"
    }
}

/// HTML <rt> element
pub struct Rt;

impl HtmlElement for Rt {
    fn tag() -> &'static str {
        "rt"
    }
}

/// HTML <rtc> element
pub struct Rtc;

impl HtmlElement for Rtc {
    fn tag() -> &'static str {
        "rtc"
    }
}

/// HTML <ruby> element
pub struct Ruby;

impl HtmlElement for Ruby {
    fn tag() -> &'static str {
        "ruby"
    }
}

/// HTML <s> element
pub struct S;

impl HtmlElement for S {
    fn tag() -> &'static str {
        "s"
    }
}

/// HTML <samp> element
pub struct Samp;

impl HtmlElement for Samp {
    fn tag() -> &'static str {
        "samp"
    }
}

/// HTML <small> element
pub struct Small;

impl HtmlElement for Small {
    fn tag() -> &'static str {
        "small"
    }
}

/// HTML <span> element
pub struct Span;

impl HtmlElement for Span {
    fn tag() -> &'static str {
        "span"
    }
}

/// HTML <strong> element
pub struct Strong;

impl HtmlElement for Strong {
    fn tag() -> &'static str {
        "strong"
    }
}

/// HTML <sub> element
pub struct Sub;

impl HtmlElement for Sub {
    fn tag() -> &'static str {
        "sub"
    }
}

/// HTML <sup> element
pub struct Sup;

impl HtmlElement for Sup {
    fn tag() -> &'static str {
        "sup"
    }
}

/// HTML <time> element
pub struct Time;

impl HtmlElement for Time {
    fn tag() -> &'static str {
        "time"
    }
}

/// HTML <u> element
pub struct U;

impl HtmlElement for U {
    fn tag() -> &'static str {
        "u"
    }
}

/// HTML <var> element
pub struct Var;

impl HtmlElement for Var {
    fn tag() -> &'static str {
        "var"
    }
}

/// HTML <wbr> element
pub struct Wbr;

impl HtmlElement for Wbr {
    fn tag() -> &'static str {
        "wbr"
    }
}

// Image and multimedia

/// HTML <area> element
pub struct Area;

impl HtmlElement for Area {
    fn tag() -> &'static str {
        "area"
    }
}

/// HTML <audio> element
pub struct Audio;

impl HtmlElement for Audio {
    fn tag() -> &'static str {
        "audio"
    }
}

/// HTML <img> element
pub struct Img;

impl HtmlElement for Img {
    fn tag() -> &'static str {
        "img"
    }
}

/// HTML <map> element
pub struct Map;

impl HtmlElement for Map {
    fn tag() -> &'static str {
        "map"
    }
}

/// HTML <track> element
pub struct Track;

impl HtmlElement for Track {
    fn tag() -> &'static str {
        "track"
    }
}

/// HTML <video> element
pub struct Video;

impl HtmlElement for Video {
    fn tag() -> &'static str {
        "video"
    }
}

// Embedded content

/// HTML <embed> element
pub struct Embed;

impl HtmlElement for Embed {
    fn tag() -> &'static str {
        "embed"
    }
}

/// HTML <iframe> element
pub struct Iframe;

impl HtmlElement for Iframe {
    fn tag() -> &'static str {
        "iframe"
    }
}

/// HTML <object> element
pub struct Object;

impl HtmlElement for Object {
    fn tag() -> &'static str {
        "object"
    }
}

/// HTML <param> element
pub struct Param;

impl HtmlElement for Param {
    fn tag() -> &'static str {
        "param"
    }
}

/// HTML <picture> element
pub struct Picture;

impl HtmlElement for Picture {
    fn tag() -> &'static str {
        "picture"
    }
}

/// HTML <source> element
pub struct Source;

impl HtmlElement for Source {
    fn tag() -> &'static str {
        "source"
    }
}

// Scripting

/// HTML <canvas> element
pub struct Canvas;

impl HtmlElement for Canvas {
    fn tag() -> &'static str {
        "canvas"
    }
}

/// HTML <noscript> element
pub struct Noscript;

impl HtmlElement for Noscript {
    fn tag() -> &'static str {
        "noscript"
    }
}

/// HTML <script> element
pub struct Script;

impl HtmlElement for Script {
    fn tag() -> &'static str {
        "script"
    }
}

// Demarcating edits

/// HTML <del> element
pub struct Del;

impl HtmlElement for Del {
    fn tag() -> &'static str {
        "del"
    }
}

/// HTML <ins> element
pub struct Ins;

impl HtmlElement for Ins {
    fn tag() -> &'static str {
        "ins"
    }
}

// Table content

/// HTML <caption> element
pub struct Caption;

impl HtmlElement for Caption {
    fn tag() -> &'static str {
        "caption"
    }
}

/// HTML <col> element
pub struct Col;

impl HtmlElement for Col {
    fn tag() -> &'static str {
        "col"
    }
}

/// HTML <colgroup> element
pub struct Colgroup;

impl HtmlElement for Colgroup {
    fn tag() -> &'static str {
        "colgroup"
    }
}

/// HTML <table> element
pub struct Table;

impl HtmlElement for Table {
    fn tag() -> &'static str {
        "table"
    }
}

/// HTML <tbody> element
pub struct Tbody;

impl HtmlElement for Tbody {
    fn tag() -> &'static str {
        "tbody"
    }
}

/// HTML <td> element
pub struct Td;

impl HtmlElement for Td {
    fn tag() -> &'static str {
        "td"
    }
}

/// HTML <tfoot> element
pub struct Tfoot;

impl HtmlElement for Tfoot {
    fn tag() -> &'static str {
        "tfoot"
    }
}

/// HTML <th> element
pub struct Th;

impl HtmlElement for Th {
    fn tag() -> &'static str {
        "th"
    }
}

/// HTML <thead> element
pub struct Thead;

impl HtmlElement for Thead {
    fn tag() -> &'static str {
        "thead"
    }
}

/// HTML <tr> element
pub struct Tr;

impl HtmlElement for Tr {
    fn tag() -> &'static str {
        "tr"
    }
}

// Forms

/// HTML <button> element
pub struct Button;

impl HtmlElement for Button {
    fn tag() -> &'static str {
        "button"
    }
}

/// HTML <datalist> element
pub struct Datalist;

impl HtmlElement for Datalist {
    fn tag() -> &'static str {
        "datalist"
    }
}

/// HTML <fieldset> element
pub struct Fieldset;

impl HtmlElement for Fieldset {
    fn tag() -> &'static str {
        "fieldset"
    }
}

/// HTML <form> element
pub struct Form;

impl HtmlElement for Form {
    fn tag() -> &'static str {
        "form"
    }
}

/// HTML <input> element
pub struct Input;

impl HtmlElement for Input {
    fn tag() -> &'static str {
        "input"
    }
}

/// HTML <label> element
pub struct Label;

impl HtmlElement for Label {
    fn tag() -> &'static str {
        "label"
    }
}

/// HTML <legend> element
pub struct Legend;

impl HtmlElement for Legend {
    fn tag() -> &'static str {
        "legend"
    }
}

/// HTML <meter> element
pub struct Meter;

impl HtmlElement for Meter {
    fn tag() -> &'static str {
        "meter"
    }
}

/// HTML <optgroup> element
pub struct Optgroup;

impl HtmlElement for Optgroup {
    fn tag() -> &'static str {
        "optgroup"
    }
}

/// HTML <option> element
pub struct Option;

impl HtmlElement for Option {
    fn tag() -> &'static str {
        "option"
    }
}

/// HTML <output> element
pub struct Output;

impl HtmlElement for Output {
    fn tag() -> &'static str {
        "output"
    }
}

/// HTML <progress> element
pub struct Progress;

impl HtmlElement for Progress {
    fn tag() -> &'static str {
        "progress"
    }
}

/// HTML <select> element
pub struct Select;

impl HtmlElement for Select {
    fn tag() -> &'static str {
        "select"
    }
}

/// HTML <textarea> element
pub struct Textarea;

impl HtmlElement for Textarea {
    fn tag() -> &'static str {
        "textarea"
    }
}

// Interactive elements

/// HTML <details> element
pub struct Details;

impl HtmlElement for Details {
    fn tag() -> &'static str {
        "details"
    }
}

/// HTML <dialog> element
pub struct Dialog;

impl HtmlElement for Dialog {
    fn tag() -> &'static str {
        "dialog"
    }
}

/// HTML <menu> element
pub struct Menu;

impl HtmlElement for Menu {
    fn tag() -> &'static str {
        "menu"
    }
}

/// HTML <summary> element
pub struct Summary;

impl HtmlElement for Summary {
    fn tag() -> &'static str {
        "summary"
    }
}

// Web Components

/// HTML <slot> element
pub struct Slot;

impl HtmlElement for Slot {
    fn tag() -> &'static str {
        "slot"
    }
}

/// HTML <template> element
pub struct Template;

impl HtmlElement for Template {
    fn tag() -> &'static str {
        "template"
    }
}
