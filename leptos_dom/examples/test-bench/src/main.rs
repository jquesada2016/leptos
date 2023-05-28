#![allow(warnings)]

#[macro_use]
extern crate tracing;

mod utils;

use leptos::*;
use tracing::field::debug;
use tracing_subscriber::util::SubscriberInitExt;

fn main() {
  console_error_panic_hook::set_once();

  tracing_subscriber::fmt()
    .with_max_level(tracing::Level::TRACE)
    .without_time()
    .with_file(true)
    .with_line_number(true)
    .with_target(false)
    .with_writer(utils::MakeConsoleWriter)
    .with_ansi(false)
    .pretty()
    .finish()
    .init();

  mount_to_body(view_fn);
}

fn view_fn(cx: Scope) -> impl IntoView {
  let (list, set_list) = create_signal(cx, vec![2]);//vec![1, 2, 3, 4, 5]);

  request_animation_frame(move || {
    set_list(vec![1, 2]);//vec![0, 1, 3, 6, 4, 5, 2, 7])
  });

  view! { cx,
      <h2>"Passing Tests"</h2>
      <ul>
        /* These work! */
        <Test from=&[1] to=&[]/>
        <Test from=&[1, 2] to=&[]/>
        <Test from=&[1, 2, 3] to=&[]/>
        <Test from=&[] to=&[1]/>
        <Test from=&[1, 2] to=&[1]/>
        <Test from=&[2, 1] to=&[1]/>
        <Test from=&[1] to=&[1, 2]/>
        <Test from=&[2] to=&[1, 2]/>
        <Test from=&[1, 2, 3] to=&[1, 2]/>
        <Test from=&[] to=&[1, 2, 3]/>
        <Test from=&[2] to=&[1, 2, 3]/>
        <Test from=&[1] to=&[1, 2, 3]/>
        <Test from=&[3] to=&[1, 2, 3]/>
        <Test from=&[3, 1] to=&[1, 2, 3]/>
        <Test from=&[1, 3, 2] to=&[1, 2, 3]/>
        <Test from=&[2, 1, 3] to=&[1, 2, 3]/>
        <Test from=&[2, 1, 3] to=&[1, 2, 3]/>
        <Test from=&[3, 2, 1] to=&[1, 2, 3]/>  
         
        <Test from=&[1, 4, 3, 2, 5] to=&[1, 2, 3, 4, 5]/>
      </ul>
      <h2>"Broken Tests"</h2>
      <ul>
        // TODO remove followed by move
        // seems to None-ify it in children but now the indices are off,
        // am probably missing a search for next non-None at one or more points
        <Test from=&[3, 2, 4, 1] to=&[1, 2, 3]/> 
      </ul>
      <h2>"Todo (non-dense moves)"</h2>
      <ul>
        // TODO these generate both a dense anda non-dense move
        <Test from=&[1, 4, 2, 3] to=&[1, 2, 3, 4]/>   
        <Test from=&[4, 5, 3, 1, 2] to=&[1, 2, 3, 4, 5]/>           
      </ul>
  }
}

#[component]
fn Test(cx: Scope, from: &'static [usize], to: &'static [usize]) -> impl IntoView {
  let (list, set_list) = create_signal(cx, from.to_vec());
  request_animation_frame(move || {
    set_list(to.to_vec());
  });

  view! { cx, 
    <li>
        <For
            each=list
            key=|i| *i
            view=|cx, i| {
                view! { cx, <span>{i}</span> }
            }
        />
      /* <p>
        "Pre | "
        <For
            each=list
            key=|i| *i
            view=|cx, i| {
                view! { cx, <span>{i}</span> }
            }
        />
        " | Post"
      </p> */
    </li>
  }
}