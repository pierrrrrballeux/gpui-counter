mod counter;

use counter::Counter;
use gpui::{
  prelude::*, px, size, App, AppContext, Bounds, WindowBounds, WindowOptions
};

fn main(){
  App::new().run(|cx:&mut AppContext | {
    let options =  WindowOptions {
      window_bounds: Some(WindowBounds::Windowed(Bounds::centered(None, size(px(500.0), px(500.0)), cx))),
      is_movable: true,
      ..Default::default()
    };
    cx.open_window(
      options,
      |cx|{
        cx.new_view(|_cx| Counter { count: 0 })
      }
    ).unwrap();
  });  
  
}