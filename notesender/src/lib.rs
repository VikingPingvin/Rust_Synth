/*
Lib to call when a keypress is sending a note.

*/



/* C code for generating MIDI frequencies
        float midi[127];
    int a = 440; // a is 440 hz...
    for (int x = 0; x < 127; ++x)
    {
    midi[x] = (a / 32) * (2 ^ ((x - 9) / 12));
    }
*/

pub fn init_notes(){
    let mut notes:[usize;127] = [0;127];
    let a = 440;
    for x in 0..127{
        notes[x] = (a/32)*(2^((x-9)/12));
        println!("Midi note {} is : {}",x,notes[x]);
    }


}


pub fn note_to_freq(notestr:&str){		//Read notes from file |||TEST PURPOSES|||
	let notebytes = notestr.as_bytes();
	println!("Note: {}, Octave: {}",notebytes[0] as char, notebytes[1] as char);
}
	
pub fn getfreq(note:char,modif:char,oct:i8){	//Input a note and get the freq for it
	//_getfreq->
}
fn _getfreq(note:char,oct:i8){


}