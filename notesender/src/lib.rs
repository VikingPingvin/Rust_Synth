/*
Lib to call when a keypress is sending a note.

*/





pub fn init_notes(){
    let mut notes:[f64;127] = [0f64;127];
    let a = 440.0;
    for x in 0..127{
        notes[x] = (a/32.0)*2f64.powf((x as f64-9.0)/12.0);
        println!("Midi note {} is : {}",x,notes[x]);
    }
}

/*
pub fn getfreq(notenum:&i32) -> f64{
    return notes[notenum]
}
*/

pub fn note_to_freq(notestr:&str){		//Read notes from file |||TEST PURPOSES|||
	let notebytes = notestr.as_bytes();
	println!("Note: {}, Octave: {}",notebytes[0] as char, notebytes[1] as char);
}
	
