use windows::{
    core::*,
    Win32::Graphics::Gdi::*,
    Win32::{Foundation::*, System::LibraryLoader::GetModuleHandleW, UI::WindowsAndMessaging::*},
};

fn main() {
    unsafe {
        let hwnd = create_main_window().expect("Expected a window");
        let _ = ShowWindow(hwnd, SW_SHOWNORMAL);
        main_loop();
    }
}

unsafe fn create_main_window() -> Result<HWND> {
    let class_name = w!("MyClass");
    let window_name = w!("MyWindow");
    let window_style = WS_OVERLAPPEDWINDOW;

    let hinstance = GetModuleHandleW(Option::None).expect("Need hinstance");
    let wc = WNDCLASSW {
        hCursor: LoadCursorW(None, IDC_ARROW)?,
        hInstance: hinstance.into(),
        lpszClassName: class_name,
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(wndproc),

        ..Default::default()
    };
    let atom = RegisterClassW(&wc);
    debug_assert!(atom != 0);

    let hwnd = CreateWindowExW(
        WINDOW_EX_STYLE::default(),
        class_name,
        window_name,
        window_style,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        Option::None,
        Option::None,
        hinstance,
        Option::None,
    )?;
    Ok(hwnd)
}

unsafe fn main_loop() {
    let wmsgfiltermin = 0;
    let wmsgfiltermax = 0;
    let mut msg = MSG::default();
    loop {
        let message_available: bool =
            GetMessageW(&mut msg, HWND::default(), wmsgfiltermin, wmsgfiltermax).into();
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

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_PAINT => {
                let _ = ValidateRect(window, None);
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
