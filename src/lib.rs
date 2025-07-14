use std::time::Duration;

use eldenring::{
    cs::{CSTaskGroupIndex, CSTaskImp, WorldChrMan},
    fd4::FD4TaskData,
};
use eldenring_util::{
    program::Program, singleton::get_instance, system::wait_for_system_init, task::CSTaskImpExt,
};

#[link(name = "kernel32")]
unsafe extern "C" {
    unsafe fn DisableThreadLibraryCalls(hmodule: usize) -> bool;
}


#[unsafe(no_mangle)]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn DllMain(hmodule: usize, reason: u32) -> bool {
    const GAME_INJECTION:u32 = 1;
    if reason != GAME_INJECTION {return true;}


    DisableThreadLibraryCalls(hmodule);
    

    std::thread::spawn(|| {
        wait_for_system_init(&Program::current(), Duration::MAX)
            .expect("Could not await system init.");

        let task_system = get_instance::<CSTaskImp>().unwrap().unwrap();

        task_system.run_recurring(
            |_task_data: &FD4TaskData| {
                let Some(main_player) = get_instance::<WorldChrMan>()
                    .expect("No reflection data for WorldChrMan")
                    .and_then(|wcm| wcm.main_player.as_mut())
                else {
                    return;
                };

                main_player.player_game_data.rune_arc_active = true;
            },
            CSTaskGroupIndex::ChrIns_PostPhysics,
        );
    });

    true
}
