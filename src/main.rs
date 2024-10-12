use std::ops::{Index, Range};
use rand::*;
use anyhow::*;
use rngs::ThreadRng;


fn test_win_case(_mode: i8,_rng_engine: &mut ThreadRng) -> Result<i32> {
    //Three doors
    let mut _doors_unopened = vec![0,1,2];
    
    //Pick a winner and pick another door
    let _range = 0.._doors_unopened.len() as i32;
    let _door_winner = _rng_engine.gen_range::<i32, Range<i32>>(_range.clone());
    let _door_pick = _rng_engine.gen_range::<i32, Range<i32>>(_range);

    //switch mode
    if _mode == 1 {
        let mut _doors_to_open: Vec<i32> = _doors_unopened.clone();

        //we can not open the winner or the pick
        _doors_to_open.retain(
            |&door| 
                door != _door_winner 
                && 
                door != _door_pick
        );
        let _door_opened = _doors_to_open[0];

        _doors_unopened.retain(
            |&door| 
                door != _door_opened 
                && 
                door != _door_pick
        );
        let _door_last = _doors_unopened[0];

        if  _door_last == _door_winner {
            return Ok(1);
        } else {
            return Ok(0);
        }

    }

    //stick mode
    if _mode == 0 {
        if _door_pick == _door_winner {
            return Ok(1);
        } else {
            return Ok(0);
        }
    }

    return Err(anyhow!("Failed to determine win or loss"));
}

fn main() -> Result<()> {
    let mut RANDOM_GEN = rand::thread_rng();
    let mut switch_total = 0;
    let mut stick_total = 0;
    let mut rand_total = 0;

    for _ in 0..1000000 {
        stick_total += test_win_case(0, &mut RANDOM_GEN).unwrap();
        switch_total += test_win_case(1, &mut RANDOM_GEN).unwrap();
        let rnd_i = RANDOM_GEN.gen_range::<i8, Range<i8>>(0..2);
        rand_total += test_win_case(rnd_i, &mut RANDOM_GEN).unwrap();
    }
    
    println!("Total wins for sticking: {:?}", stick_total);
    println!("Total wins for switching: {:?}", switch_total);
    println!("Total wins for random mode: {:?}", rand_total);

    return Ok(());
}
