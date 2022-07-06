use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Gdi::ValidateRect,
    Win32::System::LibraryLoader::GetModuleHandleA, Win32::UI::WindowsAndMessaging::*,
};

const ID_BTN_PLUS: isize = 10;
const ID_BTN_MINUS: isize = 11;
const ID_BTN_MULTIPLY: isize = 12;
const ID_BTN_DIVIDE: isize = 13;
const ID_BTN_C: isize = 14;
const ID_BTN_EQ: isize = 15;
const ID_ST_RESULT: isize = 16;

fn create_button(phwnd:HWND, inst: HINSTANCE, txt: &str, 
    x: i32, y: i32, w: i32, h: i32, hmenu: HMENU) -> HWND {
    unsafe {
        let hwnd = CreateWindowExA(
        WS_EX_PALETTEWINDOW,
        "button",
        txt,
        WS_CHILD | WS_VISIBLE,
        x,
        y,
        w,
        h,
        phwnd,
        hmenu,
        inst,
        std::ptr::null_mut(),
    );
    SetWindowTextW(hwnd, txt);
    hwnd
    }
}
fn main() -> Result<()> {
    unsafe {
        let instance = GetModuleHandleA(None);
        debug_assert!(instance.0 != 0);

        let window_class = "window";

        let wc = WNDCLASSA {
            hCursor: LoadCursorW(None, IDC_ARROW),
            hInstance: instance,
            lpszClassName: PSTR(b"window\0".as_ptr() as _),

            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            ..Default::default()
        };

        let atom = RegisterClassA(&wc);
        debug_assert!(atom != 0);

        let hwnd = CreateWindowExA(
            Default::default(),
            window_class,
            "Calculator",
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            215,
            350,
            None,
            None,
            instance,
            std::ptr::null_mut(),
        );

        for i in 1..10 {
            let x = 15 + ((i-1) % 3)*60;
            let y = ((i-1)/3)*40 + 60;
            create_button(hwnd, instance, &i.to_string(), x, y, 50, 30, HMENU(i as isize));
        }
        create_button(hwnd, instance, "0", 15, 180, 170, 30, HMENU(0));
        create_button(hwnd, instance, "+", 15, 220, 50, 30, HMENU(ID_BTN_PLUS));
        create_button(hwnd, instance, "-", 75, 220, 50, 30, HMENU(ID_BTN_MINUS));
        create_button(hwnd, instance, "*", 135, 220, 50, 30, HMENU(ID_BTN_MULTIPLY));
        create_button(hwnd, instance, "/", 15, 260, 50, 30, HMENU(ID_BTN_DIVIDE));
        create_button(hwnd, instance, "C", 75, 260, 50, 30, HMENU(ID_BTN_C));
        create_button(hwnd, instance, "=", 135, 260, 50, 30, HMENU(ID_BTN_EQ));

        let hwndr = CreateWindowExA(
            WS_EX_OVERLAPPEDWINDOW,
            "static",
            "0",
            WS_CHILD | WS_VISIBLE,
            15,
            10,
            170,
            30,
            hwnd,
            HMENU(ID_ST_RESULT),
            instance,
            std::ptr::null_mut(),
        );
        SetWindowTextW(hwndr, "0");

        let mut message = MSG::default();

        while GetMessageA(&mut message, HWND(0), 0, 0).into() {
            DispatchMessageA(&mut message);
        }

        Ok(())
    }
}

use std::mem;
static mut OP1: i32 = 0;
static mut OP2: i32 = 0;
static mut OP: isize = 0;
extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message as u32 {
            WM_PAINT => {
                ValidateRect(window, std::ptr::null());
                LRESULT(0)
            }
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            }
            WM_COMMAND => {
                let wp:isize = mem::transmute(wparam);
                let disp = GetDlgItem(window, ID_ST_RESULT as i32);
                if wp >= 0 && wp <= 9 {
                    OP2 = OP2*10 + wp as i32;
                } else {
                    if OP == 0 || OP == ID_BTN_C { OP = wp;}
                    match OP {
                        ID_BTN_PLUS => {
                            OP1 = OP1 + OP2;
                        }
                        ID_BTN_MINUS => {
                            OP1 = OP1 - OP2;
                        }
                        ID_BTN_MULTIPLY => {
                            OP1 = OP1 * OP2;
                        }
                        ID_BTN_DIVIDE => {
                            OP1 = OP1 / OP2;
                        }
                        _ => {}
                    }
                    match wp {
                        ID_BTN_C => {
                            OP1 = 0;
                        }
                        _ => {
                            println!("{}", wp);
                        }
                    }
                    SetWindowTextW(disp, &*OP1.to_string());
                    OP2 = 0;
                    OP = wp;
                }
                LRESULT(0)
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}

// as only allows safe casting, 
// and will for example reject an attempt to cast four bytes into a u32: