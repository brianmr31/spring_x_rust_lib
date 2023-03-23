package corp.valhalla.web.controller;

import java.text.SimpleDateFormat;
import java.util.ArrayList;
import java.util.Date;
import java.util.List;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestMethod;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.ResponseBody;
import org.springframework.web.bind.annotation.RestController;

import corp.valhalla.web.lib.HelloWorld;
import corp.valhalla.web.lib.PosgressRust;

@RestController
public class ExcelReadController {
	
	private HelloWorld test = new HelloWorld();
	private PosgressRust posgressRust = new PosgressRust();
	
	public String escapeMetaCharacters(String inputString){
	    final String[] metaCharacters = {"'"};
//	    boolean isRun = false;
	    
	    for (int i = 0 ; i < metaCharacters.length ; i++){
	        if(inputString.contains(metaCharacters[i])){
	            inputString = inputString.replaceAll(metaCharacters[i],"\\\\\"");
//	            isRun = true;
	        }
	    }
//	    
//	    if( isRun ) {
//        	System.out.println( inputString );
//	    }
	    return inputString;
	}
	
	@RequestMapping(value = "/excel/init", method=RequestMethod.GET)
	public @ResponseBody String init() {
		HelloWorld test = new HelloWorld();
		PosgressRust posgressRust = new PosgressRust();
		
		SimpleDateFormat format = new SimpleDateFormat("MM-dd-yyyy hh;mm;ss");
//		HelloWorld test = new HelloWorld();
		System.out.println( format.format(new Date()) +" LOAD ");
		
		List<String> supplierNames1 = new ArrayList<String>();
		supplierNames1.add("Testtss");
		supplierNames1.add("Testtss2");
		supplierNames1.add("Testtss3");
		
		int[] wkwkw = test.getTestArray(supplierNames1);
//		test.getTestArray(supplierNames1);
		System.out.println(wkwkw.length);
		System.out.println(wkwkw[0]);

		try {
//			test.loadExecl("/home/brian/Downloads/rust/TRI_By_ID_1987_2014.xlsx","TRI_By_ID_1987_2014","|");
			test.loadExecl("/home/brian/eclipse-workspace/rust/sampledata2.xlsx","PolicyData","|");
//			test.loadExecl("/home/brian/Downloads/rust/sampledatainsuranceSS.xlsx","PolicyData",null);
		} catch (Exception e) {
			e.printStackTrace();
		}
		
		System.out.println( format.format(new Date()) +" total "+ this.test.getRowTotal());
		System.out.println( format.format(new Date()) +" Type "+ this.test.getTypeHeaders());
		System.out.println( format.format(new Date()) +" name "+ this.test.getNameHeaders());

		String tmp = "";
		for( int i = 0; i < test.getRowTotal(); i++ ) {
//			tmp = this.escapeMetaCharacters( test.getRow(i).replaceAll("\n", ""));
			tmp = test.getRow(i);
			if( !tmp.equals("") ) {
				posgressRust.addList( tmp );
			}
//			tests.add( new TestEntity( test.getRow(i)));
		}
		System.out.println( format.format(new Date()) +" Save ALL "+ this.test.getRowTotal());
//		testRepository.saveAll( tests );
		posgressRust.runBatch();
		System.out.println( format.format(new Date()) +" END ");
		posgressRust.close();
//		test.close();
		return test.check("Init");
	}
	
	@RequestMapping(value = "/excel/read", method=RequestMethod.GET)
	public @ResponseBody String getById(@RequestParam(value = "key") int id) {
		return this.test.getRow(id);
	}
	
	@RequestMapping(value = "/excel/headers", method=RequestMethod.GET)
	public @ResponseBody String getHeaders() {
		return this.test.getTypeHeaders();
	}
	
	@RequestMapping(value = "/excel/header/total", method=RequestMethod.GET)
	public @ResponseBody long getHeaderTotal() {
		return this.test.getHeaderTotal();
	}
	
	@RequestMapping(value = "/excel/closed", method=RequestMethod.GET)
	public @ResponseBody String closed() {
		this.test.close();
		return "close";
	}
}
