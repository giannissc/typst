//! Inline- and block-level containers.

use super::prelude::*;

/// Size content and place it into a paragraph.
pub struct BoxNode;

#[class]
impl BoxNode {
    fn construct(_: &mut EvalContext, args: &mut Args) -> TypResult<Template> {
        let width = args.named("width")?;
        let height = args.named("height")?;
        let body: LayoutNode = args.find().unwrap_or_default();
        Ok(Template::inline(body.sized(Spec::new(width, height))))
    }
}

/// Place content into a separate flow.
pub struct BlockNode;

#[class]
impl BlockNode {
    fn construct(_: &mut EvalContext, args: &mut Args) -> TypResult<Template> {
        Ok(Template::Block(args.find().unwrap_or_default()))
    }
}