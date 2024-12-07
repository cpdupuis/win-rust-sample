#![windows_subsystem = "windows"]

use windows::{
    core::{w, PCWSTR, HRESULT},
    Win32::{
        Foundation::{COLORREF, HINSTANCE, HWND, LPARAM, LRESULT, WPARAM},
        Graphics::Gdi::*,
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::*,
    },
};

fn main() {
    let class_name = w!("MyClass");

    unsafe {
        let hinstance = register_class(&class_name).expect("Needs to register a class");
        let hwnd = create_main_window(&hinstance, &class_name).expect("Need to create a window");
        let _ = ShowWindow(hwnd, SW_SHOWNORMAL);
        main_loop();
    }
}

unsafe fn register_class(class_name: &PCWSTR) -> windows::core::Result<HINSTANCE> {
    let cursor = LoadCursorW(None, IDC_ARROW)?;
    let hmodule = GetModuleHandleW(Option::None)?;
    let hinstance: HINSTANCE = hmodule.into();
    let wc = WNDCLASSW {
        hCursor: cursor,
        hInstance: hinstance,
        lpszClassName: *class_name,
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(wndproc),

        ..Default::default()
    };
    let atom = RegisterClassW(&wc);
    if atom == 0 {
        return Err(windows_result::Error::from_hresult(HRESULT(0x80004005u32 /* E_FAIL */ as i32)));
    }
    return Ok(hinstance);
}

// Creates the main window for the application
// Returns HWND for the window on success, or an HRESULT error code
unsafe fn create_main_window(hinstance: &HINSTANCE, class_name: &PCWSTR) -> windows::core::Result<HWND> {
    let window_name = w!("MyWindow");
    let window_style = WS_OVERLAPPEDWINDOW;

    let hwnd = CreateWindowExW(
        WINDOW_EX_STYLE::default(),
        *class_name,
        window_name,
        window_style,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        Option::None,
        Option::None,
        *hinstance,
        Option::None,
    )?;
    return Ok(hwnd);
}

// This implements the application's message-handling loop, as documented here: https://learn.microsoft.com/en-us/windows/win32/learnwin32/window-messages#the-message-loop
unsafe fn main_loop() {
    let wmsgfiltermin = 0;
    let wmsgfiltermax = 0;
    let mut msg = MSG::default();
    let null_hwnd = HWND::default();
    loop {
        let message_available: bool =
            GetMessageW(&mut msg, null_hwnd, wmsgfiltermin, wmsgfiltermax).into();
        // WM_QUIT isn't a real message that gets put on the message queue. So we check for it even
        // if there are no messages on the queue.
        if msg.message == WM_QUIT {
            break;
        }
        if message_available {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

// Callback invoked by Windows to handle window events
extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_PAINT => {
                let mut ps = PAINTSTRUCT::default();
                let hdc = BeginPaint(window, &mut ps);
                // Clear the screen to blue. Just because.
                let colorref = COLORREF(0x00FF0000);
                let hbrush = CreateSolidBrush(colorref);
                FillRect(hdc, &ps.rcPaint, hbrush);
                let _ = EndPaint(window, &mut ps);
                LRESULT(0)
            }
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcW(window, message, wparam, lparam),
        }
    }
}
