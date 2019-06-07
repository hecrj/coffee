use stretch::{geometry, result};

use crate::graphics::Point;
use crate::ui::core::{Event, Hasher, Layout, MouseCursor, Node, Widget};

/// A generic [`Widget`].
///
/// If you have a widget, you should be able to use `widget.into()` to turn it
/// into an [`Element`].
///
/// [`Widget`]: trait.Widget.html
/// [`Element`]: struct.Element.html
pub struct Element<'a, Message, Renderer> {
    pub(crate) widget: Box<Widget<Message, Renderer> + 'a>,
}

impl<'a, Message, Renderer> Element<'a, Message, Renderer> {
    /// Create a new [`Element`] containing the given [`Widget`].
    ///
    /// [`Element`]: struct.Element.html
    /// [`Widget`]: trait.Widget.html
    pub fn new(
        widget: impl Widget<Message, Renderer> + 'a,
    ) -> Element<'a, Message, Renderer> {
        Element {
            widget: Box::new(widget),
        }
    }

    /// Applies a transformation to the produced message of the [`Element`].
    ///
    /// This method is useful when you want to decouple different parts of your
    /// UI.
    ///
    /// [`Element`]: struct.Element.html
    ///
    /// # Example
    /// Let's say that we want to have a main menu and a gameplay overlay in our
    /// game. We can decouple the interfaces nicely using modules and nested
    /// messages:
    ///
    /// ```
    /// mod main_menu {
    ///     use coffee::ui::core::Element;
    ///     # use coffee::ui::Column;
    ///     use coffee::ui::Renderer;
    ///
    ///     pub struct MainMenu {
    ///         // Our main menu state here...
    ///         // Probably a bunch of `button::State` and other stuff.
    ///     }
    ///
    ///     #[derive(Debug, Clone, Copy)]
    ///     pub enum Message {
    ///         // The different interactions of the main menu here...
    ///     }
    ///
    ///     impl MainMenu {
    ///         // We probably would have our `update` function here too...
    ///
    ///         pub fn layout(&mut self) -> Element<Message, Renderer> {
    ///             // We show the main menu here...
    ///             // The returned `Element` produces `main_menu::Message`
    ///             # Column::new().into()
    ///         }
    ///     }
    /// }
    ///
    /// mod gameplay_overlay {
    ///     // Analogous to the `main_menu` module
    /// #    use coffee::ui::core::Element;
    /// #    use coffee::ui::Column;
    /// #    use coffee::ui::Renderer;
    /// #
    /// #    pub struct GameplayOverlay { /* ... */ }
    /// #
    /// #    #[derive(Debug, Clone, Copy)]
    /// #    pub enum Message { /* ... */ }
    /// #
    /// #    impl GameplayOverlay {
    /// #        pub fn layout(&mut self) -> Element<Message, Renderer> {
    /// #            // ...
    /// #            Column::new().into()
    /// #        }
    /// #    }
    /// }
    ///
    /// use coffee::ui::core::Element;
    /// use coffee::ui::Renderer;
    /// use main_menu::MainMenu;
    /// use gameplay_overlay::GameplayOverlay;
    ///
    /// // The state of our UI
    /// enum State {
    ///     MainMenu(MainMenu),
    ///     GameplayOverlay(GameplayOverlay),
    ///     // ...
    /// }
    ///
    /// // The messages of our UI
    /// // We simply nest the messages here
    /// #[derive(Debug, Clone, Copy)]
    /// enum Message {
    ///     MainMenu(main_menu::Message),
    ///     GameplayOverlay(gameplay_overlay::Message),
    ///     // ...
    /// }
    ///
    /// // We show the UI here, transforming the local messages of each branch
    /// // into the global `Message` type as needed.
    /// pub fn layout(state: &mut State) -> Element<Message, Renderer> {
    ///     match state {
    ///         State::MainMenu(main_menu) => {
    ///             main_menu.layout().map(Message::MainMenu)
    ///         }
    ///         State::GameplayOverlay(gameplay_overlay) => {
    ///             gameplay_overlay.layout().map(Message::GameplayOverlay)
    ///         }
    ///         // ...
    ///     }
    /// }
    /// ```
    ///
    /// This way, neither `main_menu` nor `gameplay_overlay` know anything about
    /// the global `Message` type and it allows the user of these modules to
    /// wire them up in any way they want.
    pub fn map<F, B>(self, f: F) -> Element<'a, B, Renderer>
    where
        Message: 'static + Copy,
        Renderer: 'a,
        B: 'static,
        F: 'static + Fn(Message) -> B,
    {
        Element {
            widget: Box::new(Map::new(self.widget, f)),
        }
    }

    pub(crate) fn compute_layout(&self, renderer: &Renderer) -> result::Layout {
        let node = self.widget.node(renderer);

        node.0.compute_layout(geometry::Size::undefined()).unwrap()
    }

    pub(crate) fn hash(&self, state: &mut Hasher) {
        self.widget.hash(state);
    }
}

pub struct Map<'a, A, B, Renderer> {
    widget: Box<Widget<A, Renderer> + 'a>,
    mapper: Box<Fn(A) -> B>,
}

impl<'a, A, B, Renderer> Map<'a, A, B, Renderer> {
    pub fn new<F>(
        widget: Box<Widget<A, Renderer> + 'a>,
        mapper: F,
    ) -> Map<'a, A, B, Renderer>
    where
        F: 'static + Fn(A) -> B,
    {
        Map {
            widget,
            mapper: Box::new(mapper),
        }
    }
}

impl<'a, A, B, Renderer> Widget<B, Renderer> for Map<'a, A, B, Renderer>
where
    A: Copy,
{
    fn node(&self, renderer: &Renderer) -> Node {
        self.widget.node(renderer)
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout,
        cursor_position: Point,
        messages: &mut Vec<B>,
    ) {
        let mut original_messages = Vec::new();

        self.widget.on_event(
            event,
            layout,
            cursor_position,
            &mut original_messages,
        );

        original_messages
            .iter()
            .cloned()
            .for_each(|message| messages.push((self.mapper)(message)));
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        layout: Layout,
        cursor_position: Point,
    ) -> MouseCursor {
        self.widget.draw(renderer, layout, cursor_position)
    }

    fn hash(&self, state: &mut Hasher) {
        self.widget.hash(state);
    }
}
