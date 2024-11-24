use windows::{
    core::{w, HRESULT},
    Win32::{
        Foundation::{COLORREF, HINSTANCE, HWND, LPARAM, LRESULT, WPARAM},
        Graphics::Gdi::*,
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::*,
    },
};

fn main() {
    unsafe {
        let hwnd = create_main_window().expect("Expected a window");
        let _ = ShowWindow(hwnd, SW_SHOWNORMAL);
        main_loop();
    }
}

// Creates the main window for the application
// Returns HWND for the window on success, or an HRESULT error code
unsafe fn create_main_window() -> Result<HWND, HRESULT> {
    let class_name = w!("MyClass");
    let window_name = w!("MyWindow");
    let window_style = WS_OVERLAPPEDWINDOW;

    let hmodule = GetModuleHandleW(Option::None).expect("Need module handle");
    let hinstance: HINSTANCE = hmodule.into();
    let wc = WNDCLASSW {
        hCursor: LoadCursorW(None, IDC_ARROW)?,
        hInstance: hinstance,
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
