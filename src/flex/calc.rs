use crate::flex::r#type::{Axis, Node, Rect};

#[allow(dead_code)]
pub fn calc(node: &mut Node, aw: f32, ah: f32, ox: f32, oy: f32) {
    node.rect = Rect::new(ox, oy, aw, ah);
    if node.kids.is_empty() {
        return;
    }

    let ix = ox + node.pad;
    let iy = oy + node.pad;
    let iw = (aw - node.pad * 2.0).max(0.0);
    let ih = (ah - node.pad * 2.0).max(0.0);

    match node.axis {
        Axis::Row => {
            let mut fix_sum = 0.0;
            let mut flx_sum = 0.0;
            for k in &node.kids {
                if let Some(f) = k.fix {
                    fix_sum += f;
                } else {
                    flx_sum += k.grow;
                }
            }
            let rem_w = (iw - fix_sum).max(0.0);
            let mut cx = ix;
            for k in &mut node.kids {
                let kw = if let Some(f) = k.fix {
                    f
                } else if flx_sum > 0.0 {
                    (k.grow / flx_sum) * rem_w
                } else {
                    0.0
                };
                calc(k, kw, ih, cx, iy);
                cx += kw;
            }
        }
        Axis::Col => {
            let mut fix_sum = 0.0;
            let mut flx_sum = 0.0;
            for k in &node.kids {
                if let Some(f) = k.fix {
                    fix_sum += f;
                } else {
                    flx_sum += k.grow;
                }
            }
            let rem_h = (ih - fix_sum).max(0.0);
            let mut cy = iy;
            for k in &mut node.kids {
                let kh = if let Some(f) = k.fix {
                    f
                } else if flx_sum > 0.0 {
                    (k.grow / flx_sum) * rem_h
                } else {
                    0.0
                };
                calc(k, iw, kh, ix, cy);
                cy += kh;
            }
        }
    }
}
