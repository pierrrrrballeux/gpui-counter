use gpui::{
  div, prelude::*, px, rgb, ClickEvent, SharedString, ViewContext
};
pub struct Counter {
  pub count: i32,
}

impl Counter {
  fn increment(&mut self,_event: &ClickEvent,cx: &mut ViewContext<Self>){
    self.count += 1;
    cx.notify()
  }

  fn render_button(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
    let text = "Click Me !";
    div()
      .id(SharedString::from("Counter".to_string()))
      .flex()
      .px_4()
      .py_2()
      .justify_center()
      .items_center()
      .bg(rgb(0x00ffff))
      .rounded(px(4.0))
      .child(text.to_string())
      .on_click(cx.listener(Self::increment))
  }
}

impl Render for Counter {
  fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
    div()
      .flex()
      .flex_col()
      .justify_center()
      .items_center()
      .size_full()
      .bg(gpui::white())
      .child(format!("{:04}", self.count))
      .child(self.render_button(cx))
  }
}
