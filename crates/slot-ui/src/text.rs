use std::sync::OnceLock;

use fontdue::{Font, FontSettings};

const LABEL_TTF: &[u8] = include_bytes!("../assets/label.ttf");
// Interface-only subset of Plix Bold. The full card font and its OFL text ship under
// card/System/fonts; source: https://github.com/cathree3/Plix.
#[cfg(feature = "lang-zh-cn")]
const ZH_CN_TTF: &[u8] = include_bytes!("../assets/zh-cn.otf");

static USER_FONT: OnceLock<Font> = OnceLock::new();
static EMBEDDED_FONT: OnceLock<Font> = OnceLock::new();
#[cfg(feature = "lang-zh-cn")]
static ZH_CN_FONT: OnceLock<Font> = OnceLock::new();

pub struct Layout {
    pub lines: Vec<String>,
    pub px: f32,
    pub tracking: f32,
}

/// Installs a card-provided TTF, OTF or TTC face. Invalid data and later calls are ignored.
pub fn set_font(bytes: Vec<u8>) {
    let data: &'static [u8] = Box::leak(bytes.into_boxed_slice());
    if let Ok(font) = Font::from_bytes(data, FontSettings::default()) {
        let _ = USER_FONT.set(font);
    }
}

fn embedded_font() -> &'static Font {
    EMBEDDED_FONT.get_or_init(|| {
        Font::from_bytes(LABEL_TTF, FontSettings::default()).expect("embedded label.ttf must parse")
    })
}

#[cfg(feature = "lang-zh-cn")]
fn zh_cn_font() -> &'static Font {
    ZH_CN_FONT.get_or_init(|| {
        Font::from_bytes(ZH_CN_TTF, FontSettings::default()).expect("embedded zh-cn.otf must parse")
    })
}

fn has_glyph(font: &Font, ch: char) -> bool {
    font.rasterize(ch, 1.0).0.width > 0
}

fn font_for(ch: char) -> &'static Font {
    if let Some(font) = USER_FONT.get() {
        if has_glyph(font, ch) {
            return font;
        }
    }
    #[cfg(feature = "lang-zh-cn")]
    if has_glyph(zh_cn_font(), ch) {
        return zh_cn_font();
    }
    embedded_font()
}

pub fn label_font() -> Option<&'static Font> {
    if let Some(font) = USER_FONT.get() {
        return Some(font);
    }
    #[cfg(feature = "lang-zh-cn")]
    {
        Some(zh_cn_font())
    }
    #[cfg(not(feature = "lang-zh-cn"))]
    {
        Some(embedded_font())
    }
}

pub fn fit(
    font: &Font,
    text: &str,
    max_w: f32,
    max_lines: usize,
    max_px: f32,
    min_px: f32,
) -> Layout {
    fit_box(font, text, max_w, f32::INFINITY, max_lines, max_px, min_px)
}

#[allow(clippy::too_many_arguments)]
pub fn fit_box(
    font: &Font,
    text: &str,
    max_w: f32,
    max_h: f32,
    max_lines: usize,
    max_px: f32,
    min_px: f32,
) -> Layout {
    let upper = text.to_uppercase();
    let mut px = max_px;
    while px > min_px {
        let tracking = tracking_for(px);
        let lines = wrap(font, &upper, px, tracking, max_w, false);
        if lines.len() <= max_lines
            && lines
                .iter()
                .all(|l| line_width(font, l, px, tracking) <= max_w)
            && block_height(font, px, lines.len()) <= max_h
        {
            return Layout {
                lines,
                px,
                tracking,
            };
        }
        px -= 1.0;
    }
    let tracking = tracking_for(min_px);
    Layout {
        lines: wrap(font, &upper, min_px, tracking, max_w, true)
            .into_iter()
            .take(max_lines)
            .collect(),
        px: min_px,
        tracking,
    }
}

pub fn draw_centred(dst: &mut [u8], dst_w: u32, dst_h: u32, layout: &Layout, colour: [u8; 3]) {
    for (i, a) in coverage(dst_w, dst_h, layout).into_iter().enumerate() {
        if a > 0 {
            blend(&mut dst[i * 4..i * 4 + 4], a as u32, colour);
        }
    }
}

pub fn coverage(dst_w: u32, dst_h: u32, layout: &Layout) -> Vec<u8> {
    let mut out = vec![0u8; (dst_w * dst_h) as usize];
    let Some(font) = label_font() else { return out };
    let Some(vm) = font.horizontal_line_metrics(layout.px) else {
        return out;
    };
    let line_h = vm.new_line_size - vm.line_gap;
    let block_h = line_h * layout.lines.len() as f32;
    let mut baseline = (dst_h as f32 - block_h) / 2.0 + vm.ascent;

    for line in &layout.lines {
        let mut pen = (dst_w as f32 - line_width(font, line, layout.px, layout.tracking)) / 2.0;
        let mut prev = None;
        for ch in line.chars() {
            pen += pair_tracking(prev, ch, layout.tracking);
            let glyph_font = font_for(ch);
            let (m, cov) = glyph_font.rasterize(ch, layout.px);
            stamp(
                &mut out,
                dst_w,
                dst_h,
                (pen + m.xmin as f32).round() as i32,
                (baseline - (m.height as f32 + m.ymin as f32)).round() as i32,
                &cov,
                m.width as u32,
                m.height as u32,
            );
            pen += m.advance_width;
            prev = Some(ch);
        }
        baseline += line_h;
    }
    out
}

fn block_height(font: &Font, px: f32, lines: usize) -> f32 {
    match font.horizontal_line_metrics(px) {
        Some(vm) => (vm.new_line_size - vm.line_gap) * lines as f32,
        None => 0.0,
    }
}

pub fn line_width(_font: &Font, line: &str, px: f32, tracking: f32) -> f32 {
    let mut w = 0.0;
    let mut prev = None;
    for ch in line.chars() {
        w += pair_tracking(prev, ch, tracking) + font_for(ch).metrics(ch, px).advance_width;
        prev = Some(ch);
    }
    w
}

pub(crate) fn tracking_for(px: f32) -> f32 {
    (px * 0.10).round()
}

pub(crate) fn is_cjk(c: char) -> bool {
    matches!(c as u32,
        0x1100..=0x11FF
        | 0x2E80..=0x2EFF
        | 0x3000..=0x303F
        | 0x3040..=0x30FF
        | 0x3130..=0x318F
        | 0x31C0..=0x31EF
        | 0x3400..=0x4DBF
        | 0x4E00..=0x9FFF
        | 0xA960..=0xA97F
        | 0xAC00..=0xD7FF
        | 0xF900..=0xFAFF
        | 0xFE30..=0xFE4F
        | 0xFF00..=0xFF60
        | 0xFFE0..=0xFFE6
        | 0x20000..=0x2FA1F)
}

fn pair_tracking(prev: Option<char>, ch: char, tracking: f32) -> f32 {
    if tracking == 0.0 || prev.is_some_and(is_cjk) || is_cjk(ch) {
        0.0
    } else {
        tracking
    }
}

struct Token {
    text: String,
    space_before: bool,
}

fn tokens(text: &str) -> Vec<Token> {
    let mut out = Vec::new();
    let mut run = String::new();
    let mut pending_space = false;
    for ch in text.chars() {
        if ch.is_whitespace() {
            if !run.is_empty() {
                out.push(Token {
                    text: std::mem::take(&mut run),
                    space_before: pending_space,
                });
            }
            pending_space = true;
        } else if is_cjk(ch) {
            if !run.is_empty() {
                out.push(Token {
                    text: std::mem::take(&mut run),
                    space_before: pending_space,
                });
                pending_space = false;
            }
            out.push(Token {
                text: ch.to_string(),
                space_before: pending_space,
            });
            pending_space = false;
        } else {
            run.push(ch);
        }
    }
    if !run.is_empty() {
        out.push(Token {
            text: run,
            space_before: pending_space,
        });
    }
    out
}

fn wrap(
    font: &Font,
    text: &str,
    px: f32,
    tracking: f32,
    max_w: f32,
    break_words: bool,
) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let mut line = String::new();
    for token in tokens(text) {
        let parts = if break_words {
            break_word(font, &token.text, px, tracking, max_w)
        } else {
            vec![token.text.clone()]
        };
        for (i, part) in parts.iter().enumerate() {
            let sep = if line.is_empty() {
                ""
            } else if i == 0 && token.space_before {
                " "
            } else {
                ""
            };
            let joined = format!("{line}{sep}{part}");
            if line.is_empty() || line_width(font, &joined, px, tracking) <= max_w {
                line = joined;
            } else {
                lines.push(std::mem::take(&mut line));
                line = part.clone();
            }
        }
    }
    if !line.is_empty() {
        lines.push(line);
    }
    lines
}

fn break_word(font: &Font, word: &str, px: f32, tracking: f32, max_w: f32) -> Vec<String> {
    if line_width(font, word, px, tracking) <= max_w {
        return vec![word.to_string()];
    }
    let mut parts = Vec::new();
    let mut part = String::new();
    for ch in word.chars() {
        part.push(ch);
        if line_width(font, &part, px, tracking) > max_w && part.chars().count() > 1 {
            part.pop();
            parts.push(std::mem::take(&mut part));
            part.push(ch);
        }
    }
    if !part.is_empty() {
        parts.push(part);
    }
    parts
}

#[allow(clippy::too_many_arguments)]
fn stamp(dst: &mut [u8], dst_w: u32, dst_h: u32, x: i32, y: i32, cov: &[u8], gw: u32, gh: u32) {
    for gy in 0..gh {
        let dy = y + gy as i32;
        if dy < 0 || dy >= dst_h as i32 {
            continue;
        }
        for gx in 0..gw {
            let dx = x + gx as i32;
            if dx < 0 || dx >= dst_w as i32 {
                continue;
            }
            let i = (dy as u32 * dst_w + dx as u32) as usize;
            dst[i] = dst[i].max(cov[(gy * gw + gx) as usize]);
        }
    }
}

fn blend(px: &mut [u8], a: u32, colour: [u8; 3]) {
    let under = px[3] as u32 * (255 - a) / 255;
    let out = a + under;
    for c in 0..3 {
        px[c] = ((colour[c] as u32 * a + px[c] as u32 * under + out / 2) / out) as u8;
    }
    px[3] = out as u8;
}

#[cfg(all(test, feature = "lang-zh-cn"))]
mod tests {
    use super::*;

    #[test]
    fn the_bundled_face_covers_every_interface_hanzi() {
        for source in [include_str!("lang.rs"), include_str!("clock.rs")] {
            for ch in source.chars().filter(|ch| is_cjk(*ch)) {
                assert!(
                    has_glyph(zh_cn_font(), ch),
                    "the bundled Chinese face is missing {ch:?}"
                );
            }
        }
    }
}
