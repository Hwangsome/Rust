//! Downcasting 完整例子：自定义 trait + `as_any()` 让下游能转回具体类型。
//!
//! 惯用法：
//!
//! ```ignore
//! trait Widget: Any {
//!     fn as_any(&self) -> &dyn Any;
//! }
//! ```
//!
//! 这样 `&dyn Widget` 的使用者能通过 `widget.as_any().downcast_ref::<Button>()` 拿回具体类型。

use std::any::Any;

trait Widget: Any {
    fn name(&self) -> &str;
    fn as_any(&self) -> &dyn Any;
}

struct Button { label: String }
struct Label  { text:  String }

impl Widget for Button {
    fn name(&self) -> &str { "Button" }
    fn as_any(&self) -> &dyn Any { self }
}

impl Widget for Label {
    fn name(&self) -> &str { "Label" }
    fn as_any(&self) -> &dyn Any { self }
}

pub fn run() {
    println!("== Downcasting Example ==");

    let widgets: Vec<Box<dyn Widget>> = vec![
        Box::new(Button { label: "Submit".into() }),
        Box::new(Label  { text:  "Ready".into()  }),
    ];

    for w in &widgets {
        println!("  {} sees:", w.name());
        if let Some(b) = w.as_any().downcast_ref::<Button>() {
            println!("    -> Button with label '{}'", b.label);
        } else if let Some(l) = w.as_any().downcast_ref::<Label>() {
            println!("    -> Label with text '{}'", l.text);
        }
    }
    println!();
}
