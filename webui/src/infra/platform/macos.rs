use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use std::ffi::c_void;

use objc2::rc::Id;
use objc2::runtime::AnyObject;

use objc2_app_kit::{NSWindow, NSWindowStyleMask, NSWindowTitleVisibility};
use objc2_foundation::MainThreadMarker;

pub fn patch_nswindow_for_titlebar(cc: &eframe::CreationContext<'_>) {
    // 拿到窗口句柄（eframe 底层是 winit）
    let Ok(handle) = cc.window_handle() else { return; };

    let ns_view_ptr = match handle.as_raw() {
        RawWindowHandle::AppKit(h) => h.ns_view.as_ptr(),
        _ => return,
    };

    if ns_view_ptr.is_null() {
        return;
    }

    unsafe {
        let mtm = MainThreadMarker::new().expect("Must be on main thread on macOS");

        // ns_view_ptr 是 NSView*
        let ns_view: *mut AnyObject = ns_view_ptr as *mut AnyObject;

        // [ns_view window] -> NSWindow*
        let ns_window: *mut AnyObject = msg_send![ns_view, window];
        if ns_window.is_null() {
            return;
        }

        // 把它包装成 objc2 的 NSWindow 引用
        let ns_window: Id<NSWindow> = Id::retain(ns_window.cast()).unwrap();

        // 1) 让标题栏透明、隐藏标题文字
        ns_window.setTitleVisibility(NSWindowTitleVisibility::Hidden);
        ns_window.setTitlebarAppearsTransparent(true);

        // 2) 让内容区域延伸到标题栏（保留红黄绿按钮）
        let mut mask = ns_window.styleMask();
        mask |= NSWindowStyleMask::FullSizeContentView;
        ns_window.setStyleMask(mask);

        // 可选：允许在标题栏区域拖动窗口（如果你上面要放交互控件，可能需要更细的处理）
        ns_window.setMovableByWindowBackground(false);

        // 强制刷新布局（有时需要）
        ns_window.displayIfNeeded();
    }
}

// 兼容 objc2 的 msg_send（objc2 推荐用其宏）
use objc2::msg_send;
