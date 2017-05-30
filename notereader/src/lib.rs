/* NoteReader.rs

By: VikingPingvin  ---  sch1458@gmail.com
*/



pub fn note_to_freq(notestr:&str){		//Read notes from file |||TEST PURPOSES|||
	let notebytes = notestr.as_bytes();
	println!("Note: {}, Octave: {}",notebytes[0] as char, notebytes[1] as char);
}
	
pub fn getfreq(note:char,mod:char,oct:i8){	//Input a note and get the freq for it
	//_getfreq->
}
fn _getfreq(note:char,oct:i8){


}
