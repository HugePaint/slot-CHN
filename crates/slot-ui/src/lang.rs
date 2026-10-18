//! User-facing interface text.
//!
//! English remains the default for upstream-compatible host builds. `lang-zh-cn` produces the
//! Simplified Chinese device interface without carrying both sets of strings in one binary.

#[cfg(not(feature = "lang-zh-cn"))]
mod words {
    pub const SET_CLOCK: &str = "set the clock";
    pub const PRESS_A_BUTTON: &str = "Press a button";
    pub const BACK: &str = "Back";
    pub const CHANGE: &str = "Change";
    pub const OPEN: &str = "Open";
    pub const RESUME: &str = "Resume";
    pub const RESTART: &str = "Restart";
    pub const SET: &str = "Set";
    pub const CLEAR: &str = "Clear";
    pub const RESET_ALL: &str = "Reset All";
    pub const CANCEL: &str = "Cancel";
    pub const SWAP: &str = "Swap";
    pub const CHOOSE: &str = "Choose";
    pub const MODE: &str = "Mode";
    pub const LINK: &str = "Link";
    pub const OK: &str = "OK";
    pub const END_LINK: &str = "End Link";
    pub const NOT_NOW: &str = "Not now";
    pub const REPAIR: &str = "Repair";
    pub const DELETE: &str = "Delete";
    pub const LOAD: &str = "Load";

    pub const POWER_RESTART: &str = "Restart";
    pub const POWER_OFF: &str = "Power Off";
    pub const POWER_REPAIR: &str = "Repair SD Card";
    pub const SHUTDOWN_RESTART: &str = "Restarting";
    pub const SHUTDOWN_POWER_OFF: &str = "Powering Down";
    pub const SHUTDOWN_REPAIR: &str = "Repairing SD Card";

    pub const LINK_HOST: &str = "Host";
    pub const LINK_JOIN: &str = "Join";
    pub const LINKED: &str = "Linked";
    pub const LINK_RADIO: &str = "Bringing the radio up";
    pub const LINK_WAITING: &str = "Looking for the other player";
    pub const LINK_FAIL_RADIO: &str = "The radio did not come up";
    pub const LINK_FAIL_NOBODY: &str = "Nobody arrived";
    pub const LINK_FAIL_PEER: &str = "The other player vanished";
    pub const LINK_FAIL_CANCELLED: &str = "Cancelled";

    pub const RECENTS: &str = "Recents";
    pub const FAVORITES: &str = "Favorites";
    pub const UNDO_SAVE: &str = "undo save";
    pub const UNDO_LOAD: &str = "undo load";

    pub const FAST_FORWARD: &str = "Fast Forward";
    pub const FAST_FORWARD_SOUND: &str = "Fast Forward Sound";
    pub const SCREEN: &str = "Screen";
    pub const RUMBLE: &str = "Rumble";
    pub const LEFT_STICK: &str = "Left Stick as D-Pad";
    pub const DATE_TIME: &str = "Date & Time";
    pub const ABOUT: &str = "About";
    pub const SHADER: &str = "Shader";
    pub const COLOR_CORRECTION: &str = "Color Correction";
    pub const GAMEPLAY: &str = "Gameplay";
    pub const EJECT_SAVE: &str = "Auto Save on Eject";
    pub const TURBO: &str = "Turbo Buttons";
    pub const REWIND: &str = "Rewind";
    pub const PALETTES: &str = "Palettes";
    pub const THEME: &str = "Theme";
    pub const WALLPAPER: &str = "Wallpaper";
    pub const GAME_TITLES: &str = "Game Titles";
    pub const WALLPAPER_TINT: &str = "Wallpaper Tint";
    pub const STATES: &str = "States";
    pub const SETTINGS: &str = "Settings";
    pub const EXIT_GAME: &str = "Exit Game";
    pub const GENERAL: &str = "General";
    pub const LID_POWER_OFF: &str = "Lid Power Off";
    pub const CONTROLS: &str = "Controls";
    pub const SHELF: &str = "Shelf";
    pub const IN_GAME: &str = "In-Game";
    pub const SHORTCUTS: &str = "Shortcuts";
    pub const PALETTE: &str = "Palette";
    pub const BEZEL: &str = "Bezel";

    pub const ON: &str = "On";
    pub const OFF: &str = "Off";
    pub const GRID: &str = "Grid";
    pub const DOT: &str = "Dot";
    pub const DARK: &str = "Dark";
    pub const LIGHT: &str = "Light";
    pub const SHOW: &str = "Show";
    pub const HIDE: &str = "Hide";
    pub const MIN_1: &str = "1 min";
    pub const MIN_3: &str = "3 min";
    pub const MIN_5: &str = "5 min";
    pub const MIN_10: &str = "10 min";
    pub const MIN_15: &str = "15 min";
    pub const MIN_30: &str = "30 min";

    pub const TOAST_STATE_SAVED: &str = "State Saved";
    pub const TOAST_STATE_LOADED: &str = "State Loaded";
    pub const TOAST_NEEDS_GPSP: &str = "Please switch to gpSP";
    pub const TOAST_NO_LINK: &str = "No link support";
    pub const TOAST_LINK_ENDED: &str = "Link ended";
    pub const TOAST_PEER_ENDED: &str = "Link was ended";
    pub const TOAST_BIOS_MISMATCH: &str = "BIOS does not match";
    pub const TOAST_COLOUR_ON: &str = "Correction On";
    pub const TOAST_COLOUR_OFF: &str = "Correction Off";
    pub const TOAST_SHADER_OFF: &str = "Shader: Off";
    pub const TOAST_SHADER_LCD3X: &str = "Shader: LCD3x";
    pub const TOAST_SHADER_GRID: &str = "Shader: Grid";
    pub const TOAST_SHADER_DOT: &str = "Shader: Dot";
    pub const TOAST_SHADER_SIMPLETEX: &str = "Shader: Simpletex";
    pub const TOAST_CARD_UNWRITABLE: &str = "Can't write to SD card";
    pub const TOAST_CARD_REPAIRED: &str = "SD card repaired";
    pub const TOAST_CARD_REPAIR_FAILED: &str = "Card repair failed";
    pub const DETAIL_NOT_SAVED: &str = "Progress is not being saved";
    pub const DETAIL_REPAIR: &str = "Press SELECT + START to repair";

    pub const CACHING_LABELS: &str = "Caching New Labels";
    pub const SINGLE_BUTTON: &str = "Supports single button and MENU+button.";
    pub const SELECT_BUTTON: &str = "Supports single button, SELECT+button and MENU+button.";
    pub const REPAIR_QUESTION: &str = "Repair the SD card?";
    pub const REPAIR_RESTARTS: &str = "slot restarts when it is done.";
    pub const RESET: &str = "Reset";
    pub const RESET_CONTROLS: &str = "Reset all controls?";
    pub const RESET_CONTROLS_DETAIL: &str = "Every button goes back to its default.";
    pub const RESET_SHORTCUTS: &str = "Reset all shortcuts?";
    pub const RESET_SHORTCUTS_DETAIL: &str = "Every shortcut goes back to its default.";
    pub const RESET_SHELF: &str = "Reset all shelf keys?";
    pub const RESET_SHELF_DETAIL: &str = "Every shelf key goes back to its default.";

    pub const NONE: &str = "None";
    pub const BINDING_NONE: &str = "NONE";
    pub const RANDOM: &str = "Random";
    pub const DEFAULT: &str = "Default";
    pub const DEFAULT_BLUR: &str = "Default Blur";
    pub const JUST_NOW: &str = "just now";
    pub const MIN_AGO: &str = "min ago";
    pub const HR_AGO: &str = "hr ago";
}

#[cfg(feature = "lang-zh-cn")]
mod words {
    pub const SET_CLOCK: &str = "设置时钟";
    pub const PRESS_A_BUTTON: &str = "请按一个按键";
    pub const BACK: &str = "返回";
    pub const CHANGE: &str = "更改";
    pub const OPEN: &str = "打开";
    pub const RESUME: &str = "继续";
    pub const RESTART: &str = "重启";
    pub const SET: &str = "设置";
    pub const CLEAR: &str = "清除";
    pub const RESET_ALL: &str = "全部重置";
    pub const CANCEL: &str = "取消";
    pub const SWAP: &str = "切换";
    pub const CHOOSE: &str = "选择";
    pub const MODE: &str = "模式";
    pub const LINK: &str = "联机";
    pub const OK: &str = "确定";
    pub const END_LINK: &str = "结束联机";
    pub const NOT_NOW: &str = "暂不";
    pub const REPAIR: &str = "修复";
    pub const DELETE: &str = "删除";
    pub const LOAD: &str = "读取";

    pub const POWER_RESTART: &str = "重启";
    pub const POWER_OFF: &str = "关机";
    pub const POWER_REPAIR: &str = "修复存储卡";
    pub const SHUTDOWN_RESTART: &str = "正在重启";
    pub const SHUTDOWN_POWER_OFF: &str = "正在关机";
    pub const SHUTDOWN_REPAIR: &str = "正在修复存储卡";

    pub const LINK_HOST: &str = "主机";
    pub const LINK_JOIN: &str = "加入";
    pub const LINKED: &str = "已连接";
    pub const LINK_RADIO: &str = "正在启动无线连接";
    pub const LINK_WAITING: &str = "正在寻找其他玩家";
    pub const LINK_FAIL_RADIO: &str = "无线连接未能启动";
    pub const LINK_FAIL_NOBODY: &str = "无人加入";
    pub const LINK_FAIL_PEER: &str = "对方已断开";
    pub const LINK_FAIL_CANCELLED: &str = "已取消";

    pub const RECENTS: &str = "最近使用";
    pub const FAVORITES: &str = "收藏";
    pub const UNDO_SAVE: &str = "撤销存档";
    pub const UNDO_LOAD: &str = "撤销读取";

    pub const FAST_FORWARD: &str = "快进";
    pub const FAST_FORWARD_SOUND: &str = "快进声音";
    pub const SCREEN: &str = "显示";
    pub const RUMBLE: &str = "震动";
    pub const LEFT_STICK: &str = "左摇杆映射十字键";
    pub const DATE_TIME: &str = "日期和时间";
    pub const ABOUT: &str = "关于";
    pub const SHADER: &str = "滤镜";
    pub const COLOR_CORRECTION: &str = "色彩校正";
    pub const GAMEPLAY: &str = "游戏";
    pub const EJECT_SAVE: &str = "弹出时自动保存";
    pub const TURBO: &str = "连发按键";
    pub const REWIND: &str = "倒带";
    pub const PALETTES: &str = "调色板";
    pub const THEME: &str = "主题";
    pub const WALLPAPER: &str = "壁纸";
    pub const GAME_TITLES: &str = "游戏标题";
    pub const WALLPAPER_TINT: &str = "壁纸调色";
    pub const STATES: &str = "即时存档";
    pub const SETTINGS: &str = "设置";
    pub const EXIT_GAME: &str = "退出游戏";
    pub const GENERAL: &str = "通用";
    pub const LID_POWER_OFF: &str = "合盖关机";
    pub const CONTROLS: &str = "控制";
    pub const SHELF: &str = "游戏架";
    pub const IN_GAME: &str = "游戏内";
    pub const SHORTCUTS: &str = "快捷键";
    pub const PALETTE: &str = "调色板";
    pub const BEZEL: &str = "边框";

    pub const ON: &str = "开";
    pub const OFF: &str = "关";
    pub const GRID: &str = "网格";
    pub const DOT: &str = "点阵";
    pub const DARK: &str = "深色";
    pub const LIGHT: &str = "浅色";
    pub const SHOW: &str = "显示";
    pub const HIDE: &str = "隐藏";
    pub const MIN_1: &str = "1 分钟";
    pub const MIN_3: &str = "3 分钟";
    pub const MIN_5: &str = "5 分钟";
    pub const MIN_10: &str = "10 分钟";
    pub const MIN_15: &str = "15 分钟";
    pub const MIN_30: &str = "30 分钟";

    pub const TOAST_STATE_SAVED: &str = "存档已保存";
    pub const TOAST_STATE_LOADED: &str = "存档已读取";
    pub const TOAST_NEEDS_GPSP: &str = "请切换至 gpSP";
    pub const TOAST_NO_LINK: &str = "不支持联机";
    pub const TOAST_LINK_ENDED: &str = "联机已结束";
    pub const TOAST_PEER_ENDED: &str = "对方结束了联机";
    pub const TOAST_BIOS_MISMATCH: &str = "BIOS 不匹配";
    pub const TOAST_COLOUR_ON: &str = "色彩校正：开";
    pub const TOAST_COLOUR_OFF: &str = "色彩校正：关";
    pub const TOAST_SHADER_OFF: &str = "滤镜：关闭";
    pub const TOAST_SHADER_LCD3X: &str = "滤镜：LCD3x";
    pub const TOAST_SHADER_GRID: &str = "滤镜：网格";
    pub const TOAST_SHADER_DOT: &str = "滤镜：点阵";
    pub const TOAST_SHADER_SIMPLETEX: &str = "滤镜：Simpletex";
    pub const TOAST_CARD_UNWRITABLE: &str = "无法写入存储卡";
    pub const TOAST_CARD_REPAIRED: &str = "存储卡已修复";
    pub const TOAST_CARD_REPAIR_FAILED: &str = "存储卡修复失败";
    pub const DETAIL_NOT_SAVED: &str = "当前进度不会保存";
    pub const DETAIL_REPAIR: &str = "按 SELECT + START 修复";

    pub const CACHING_LABELS: &str = "正在缓存新标签";
    pub const SINGLE_BUTTON: &str = "支持单键及 MENU+按键组合。";
    pub const SELECT_BUTTON: &str = "支持单键、SELECT+按键及 MENU+按键组合。";
    pub const REPAIR_QUESTION: &str = "是否修复存储卡？";
    pub const REPAIR_RESTARTS: &str = "完成后 slot 将重新启动。";
    pub const RESET: &str = "重置";
    pub const RESET_CONTROLS: &str = "重置全部游戏按键？";
    pub const RESET_CONTROLS_DETAIL: &str = "所有游戏按键将恢复默认设置。";
    pub const RESET_SHORTCUTS: &str = "重置全部快捷键？";
    pub const RESET_SHORTCUTS_DETAIL: &str = "所有快捷键将恢复默认设置。";
    pub const RESET_SHELF: &str = "重置全部游戏架按键？";
    pub const RESET_SHELF_DETAIL: &str = "所有游戏架按键将恢复默认设置。";

    pub const NONE: &str = "无";
    pub const BINDING_NONE: &str = "未绑定";
    pub const RANDOM: &str = "随机";
    pub const DEFAULT: &str = "默认";
    pub const DEFAULT_BLUR: &str = "默认模糊";
    pub const JUST_NOW: &str = "刚刚";
    pub const MIN_AGO: &str = "分钟前";
    pub const HR_AGO: &str = "小时前";
}

pub use words::*;

pub fn game_button(button: slot_store::GameButton) -> &'static str {
    use slot_store::GameButton::*;
    #[cfg(not(feature = "lang-zh-cn"))]
    {
        match button {
            Up => "Up",
            Down => "Down",
            Left => "Left",
            Right => "Right",
            A => "A Button",
            B => "B Button",
            Select => "Select",
            Start => "Start",
            L => "L Button",
            R => "R Button",
        }
    }
    #[cfg(feature = "lang-zh-cn")]
    {
        match button {
            Up => "上",
            Down => "下",
            Left => "左",
            Right => "右",
            A => "A 键",
            B => "B 键",
            Select => "SELECT 键",
            Start => "START 键",
            L => "L 键",
            R => "R 键",
        }
    }
}

pub fn shortcut(shortcut: slot_store::Shortcut) -> &'static str {
    use slot_store::Shortcut::*;
    #[cfg(not(feature = "lang-zh-cn"))]
    {
        match shortcut {
            SaveState => "Save State",
            LoadState => "Load State",
            BrightnessUp => "Brightness Up",
            BrightnessDown => "Brightness Down",
            BlueLightDown => "Blue Light Down",
            BlueLightUp => "Blue Light Up",
            ColorCorrection => "Color Correction",
            PaletteBack => "Palette Back",
            PaletteNext => "Palette Next",
            ShaderNext => "Shader Next",
            ShaderBack => "Shader Back",
            FastForward => "Fast Forward",
            Rewind => "Rewind",
        }
    }
    #[cfg(feature = "lang-zh-cn")]
    {
        match shortcut {
            SaveState => "保存即时存档",
            LoadState => "读取即时存档",
            BrightnessUp => "提高亮度",
            BrightnessDown => "降低亮度",
            BlueLightDown => "降低护眼强度",
            BlueLightUp => "提高护眼强度",
            ColorCorrection => "切换色彩校正",
            PaletteBack => "上一个调色板",
            PaletteNext => "下一个调色板",
            ShaderNext => "下一个滤镜",
            ShaderBack => "上一个滤镜",
            FastForward => "快进",
            Rewind => "倒带",
        }
    }
}

pub fn carousel(action: slot_store::CarouselAction) -> &'static str {
    use slot_store::CarouselAction::*;
    #[cfg(not(feature = "lang-zh-cn"))]
    {
        match action {
            PreviousShelf => "Previous Shelf",
            NextShelf => "Next Shelf",
            Favorite => "Favorite",
        }
    }
    #[cfg(feature = "lang-zh-cn")]
    {
        match action {
            PreviousShelf => "上一个游戏架",
            NextShelf => "下一个游戏架",
            Favorite => "收藏",
        }
    }
}

pub fn wallpaper(pick: &slot_store::WallpaperPick) -> String {
    use slot_store::WallpaperPick;
    match pick {
        WallpaperPick::None => NONE.to_string(),
        WallpaperPick::Random => RANDOM.to_string(),
        WallpaperPick::Default => DEFAULT.to_string(),
        WallpaperPick::Blur => DEFAULT_BLUR.to_string(),
        WallpaperPick::File(name) => file_stem(name),
    }
}

pub fn bezel(pick: &slot_store::BezelPick) -> String {
    use slot_store::BezelPick;
    match pick {
        BezelPick::None => NONE.to_string(),
        BezelPick::Default => DEFAULT.to_string(),
        BezelPick::Ags102 => "AGS-102".to_string(),
        BezelPick::File(name) => file_stem(name),
    }
}

fn file_stem(name: &str) -> String {
    name.rsplit_once('.')
        .map(|(stem, _)| stem)
        .unwrap_or(name)
        .to_string()
}
