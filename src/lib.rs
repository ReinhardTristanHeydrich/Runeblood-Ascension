/*✦════════════════════════════════════════════════════════════ ✦ ═════════════════════════════════════════════════════════════✦*/
 /*✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ .  Compile-Time ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . */
/*✦════════════════════════════════════════════════════════════ ✦ ═════════════════════════════════════════════════════════════✦*/
/*✦════════════════════════ Imports ════════════════════════✦*/
/*✦───────── From-Softwaer-RS ─────────✦*/
use eldenring::{
    cs::{CSTaskGroupIndex, CSTaskImp, WorldChrMan},
    fd4::FD4TaskData,
};
use eldenring_util::{
    program::Program,
    singleton::get_instance,
    system::wait_for_system_init,
    task::CSTaskImpExt,
};

/*✦───────── Thread ─────────✦*/
use winapi::um::libloaderapi::DisableThreadLibraryCalls;
use winapi::um::processthreadsapi::CreateThread;
use winapi::shared::minwindef::{LPVOID, DWORD};
use winapi::um::handleapi::CloseHandle;
use std::ptr;

/*✦───────── Others ─────────✦*/
use std::time::Duration;

/*✦═════════════════════════ Consts ═════════════════════════✦*/
const GAME_INJECTION:u32 = 1;

/*✦════════════════════════════════════════════════════════════ ✦ ═════════════════════════════════════════════════════════════✦*/
 /*✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . ⁺ ✦ Code ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . ⁺ . ✦*/
/*✦════════════════════════════════════════════════════════════ ✦ ═════════════════════════════════════════════════════════════✦*/
#[allow(unsafe_op_in_unsafe_fn)]
unsafe extern "system" fn main(_: LPVOID) -> DWORD {
    /*✦═════════════════════════════════════════════✦══════════════════════════════════════════════✦*/
    wait_for_system_init(&Program::current(), Duration::MAX).expect("Could not await system init.");
    /*✦─────────────────────────────────────────────────────────────────────────────────────────────✦*/
    let task_system = get_instance::<CSTaskImp>().unwrap().unwrap();

    task_system.run_recurring(|_task_data: &FD4TaskData| {
        let Some(main_player) = get_instance::<WorldChrMan>()
            .expect("No reflection data for WorldChrMan")
            .and_then(|wcm| wcm.main_player.as_mut()) else {
            return;
        };

        if !main_player.player_game_data.rune_arc_active {
            main_player.player_game_data.rune_arc_active = true;
        }
    }, CSTaskGroupIndex::FrameBegin);
    /*✦═════════════════════════════════════════════✦══════════════════════════════════════════════✦*/
    0
}



/*✦════════════════════════════════════════════════════════════ ✦ ═════════════════════════════════════════════════════════════✦*/
/*✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . ⁺ Dll-Main ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . 　⁺ 　 . ✦ . ⁺ . ✦*/
/*✦════════════════════════════════════════════════════════════ ✦ ═════════════════════════════════════════════════════════════✦*/
#[unsafe(no_mangle)]
#[allow(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn DllMain(hmodule: usize, reason: u32) -> bool {
    if reason != GAME_INJECTION {return true;}
    DisableThreadLibraryCalls(hmodule as *mut _);

    let thread = CreateThread(ptr::null_mut(), 0, Some(main), ptr::null_mut(), 0, ptr::null_mut());

    if !thread.is_null() {CloseHandle(thread);}



    true
}
