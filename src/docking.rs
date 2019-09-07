use crate::{sys, Ui};
use bitflags::bitflags;

bitflags! {
    /// Dock node flags
    #[repr(transparent)]
    pub struct DockNodeFlags: u32 {
        const NONE = sys::ImGuiDockNodeFlags_None;
        const KEEP_ALIVE_ONLY = sys::ImGuiDockNodeFlags_KeepAliveOnly;
        const NO_SPLIT = sys::ImGuiDockNodeFlags_NoSplit;
        const NO_DOCKING_IN_CENTRAL_NODE = sys::ImGuiDockNodeFlags_NoDockingInCentralNode;
        const PASSTHRU_DOCKSPACE = sys::ImGuiDockNodeFlags_PassthruCentralNode;
        const NO_RESIZE = sys::ImGuiDockNodeFlags_NoResize;
        const AUTOHIDE_TABBAR = sys::ImGuiDockNodeFlags_AutoHideTabBar;
    }
}

bitflags! {
    /// Viewport flags
    #[repr(transparent)]
    pub struct ViewportFlags: u32 {
        const NONE = sys::ImGuiViewportFlags_None;
        const NO_DECORATION = sys::ImGuiViewportFlags_NoDecoration;
        const NO_TASKBAR_ICON = sys::ImGuiViewportFlags_NoTaskBarIcon;
        const NO_FOCUS_ON_APPEARING = sys::ImGuiViewportFlags_NoFocusOnAppearing;
        const NO_FOCUS_ON_CLICK = sys::ImGuiViewportFlags_NoFocusOnClick;
        const NO_INPUTS = sys::ImGuiViewportFlags_NoInputs;
        const NO_RENDERER_CLEAR = sys::ImGuiViewportFlags_NoRendererClear;
        const TOP_MOST = sys::ImGuiViewportFlags_TopMost;
        const MINIMIZED = sys::ImGuiViewportFlags_Minimized;
        const NO_AUTO_MERGE = sys::ImGuiViewportFlags_NoAutoMerge;
        const CAN_HOST_OTHER_WINDOWS = sys::ImGuiViewportFlags_CanHostOtherWindows;
    }
}

#[derive(Copy, Clone)]
pub struct Viewport {
    ptr: *mut sys::ImGuiViewport,
}
impl Viewport {
    fn from_ptr(ptr: *mut sys::ImGuiViewport) -> Self {
        Self {
            ptr,
        }
    }
}

impl<'ui> Ui<'ui> {
    pub fn get_window_viewport() -> Viewport {
        Viewport::from_ptr(unsafe { sys::igGetWindowViewport() } )
    }

    pub fn dockspace_over_viewport(viewport: Viewport, flags: DockNodeFlags, ) -> sys::ImGuiID {
        unimplemented!()
       /*unsafe {
            igDockSpaceOverViewport(
            viewport: *mut ImGuiViewport,
            flags: ImGuiDockNodeFlags,
            window_class: *const ImGuiWindowClass,
        ) -> ImGuiID;
        */
    }

    //TODO:
    pub fn is_window_docked(&self) -> bool {
        unsafe { sys::igIsWindowDocked() }
    }

    //TODO:
    pub fn get_window_dock_id(&self) -> sys::ImGuiID {
        unsafe { sys::igGetWindowDockID() }
    }
}