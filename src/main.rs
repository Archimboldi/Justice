#[macro_use] extern crate native_windows_gui as nwg;
use nwg::{Ui, Event, dispatch_events};
use nwg::constants::CheckState;
use winapi::um::memoryapi::{ReadProcessMemory, WriteProcessMemory};
use winapi::um::winuser::{FindWindowW, GetWindowThreadProcessId};
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::handleapi::CloseHandle;
use winapi::ctypes::{wchar_t, c_ulong, c_void};
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::ptr::{null, null_mut};

#[derive(Debug, Clone, Hash)]
pub enum Id {
    MainWindow,
    Btn,
    Timer,
    Timer1,
    Timer2,
    CheckBox1,
    CheckBox2,
    CheckBox3,
    CheckBox4,
    CheckBox5,
    CheckBox6,
    CheckBox7,
    CheckBox8,
    CheckBox9,
    CheckBox10,
    Run,
    UpdateA,
    Write1,
    Write2,
    Write3,
    Font,
    Phw,
    P,
    Pt,
    Ptr
}

use Id::*;

nwg_template! (
    head: setup_ui<Id>,
    controls: [
        (MainWindow, nwg_window!( title="Justice"; position=(70, 290); size=(230, 247) )),
        (Btn, nwg_button!(
            parent=MainWindow; 
            text="Да"; 
            position=(140, 25); 
            size=(50, 24); 
            font=Some(Font)
        )),
        (CheckBox1, nwg_checkbox!(
            parent=MainWindow; 
            text="子弹"; 
            position=(24, 70); 
            size=(56, 25); 
            font=Some(Font);
            checkstate=CheckState::Checked
        )),
        (CheckBox2, nwg_checkbox!(
            parent=MainWindow; 
            text="后坐"; 
            position=(94, 70); 
            size=(56, 25); 
            font=Some(Font);
            checkstate=CheckState::Checked
        )),
        (CheckBox3, nwg_checkbox!(
            parent=MainWindow; 
            text="生存"; 
            position=(164, 70); 
            size=(56, 25); 
            font=Some(Font);
            checkstate=CheckState::Checked
        )),
        (CheckBox4, nwg_checkbox!(
            parent=MainWindow; 
            text="钱币"; 
            position=(24, 110); 
            size=(56, 25); 
            font=Some(Font);
            checkstate=CheckState::Checked
        )),
        (CheckBox5, nwg_checkbox!(
            parent=MainWindow; 
            text="盔甲"; 
            position=(94, 110); 
            size=(56, 25); 
            font=Some(Font);
            checkstate=CheckState::Checked
        )),
        (CheckBox6, nwg_checkbox!(
            parent=MainWindow; 
            text="手雷"; 
            position=(164, 110); 
            size=(56, 25); 
            font=Some(Font);
            checkstate=CheckState::Checked
        )),
        (CheckBox7, nwg_checkbox!(
            parent=MainWindow; 
            text="高闪"; 
            position=(24, 150); 
            size=(56, 25); 
            font=Some(Font);
            checkstate=CheckState::Checked
        )),
        (CheckBox8, nwg_checkbox!(
            parent=MainWindow; 
            text="烟雾"; 
            position=(94, 150); 
            size=(56, 25); 
            font=Some(Font);
            checkstate=CheckState::Checked
        )),
        (CheckBox9, nwg_checkbox!(
            parent=MainWindow; 
            text="刀"; 
            position=(164, 150); 
            size=(56, 25); 
            font=Some(Font);
            checkstate=CheckState::Checked
        )),
        (CheckBox10, nwg_checkbox!(
            parent=MainWindow; 
            text="军火库"; 
            position=(24, 190); 
            size=(78, 25); 
            font=Some(Font);
            checkstate=CheckState::Checked
        )),
        (Timer, nwg_timer!(interval=140)),
        (Timer1, nwg_timer!(interval=10000)),
        (Timer2, nwg_timer!(interval=24))
    ];
    events: [
        (Btn, Run, Event::Click, |app,_,_,_|{
            let (mut phw, mut timer, mut timer1, mut timer2, btn) = nwg_get_mut!(app; [
                (Phw, usize),
                (Timer, nwg::Timer),
                (Timer1, nwg::Timer),
                (Timer2, nwg::Timer),
                (Btn, nwg::Button)
            ]);
            if timer.running() {
                timer.stop();
                timer1.stop();
                timer2.stop();
                btn.set_text("Да");
                unsafe {
                    CloseHandle(**phw as i32 as *mut c_void);
                }
            }else {
                let oss: Vec<wchar_t> = OsStr::new("Counter-Strike").encode_wide().chain(once(0)).collect();
                let mut pid: c_ulong = 0;
                unsafe {
                    let hw = FindWindowW(null(), oss.as_ptr());
                    GetWindowThreadProcessId(hw, &mut pid);
                    **phw = OpenProcess(2035711, 0, pid) as usize;
                }
                timer.start();
                timer1.start();
                timer2.start();
                btn.set_text("Ура!");
            }
        }),
        (Timer, UpdateA, Event::Tick, |app,_,_,_|{
            let (phw ,mut p, mut pt, mut ptr, c9, c10) = nwg_get_mut!(app; [
                (Phw, usize),
                (P, usize),
                (Pt, usize),
                (Ptr, usize),
                (CheckBox9, nwg::CheckBox),
                (CheckBox10, nwg::CheckBox)
            ]);
            let ba = 0x025069BC as *const c_void;
            let mut val: i32 = 0;
            let left: f32 = -1.0;
            let right: f32 = -0.001000000047;
            let shop = 1;
            unsafe {
                ReadProcessMemory(**phw as *mut c_void, ba, &mut val as *mut _ as *mut c_void, 4, null_mut());
                **p = val as usize;
                ReadProcessMemory(**phw as *mut c_void, ((**p as i32)+0x7C) as *const c_void, &mut val as *mut _ as *mut c_void, 4, null_mut());
                **pt = val as usize;
                ReadProcessMemory(**phw as *mut c_void, ((**pt as i32)+0x5EC) as *const c_void, &mut val as *mut _ as *mut c_void, 4, null_mut());
                **ptr = val as usize;
                if c9.get_checkstate() == CheckState::Checked {
                    ReadProcessMemory(**phw as *mut c_void, ((**ptr as i32)+0xCC) as *const c_void, &mut val as *mut _ as *mut c_void, 4, null_mut());
                    if val == -1 {
                        WriteProcessMemory(**phw as *mut c_void, ((**ptr as i32) +0xB8) as *mut c_void, &left as *const _ as *const c_void, 4, null_mut());
                        WriteProcessMemory(**phw as *mut c_void, ((**ptr as i32) +0xBC) as *mut c_void, &right as *const _ as *const c_void, 4, null_mut());
                    }
                }
                if c10.get_checkstate() == CheckState::Checked {
                    WriteProcessMemory(**phw as *mut c_void, ((**pt as i32) +0x3C0) as *mut c_void, &shop as *const _ as *const c_void, 4, null_mut());
                }
            }
        }),
        (Timer1, Write1, Event::Tick, |app,_,_,_|{
            let (phw, ptr, pt, p, c1, c4, c5, c6, c7, c8) = nwg_get!(app; [
                (Phw, usize),
                (Ptr, usize),
                (Pt, usize),
                (P, usize),
                (CheckBox1, nwg::CheckBox),
                (CheckBox4, nwg::CheckBox),
                (CheckBox5, nwg::CheckBox),
                (CheckBox6, nwg::CheckBox),
                (CheckBox7, nwg::CheckBox),
                (CheckBox8, nwg::CheckBox)
            ]);
            let bullet = &128 as *const _ as *const c_void;
            let money = &16000 as *const _ as *const c_void;
            let armor: f32 = 999.0;
            let mut val = 0;
            unsafe {
                if c1.get_checkstate() == CheckState::Checked {
                    ReadProcessMemory(**phw as *mut c_void, ((**ptr as i32)+0xCC) as *const c_void, &mut val as *mut _ as *mut c_void, 4, null_mut());
                    if val != -1 {
                        WriteProcessMemory(**phw as *mut c_void, ((**ptr as i32) +0xCC) as *mut c_void, bullet, 4, null_mut());
                    }
                }
                if c4.get_checkstate() == CheckState::Checked {
                    WriteProcessMemory(**phw as *mut c_void, ((**pt as i32) +0x1CC) as *mut c_void, money, 4, null_mut());
                }
                if c5.get_checkstate() == CheckState::Checked {
                    WriteProcessMemory(**phw as *mut c_void, ((**p as i32) +0x23C) as *mut c_void, &armor as *const _ as *const c_void, 4, null_mut());
                }
                if c6.get_checkstate() == CheckState::Checked {
                    ReadProcessMemory(**phw as *mut c_void, ((**pt as i32)+0x628) as *const c_void, &mut val as *mut _ as *mut c_void, 4, null_mut());
                    if val == 1 {
                        WriteProcessMemory(**phw as *mut c_void, ((**pt as i32) +0x628) as *mut c_void, bullet, 4, null_mut());
                    }
                }
                if c7.get_checkstate() == CheckState::Checked {
                    ReadProcessMemory(**phw as *mut c_void, ((**pt as i32)+0x624) as *const c_void, &mut val as *mut _ as *mut c_void, 4, null_mut());
                    if val == 1 {
                        WriteProcessMemory(**phw as *mut c_void, ((**pt as i32) +0x624) as *mut c_void, bullet, 4, null_mut());
                    }
                }
                if c8.get_checkstate() == CheckState::Checked {
                    ReadProcessMemory(**phw as *mut c_void, ((**pt as i32)+0x62C) as *const c_void, &mut val as *mut _ as *mut c_void, 4, null_mut());
                    if val == 1 {
                        WriteProcessMemory(**phw as *mut c_void, ((**pt as i32) +0x62C) as *mut c_void, bullet, 4, null_mut());
                    }
                }
            }
        }),
        (Timer2, Write2, Event::Tick, |app,_,_,_|{
            let (phw, p, ptr, c2, c3) = nwg_get!(app; [
                (Phw, usize),
                (P, usize),
                (Ptr, usize),
                (CheckBox2, nwg::CheckBox),
                (CheckBox3, nwg::CheckBox)
            ]);
            let recoil = &0 as *const _ as *const c_void;
            let blood: f32 = 255.0;
            unsafe {
                if c2.get_checkstate() == CheckState::Checked {
                    WriteProcessMemory(**phw as *mut c_void, ((**ptr as i32)+0x100) as *mut c_void, recoil, 4, null_mut());
                }
                if c3.get_checkstate() == CheckState::Checked {
                    WriteProcessMemory(**phw as *mut c_void, ((**p as i32)+0x1E0) as *mut c_void, &blood as *const _ as *const c_void, 4, null_mut());
                }
            }
        })
    ];
    resources: [(Font, nwg_font!( family="Microsoft YaHei"; size=21 ))];
    values: [
        (Phw, 0usize),
        (P, 0usize),
        (Pt, 0usize),
        (Ptr, 0usize)
    ]
);

pub fn main() {
    let app: Ui<Id> = Ui::new().expect("Failed to initalize the Ui");
    if let Err(e) = setup_ui(&app) {
        panic!("Commit failed: {:?}", e);
    }
    dispatch_events();
}