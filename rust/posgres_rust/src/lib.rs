use jni::JNIEnv;
use jni::objects::{JClass, JString };
use postgres::{Client, NoTls, Error};

static mut LIST_INSERT_POSGRES: Vec<String> = Vec::new();

fn clear_vec(){
	unsafe {
		LIST_INSERT_POSGRES.clear();
	}
}

fn get_vec() -> Vec<String> {
	unsafe {
	    let tmp = &LIST_INSERT_POSGRES;
	    // println!(" {} ", LIST_INSERT_POSGRES.len() );
		return tmp.to_vec();
	}
}

fn set_vec_add( tmp: String ) {
	unsafe {
		LIST_INSERT_POSGRES.push( tmp );
	}
}

fn insert_data_bulk() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://spring:spring@localhost/spring", NoTls)?;
    let sperator = ",";                                    
    let open  = "('";
    let close = "')";
    let places = &get_vec();    
    let mut tmp = String::new();
    println!( " {} ", &places.len() );
    for i in 0..places.len() {
        tmp.push_str( &open );
        tmp.push_str( &places.get(i).unwrap() );
        tmp.push_str( &close );
        if (places.len()-1) != i {
           tmp.push_str(&sperator); 
        }
    }
    let query = format!( "INSERT INTO public.person ( name) VALUES {} ;", tmp );
    // println!( " {} ", &query );    
    let re = client.execute( &query, &[]).unwrap();
    
    // let mut transaction = client.transaction()?;
    // transaction.execute( &query, &[])?;

    // transaction.commit()?;
    
    println!( " done {} ", re );    
    client.close();
    Ok(())
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_corp_valhalla_web_lib_PosgressRust_close
	(_env: JNIEnv, _class: JClass ) {
	clear_vec();
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_corp_valhalla_web_lib_PosgressRust_runBatch
	(_env: JNIEnv, _class: JClass ) {
	insert_data_bulk();
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_corp_valhalla_web_lib_PosgressRust_addList
	(_env: JNIEnv, _class: JClass, input: JString ) {
    let mut inputRust: String = "".to_string();
    let inputTmp = _env.get_string(input);    
	if let Err(_err) = inputTmp{
		_env.throw("Input is null!");
	} else {
	    inputRust = inputTmp.unwrap().into();	
	}
    set_vec_add( inputRust.replace('\'', &'\"'.to_string()) );
}