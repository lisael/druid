use druid::widget::prelude::*;
use druid::widget::{Checkbox, Either, Flex, Label, TextBox};
use druid::{AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc};

#[derive(Clone, Default, Data, Lens)]
struct AppState {
    data1: String,
    edit1: bool,
    data2: String,
    edit2: bool,
}

struct Observer<T> {
    inner: Box<dyn Widget<T>>,
}

impl<T> Widget<T> for Observer<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        if let Event::Internal(_) = event {
            eprintln!("{:?}", event);
        }
        self.inner.event(ctx, event, data, env)
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        eprintln!("{:?}", event);
        self.inner.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.inner.update(ctx, old_data, data, env)
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.inner.paint(ctx, data, env)
    }
}

impl<T> Observer<T> {
    fn new<W>(inner: W) -> Self
    where
        W: Widget<T> + 'static,
    {
        Self {
            inner: Box::new(inner),
        }
    }
}

fn ui_builder() -> impl Widget<AppState> {
    let button1 = Checkbox::new("Edit 1").lens(AppState::edit1).padding(5.0);
    let button2 = Checkbox::new("Edit 2").lens(AppState::edit2).padding(5.0);

    let either1 = Either::new(
        |state, _env| state.edit1,
        TextBox::new().lens(AppState::data1),
        Label::dynamic(|state: &AppState, _env| state.data1.clone()).padding(5.0),
    );
    let either2 = Either::new(
        |state, _env| state.edit2,
        TextBox::new().lens(AppState::data2),
        Label::dynamic(|state: &AppState, _env| state.data2.clone()).padding(5.0),
    );
    Flex::column()
        .with_child(button1)
        .with_child(button2)
        .with_child(Observer::new(either1))
        .with_child(Observer::new(either2))
}

pub fn main() {
    let main_window = WindowDesc::new(ui_builder()).title("Switcheroo");
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(AppState::default())
        .expect("launch failed");
}
