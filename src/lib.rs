use std::time::Duration;

use eldenring::{
    cs::{CSTaskGroupIndex, CSTaskImp, WorldChrMan},
    fd4::FD4TaskData,
};

use eldenring_util::{
    program::Program, singleton::get_instance, system::wait_for_system_init, task::CSTaskImpExt,
};

#[unsafe(no_mangle)]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn DllMain(_hmodule: usize, reason: u32) -> bool {
    //If we aren't attached to the game, returns.
    if reason != 1 {return true;}

    //Create a new Thread.
    std::thread::spawn(|| {
        // Wait for game to boot up.
        wait_for_system_init(&Program::current(), Duration::MAX).expect("Could not await system init.");
        
        //Get Game's Task Runner.
        let cs_task = get_instance::<CSTaskImp>().unwrap().unwrap();
        /*
        Register a new task which will be executed every frame during ChrIns_PostPhysics.
        This will 
        */
        cs_task.run_recurring(
            |_task_data: &FD4TaskData| {
                // Obtain a mutable reference to Main_Player.
                let Some(main_player) = get_instance::<WorldChrMan>()
                    .expect("No reflection data for WorldChrMan")
                    .and_then(|wcm| wcm.main_player.as_mut())
                else {return;};

                /*Set the variable which defines if Great Rune is activated and put that to true.*/
                main_player.player_game_data.rune_arc_active = true;
            },
            CSTaskGroupIndex::ChrIns_PostPhysics,
        );
    });

    true
}