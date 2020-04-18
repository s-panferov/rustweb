use wasm_bindgen::JsValue;
use web_sys::Node;

use crate::component::Component;
use crate::mount::Mount;

use super::{utils, Html};

pub use super::{attr::DefaultAttributes, text::*};

#[derive(Debug)]
pub struct Div {}

pub fn div() -> Div {
    Div {}
}

#[derive(Default, Debug)]
pub struct HtmlProps {
    pub attributes: DefaultAttributes,
}

macro_rules! attr {
    ($el:expr, $x:expr, $eval:expr, $where:expr) => {
        let prop = &$where;
        if !prop.is_empty() {
            let value = prop.observe($eval);
            $el.set_attribute(
                $x,
                &(*value)
                    .as_ref()
                    .map(|v| v.to_string())
                    .unwrap_or(String::from("")),
            )?
        }
    };
}

impl Component for Div {
    type Props = HtmlProps;
    type Target = Html;

    fn mount(&self, ctx: &mut Mount<Html>) -> Result<Node, JsValue> {
        let el = ctx.doc.create_element("div")?;

        ctx.add_node(&el);
        utils::mount_children(ctx, &el)?;
        ctx.reactions.add(self, {
            let el = el.clone();
            move |ctx| {
                let attrs = &ctx.props.attributes;
                attr!(el, "class", ctx.eval, attrs.class);
                attr!(el, "style", ctx.eval, attrs.style);
                attr!(el, "contenteditable", ctx.eval, attrs.contenteditable);
                Ok(())
            }
        });
        Ok(el.into())
    }
}
