use druid::widget::{Align, Flex, Label, Padding};
use druid::{Widget, WidgetExt};

pub(crate) struct Node {
    pub(crate) value: i64,
    pub(crate) left: Option<Box<Node>>,
    pub(crate) right: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: i64, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Self {
        Node {
            value,
            left,
            right,
        }
    }

    pub fn build_node_widget(&self) -> impl Widget<()> {
        let mut widget = Flex::column();

        widget.add_child(
            Padding::new(
                10.0,
                Label::new(format!("{}", self.value)).align_horizontal(druid::UnitPoint::CENTER),
            )
                .fix_height(30.0),
        );

        let mut children_widget = Flex::row();
        if let Some(left) = &self.left {
            children_widget.add_child(left.build_node_widget());
        }
        if let Some(right) = &self.right {
            children_widget.add_child(right.build_node_widget());
        }

        widget.add_child(children_widget);

        Align::centered(widget)
    }
}