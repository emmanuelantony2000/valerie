use alloc::string::String;

use super::elements;
use crate::value;
use crate::Tag;

/// Trait for the attribute `accept`
pub trait Accept: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "accept"
    }
}

impl<T> Tag<T>
where
    T: Accept + 'static,
{
    /// Set the value of the `accept` attribute.
    pub fn accept(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Accept,
{
    /// Get the value of the `accept` attribute.
    pub fn get_accept(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `accept` attribute from the `Tag`.
    pub fn rem_accept(&self) {
        self.rem_attr(T::attr())
    }
}

impl Accept for elements::Form {}
impl Accept for elements::Input {}

/// Trait for the attribute `accept-charset`
pub trait AcceptCharset: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "accept-charset"
    }
}

impl<T> Tag<T>
where
    T: AcceptCharset + 'static,
{
    /// Set the value of the `accept-charset` attribute.
    pub fn accept_charset(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: AcceptCharset,
{
    /// Get the value of the `accept-charset` attribute.
    pub fn get_accept_charset(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `accept-charset` attribute from the `Tag`.
    pub fn rem_accept_charset(&self) {
        self.rem_attr(T::attr())
    }
}

impl AcceptCharset for elements::Form {}

/// Trait for the attribute `accesskey`
pub trait Accesskey: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "accesskey"
    }
}

impl<T> Tag<T>
where
    T: Accesskey + 'static,
{
    /// Set the value of the `accesskey` attribute.
    pub fn accesskey(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Accesskey,
{
    /// Get the value of the `accesskey` attribute.
    pub fn get_accesskey(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `accesskey` attribute from the `Tag`.
    pub fn rem_accesskey(&self) {
        self.rem_attr(T::attr())
    }
}

impl<T> Accesskey for T where T: elements::HtmlElement {}

/// Trait for the attribute `action`
pub trait Action: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "action"
    }
}

impl<T> Tag<T>
where
    T: Action + 'static,
{
    /// Set the value of the `action` attribute.
    pub fn action(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Action,
{
    /// Get the value of the `action` attribute.
    pub fn get_action(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `action` attribute from the `Tag`.
    pub fn rem_action(&self) {
        self.rem_attr(T::attr())
    }
}

impl Action for elements::Form {}

/// Trait for the attribute `align`
pub trait Align: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "align"
    }
}

impl<T> Tag<T>
where
    T: Align + 'static,
{
    /// Set the value of the `align` attribute.
    pub fn align(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Align,
{
    /// Get the value of the `align` attribute.
    pub fn get_align(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `align` attribute from the `Tag`.
    pub fn rem_align(&self) {
        self.rem_attr(T::attr())
    }
}

impl Align for elements::Caption {}
impl Align for elements::Col {}
impl Align for elements::Colgroup {}
impl Align for elements::Hr {}
impl Align for elements::Iframe {}
impl Align for elements::Img {}
impl Align for elements::Table {}
impl Align for elements::Tbody {}
impl Align for elements::Td {}
impl Align for elements::Tfoot {}
impl Align for elements::Th {}
impl Align for elements::Thead {}
impl Align for elements::Tr {}

/// Trait for the attribute `allow`
pub trait Allow: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "allow"
    }
}

impl<T> Tag<T>
where
    T: Allow + 'static,
{
    /// Set the value of the `allow` attribute.
    pub fn allow(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Allow,
{
    /// Get the value of the `allow` attribute.
    pub fn get_allow(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `allow` attribute from the `Tag`.
    pub fn rem_allow(&self) {
        self.rem_attr(T::attr())
    }
}

impl Allow for elements::Iframe {}

/// Trait for the attribute `alt`
pub trait Alt: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "alt"
    }
}

impl<T> Tag<T>
where
    T: Alt + 'static,
{
    /// Set the value of the `alt` attribute.
    pub fn alt(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Alt,
{
    /// Get the value of the `alt` attribute.
    pub fn get_alt(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `alt` attribute from the `Tag`.
    pub fn rem_alt(&self) {
        self.rem_attr(T::attr())
    }
}

impl Alt for elements::Area {}
impl Alt for elements::Img {}
impl Alt for elements::Input {}

/// Trait for the attribute `async`
pub trait Async: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "async"
    }
}

impl<T> Tag<T>
where
    T: Async + 'static,
{
    /// Set the value of the `async` attribute.
    pub fn set_async(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Async,
{
    /// Get the value of the `async` attribute.
    pub fn get_set_async(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `async` attribute from the `Tag`.
    pub fn rem_set_async(&self) {
        self.rem_attr(T::attr())
    }
}

impl Async for elements::Script {}

/// Trait for the attribute `autocapitalize`
pub trait Autocapitalize: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "autocapitalize"
    }
}

impl<T> Tag<T>
where
    T: Autocapitalize + 'static,
{
    /// Set the value of the `autocapitalize` attribute.
    pub fn autocapitalize(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Autocapitalize,
{
    /// Get the value of the `autocapitalize` attribute.
    pub fn get_autocapitalize(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `autocapitalize` attribute from the `Tag`.
    pub fn rem_autocapitalize(&self) {
        self.rem_attr(T::attr())
    }
}

impl<T> Autocapitalize for T where T: elements::HtmlElement {}

/// Trait for the attribute `autocomplete`
pub trait Autocomplete: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "autocomplete"
    }
}

impl<T> Tag<T>
where
    T: Autocomplete + 'static,
{
    /// Set the value of the `autocomplete` attribute.
    pub fn autocomplete(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Autocomplete,
{
    /// Get the value of the `autocomplete` attribute.
    pub fn get_autocomplete(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `autocomplete` attribute from the `Tag`.
    pub fn rem_autocomplete(&self) {
        self.rem_attr(T::attr())
    }
}

impl Autocomplete for elements::Form {}
impl Autocomplete for elements::Input {}
impl Autocomplete for elements::Select {}
impl Autocomplete for elements::Textarea {}

/// Trait for the attribute `autofocus`
pub trait Autofocus: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "autofocus"
    }
}

impl<T> Tag<T>
where
    T: Autofocus + 'static,
{
    /// Set the value of the `autofocus` attribute.
    pub fn autofocus(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Autofocus,
{
    /// Get the value of the `autofocus` attribute.
    pub fn get_autofocus(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `autofocus` attribute from the `Tag`.
    pub fn rem_autofocus(&self) {
        self.rem_attr(T::attr())
    }
}

impl Autofocus for elements::Button {}
impl Autofocus for elements::Input {}
impl Autofocus for elements::Select {}
impl Autofocus for elements::Textarea {}

/// Trait for the attribute `autoplay`
pub trait Autoplay: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "autoplay"
    }
}

impl<T> Tag<T>
where
    T: Autoplay + 'static,
{
    /// Set the value of the `autoplay` attribute.
    pub fn autoplay(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Autoplay,
{
    /// Get the value of the `autoplay` attribute.
    pub fn get_autoplay(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `autoplay` attribute from the `Tag`.
    pub fn rem_autoplay(&self) {
        self.rem_attr(T::attr())
    }
}

impl Autoplay for elements::Audio {}
impl Autoplay for elements::Video {}

/// Trait for the attribute `background`
pub trait Background: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "background"
    }
}

impl<T> Tag<T>
where
    T: Background + 'static,
{
    /// Set the value of the `background` attribute.
    pub fn background(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Background,
{
    /// Get the value of the `background` attribute.
    pub fn get_background(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `background` attribute from the `Tag`.
    pub fn rem_background(&self) {
        self.rem_attr(T::attr())
    }
}

impl Background for elements::Body {}
impl Background for elements::Table {}
impl Background for elements::Td {}
impl Background for elements::Th {}

/// Trait for the attribute `bgcolor`
pub trait Bgcolor: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "bgcolor"
    }
}

impl<T> Tag<T>
where
    T: Bgcolor + 'static,
{
    /// Set the value of the `bgcolor` attribute.
    pub fn bgcolor(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Bgcolor,
{
    /// Get the value of the `bgcolor` attribute.
    pub fn get_bgcolor(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `bgcolor` attribute from the `Tag`.
    pub fn rem_bgcolor(&self) {
        self.rem_attr(T::attr())
    }
}

impl Bgcolor for elements::Body {}
impl Bgcolor for elements::Col {}
impl Bgcolor for elements::Colgroup {}
impl Bgcolor for elements::Table {}
impl Bgcolor for elements::Tbody {}
impl Bgcolor for elements::Tfoot {}
impl Bgcolor for elements::Td {}
impl Bgcolor for elements::Th {}
impl Bgcolor for elements::Tr {}

/// Trait for the attribute `border`
pub trait Border: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "border"
    }
}

impl<T> Tag<T>
where
    T: Border + 'static,
{
    /// Set the value of the `border` attribute.
    pub fn border(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Border,
{
    /// Get the value of the `border` attribute.
    pub fn get_border(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `border` attribute from the `Tag`.
    pub fn rem_border(&self) {
        self.rem_attr(T::attr())
    }
}

impl Border for elements::Img {}
impl Border for elements::Object {}
impl Border for elements::Table {}

/// Trait for the attribute `buffered`
pub trait Buffered: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "buffered"
    }
}

impl<T> Tag<T>
where
    T: Buffered + 'static,
{
    /// Set the value of the `buffered` attribute.
    pub fn buffered(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Buffered,
{
    /// Get the value of the `buffered` attribute.
    pub fn get_buffered(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `buffered` attribute from the `Tag`.
    pub fn rem_buffered(&self) {
        self.rem_attr(T::attr())
    }
}

impl Buffered for elements::Audio {}
impl Buffered for elements::Video {}

/// Trait for the attribute `capture`
pub trait Capture: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "capture"
    }
}

impl<T> Tag<T>
where
    T: Capture + 'static,
{
    /// Set the value of the `capture` attribute.
    pub fn capture(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Capture,
{
    /// Get the value of the `capture` attribute.
    pub fn get_capture(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `capture` attribute from the `Tag`.
    pub fn rem_capture(&self) {
        self.rem_attr(T::attr())
    }
}

impl Capture for elements::Input {}

/// Trait for the attribute `charset`
pub trait Charset: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "charset"
    }
}

impl<T> Tag<T>
where
    T: Charset + 'static,
{
    /// Set the value of the `charset` attribute.
    pub fn charset(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Charset,
{
    /// Get the value of the `charset` attribute.
    pub fn get_charset(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `charset` attribute from the `Tag`.
    pub fn rem_charset(&self) {
        self.rem_attr(T::attr())
    }
}

impl Charset for elements::Meta {}
impl Charset for elements::Script {}

/// Trait for the attribute `checked`
pub trait Checked: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "checked"
    }
}

impl<T> Tag<T>
where
    T: Checked + 'static,
{
    /// Set the value of the `checked` attribute.
    pub fn checked(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Checked,
{
    /// Get the value of the `checked` attribute.
    pub fn get_checked(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `checked` attribute from the `Tag`.
    pub fn rem_checked(&self) {
        self.rem_attr(T::attr())
    }
}

impl Checked for elements::Input {}

/// Trait for the attribute `cite`
pub trait Cite: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "cite"
    }
}

impl<T> Tag<T>
where
    T: Cite + 'static,
{
    /// Set the value of the `cite` attribute.
    pub fn cite(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Cite,
{
    /// Get the value of the `cite` attribute.
    pub fn get_cite(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `cite` attribute from the `Tag`.
    pub fn rem_cite(&self) {
        self.rem_attr(T::attr())
    }
}

impl Cite for elements::Blockquote {}
impl Cite for elements::Del {}
impl Cite for elements::Ins {}
impl Cite for elements::Q {}

/// Trait for the attribute `class`
pub trait Class: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "class"
    }
}

impl<T> Class for T where T: elements::HtmlElement {}

/// Trait for the attribute `color`
pub trait Color: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "color"
    }
}

impl<T> Tag<T>
where
    T: Color + 'static,
{
    /// Set the value of the `color` attribute.
    pub fn color(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Color,
{
    /// Get the value of the `color` attribute.
    pub fn get_color(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `color` attribute from the `Tag`.
    pub fn rem_color(&self) {
        self.rem_attr(T::attr())
    }
}

impl Color for elements::Hr {}

/// Trait for the attribute `cols`
pub trait Cols: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "cols"
    }
}

impl<T> Tag<T>
where
    T: Cols + 'static,
{
    /// Set the value of the `cols` attribute.
    pub fn cols(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Cols,
{
    /// Get the value of the `cols` attribute.
    pub fn get_cols(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `cols` attribute from the `Tag`.
    pub fn rem_cols(&self) {
        self.rem_attr(T::attr())
    }
}

impl Cols for elements::Textarea {}

/// Trait for the attribute `colspan`
pub trait Colspan: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "colspan"
    }
}

impl<T> Tag<T>
where
    T: Colspan + 'static,
{
    /// Set the value of the `colspan` attribute.
    pub fn colspan(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Colspan,
{
    /// Get the value of the `colspan` attribute.
    pub fn get_colspan(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `colspan` attribute from the `Tag`.
    pub fn rem_colspan(&self) {
        self.rem_attr(T::attr())
    }
}

impl Colspan for elements::Td {}
impl Colspan for elements::Th {}

/// Trait for the attribute `content`
pub trait Content: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "content"
    }
}

impl<T> Tag<T>
where
    T: Content + 'static,
{
    /// Set the value of the `content` attribute.
    pub fn content(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Content,
{
    /// Get the value of the `content` attribute.
    pub fn get_content(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `content` attribute from the `Tag`.
    pub fn rem_content(&self) {
        self.rem_attr(T::attr())
    }
}

impl Content for elements::Meta {}

/// Trait for the attribute `contenteditable`
pub trait Contenteditable: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "contenteditable"
    }
}

impl<T> Tag<T>
where
    T: Contenteditable + 'static,
{
    /// Set the value of the `contenteditable` attribute.
    pub fn contenteditable(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Contenteditable,
{
    /// Get the value of the `contenteditable` attribute.
    pub fn get_contenteditable(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `contenteditable` attribute from the `Tag`.
    pub fn rem_contenteditable(&self) {
        self.rem_attr(T::attr())
    }
}

impl<T> Contenteditable for T where T: elements::HtmlElement {}

/// Trait for the attribute `contextmenu`
pub trait Contextmenu: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "contextmenu"
    }
}

impl<T> Tag<T>
where
    T: Contextmenu + 'static,
{
    /// Set the value of the `contextmenu` attribute.
    pub fn contextmenu(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Contextmenu,
{
    /// Get the value of the `contextmenu` attribute.
    pub fn get_contextmenu(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `contextmenu` attribute from the `Tag`.
    pub fn rem_contextmenu(&self) {
        self.rem_attr(T::attr())
    }
}

impl<T> Contextmenu for T where T: elements::HtmlElement {}

/// Trait for the attribute `controls`
pub trait Controls: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "controls"
    }
}

impl<T> Tag<T>
where
    T: Controls + 'static,
{
    /// Set the value of the `controls` attribute.
    pub fn controls(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Controls,
{
    /// Get the value of the `controls` attribute.
    pub fn get_controls(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `controls` attribute from the `Tag`.
    pub fn rem_controls(&self) {
        self.rem_attr(T::attr())
    }
}

impl Controls for elements::Audio {}
impl Controls for elements::Video {}

/// Trait for the attribute `coords`
pub trait Coords: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "coords"
    }
}

impl<T> Tag<T>
where
    T: Coords + 'static,
{
    /// Set the value of the `coords` attribute.
    pub fn coords(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Coords,
{
    /// Get the value of the `coords` attribute.
    pub fn get_coords(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `coords` attribute from the `Tag`.
    pub fn rem_coords(&self) {
        self.rem_attr(T::attr())
    }
}

impl Coords for elements::Area {}

/// Trait for the attribute `crossorigin`
pub trait Crossorigin: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "crossorigin"
    }
}

impl<T> Tag<T>
where
    T: Crossorigin + 'static,
{
    /// Set the value of the `crossorigin` attribute.
    pub fn crossorigin(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Crossorigin,
{
    /// Get the value of the `crossorigin` attribute.
    pub fn get_crossorigin(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `crossorigin` attribute from the `Tag`.
    pub fn rem_crossorigin(&self) {
        self.rem_attr(T::attr())
    }
}

impl Crossorigin for elements::Audio {}
impl Crossorigin for elements::Img {}
impl Crossorigin for elements::Link {}
impl Crossorigin for elements::Script {}
impl Crossorigin for elements::Video {}

/// Trait for the attribute `csp`
pub trait Csp: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "csp"
    }
}

impl<T> Tag<T>
where
    T: Csp + 'static,
{
    /// Set the value of the `csp` attribute.
    pub fn csp(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Csp,
{
    /// Get the value of the `csp` attribute.
    pub fn get_csp(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `csp` attribute from the `Tag`.
    pub fn rem_csp(&self) {
        self.rem_attr(T::attr())
    }
}

impl Csp for elements::Iframe {}

/// Trait for the attribute `data`
pub trait Data: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "data"
    }
}

impl<T> Tag<T>
where
    T: Data + 'static,
{
    /// Set the value of the `data` attribute.
    pub fn data(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Data,
{
    /// Get the value of the `data` attribute.
    pub fn get_data(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `data` attribute from the `Tag`.
    pub fn rem_data(&self) {
        self.rem_attr(T::attr())
    }
}

impl Data for elements::Object {}

/// Trait for the attribute `datetime`
pub trait Datetime: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "datetime"
    }
}

impl<T> Tag<T>
where
    T: Datetime + 'static,
{
    /// Set the value of the `datetime` attribute.
    pub fn datetime(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Datetime,
{
    /// Get the value of the `datetime` attribute.
    pub fn get_datetime(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `datetime` attribute from the `Tag`.
    pub fn rem_datetime(&self) {
        self.rem_attr(T::attr())
    }
}

impl Datetime for elements::Del {}
impl Datetime for elements::Ins {}
impl Datetime for elements::Time {}

/// Trait for the attribute `decoding`
pub trait Decoding: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "decoding"
    }
}

impl<T> Tag<T>
where
    T: Decoding + 'static,
{
    /// Set the value of the `decoding` attribute.
    pub fn decoding(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Decoding,
{
    /// Get the value of the `decoding` attribute.
    pub fn get_decoding(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `decoding` attribute from the `Tag`.
    pub fn rem_decoding(&self) {
        self.rem_attr(T::attr())
    }
}

impl Decoding for elements::Img {}

/// Trait for the attribute `default`
pub trait Default: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "default"
    }
}

impl<T> Tag<T>
where
    T: Default + 'static,
{
    /// Set the value of the `default` attribute.
    pub fn default(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Default,
{
    /// Get the value of the `default` attribute.
    pub fn get_default(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `default` attribute from the `Tag`.
    pub fn rem_default(&self) {
        self.rem_attr(T::attr())
    }
}

impl Default for elements::Track {}

/// Trait for the attribute `defer`
pub trait Defer: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "defer"
    }
}

impl<T> Tag<T>
where
    T: Defer + 'static,
{
    /// Set the value of the `defer` attribute.
    pub fn defer(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Defer,
{
    /// Get the value of the `defer` attribute.
    pub fn get_defer(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `defer` attribute from the `Tag`.
    pub fn rem_defer(&self) {
        self.rem_attr(T::attr())
    }
}

impl Defer for elements::Script {}

/// Trait for the attribute `dir`
pub trait Dir: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "dir"
    }
}

impl<T> Tag<T>
where
    T: Dir + 'static,
{
    /// Set the value of the `dir` attribute.
    pub fn dir(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Dir,
{
    /// Get the value of the `dir` attribute.
    pub fn get_dir(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `dir` attribute from the `Tag`.
    pub fn rem_dir(&self) {
        self.rem_attr(T::attr())
    }
}

impl<T> Dir for T where T: elements::HtmlElement {}

/// Trait for the attribute `dirname`
pub trait Dirname: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "dirname"
    }
}

impl<T> Tag<T>
where
    T: Dirname + 'static,
{
    /// Set the value of the `dirname` attribute.
    pub fn dirname(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Dirname,
{
    /// Get the value of the `dirname` attribute.
    pub fn get_dirname(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `dirname` attribute from the `Tag`.
    pub fn rem_dirname(&self) {
        self.rem_attr(T::attr())
    }
}

impl Dirname for elements::Input {}
impl Dirname for elements::Textarea {}

/// Trait for the attribute `disabled`
pub trait Disabled: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "disabled"
    }
}

impl<T> Tag<T>
where
    T: Disabled + 'static,
{
    /// Set the value of the `disabled` attribute.
    pub fn disabled(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Disabled,
{
    /// Get the value of the `disabled` attribute.
    pub fn get_disabled(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `disabled` attribute from the `Tag`.
    pub fn rem_disabled(&self) {
        self.rem_attr(T::attr())
    }
}

impl Disabled for elements::Button {}
impl Disabled for elements::Fieldset {}
impl Disabled for elements::Input {}
impl Disabled for elements::Optgroup {}
impl Disabled for elements::Option {}
impl Disabled for elements::Select {}
impl Disabled for elements::Textarea {}

/// Trait for the attribute `download`
pub trait Download: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "download"
    }
}

impl<T> Tag<T>
where
    T: Download + 'static,
{
    /// Set the value of the `download` attribute.
    pub fn download(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Download,
{
    /// Get the value of the `download` attribute.
    pub fn get_download(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `download` attribute from the `Tag`.
    pub fn rem_download(&self) {
        self.rem_attr(T::attr())
    }
}

impl Download for elements::A {}
impl Download for elements::Area {}

/// Trait for the attribute `draggable`
pub trait Draggable: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "draggable"
    }
}

impl<T> Tag<T>
where
    T: Draggable + 'static,
{
    /// Set the value of the `draggable` attribute.
    pub fn draggable(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Draggable,
{
    /// Get the value of the `draggable` attribute.
    pub fn get_draggable(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `draggable` attribute from the `Tag`.
    pub fn rem_draggable(&self) {
        self.rem_attr(T::attr())
    }
}

impl<T> Draggable for T where T: elements::HtmlElement {}

/// Trait for the attribute `dropzone`
pub trait Dropzone: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "dropzone"
    }
}

impl<T> Tag<T>
where
    T: Dropzone + 'static,
{
    /// Set the value of the `dropzone` attribute.
    pub fn dropzone(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Dropzone,
{
    /// Get the value of the `dropzone` attribute.
    pub fn get_dropzone(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `dropzone` attribute from the `Tag`.
    pub fn rem_dropzone(&self) {
        self.rem_attr(T::attr())
    }
}

impl<T> Dropzone for T where T: elements::HtmlElement {}

/// Trait for the attribute `enctype`
pub trait Enctype: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "enctype"
    }
}

impl<T> Tag<T>
where
    T: Enctype + 'static,
{
    /// Set the value of the `enctype` attribute.
    pub fn enctype(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Enctype,
{
    /// Get the value of the `enctype` attribute.
    pub fn get_enctype(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `enctype` attribute from the `Tag`.
    pub fn rem_enctype(&self) {
        self.rem_attr(T::attr())
    }
}

impl Enctype for elements::Form {}

/// Trait for the attribute `enterkeyhint`
pub trait Enterkeyhint: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "enterkeyhint"
    }
}

impl<T> Tag<T>
where
    T: Enterkeyhint + 'static,
{
    /// Set the value of the `enterkeyhint` attribute.
    pub fn enterkeyhint(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Enterkeyhint,
{
    /// Get the value of the `enterkeyhint` attribute.
    pub fn get_enterkeyhint(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `enterkeyhint` attribute from the `Tag`.
    pub fn rem_enterkeyhint(&self) {
        self.rem_attr(T::attr())
    }
}

impl Enterkeyhint for elements::Textarea {}

/// Trait for the attribute `for`
pub trait For: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "for"
    }
}

impl<T> Tag<T>
where
    T: For + 'static,
{
    /// Set the value of the `for` attribute.
    pub fn set_for(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: For,
{
    /// Get the value of the `for` attribute.
    pub fn get_set_for(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `for` attribute from the `Tag`.
    pub fn rem_set_for(&self) {
        self.rem_attr(T::attr())
    }
}

impl For for elements::Label {}
impl For for elements::Output {}

/// Trait for the attribute `form`
pub trait Form: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "form"
    }
}

impl<T> Tag<T>
where
    T: Form + 'static,
{
    /// Set the value of the `form` attribute.
    pub fn form(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Form,
{
    /// Get the value of the `form` attribute.
    pub fn get_form(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `form` attribute from the `Tag`.
    pub fn rem_form(&self) {
        self.rem_attr(T::attr())
    }
}

impl Form for elements::Button {}
impl Form for elements::Fieldset {}
impl Form for elements::Input {}
impl Form for elements::Label {}
impl Form for elements::Meter {}
impl Form for elements::Object {}
impl Form for elements::Output {}
impl Form for elements::Progress {}
impl Form for elements::Select {}
impl Form for elements::Textarea {}

/// Trait for the attribute `formaction`
pub trait Formaction: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "formaction"
    }
}

impl<T> Tag<T>
where
    T: Formaction + 'static,
{
    /// Set the value of the `formaction` attribute.
    pub fn formaction(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Formaction,
{
    /// Get the value of the `formaction` attribute.
    pub fn get_formaction(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `formaction` attribute from the `Tag`.
    pub fn rem_formaction(&self) {
        self.rem_attr(T::attr())
    }
}

impl Formaction for elements::Input {}
impl Formaction for elements::Button {}

/// Trait for the attribute `formenctype`
pub trait Formenctype: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "formenctype"
    }
}

impl<T> Tag<T>
where
    T: Formenctype + 'static,
{
    /// Set the value of the `formenctype` attribute.
    pub fn formenctype(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Formenctype,
{
    /// Get the value of the `formenctype` attribute.
    pub fn get_formenctype(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `formenctype` attribute from the `Tag`.
    pub fn rem_formenctype(&self) {
        self.rem_attr(T::attr())
    }
}

impl Formenctype for elements::Button {}
impl Formenctype for elements::Input {}

/// Trait for the attribute `formmethod`
pub trait Formmethod: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "formmethod"
    }
}

impl<T> Tag<T>
where
    T: Formmethod + 'static,
{
    /// Set the value of the `formmethod` attribute.
    pub fn formmethod(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Formmethod,
{
    /// Get the value of the `formmethod` attribute.
    pub fn get_formmethod(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `formmethod` attribute from the `Tag`.
    pub fn rem_formmethod(&self) {
        self.rem_attr(T::attr())
    }
}

impl Formmethod for elements::Button {}
impl Formmethod for elements::Input {}

/// Trait for the attribute `formnovalidate`
pub trait Formnovalidate: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "formnovalidate"
    }
}

impl<T> Tag<T>
where
    T: Formnovalidate + 'static,
{
    /// Set the value of the `formnovalidate` attribute.
    pub fn formnovalidate(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Formnovalidate,
{
    /// Get the value of the `formnovalidate` attribute.
    pub fn get_formnovalidate(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `formnovalidate` attribute from the `Tag`.
    pub fn rem_formnovalidate(&self) {
        self.rem_attr(T::attr())
    }
}

impl Formnovalidate for elements::Button {}
impl Formnovalidate for elements::Input {}

/// Trait for the attribute `formtarget`
pub trait Formtarget: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "formtarget"
    }
}

impl<T> Tag<T>
where
    T: Formtarget + 'static,
{
    /// Set the value of the `formtarget` attribute.
    pub fn formtarget(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Formtarget,
{
    /// Get the value of the `formtarget` attribute.
    pub fn get_formtarget(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `formtarget` attribute from the `Tag`.
    pub fn rem_formtarget(&self) {
        self.rem_attr(T::attr())
    }
}

impl Formtarget for elements::Button {}
impl Formtarget for elements::Input {}

/// Trait for the attribute `headers`
pub trait Headers: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "headers"
    }
}

impl<T> Tag<T>
where
    T: Headers + 'static,
{
    /// Set the value of the `headers` attribute.
    pub fn headers(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Headers,
{
    /// Get the value of the `headers` attribute.
    pub fn get_headers(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `headers` attribute from the `Tag`.
    pub fn rem_headers(&self) {
        self.rem_attr(T::attr())
    }
}

impl Headers for elements::Td {}
impl Headers for elements::Th {}

/// Trait for the attribute `height`
pub trait Height: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "height"
    }
}

impl<T> Tag<T>
where
    T: Height + 'static,
{
    /// Set the value of the `height` attribute.
    pub fn height(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Height,
{
    /// Get the value of the `height` attribute.
    pub fn get_height(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `height` attribute from the `Tag`.
    pub fn rem_height(&self) {
        self.rem_attr(T::attr())
    }
}

impl Height for elements::Canvas {}
impl Height for elements::Embed {}
impl Height for elements::Iframe {}
impl Height for elements::Img {}
impl Height for elements::Input {}
impl Height for elements::Object {}
impl Height for elements::Video {}

/// Trait for the attribute `hidden`
pub trait Hidden: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "hidden"
    }
}

impl<T> Tag<T>
where
    T: Hidden + 'static,
{
    /// Set the value of the `hidden` attribute.
    pub fn hidden(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Hidden,
{
    /// Get the value of the `hidden` attribute.
    pub fn get_hidden(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `hidden` attribute from the `Tag`.
    pub fn rem_hidden(&self) {
        self.rem_attr(T::attr())
    }
}

impl<T> Hidden for T where T: elements::HtmlElement {}

/// Trait for the attribute `high`
pub trait High: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "high"
    }
}

impl<T> Tag<T>
where
    T: High + 'static,
{
    /// Set the value of the `high` attribute.
    pub fn high(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: High,
{
    /// Get the value of the `high` attribute.
    pub fn get_high(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `high` attribute from the `Tag`.
    pub fn rem_high(&self) {
        self.rem_attr(T::attr())
    }
}

impl High for elements::Meter {}

/// Trait for the attribute `href`
pub trait Href: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "href"
    }
}

impl<T> Tag<T>
where
    T: Href + 'static,
{
    /// Set the value of the `href` attribute.
    pub fn href(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Href,
{
    /// Get the value of the `href` attribute.
    pub fn get_href(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `href` attribute from the `Tag`.
    pub fn rem_href(&self) {
        self.rem_attr(T::attr())
    }
}

impl Href for elements::A {}
impl Href for elements::Area {}
impl Href for elements::Base {}
impl Href for elements::Link {}

/// Trait for the attribute `hreflang`
pub trait Hreflang: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "hreflang"
    }
}

impl<T> Tag<T>
where
    T: Hreflang + 'static,
{
    /// Set the value of the `hreflang` attribute.
    pub fn hreflang(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Hreflang,
{
    /// Get the value of the `hreflang` attribute.
    pub fn get_hreflang(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `hreflang` attribute from the `Tag`.
    pub fn rem_hreflang(&self) {
        self.rem_attr(T::attr())
    }
}

impl Hreflang for elements::A {}
impl Hreflang for elements::Area {}
impl Hreflang for elements::Link {}

/// Trait for the attribute `http-equiv`
pub trait HttpEquiv: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "http-equiv"
    }
}

impl<T> Tag<T>
where
    T: HttpEquiv + 'static,
{
    /// Set the value of the `http-equiv` attribute.
    pub fn http_equiv(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: HttpEquiv,
{
    /// Get the value of the `http-equiv` attribute.
    pub fn get_http_equiv(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `http-equiv` attribute from the `Tag`.
    pub fn rem_http_equiv(&self) {
        self.rem_attr(T::attr())
    }
}

impl HttpEquiv for elements::Meta {}

/// Trait for the attribute `id`
pub trait Id: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "id"
    }
}

impl<T> Id for T where T: elements::HtmlElement {}

/// Trait for the attribute `importance`
pub trait Importance: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "importance"
    }
}

impl<T> Tag<T>
where
    T: Importance + 'static,
{
    /// Set the value of the `importance` attribute.
    pub fn importance(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Importance,
{
    /// Get the value of the `importance` attribute.
    pub fn get_importance(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `importance` attribute from the `Tag`.
    pub fn rem_importance(&self) {
        self.rem_attr(T::attr())
    }
}

impl Importance for elements::Iframe {}
impl Importance for elements::Img {}
impl Importance for elements::Link {}
impl Importance for elements::Script {}

/// Trait for the attribute `integrity`
pub trait Integrity: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "integrity"
    }
}

impl<T> Tag<T>
where
    T: Integrity + 'static,
{
    /// Set the value of the `integrity` attribute.
    pub fn integrity(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Integrity,
{
    /// Get the value of the `integrity` attribute.
    pub fn get_integrity(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `integrity` attribute from the `Tag`.
    pub fn rem_integrity(&self) {
        self.rem_attr(T::attr())
    }
}

impl Integrity for elements::Link {}
impl Integrity for elements::Script {}

/// Trait for the attribute `intrinsicsize`
pub trait Intrinsicsize: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "intrinsicsize"
    }
}

impl<T> Tag<T>
where
    T: Intrinsicsize + 'static,
{
    /// Set the value of the `intrinsicsize` attribute.
    pub fn intrinsicsize(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Intrinsicsize,
{
    /// Get the value of the `intrinsicsize` attribute.
    pub fn get_intrinsicsize(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `intrinsicsize` attribute from the `Tag`.
    pub fn rem_intrinsicsize(&self) {
        self.rem_attr(T::attr())
    }
}

impl Intrinsicsize for elements::Img {}

/// Trait for the attribute `inputmode`
pub trait Inputmode: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "inputmode"
    }
}

impl<T> Tag<T>
where
    T: Inputmode + 'static,
{
    /// Set the value of the `inputmode` attribute.
    pub fn inputmode(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Inputmode,
{
    /// Get the value of the `inputmode` attribute.
    pub fn get_inputmode(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `inputmode` attribute from the `Tag`.
    pub fn rem_inputmode(&self) {
        self.rem_attr(T::attr())
    }
}

impl Inputmode for elements::Textarea {}

/// Trait for the attribute `ismap`
pub trait Ismap: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "ismap"
    }
}

impl<T> Tag<T>
where
    T: Ismap + 'static,
{
    /// Set the value of the `ismap` attribute.
    pub fn ismap(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Ismap,
{
    /// Get the value of the `ismap` attribute.
    pub fn get_ismap(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `ismap` attribute from the `Tag`.
    pub fn rem_ismap(&self) {
        self.rem_attr(T::attr())
    }
}

impl Ismap for elements::Img {}

/// Trait for the attribute `itemprop`
pub trait Itemprop: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "itemprop"
    }
}

impl<T> Tag<T>
where
    T: Itemprop + 'static,
{
    /// Set the value of the `itemprop` attribute.
    pub fn itemprop(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Itemprop,
{
    /// Get the value of the `itemprop` attribute.
    pub fn get_itemprop(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `itemprop` attribute from the `Tag`.
    pub fn rem_itemprop(&self) {
        self.rem_attr(T::attr())
    }
}

impl<T> Itemprop for T where T: elements::HtmlElement {}

/// Trait for the attribute `kind`
pub trait Kind: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "kind"
    }
}

impl<T> Tag<T>
where
    T: Kind + 'static,
{
    /// Set the value of the `kind` attribute.
    pub fn kind(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Kind,
{
    /// Get the value of the `kind` attribute.
    pub fn get_kind(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `kind` attribute from the `Tag`.
    pub fn rem_kind(&self) {
        self.rem_attr(T::attr())
    }
}

impl Kind for elements::Track {}

/// Trait for the attribute `label`
pub trait Label: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "label"
    }
}

impl<T> Tag<T>
where
    T: Label + 'static,
{
    /// Set the value of the `label` attribute.
    pub fn label(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Label,
{
    /// Get the value of the `label` attribute.
    pub fn get_label(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `label` attribute from the `Tag`.
    pub fn rem_label(&self) {
        self.rem_attr(T::attr())
    }
}

impl Label for elements::Optgroup {}
impl Label for elements::Option {}
impl Label for elements::Track {}

/// Trait for the attribute `lang`
pub trait Lang: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "lang"
    }
}

impl<T> Tag<T>
where
    T: Lang + 'static,
{
    /// Set the value of the `lang` attribute.
    pub fn lang(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Lang,
{
    /// Get the value of the `lang` attribute.
    pub fn get_lang(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `lang` attribute from the `Tag`.
    pub fn rem_lang(&self) {
        self.rem_attr(T::attr())
    }
}

impl<T> Lang for T where T: elements::HtmlElement {}

/// Trait for the attribute `language`
pub trait Language: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "language"
    }
}

impl<T> Tag<T>
where
    T: Language + 'static,
{
    /// Set the value of the `language` attribute.
    pub fn language(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Language,
{
    /// Get the value of the `language` attribute.
    pub fn get_language(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `language` attribute from the `Tag`.
    pub fn rem_language(&self) {
        self.rem_attr(T::attr())
    }
}

impl Language for elements::Script {}

/// Trait for the attribute `loading`
pub trait Loading: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "loading"
    }
}

impl<T> Tag<T>
where
    T: Loading + 'static,
{
    /// Set the value of the `loading` attribute.
    pub fn loading(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Loading,
{
    /// Get the value of the `loading` attribute.
    pub fn get_loading(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `loading` attribute from the `Tag`.
    pub fn rem_loading(&self) {
        self.rem_attr(T::attr())
    }
}

impl Loading for elements::Img {}
impl Loading for elements::Iframe {}

/// Trait for the attribute `list`
pub trait List: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "list"
    }
}

impl<T> Tag<T>
where
    T: List + 'static,
{
    /// Set the value of the `list` attribute.
    pub fn list(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: List,
{
    /// Get the value of the `list` attribute.
    pub fn get_list(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `list` attribute from the `Tag`.
    pub fn rem_list(&self) {
        self.rem_attr(T::attr())
    }
}

impl List for elements::Input {}

/// Trait for the attribute `loop`
pub trait Loop: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "loop"
    }
}

impl<T> Tag<T>
where
    T: Loop + 'static,
{
    /// Set the value of the `loop` attribute.
    pub fn set_loop(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Loop,
{
    /// Get the value of the `loop` attribute.
    pub fn get_set_loop(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `loop` attribute from the `Tag`.
    pub fn rem_set_loop(&self) {
        self.rem_attr(T::attr())
    }
}

impl Loop for elements::Audio {}
impl Loop for elements::Video {}

/// Trait for the attribute `low`
pub trait Low: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "low"
    }
}

impl<T> Tag<T>
where
    T: Low + 'static,
{
    /// Set the value of the `low` attribute.
    pub fn low(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Low,
{
    /// Get the value of the `low` attribute.
    pub fn get_low(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `low` attribute from the `Tag`.
    pub fn rem_low(&self) {
        self.rem_attr(T::attr())
    }
}

impl Low for elements::Meter {}

/// Trait for the attribute `manifest`
pub trait Manifest: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "manifest"
    }
}

impl<T> Tag<T>
where
    T: Manifest + 'static,
{
    /// Set the value of the `manifest` attribute.
    pub fn manifest(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Manifest,
{
    /// Get the value of the `manifest` attribute.
    pub fn get_manifest(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `manifest` attribute from the `Tag`.
    pub fn rem_manifest(&self) {
        self.rem_attr(T::attr())
    }
}

impl Manifest for elements::Html {}

/// Trait for the attribute `max`
pub trait Max: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "max"
    }
}

impl<T> Tag<T>
where
    T: Max + 'static,
{
    /// Set the value of the `max` attribute.
    pub fn max(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Max,
{
    /// Get the value of the `max` attribute.
    pub fn get_max(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `max` attribute from the `Tag`.
    pub fn rem_max(&self) {
        self.rem_attr(T::attr())
    }
}

impl Max for elements::Input {}
impl Max for elements::Meter {}
impl Max for elements::Progress {}

/// Trait for the attribute `maxlength`
pub trait Maxlength: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "maxlength"
    }
}

impl<T> Tag<T>
where
    T: Maxlength + 'static,
{
    /// Set the value of the `maxlength` attribute.
    pub fn maxlength(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Maxlength,
{
    /// Get the value of the `maxlength` attribute.
    pub fn get_maxlength(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `maxlength` attribute from the `Tag`.
    pub fn rem_maxlength(&self) {
        self.rem_attr(T::attr())
    }
}

impl Maxlength for elements::Input {}
impl Maxlength for elements::Textarea {}

/// Trait for the attribute `minlength`
pub trait Minlength: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "minlength"
    }
}

impl<T> Tag<T>
where
    T: Minlength + 'static,
{
    /// Set the value of the `minlength` attribute.
    pub fn minlength(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Minlength,
{
    /// Get the value of the `minlength` attribute.
    pub fn get_minlength(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `minlength` attribute from the `Tag`.
    pub fn rem_minlength(&self) {
        self.rem_attr(T::attr())
    }
}

impl Minlength for elements::Input {}
impl Minlength for elements::Textarea {}

/// Trait for the attribute `media`
pub trait Media: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "media"
    }
}

impl<T> Tag<T>
where
    T: Media + 'static,
{
    /// Set the value of the `media` attribute.
    pub fn media(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Media,
{
    /// Get the value of the `media` attribute.
    pub fn get_media(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `media` attribute from the `Tag`.
    pub fn rem_media(&self) {
        self.rem_attr(T::attr())
    }
}

impl Media for elements::A {}
impl Media for elements::Area {}
impl Media for elements::Link {}
impl Media for elements::Source {}
impl Media for elements::Style {}

/// Trait for the attribute `method`
pub trait Method: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "method"
    }
}

impl<T> Tag<T>
where
    T: Method + 'static,
{
    /// Set the value of the `method` attribute.
    pub fn method(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Method,
{
    /// Get the value of the `method` attribute.
    pub fn get_method(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `method` attribute from the `Tag`.
    pub fn rem_method(&self) {
        self.rem_attr(T::attr())
    }
}

impl Method for elements::Form {}

/// Trait for the attribute `min`
pub trait Min: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "min"
    }
}

impl<T> Tag<T>
where
    T: Min + 'static,
{
    /// Set the value of the `min` attribute.
    pub fn min(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Min,
{
    /// Get the value of the `min` attribute.
    pub fn get_min(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `min` attribute from the `Tag`.
    pub fn rem_min(&self) {
        self.rem_attr(T::attr())
    }
}

impl Min for elements::Input {}
impl Min for elements::Meter {}

/// Trait for the attribute `multiple`
pub trait Multiple: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "multiple"
    }
}

impl<T> Tag<T>
where
    T: Multiple + 'static,
{
    /// Set the value of the `multiple` attribute.
    pub fn multiple(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Multiple,
{
    /// Get the value of the `multiple` attribute.
    pub fn get_multiple(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `multiple` attribute from the `Tag`.
    pub fn rem_multiple(&self) {
        self.rem_attr(T::attr())
    }
}

impl Multiple for elements::Input {}
impl Multiple for elements::Select {}

/// Trait for the attribute `muted`
pub trait Muted: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "muted"
    }
}

impl<T> Tag<T>
where
    T: Muted + 'static,
{
    /// Set the value of the `muted` attribute.
    pub fn muted(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Muted,
{
    /// Get the value of the `muted` attribute.
    pub fn get_muted(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `muted` attribute from the `Tag`.
    pub fn rem_muted(&self) {
        self.rem_attr(T::attr())
    }
}

impl Muted for elements::Audio {}
impl Muted for elements::Video {}

/// Trait for the attribute `name`
pub trait Name: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "name"
    }
}

impl<T> Tag<T>
where
    T: Name + 'static,
{
    /// Set the value of the `name` attribute.
    pub fn name(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Name,
{
    /// Get the value of the `name` attribute.
    pub fn get_name(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `name` attribute from the `Tag`.
    pub fn rem_name(&self) {
        self.rem_attr(T::attr())
    }
}

impl Name for elements::Button {}
impl Name for elements::Form {}
impl Name for elements::Fieldset {}
impl Name for elements::Iframe {}
impl Name for elements::Input {}
impl Name for elements::Object {}
impl Name for elements::Output {}
impl Name for elements::Select {}
impl Name for elements::Textarea {}
impl Name for elements::Map {}
impl Name for elements::Meta {}
impl Name for elements::Param {}

/// Trait for the attribute `novalidate`
pub trait Novalidate: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "novalidate"
    }
}

impl<T> Tag<T>
where
    T: Novalidate + 'static,
{
    /// Set the value of the `novalidate` attribute.
    pub fn novalidate(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Novalidate,
{
    /// Get the value of the `novalidate` attribute.
    pub fn get_novalidate(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `novalidate` attribute from the `Tag`.
    pub fn rem_novalidate(&self) {
        self.rem_attr(T::attr())
    }
}

impl Novalidate for elements::Form {}

/// Trait for the attribute `open`
pub trait Open: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "open"
    }
}

impl<T> Tag<T>
where
    T: Open + 'static,
{
    /// Set the value of the `open` attribute.
    pub fn open(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Open,
{
    /// Get the value of the `open` attribute.
    pub fn get_open(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `open` attribute from the `Tag`.
    pub fn rem_open(&self) {
        self.rem_attr(T::attr())
    }
}

impl Open for elements::Details {}

/// Trait for the attribute `optimum`
pub trait Optimum: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "optimum"
    }
}

impl<T> Tag<T>
where
    T: Optimum + 'static,
{
    /// Set the value of the `optimum` attribute.
    pub fn optimum(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Optimum,
{
    /// Get the value of the `optimum` attribute.
    pub fn get_optimum(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `optimum` attribute from the `Tag`.
    pub fn rem_optimum(&self) {
        self.rem_attr(T::attr())
    }
}

impl Optimum for elements::Meter {}

/// Trait for the attribute `pattern`
pub trait Pattern: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "pattern"
    }
}

impl<T> Tag<T>
where
    T: Pattern + 'static,
{
    /// Set the value of the `pattern` attribute.
    pub fn pattern(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Pattern,
{
    /// Get the value of the `pattern` attribute.
    pub fn get_pattern(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `pattern` attribute from the `Tag`.
    pub fn rem_pattern(&self) {
        self.rem_attr(T::attr())
    }
}

impl Pattern for elements::Input {}

/// Trait for the attribute `ping`
pub trait Ping: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "ping"
    }
}

impl<T> Tag<T>
where
    T: Ping + 'static,
{
    /// Set the value of the `ping` attribute.
    pub fn ping(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Ping,
{
    /// Get the value of the `ping` attribute.
    pub fn get_ping(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `ping` attribute from the `Tag`.
    pub fn rem_ping(&self) {
        self.rem_attr(T::attr())
    }
}

impl Ping for elements::A {}
impl Ping for elements::Area {}

/// Trait for the attribute `placeholder`
pub trait Placeholder: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "placeholder"
    }
}

impl<T> Tag<T>
where
    T: Placeholder + 'static,
{
    /// Set the value of the `placeholder` attribute.
    pub fn placeholder(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Placeholder,
{
    /// Get the value of the `placeholder` attribute.
    pub fn get_placeholder(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `placeholder` attribute from the `Tag`.
    pub fn rem_placeholder(&self) {
        self.rem_attr(T::attr())
    }
}

impl Placeholder for elements::Input {}
impl Placeholder for elements::Textarea {}

/// Trait for the attribute `poster`
pub trait Poster: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "poster"
    }
}

impl<T> Tag<T>
where
    T: Poster + 'static,
{
    /// Set the value of the `poster` attribute.
    pub fn poster(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Poster,
{
    /// Get the value of the `poster` attribute.
    pub fn get_poster(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `poster` attribute from the `Tag`.
    pub fn rem_poster(&self) {
        self.rem_attr(T::attr())
    }
}

impl Poster for elements::Video {}

/// Trait for the attribute `preload`
pub trait Preload: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "preload"
    }
}

impl<T> Tag<T>
where
    T: Preload + 'static,
{
    /// Set the value of the `preload` attribute.
    pub fn preload(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Preload,
{
    /// Get the value of the `preload` attribute.
    pub fn get_preload(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `preload` attribute from the `Tag`.
    pub fn rem_preload(&self) {
        self.rem_attr(T::attr())
    }
}

impl Preload for elements::Audio {}
impl Preload for elements::Video {}

/// Trait for the attribute `readonly`
pub trait Readonly: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "readonly"
    }
}

impl<T> Tag<T>
where
    T: Readonly + 'static,
{
    /// Set the value of the `readonly` attribute.
    pub fn readonly(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Readonly,
{
    /// Get the value of the `readonly` attribute.
    pub fn get_readonly(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `readonly` attribute from the `Tag`.
    pub fn rem_readonly(&self) {
        self.rem_attr(T::attr())
    }
}

impl Readonly for elements::Input {}
impl Readonly for elements::Textarea {}

/// Trait for the attribute `referrerpolicy`
pub trait Referrerpolicy: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "referrerpolicy"
    }
}

impl<T> Tag<T>
where
    T: Referrerpolicy + 'static,
{
    /// Set the value of the `referrerpolicy` attribute.
    pub fn referrerpolicy(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Referrerpolicy,
{
    /// Get the value of the `referrerpolicy` attribute.
    pub fn get_referrerpolicy(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `referrerpolicy` attribute from the `Tag`.
    pub fn rem_referrerpolicy(&self) {
        self.rem_attr(T::attr())
    }
}

impl Referrerpolicy for elements::A {}
impl Referrerpolicy for elements::Area {}
impl Referrerpolicy for elements::Iframe {}
impl Referrerpolicy for elements::Img {}
impl Referrerpolicy for elements::Link {}
impl Referrerpolicy for elements::Script {}

/// Trait for the attribute `rel`
pub trait Rel: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "rel"
    }
}

impl<T> Tag<T>
where
    T: Rel + 'static,
{
    /// Set the value of the `rel` attribute.
    pub fn rel(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Rel,
{
    /// Get the value of the `rel` attribute.
    pub fn get_rel(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `rel` attribute from the `Tag`.
    pub fn rem_rel(&self) {
        self.rem_attr(T::attr())
    }
}

impl Rel for elements::A {}
impl Rel for elements::Area {}
impl Rel for elements::Link {}

/// Trait for the attribute `required`
pub trait Required: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "required"
    }
}

impl<T> Tag<T>
where
    T: Required + 'static,
{
    /// Set the value of the `required` attribute.
    pub fn required(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Required,
{
    /// Get the value of the `required` attribute.
    pub fn get_required(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `required` attribute from the `Tag`.
    pub fn rem_required(&self) {
        self.rem_attr(T::attr())
    }
}

impl Required for elements::Input {}
impl Required for elements::Select {}
impl Required for elements::Textarea {}

/// Trait for the attribute `reversed`
pub trait Reversed: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "reversed"
    }
}

impl<T> Tag<T>
where
    T: Reversed + 'static,
{
    /// Set the value of the `reversed` attribute.
    pub fn reversed(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Reversed,
{
    /// Get the value of the `reversed` attribute.
    pub fn get_reversed(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `reversed` attribute from the `Tag`.
    pub fn rem_reversed(&self) {
        self.rem_attr(T::attr())
    }
}

impl Reversed for elements::Ol {}

/// Trait for the attribute `rows`
pub trait Rows: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "rows"
    }
}

impl<T> Tag<T>
where
    T: Rows + 'static,
{
    /// Set the value of the `rows` attribute.
    pub fn rows(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Rows,
{
    /// Get the value of the `rows` attribute.
    pub fn get_rows(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `rows` attribute from the `Tag`.
    pub fn rem_rows(&self) {
        self.rem_attr(T::attr())
    }
}

impl Rows for elements::Textarea {}

/// Trait for the attribute `rowspan`
pub trait Rowspan: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "rowspan"
    }
}

impl<T> Tag<T>
where
    T: Rowspan + 'static,
{
    /// Set the value of the `rowspan` attribute.
    pub fn rowspan(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Rowspan,
{
    /// Get the value of the `rowspan` attribute.
    pub fn get_rowspan(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `rowspan` attribute from the `Tag`.
    pub fn rem_rowspan(&self) {
        self.rem_attr(T::attr())
    }
}

impl Rowspan for elements::Td {}
impl Rowspan for elements::Th {}

/// Trait for the attribute `sandbox`
pub trait Sandbox: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "sandbox"
    }
}

impl<T> Tag<T>
where
    T: Sandbox + 'static,
{
    /// Set the value of the `sandbox` attribute.
    pub fn sandbox(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Sandbox,
{
    /// Get the value of the `sandbox` attribute.
    pub fn get_sandbox(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `sandbox` attribute from the `Tag`.
    pub fn rem_sandbox(&self) {
        self.rem_attr(T::attr())
    }
}

impl Sandbox for elements::Iframe {}

/// Trait for the attribute `scope`
pub trait Scope: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "scope"
    }
}

impl<T> Tag<T>
where
    T: Scope + 'static,
{
    /// Set the value of the `scope` attribute.
    pub fn scope(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Scope,
{
    /// Get the value of the `scope` attribute.
    pub fn get_scope(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `scope` attribute from the `Tag`.
    pub fn rem_scope(&self) {
        self.rem_attr(T::attr())
    }
}

impl Scope for elements::Th {}

/// Trait for the attribute `scoped`
pub trait Scoped: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "scoped"
    }
}

impl<T> Tag<T>
where
    T: Scoped + 'static,
{
    /// Set the value of the `scoped` attribute.
    pub fn scoped(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Scoped,
{
    /// Get the value of the `scoped` attribute.
    pub fn get_scoped(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `scoped` attribute from the `Tag`.
    pub fn rem_scoped(&self) {
        self.rem_attr(T::attr())
    }
}

impl Scoped for elements::Style {}

/// Trait for the attribute `selected`
pub trait Selected: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "selected"
    }
}

impl<T> Tag<T>
where
    T: Selected + 'static,
{
    /// Set the value of the `selected` attribute.
    pub fn selected(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Selected,
{
    /// Get the value of the `selected` attribute.
    pub fn get_selected(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `selected` attribute from the `Tag`.
    pub fn rem_selected(&self) {
        self.rem_attr(T::attr())
    }
}

impl Selected for elements::Option {}

/// Trait for the attribute `shape`
pub trait Shape: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "shape"
    }
}

impl<T> Tag<T>
where
    T: Shape + 'static,
{
    /// Set the value of the `shape` attribute.
    pub fn shape(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Shape,
{
    /// Get the value of the `shape` attribute.
    pub fn get_shape(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `shape` attribute from the `Tag`.
    pub fn rem_shape(&self) {
        self.rem_attr(T::attr())
    }
}

impl Shape for elements::A {}
impl Shape for elements::Area {}

/// Trait for the attribute `size`
pub trait Size: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "size"
    }
}

impl<T> Tag<T>
where
    T: Size + 'static,
{
    /// Set the value of the `size` attribute.
    pub fn size(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Size,
{
    /// Get the value of the `size` attribute.
    pub fn get_size(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `size` attribute from the `Tag`.
    pub fn rem_size(&self) {
        self.rem_attr(T::attr())
    }
}

impl Size for elements::Input {}
impl Size for elements::Select {}

/// Trait for the attribute `sizes`
pub trait Sizes: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "sizes"
    }
}

impl<T> Tag<T>
where
    T: Sizes + 'static,
{
    /// Set the value of the `sizes` attribute.
    pub fn sizes(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Sizes,
{
    /// Get the value of the `sizes` attribute.
    pub fn get_sizes(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `sizes` attribute from the `Tag`.
    pub fn rem_sizes(&self) {
        self.rem_attr(T::attr())
    }
}

impl Sizes for elements::Link {}
impl Sizes for elements::Img {}
impl Sizes for elements::Source {}

/// Trait for the attribute `slot`
pub trait Slot: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "slot"
    }
}

impl<T> Tag<T>
where
    T: Slot + 'static,
{
    /// Set the value of the `slot` attribute.
    pub fn slot(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Slot,
{
    /// Get the value of the `slot` attribute.
    pub fn get_slot(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `slot` attribute from the `Tag`.
    pub fn rem_slot(&self) {
        self.rem_attr(T::attr())
    }
}

impl<T> Slot for T where T: elements::HtmlElement {}

/// Trait for the attribute `span`
pub trait Span: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "span"
    }
}

impl<T> Tag<T>
where
    T: Span + 'static,
{
    /// Set the value of the `span` attribute.
    pub fn span(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Span,
{
    /// Get the value of the `span` attribute.
    pub fn get_span(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `span` attribute from the `Tag`.
    pub fn rem_span(&self) {
        self.rem_attr(T::attr())
    }
}

impl Span for elements::Col {}
impl Span for elements::Colgroup {}

/// Trait for the attribute `spellcheck`
pub trait Spellcheck: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "spellcheck"
    }
}

impl<T> Tag<T>
where
    T: Spellcheck + 'static,
{
    /// Set the value of the `spellcheck` attribute.
    pub fn spellcheck(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Spellcheck,
{
    /// Get the value of the `spellcheck` attribute.
    pub fn get_spellcheck(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `spellcheck` attribute from the `Tag`.
    pub fn rem_spellcheck(&self) {
        self.rem_attr(T::attr())
    }
}

impl<T> Spellcheck for T where T: elements::HtmlElement {}

/// Trait for the attribute `src`
pub trait Src: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "src"
    }
}

impl<T> Tag<T>
where
    T: Src + 'static,
{
    /// Set the value of the `src` attribute.
    pub fn src(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Src,
{
    /// Get the value of the `src` attribute.
    pub fn get_src(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `src` attribute from the `Tag`.
    pub fn rem_src(&self) {
        self.rem_attr(T::attr())
    }
}

impl Src for elements::Audio {}
impl Src for elements::Embed {}
impl Src for elements::Iframe {}
impl Src for elements::Img {}
impl Src for elements::Input {}
impl Src for elements::Script {}
impl Src for elements::Source {}
impl Src for elements::Track {}
impl Src for elements::Video {}

/// Trait for the attribute `srcdoc`
pub trait Srcdoc: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "srcdoc"
    }
}

impl<T> Tag<T>
where
    T: Srcdoc + 'static,
{
    /// Set the value of the `srcdoc` attribute.
    pub fn srcdoc(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Srcdoc,
{
    /// Get the value of the `srcdoc` attribute.
    pub fn get_srcdoc(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `srcdoc` attribute from the `Tag`.
    pub fn rem_srcdoc(&self) {
        self.rem_attr(T::attr())
    }
}

impl Srcdoc for elements::Iframe {}

/// Trait for the attribute `srclang`
pub trait Srclang: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "srclang"
    }
}

impl<T> Tag<T>
where
    T: Srclang + 'static,
{
    /// Set the value of the `srclang` attribute.
    pub fn srclang(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Srclang,
{
    /// Get the value of the `srclang` attribute.
    pub fn get_srclang(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `srclang` attribute from the `Tag`.
    pub fn rem_srclang(&self) {
        self.rem_attr(T::attr())
    }
}

impl Srclang for elements::Track {}

/// Trait for the attribute `srcset`
pub trait Srcset: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "srcset"
    }
}

impl<T> Tag<T>
where
    T: Srcset + 'static,
{
    /// Set the value of the `srcset` attribute.
    pub fn srcset(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Srcset,
{
    /// Get the value of the `srcset` attribute.
    pub fn get_srcset(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `srcset` attribute from the `Tag`.
    pub fn rem_srcset(&self) {
        self.rem_attr(T::attr())
    }
}

impl Srcset for elements::Img {}
impl Srcset for elements::Source {}

/// Trait for the attribute `start`
pub trait Start: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "start"
    }
}

impl<T> Tag<T>
where
    T: Start + 'static,
{
    /// Set the value of the `start` attribute.
    pub fn start(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Start,
{
    /// Get the value of the `start` attribute.
    pub fn get_start(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `start` attribute from the `Tag`.
    pub fn rem_start(&self) {
        self.rem_attr(T::attr())
    }
}

impl Start for elements::Ol {}

/// Trait for the attribute `step`
pub trait Step: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "step"
    }
}

impl<T> Tag<T>
where
    T: Step + 'static,
{
    /// Set the value of the `step` attribute.
    pub fn step(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Step,
{
    /// Get the value of the `step` attribute.
    pub fn get_step(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `step` attribute from the `Tag`.
    pub fn rem_step(&self) {
        self.rem_attr(T::attr())
    }
}

impl Step for elements::Input {}

/// Trait for the attribute `style`
pub trait Style: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "style"
    }
}

impl<T> Tag<T>
where
    T: Style + 'static,
{
    /// Set the value of the `style` attribute.
    pub fn style(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Style,
{
    /// Get the value of the `style` attribute.
    pub fn get_style(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `style` attribute from the `Tag`.
    pub fn rem_style(&self) {
        self.rem_attr(T::attr())
    }
}

impl<T> Style for T where T: elements::HtmlElement {}

/// Trait for the attribute `summary`
pub trait Summary: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "summary"
    }
}

impl<T> Tag<T>
where
    T: Summary + 'static,
{
    /// Set the value of the `summary` attribute.
    pub fn summary(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Summary,
{
    /// Get the value of the `summary` attribute.
    pub fn get_summary(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `summary` attribute from the `Tag`.
    pub fn rem_summary(&self) {
        self.rem_attr(T::attr())
    }
}

impl Summary for elements::Table {}

/// Trait for the attribute `tabindex`
pub trait Tabindex: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "tabindex"
    }
}

impl<T> Tag<T>
where
    T: Tabindex + 'static,
{
    /// Set the value of the `tabindex` attribute.
    pub fn tabindex(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Tabindex,
{
    /// Get the value of the `tabindex` attribute.
    pub fn get_tabindex(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `tabindex` attribute from the `Tag`.
    pub fn rem_tabindex(&self) {
        self.rem_attr(T::attr())
    }
}

impl<T> Tabindex for T where T: elements::HtmlElement {}

/// Trait for the attribute `target`
pub trait Target: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "target"
    }
}

impl<T> Tag<T>
where
    T: Target + 'static,
{
    /// Set the value of the `target` attribute.
    pub fn target(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Target,
{
    /// Get the value of the `target` attribute.
    pub fn get_target(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `target` attribute from the `Tag`.
    pub fn rem_target(&self) {
        self.rem_attr(T::attr())
    }
}

impl Target for elements::A {}
impl Target for elements::Area {}
impl Target for elements::Base {}
impl Target for elements::Form {}

/// Trait for the attribute `title`
pub trait Title: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "title"
    }
}

impl<T> Tag<T>
where
    T: Title + 'static,
{
    /// Set the value of the `title` attribute.
    pub fn title(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Title,
{
    /// Get the value of the `title` attribute.
    pub fn get_title(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `title` attribute from the `Tag`.
    pub fn rem_title(&self) {
        self.rem_attr(T::attr())
    }
}

impl<T> Title for T where T: elements::HtmlElement {}

/// Trait for the attribute `translate`
pub trait Translate: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "translate"
    }
}

impl<T> Tag<T>
where
    T: Translate + 'static,
{
    /// Set the value of the `translate` attribute.
    pub fn translate(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Translate,
{
    /// Get the value of the `translate` attribute.
    pub fn get_translate(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `translate` attribute from the `Tag`.
    pub fn rem_translate(&self) {
        self.rem_attr(T::attr())
    }
}

impl<T> Translate for T where T: elements::HtmlElement {}

/// Trait for the attribute `type`
pub trait Type: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "type"
    }
}

impl<T> Tag<T>
where
    T: Type + 'static,
{
    /// Set the value of the `type` attribute.
    pub fn set_type(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Type,
{
    /// Get the value of the `type` attribute.
    pub fn get_set_type(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `type` attribute from the `Tag`.
    pub fn rem_set_type(&self) {
        self.rem_attr(T::attr())
    }
}

impl Type for elements::Button {}
impl Type for elements::Input {}
impl Type for elements::Embed {}
impl Type for elements::Object {}
impl Type for elements::Script {}
impl Type for elements::Source {}
impl Type for elements::Style {}
impl Type for elements::Menu {}

/// Trait for the attribute `usemap`
pub trait Usemap: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "usemap"
    }
}

impl<T> Tag<T>
where
    T: Usemap + 'static,
{
    /// Set the value of the `usemap` attribute.
    pub fn usemap(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Usemap,
{
    /// Get the value of the `usemap` attribute.
    pub fn get_usemap(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `usemap` attribute from the `Tag`.
    pub fn rem_usemap(&self) {
        self.rem_attr(T::attr())
    }
}

impl Usemap for elements::Img {}
impl Usemap for elements::Input {}
impl Usemap for elements::Object {}

/// Trait for the attribute `value`
pub trait Value: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "value"
    }
}

impl<T> Tag<T>
where
    T: Value + 'static,
{
    /// Set the value of the `value` attribute.
    pub fn value(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Value,
{
    /// Get the value of the `value` attribute.
    pub fn get_value(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `value` attribute from the `Tag`.
    pub fn rem_value(&self) {
        self.rem_attr(T::attr())
    }
}

impl Value for elements::Button {}
impl Value for elements::Data {}
impl Value for elements::Input {}
impl Value for elements::Li {}
impl Value for elements::Meter {}
impl Value for elements::Option {}
impl Value for elements::Progress {}
impl Value for elements::Param {}

/// Trait for the attribute `width`
pub trait Width: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "width"
    }
}

impl<T> Tag<T>
where
    T: Width + 'static,
{
    /// Set the value of the `width` attribute.
    pub fn width(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Width,
{
    /// Get the value of the `width` attribute.
    pub fn get_width(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `width` attribute from the `Tag`.
    pub fn rem_width(&self) {
        self.rem_attr(T::attr())
    }
}

impl Width for elements::Canvas {}
impl Width for elements::Embed {}
impl Width for elements::Iframe {}
impl Width for elements::Img {}
impl Width for elements::Input {}
impl Width for elements::Object {}
impl Width for elements::Video {}

/// Trait for the attribute `wrap`
pub trait Wrap: elements::HtmlElement {
    /// Get the name of the attribute
    fn attr() -> &'static str {
        "wrap"
    }
}

impl<T> Tag<T>
where
    T: Wrap + 'static,
{
    /// Set the value of the `wrap` attribute.
    pub fn wrap(self, value: impl value::Value) -> Self {
        self.attr(T::attr(), value)
    }
}

impl<T> Tag<T>
where
    T: Wrap,
{
    /// Get the value of the `wrap` attribute.
    pub fn get_wrap(&self) -> Option<String> {
        self.get_attr(T::attr())
    }

    /// Remove the `wrap` attribute from the `Tag`.
    pub fn rem_wrap(&self) {
        self.rem_attr(T::attr())
    }
}

impl Wrap for elements::Textarea {}
