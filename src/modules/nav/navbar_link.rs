use yew::{classes, Classes};
use yew_router::Routable;

#[derive(PartialEq, Clone, Copy)]
pub struct BBNavbarLink<T>
where
    T: Routable + 'static,
{
    pub to: T,
    pub label: &'static str,
    pub active: bool,
}

impl<T> BBNavbarLink<T>
where
    T: Routable + 'static,
{
    pub fn classes(&self) -> Classes {
        let active = if self.active { Some("active") } else { None };

        classes!("nav-link", active)
    }
}

pub struct BBNavbarLinkBuilder<T>
where
    T: Routable + 'static,
{
    pub to: Option<T>,
    pub label: &'static str,
    pub active: bool,
}

impl<T: Routable + 'static> BBNavbarLinkBuilder<T> {
    pub fn new() -> Self {
        Self {
            to: None,
            label: "",
            active: false,
        }
    }

    pub fn to(mut self, to: T) -> Self {
        self.to = Some(to);
        self
    }

    pub fn label(mut self, label: &'static str) -> Self {
        self.label = label;
        self
    }

    pub fn active(mut self) -> Self {
        self.active = true;
        self
    }

    pub fn build(self) -> Option<BBNavbarLink<T>> {
        let label = self.label;
        let active = self.active;

        self.to.map(move |to| BBNavbarLink { to, label, active })
    }
}
