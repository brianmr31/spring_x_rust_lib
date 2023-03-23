use jni::JNIEnv;
use jni::objects::{JClass, JString, JObjectArray, JObject, ReleaseMode };
use jni::sys::{ jstring, jlong, jint, jobject, jintArray};
use calamine::{Reader, open_workbook_auto};
use calamine::DataType::{ Float };
use chrono::{ DateTime, NaiveDateTime, NaiveDate, TimeZone, Utc};

static mut ROW_TOTAL: jlong = 0;
static mut HEADER_TOTAL: jlong = 0;
static mut COLLECTED_ITERATOR: Vec<String> = Vec::new();
static mut NAME_HEADERS: String = String::new();
static mut TYPE_HEADERS: String = String::new();

const DATATYPE_INT: &'static str = "Int";
const DATATYPE_FLOAT: &'static str = "Float";
const DATATYPE_STRING: &'static str = "String";
const DATATYPE_DATETIME: &'static str = "DateTime";
const DATATYPE_BOOL: &'static str = "Bool";
const DATATYPE_ERROR: &'static str = "Error";
const DATATYPE_EMPTY: &'static str = "Empty";

struct Counter {
    count: i32
}

impl Counter {
    pub fn new(count: i32) -> Counter {
        Counter {
            count: count
        }
    }
	pub fn get(&mut self) -> i32 {
		return self.count;
	}
	pub fn increment(&mut self) {
	    self.count = self.count + 1;
   }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_corp_valhalla_web_lib_HelloWorld_getTestArray2
	(mut _env: JNIEnv, _class: JClass) {
	let mut counter = Counter::new(99);
	println!("{}", counter.get());
}

fn get_header_total() -> jlong {
	unsafe {
		return HEADER_TOTAL;
	}
}

fn add_header_total() {
	unsafe {
    	return { let tmp = HEADER_TOTAL; HEADER_TOTAL += 1;  };
	}
}

fn clear_header_total(){
	unsafe {
    	HEADER_TOTAL = 0 ;
	}
}

fn get_type_headers(_env: JNIEnv) -> jstring {
	unsafe {
		let tmp = &TYPE_HEADERS;
		return **_env.new_string::<String>( tmp.to_string() ).expect("Couldn't create java string!");
	}
}

fn set_type_headers( tmp: String ){
	unsafe {
		TYPE_HEADERS = tmp;
	}
}

fn get_name_headers(_env: JNIEnv) -> jstring {
	unsafe {
		let tmp = &NAME_HEADERS;
		return **_env.new_string::<String>( tmp.to_string() ).expect("Couldn't create java string!");
	}
}

fn set_name_headers( tmp: String ){
	unsafe {
		NAME_HEADERS = tmp;
	}
}

fn clear_vec(){
	unsafe {
		COLLECTED_ITERATOR.clear();
	}
}

fn set_vec_add( tmp: String ) {
	unsafe {
		COLLECTED_ITERATOR.push( tmp );
	}
}
fn get_vec_by_id(_env: JNIEnv, id: usize ) -> jstring {
	unsafe {
		let tmp = COLLECTED_ITERATOR.get(id);
		if tmp.is_none() {
			return **_env.new_string("".to_string()).expect("Couldn't create java string!");
		}
		return **_env.new_string( tmp.unwrap() ).expect("Couldn't create java string!");
	}
}

fn get_row_total() -> jlong {
	unsafe {
		return ROW_TOTAL;
	}
}

fn get_row_total_add() {
	unsafe {
    	return { let tmp = ROW_TOTAL; ROW_TOTAL += 1 };
    }
}

fn get_row_total_zero() {
	unsafe {
    	ROW_TOTAL = 0;
    }
}


#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_corp_valhalla_web_lib_HelloWorld_getTestArray
	(mut _env: JNIEnv, _class: JClass, tests: JObject ) -> jintArray {
	let test = _env.get_list( &tests ).unwrap();
	// let elem = test.get(&mut _env, 0).unwrap();
	// println!( "{:?}", elem.unwrap() );

	// let test2 = jni::objects::JString::from( elem.unwrap() );
	// let tmp = _env.get_string(&test2);
	// println!( "{}", tmp.unwrap().to_str().unwrap()  );
	let mut iterator = test.iter(&mut _env).unwrap();

	while let Some(obj) = iterator.next(&mut _env).unwrap() {
		let tmp2 = JString::from( obj );
		let tmp = _env.get_string( &tmp2 ).unwrap() ;
		println!( "{}", &tmp.to_str().unwrap()  );
	}
	let mut size = test.size( &mut _env ).unwrap();

	let mut test2 = _env.new_int_array( size ).expect("Couldn't create java int array!");

	// unsafe {
		// let mut b = _env.get_array_elements(&test2, ReleaseMode::CopyBack );
		// let mut c = b.expect("REASON").get(0).unwrap();
		// println!("{}",  c);
	// }
	// test2.from_raw();
	// println!("{}", size);

	let mut test4:[i32; 1] = [0];
	_env.set_int_array_region(&test2, 2, &mut [22]);
	_env.get_int_array_region(&test2, 2, &mut test4).unwrap();

	println!("test3 {:?}", test4);

	_env.set_int_array_region(&test2, 0, &mut [9]);
	_env.set_int_array_region(&test2, 1, &mut [10]);
	_env.set_int_array_region(&test2, 2, &mut [11]);
	return **test2;
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_corp_valhalla_web_lib_HelloWorld_getTypeHeaders
	(_env: JNIEnv, _class: JClass ) -> jstring {
	get_type_headers(_env)
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_corp_valhalla_web_lib_HelloWorld_getNameHeaders
	(mut _env: JNIEnv, _class: JClass ) -> jstring {
	get_name_headers(_env)
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_corp_valhalla_web_lib_HelloWorld_check
	(mut _env: JNIEnv, _class: JClass, input: JString) -> jstring {
	let tmp = _env.get_string(&input);
	if let Err(ref _err) = tmp {
		_env.throw("input is null!");
	}
    let output = _env.new_string::<String>( tmp.unwrap().into() ).expect("Error create response!");
    **output
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_corp_valhalla_web_lib_HelloWorld_getHeaderTotal
	(_env: JNIEnv, _class: JClass ) -> jlong {
	get_header_total()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_corp_valhalla_web_lib_HelloWorld_getRowTotal
	(_env: JNIEnv, _class: JClass ) -> jlong {
	get_row_total()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_corp_valhalla_web_lib_HelloWorld_close
	(_env: JNIEnv, _class: JClass ) {
	get_row_total_zero();
	clear_vec();
	set_type_headers( String::new() );
	set_name_headers( String::new() );
	clear_header_total();
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_corp_valhalla_web_lib_HelloWorld_getRow
	(mut _env: JNIEnv, _class: JClass, id: jint ) -> jstring {
    let i = usize::try_from( id ).unwrap();
	get_vec_by_id(_env, i )
}


#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_corp_valhalla_web_lib_HelloWorld_loadExecl
	(mut _env: JNIEnv, _class: JClass, file: JString, sheet: JString, sperator: JString) {
    let mut speratorRust: String = "".to_string();
    let speratorTmp = _env.get_string(&sperator);
	if let Err(_err) = speratorTmp{
		_env.throw("Sperator is null!");
	} else {
	    speratorRust = speratorTmp.unwrap().into();
	}

    let mut fileRust: String = "".to_string();
    let fileTmp = _env.get_string(&file);
	if let Err(_err) = fileTmp{
		_env.throw("File is null!");
	} else {
	    fileRust = fileTmp.unwrap().into();
	}

    let mut sheetRust: String = "".to_string();
    let sheetTmp = _env.get_string(&sheet);
	if let Err(_err) = sheetTmp{
		_env.throw("Sheet is null!");
	} else {
	    sheetRust = sheetTmp.unwrap().into();
	}

	let excel = open_workbook_auto( fileRust );
	if let Err(ref _err) = excel{
		_env.throw("Cannot open file!");
	} else {
		if let Some(Ok(r)) = excel.unwrap().worksheet_range( &sheetRust ) {
			let mut tmp;
        	let mut type_headers;
        	let mut name_headers;
        	for row in r.rows() {
        		if get_row_total() == 0 {
        			type_headers = String::new();
        			name_headers = String::new();
        			for i in 0..row.len() {
        				if let calamine::DataType::Float(ref _data) = row[i] {
            				type_headers.push_str( &DATATYPE_FLOAT );
            			} else if let calamine::DataType::String(ref _data) = row[i] {
            				type_headers.push_str( &DATATYPE_STRING );
            			} else if let calamine::DataType::DateTime(ref _data) = row[i] {
            				type_headers.push_str( &DATATYPE_DATETIME );
            			} else if let calamine::DataType::Int(ref _data) = row[i] {
            				type_headers.push_str( &DATATYPE_INT );
            			} else if let calamine::DataType::Bool(ref _data) = row[i] {
            				type_headers.push_str( &DATATYPE_BOOL );
            			} else if let calamine::DataType::Error(ref _data) = row[i] {
            				type_headers.push_str( &DATATYPE_ERROR );
            			} else {
            				type_headers.push_str( &DATATYPE_EMPTY );
            			}
            			name_headers.push_str( &row[i].to_string() );

            			if ( row.len()-1) != i {
                			type_headers.push_str( &speratorRust );
	            			name_headers.push_str( &speratorRust );
                		}
        			}

            		set_type_headers( type_headers );
        			set_name_headers( name_headers );
            		add_header_total();
        		} else {
        			tmp = String::new();
            		for i in 0..row.len() {
            			if let calamine::DataType::Float(ref _data) = row[i] {
		                	tmp.push_str( &_data.to_string() );
            			} else if let calamine::DataType::String(ref _data) = row[i] {
		                	tmp.push_str( &_data.to_string() );
            			} else if let calamine::DataType::DateTime(ref _data) = row[i] {
            				let date_tmp : NaiveDateTime = NaiveDate::from_ymd(1900, 01, 01).and_hms(00, 00, 00).checked_add_days( chrono::naive::Days::new(*_data as u64) ).unwrap();
    						let datetime = DateTime::<Utc>::from_utc( date_tmp , Utc);
		                	tmp.push_str( &datetime.to_string() );
            			} else if let calamine::DataType::Int(ref _data) = row[i] {
		                	tmp.push_str( &row[i].to_string() );
            			} else if let calamine::DataType::Bool(ref _data) = row[i] {
		                	tmp.push_str( &row[i].to_string() );
            			} else if let calamine::DataType::Error(ref _data) = row[i] {
		                	tmp.push_str( &row[i].to_string() );
            			} else {
		                	tmp.push_str( &row[i].to_string() );
            			}

                		if (row.len()-1) != i {
                    		tmp.push_str( &speratorRust );
                		}
            		}
        			set_vec_add( tmp );
        		}
        		get_row_total_add();
        	}
    	} else {
    		_env.throw("Sheet not found!");
    	}
	}
}
